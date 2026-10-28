//! The C API's async calls (`qar_*_async`) as [`PendingCall`]: onboard and rejoin, the
//! co-process invite, the target-app invite and kill, and the render sender's begin-frame.
//!
//! The header's rules for them, and what this module builds on them:
//!
//! - **Exactly once.** A start call that succeeds fires its result callback exactly once, on a
//!   library thread: with the outcome, or with `QAR_STATUS_TIMEOUT` when its cancel token is
//!   cancelled, times out or is destroyed first, or when its owner (the runtime, session or
//!   render sender) is destroyed first, whose destroy then returns only after the callback has
//!   returned. A start call that fails never calls back. A cancel arriving just as the call
//!   finishes may still deliver the outcome. The callback records the outcome under a mutex and
//!   runs the caller's `wake`; the Rust value (not `Send`) is built on the thread that polls.
//! - **`user_state`.** The library holds one `Arc` reference to the call's state from the start
//!   call until the result callback reclaims it. Progress callbacks borrow that reference: none
//!   fires after the result callback.
//! - **What the start reads.** Whatever the init points to stays in the call's state until the
//!   callback (`keep`), wherever the header does not say it is copied during the start.
//! - **Abandoned calls.** Dropping a [`PendingCall`] before its outcome is taken cancels the call
//!   and hands it to its owner. A handle the callback still delivers (a session, an invite, a
//!   frame info) is destroyed by the owner while it lives; the owner's destroy waits for the
//!   callback, so nothing is left after it.
//! - **Runtimes before the exactly-once rule.** A runtime that predates the rule (0.9.0 and
//!   earlier) may never call back for a cancelled begin-frame. A render sender then waits at most
//!   [`crate::CANCELLED_CREATION_WAIT`] for a dropped frame's callback before refusing new frames.

use std::any::Any;
use std::ffi::{CStr, c_char, c_void};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::rc::Rc;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

use qaros_sys::{self as sys, QarApi};

use crate::callbacks::Progress;
use crate::cancel::{CancelHandle, CancelTokenInner};
use crate::error::{Error, Result, check};
use crate::library::LibraryInner;
use crate::runtime::Onboarded;
use crate::types::ActionSeverity;

/// Destroys a handle a callback delivered that nobody takes, while its owner lives.
pub(crate) type Destroy = fn(&QarApi, usize);

/// What a result callback delivered: the status, the handle it hands over (0 for none) and the id
/// it reports (the onboarding id, or a target app's peer id). Addresses are kept as integers so
/// the state is `Send`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RawOutcome {
    pub(crate) result: sys::QarResult,
    pub(crate) handle: usize,
    pub(crate) id: [u8; 16],
}

#[derive(Debug)]
enum CallState {
    Pending,
    Delivered(RawOutcome),
    /// The outcome went to its owner (a polled value, or a release).
    Taken,
}

/// The call's state. One `Arc` reference belongs to the library from the start call until the
/// result callback reclaims it.
pub(crate) struct CallShared {
    state: Mutex<CallState>,
    /// Notified once the result callback has recorded its outcome.
    called_back: Condvar,
    progress: Option<Sender<Progress>>,
    wake: Box<dyn Fn() + Send + Sync>,
    /// What the init points to, never read by Rust after the start.
    _keep: Option<Box<dyn Any + Send + Sync>>,
}

impl CallShared {
    fn lock(&self) -> MutexGuard<'_, CallState> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// The delivered outcome, marking it taken; `None` while pending or once taken.
    fn take_delivered(&self) -> Option<RawOutcome> {
        let mut state = self.lock();
        let CallState::Delivered(outcome) = *state else {
            return None;
        };
        *state = CallState::Taken;
        Some(outcome)
    }

    /// Blocks until the result callback has run or `deadline` passes; true when it ran.
    fn wait_until(&self, deadline: Option<Instant>) -> bool {
        let pending = |state: &mut CallState| matches!(*state, CallState::Pending);
        let state = match deadline {
            None => self
                .called_back
                .wait_while(self.lock(), pending)
                .unwrap_or_else(PoisonError::into_inner),
            Some(deadline) => {
                let timeout = deadline.saturating_duration_since(Instant::now());
                self.called_back
                    .wait_timeout_while(self.lock(), timeout, pending)
                    .unwrap_or_else(PoisonError::into_inner)
                    .0
            }
        };
        !matches!(*state, CallState::Pending)
    }
}

/// Records a result callback's outcome. Called once per call, by its result trampoline.
///
/// # Safety
///
/// `user_state` is null or the `Arc::into_raw` reference [`start`] handed to the library, which
/// the header lets the library return exactly once.
unsafe fn deliver(user_state: *mut c_void, outcome: RawOutcome) {
    if user_state.is_null() {
        return;
    }
    // SAFETY: forwarded to the caller: the library's reference, reclaimed here once.
    let shared = unsafe { Arc::from_raw(user_state.cast_const().cast::<CallShared>()) };
    *shared.lock() = CallState::Delivered(outcome);
    shared.called_back.notify_all();
    if catch_unwind(AssertUnwindSafe(|| (shared.wake)())).is_err() {
        log::error!("the wake callback of an async call panicked");
    }
}

pub(crate) unsafe extern "C" fn onboard_result_trampoline(
    status: sys::QarResult,
    onboarding_id: *const sys::QarOnboardingId,
    session: *mut sys::QarSession,
    user_state: *mut c_void,
) {
    // SAFETY: valid for the duration of the callback when non-null.
    let id = unsafe { onboarding_id.as_ref() }.map_or([0; 16], |id| id.data);
    // SAFETY: the result callback of a call started by `start`, called once.
    unsafe {
        deliver(
            user_state,
            RawOutcome {
                result: status,
                handle: session as usize,
                id,
            },
        );
    }
}

pub(crate) unsafe extern "C" fn handle_result_trampoline<Handle>(
    status: sys::QarResult,
    handle: *mut Handle,
    user_state: *mut c_void,
) {
    // SAFETY: the result callback of a call started by `start`, called once.
    unsafe {
        deliver(
            user_state,
            RawOutcome {
                result: status,
                handle: handle as usize,
                id: [0; 16],
            },
        );
    }
}

pub(crate) unsafe extern "C" fn peer_result_trampoline(
    status: sys::QarResult,
    peer_id: *const sys::QarPeerId,
    user_state: *mut c_void,
) {
    // SAFETY: valid for the duration of the callback when non-null.
    let id = unsafe { peer_id.as_ref() }.map_or([0; 16], |id| id.data);
    // SAFETY: the result callback of a call started by `start`, called once.
    unsafe {
        deliver(
            user_state,
            RawOutcome {
                result: status,
                handle: 0,
                id,
            },
        );
    }
}

unsafe extern "C" fn progress_trampoline(
    severity: sys::QarActionSeverity,
    percent: f32,
    message: *const c_char,
    user_state: *mut c_void,
) {
    if user_state.is_null() {
        return;
    }
    // SAFETY: the library's Arc reference keeps the state alive until the result callback, and
    // no progress callback fires after it.
    let shared = unsafe { &*user_state.cast_const().cast::<CallShared>() };
    let Some(sender) = &shared.progress else {
        return;
    };
    let message = if message.is_null() {
        String::new()
    } else {
        // SAFETY: a valid C string for the duration of the callback.
        unsafe { CStr::from_ptr(message) }
            .to_string_lossy()
            .into_owned()
    };
    // A disconnected receiver only means nobody listens any more.
    let _ = sender.send(Progress {
        severity: ActionSeverity::from_raw(severity),
        percent,
        message,
    });
}

/// A call whose [`PendingCall`] was dropped before its outcome was taken; its owner keeps it.
pub(crate) struct AbandonedCall {
    shared: Arc<CallShared>,
    /// Kept until the callback; the header would also accept its destruction (the call then
    /// ends with `QAR_STATUS_TIMEOUT`).
    _token: Rc<CancelTokenInner>,
    destroy: Option<Destroy>,
    function: &'static str,
}

impl std::fmt::Debug for AbandonedCall {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AbandonedCall")
            .field("function", &self.function)
            .field("state", &*self.shared.lock())
            .finish_non_exhaustive()
    }
}

impl AbandonedCall {
    /// Destroys a delivered handle; `false` while the callback is pending.
    pub(crate) fn release_if_delivered(&self, api: &QarApi) -> bool {
        let Some(outcome) = self.shared.take_delivered() else {
            return false;
        };
        if let Some(destroy) = self.destroy
            && outcome.handle != 0
        {
            destroy(api, outcome.handle);
        }
        true
    }

    /// Waits until the callback has run or `deadline` passes, then releases what it delivered;
    /// true when it ran.
    pub(crate) fn settle(&self, api: &QarApi, deadline: Instant) -> bool {
        self.shared.wait_until(Some(deadline)) && self.release_if_delivered(api)
    }

    /// After the owner's destroy, which waited for the callback: a handle delivered then has no
    /// owner left to be destroyed under, and is leaked.
    pub(crate) fn leak_late_delivery(self) {
        if let Some(outcome) = self.shared.take_delivered()
            && outcome.handle != 0
        {
            log::warn!(
                "{} delivered a handle while its owner was destroyed; the handle is not released",
                self.function
            );
        }
    }
}

type Finish<T> = Box<dyn FnOnce(RawOutcome) -> Result<T>>;
type Abandon = Box<dyn FnOnce(AbandonedCall)>;

struct Call<T> {
    shared: Arc<CallShared>,
    token: Rc<CancelTokenInner>,
    /// Builds the value on the owner's thread; releases a delivered handle on failure.
    finish: Finish<T>,
    /// Hands the call to its owner when this value drops before the outcome.
    abandon: Abandon,
    /// Runs before every poll: the owner releases what its abandoned calls delivered.
    before_poll: Box<dyn Fn()>,
    destroy: Option<Destroy>,
    function: &'static str,
}

/// An async call in flight: an onboard or rejoin ([`crate::Runtime::start_onboard`],
/// [`crate::Runtime::start_rejoin`]), a co-process invite
/// ([`crate::Session::start_onboarding_invite`]), a target-app invite or kill
/// ([`crate::Session::start_invite_target_app`], [`crate::Session::start_kill_target_app`]), or a
/// begin-frame ([`crate::RenderSender::start_owned_frame`]). It keeps its owner alive.
///
/// The library calls back exactly once: with the outcome, or with
/// [`crate::StatusCode::TIMEOUT`] once the call's cancel token is cancelled
/// ([`PendingCall::cancel_handle`], the start's `timeout`, or dropping this value). A cancel
/// arriving just as the call finishes may still deliver the outcome.
///
/// Dropping it before the outcome is taken cancels the call and returns at once. The owner keeps
/// the call until its callback: a handle it still delivers is destroyed by the owner's next call
/// or by the owner's teardown, whose destroy waits for the callback.
pub struct PendingCall<T> {
    call: Option<Call<T>>,
}

/// An async onboard or rejoin, delivering an [`Onboarded`].
pub type PendingOnboard = PendingCall<Onboarded>;

impl<T> std::fmt::Debug for PendingCall<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let call = self
            .call
            .as_ref()
            .map(|call| (call.function, format!("{:?}", *call.shared.lock())));
        formatter
            .debug_struct("PendingCall")
            .field("call", &call)
            .finish()
    }
}

/// How to start one async call and what to do with its outcome.
pub(crate) struct CallSpec<T> {
    pub(crate) function: &'static str,
    pub(crate) progress: Option<Sender<Progress>>,
    pub(crate) timeout: Option<Duration>,
    pub(crate) wake: Box<dyn Fn() + Send + Sync>,
    pub(crate) keep: Option<Box<dyn Any + Send + Sync>>,
    pub(crate) finish: Finish<T>,
    pub(crate) abandon: Abandon,
    pub(crate) before_poll: Box<dyn Fn()>,
    pub(crate) destroy: Option<Destroy>,
}

/// Starts the call: `start_call` passes the user state, the progress trampoline (or none) and
/// the token's handle to the C start function and returns its result.
pub(crate) fn start<T>(
    library: &Rc<LibraryInner>,
    spec: CallSpec<T>,
    start_call: impl FnOnce(
        *mut c_void,
        sys::qar_progress_callback_t,
        *mut sys::QarCancelToken,
    ) -> sys::QarResult,
) -> Result<PendingCall<T>> {
    let token = Rc::new(CancelTokenInner::create(library, spec.timeout)?);
    let progress_callback: sys::qar_progress_callback_t = spec
        .progress
        .as_ref()
        .map(|_| progress_trampoline as unsafe extern "C" fn(_, _, _, _));
    let shared = Arc::new(CallShared {
        state: Mutex::new(CallState::Pending),
        called_back: Condvar::new(),
        progress: spec.progress,
        wake: spec.wake,
        _keep: spec.keep,
    });
    let user_state = Arc::into_raw(Arc::clone(&shared))
        .cast_mut()
        .cast::<c_void>();
    let result = start_call(user_state, progress_callback, token.handle.as_ptr());
    if let Err(error) = check(&library.api, spec.function, result) {
        // The header: when the call itself returns an error, the callback never fires, so the
        // library's reference is reclaimed here.
        // SAFETY: the Arc::into_raw reference above, which the library did not take.
        drop(unsafe { Arc::from_raw(user_state.cast_const().cast::<CallShared>()) });
        return Err(error);
    }
    Ok(PendingCall {
        call: Some(Call {
            shared,
            token,
            finish: spec.finish,
            abandon: spec.abandon,
            before_poll: spec.before_poll,
            destroy: spec.destroy,
            function: spec.function,
        }),
    })
}

impl<T> PendingCall<T> {
    /// Never blocks. `None` while the call runs; once it called back, the value or the error it
    /// delivered ([`crate::StatusCode::TIMEOUT`] for a cancelled or timed-out call). Polling
    /// again after that returns [`Error::InvalidArgument`].
    pub fn poll(&mut self) -> Option<Result<T>> {
        let Some(call) = &self.call else {
            return Some(Err(already_taken()));
        };
        (call.before_poll)();
        let outcome = call.shared.take_delivered()?;
        let call = self.call.take()?;
        Some((call.finish)(outcome))
    }

    /// Blocks until the library called back, which the header guarantees, and returns what
    /// [`PendingCall::poll`] would.
    pub fn wait(mut self) -> Result<T> {
        let Some(call) = &self.call else {
            return Err(already_taken());
        };
        call.shared.wait_until(None);
        self.poll().unwrap_or_else(|| Err(already_taken()))
    }

    /// Cancels the call from any thread while it runs; the library then calls back with
    /// [`crate::StatusCode::TIMEOUT`] (or the outcome, when the call was just finishing).
    /// `None` once the outcome was taken.
    pub fn cancel_handle(&self) -> Option<CancelHandle> {
        self.call
            .as_ref()
            .map(|call| CancelHandle::for_token(&call.token))
    }
}

fn already_taken() -> Error {
    Error::InvalidArgument("the outcome of this pending call was already taken".to_string())
}

impl<T> Drop for PendingCall<T> {
    fn drop(&mut self) {
        let Some(call) = self.call.take() else {
            return;
        };
        if let Some(outcome) = call.shared.take_delivered() {
            // An unwanted value is released as it drops.
            drop((call.finish)(outcome));
            return;
        }
        if let Err(error) = call.token.cancel() {
            log::warn!("cancelling {}: {error}", call.function);
        }
        (call.abandon)(AbandonedCall {
            shared: call.shared,
            _token: call.token,
            destroy: call.destroy,
            function: call.function,
        });
    }
}

/// Keeps what an init points to alive in a call's state. The data is plain C structures and
/// strings the library reads; Rust never reads it after the start and only frees it, which is
/// sound from any thread.
pub(crate) struct Keep<Data>(pub(crate) Data);

// SAFETY: see the type's documentation.
unsafe impl<Data> Send for Keep<Data> {}
// SAFETY: see the type's documentation; there is no `&self` access.
unsafe impl<Data> Sync for Keep<Data> {}
