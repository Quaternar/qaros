//! Onboarding and rejoin without blocking (`qar_runtime_onboard_async`,
//! `qar_runtime_rejoin_async`).
//!
//! The header's async rules, and what this module builds on them:
//!
//! - **Exactly once.** A start call that succeeds fires its result callback exactly once, on a
//!   library thread: with the outcome, or with `QAR_STATUS_TIMEOUT` when its cancel token is
//!   cancelled, times out or is destroyed first, or when the runtime is destroyed first. A start
//!   call that fails never calls back. The callback records the outcome under a mutex and runs
//!   the caller's `wake`; the [`Session`] (not `Send`) is built on the thread that polls.
//! - **`user_state`.** The library holds one `Arc` reference to the call's state from the start
//!   call until the result callback reclaims it. Progress callbacks borrow that reference: none
//!   fires after the result callback.
//! - **`init` lifetime.** The runtime copies the init during the start call. The header does not
//!   say so for these calls, so the init, its extensions and their strings stay in the call's
//!   state until the callback, as for every async call of this crate. An invite or member-hub
//!   invitation chained into it is borrowed for the start call only, which the header's "borrowed
//!   for the duration of the call" and the runtime's copying both allow.
//! - **Teardown.** The runtime's destroy returns only after the callbacks of its pending calls
//!   have returned, so a dropped [`PendingOnboard`] cannot outlive its runtime's handle; the
//!   runtime keeps it until the callback and destroys a session it delivers.

use std::ffi::{CStr, c_char, c_void};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::rc::Rc;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

use qaros_sys::{self as sys, QarApi};

use crate::callbacks::Progress;
use crate::cancel::{CancelHandle, CancelTokenInner};
use crate::error::{Result, check};
use crate::library::require_handle;
use crate::runtime::{OnboardInitStorage, Onboarded, RejoinInitStorage, RuntimeInner};
use crate::session::Session;
use crate::types::{ActionSeverity, OnboardingId};

const ONBOARD_ASYNC: &str = "qar_runtime_onboard_async";
const REJOIN_ASYNC: &str = "qar_runtime_rejoin_async";

/// What the result callback delivered. The session address is kept as an integer so the state
/// is `Send`.
#[derive(Clone, Copy, Debug)]
struct OnboardOutcome {
    result: sys::QarResult,
    onboarding_id: [u8; 16],
    session: usize,
}

impl OnboardOutcome {
    fn session_pointer(self) -> *mut sys::QarSession {
        self.session as *mut sys::QarSession
    }
}

#[derive(Debug)]
enum OnboardState {
    Pending,
    Delivered(OnboardOutcome),
    /// The outcome went to its owner (a polled session, or a release).
    Taken,
}

/// The init the library may read until the callback; never read by Rust after the start.
#[allow(dead_code, reason = "held only to keep the init alive")]
enum InitStorage {
    Onboard(Box<OnboardInitStorage>),
    Rejoin(Box<RejoinInitStorage>),
}

/// The call's state. One `Arc` reference belongs to the library from the start call until the
/// result callback reclaims it.
struct OnboardShared {
    state: Mutex<OnboardState>,
    /// Notified once the result callback has recorded its outcome.
    called_back: Condvar,
    progress: Option<Sender<Progress>>,
    wake: Box<dyn Fn() + Send + Sync>,
    _init: InitStorage,
}

impl OnboardShared {
    fn lock(&self) -> MutexGuard<'_, OnboardState> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// The delivered outcome, marking it taken; `None` while pending.
    fn take_delivered(&self) -> Option<OnboardOutcome> {
        let mut state = self.lock();
        let OnboardState::Delivered(outcome) = *state else {
            return None;
        };
        *state = OnboardState::Taken;
        Some(outcome)
    }

    /// Blocks until the result callback has recorded its outcome.
    fn wait_for_callback(&self) {
        let _state = self
            .called_back
            .wait_while(self.lock(), |state| matches!(*state, OnboardState::Pending))
            .unwrap_or_else(PoisonError::into_inner);
    }
}

unsafe extern "C" fn onboard_result_trampoline(
    status: sys::QarResult,
    onboarding_id: *const sys::QarOnboardingId,
    session: *mut sys::QarSession,
    user_state: *mut c_void,
) {
    if user_state.is_null() {
        return;
    }
    // SAFETY: user_state is the Arc::into_raw reference handed over in PendingOnboard::start,
    // reclaimed here once: the header fires the result callback exactly once.
    let shared = unsafe { Arc::from_raw(user_state.cast_const().cast::<OnboardShared>()) };
    let onboarding_id = if onboarding_id.is_null() {
        [0; 16]
    } else {
        // SAFETY: valid for the duration of the callback.
        unsafe { (*onboarding_id).data }
    };
    *shared.lock() = OnboardState::Delivered(OnboardOutcome {
        result: status,
        onboarding_id,
        session: session as usize,
    });
    shared.called_back.notify_all();
    if catch_unwind(AssertUnwindSafe(|| (shared.wake)())).is_err() {
        log::error!("the wake callback of an async onboard panicked");
    }
}

unsafe extern "C" fn onboard_progress_trampoline(
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
    let shared = unsafe { &*user_state.cast_const().cast::<OnboardShared>() };
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

/// Destroys a session nobody will use. Only called while its runtime lives.
fn release_unwanted_session(api: &QarApi, outcome: OnboardOutcome) {
    let session = outcome.session_pointer();
    if !session.is_null() {
        // SAFETY: delivered to us, used by nobody, and its runtime is alive.
        unsafe { (api.qar_impl_session_handle_destroy)(session) };
    }
}

/// An onboard or rejoin whose [`PendingOnboard`] was dropped before the callback. The runtime
/// keeps it, so that a session the callback delivers later (a cancel arriving just as the call
/// finished) is destroyed on the runtime's thread while the runtime lives.
pub(crate) struct AbandonedOnboard {
    shared: Arc<OnboardShared>,
    /// Kept until the callback; the header would also accept its destruction (the call then
    /// ends with `QAR_STATUS_TIMEOUT`).
    _token: Rc<CancelTokenInner>,
}

impl std::fmt::Debug for AbandonedOnboard {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AbandonedOnboard")
            .field("state", &*self.shared.lock())
            .finish_non_exhaustive()
    }
}

impl AbandonedOnboard {
    /// Destroys a delivered session; `false` while the callback is pending.
    pub(crate) fn release_if_delivered(&self, api: &QarApi) -> bool {
        match self.shared.take_delivered() {
            Some(outcome) => {
                release_unwanted_session(api, outcome);
                true
            }
            None => false,
        }
    }

    /// After the runtime's destroy, which waited for every callback: a session delivered then
    /// (the header has it report `QAR_STATUS_TIMEOUT` instead) has no runtime left to be
    /// destroyed under, and is leaked.
    pub(crate) fn leak_session_delivered_late(self) {
        if let Some(outcome) = self.shared.take_delivered()
            && !outcome.session_pointer().is_null()
        {
            log::warn!(
                "an abandoned async onboard delivered a session while its runtime was destroyed; the session is not released"
            );
        }
    }
}

struct Call {
    shared: Arc<OnboardShared>,
    token: Rc<CancelTokenInner>,
    runtime: Rc<RuntimeInner>,
    function: &'static str,
}

impl Call {
    fn into_onboarded(self, outcome: OnboardOutcome) -> Result<Onboarded> {
        let api = &self.runtime.library.api;
        if let Err(error) = check(api, self.function, outcome.result) {
            release_unwanted_session(api, outcome);
            return Err(error);
        }
        let session = require_handle(outcome.session_pointer(), self.function)?;
        Ok(Onboarded {
            onboarding_id: OnboardingId(outcome.onboarding_id),
            session: Session::from_handle(session, Rc::clone(&self.runtime)),
        })
    }
}

/// An onboard or rejoin started by [`crate::Runtime::start_onboard`] or
/// [`crate::Runtime::start_rejoin`]. Keeps its runtime alive, so dropping the
/// [`crate::Runtime`] meanwhile is safe.
///
/// The library calls back exactly once: with the outcome, or with
/// [`crate::StatusCode::TIMEOUT`] once the call's cancel token is cancelled
/// ([`PendingOnboard::cancel_handle`], the start's `timeout`, or dropping this value). A cancel
/// arriving just as the call finishes may still deliver the outcome.
///
/// Dropping it before the outcome is taken cancels the call and returns at once. The runtime then
/// keeps the call until its callback: a session it still delivers is destroyed by the runtime's
/// next onboard, rejoin or start, or by its teardown, whose destroy waits for the callback.
pub struct PendingOnboard {
    call: Option<Call>,
}

impl std::fmt::Debug for PendingOnboard {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = self
            .call
            .as_ref()
            .map(|call| format!("{:?}", *call.shared.lock()));
        formatter
            .debug_struct("PendingOnboard")
            .field("state", &state)
            .finish()
    }
}

impl PendingOnboard {
    pub(crate) fn start_onboard(
        runtime: &Rc<RuntimeInner>,
        storage: Box<OnboardInitStorage>,
        progress: Option<Sender<Progress>>,
        timeout: Option<Duration>,
        wake: Box<dyn Fn() + Send + Sync>,
    ) -> Result<Self> {
        let init = storage.init();
        Self::start(
            runtime,
            InitStorage::Onboard(storage),
            progress,
            timeout,
            wake,
            ONBOARD_ASYNC,
            |api, runtime, callback, progress, user_state, token| {
                // SAFETY: live runtime; init lives in the call's state until the callback; the
                // token outlives the call (Call or AbandonedOnboard keeps it).
                unsafe {
                    (api.qar_impl_runtime_onboard_async)(
                        runtime, init, callback, progress, user_state, token,
                    )
                }
            },
        )
    }

    pub(crate) fn start_rejoin(
        runtime: &Rc<RuntimeInner>,
        storage: Box<RejoinInitStorage>,
        progress: Option<Sender<Progress>>,
        timeout: Option<Duration>,
        wake: Box<dyn Fn() + Send + Sync>,
    ) -> Result<Self> {
        let init = storage.init();
        Self::start(
            runtime,
            InitStorage::Rejoin(storage),
            progress,
            timeout,
            wake,
            REJOIN_ASYNC,
            |api, runtime, callback, progress, user_state, token| {
                // SAFETY: as in start_onboard.
                unsafe {
                    (api.qar_impl_runtime_rejoin_async)(
                        runtime, init, callback, progress, user_state, token,
                    )
                }
            },
        )
    }

    fn start(
        runtime: &Rc<RuntimeInner>,
        init: InitStorage,
        progress: Option<Sender<Progress>>,
        timeout: Option<Duration>,
        wake: Box<dyn Fn() + Send + Sync>,
        function: &'static str,
        start: impl FnOnce(
            &QarApi,
            *mut sys::QarRuntime,
            sys::qar_runtime_onboard_result_callback_t,
            sys::qar_progress_callback_t,
            *mut c_void,
            *mut sys::QarCancelToken,
        ) -> sys::QarResult,
    ) -> Result<Self> {
        let token = Rc::new(CancelTokenInner::create(&runtime.library, timeout)?);
        let progress_callback: sys::qar_progress_callback_t = progress
            .as_ref()
            .map(|_| onboard_progress_trampoline as unsafe extern "C" fn(_, _, _, _));
        let shared = Arc::new(OnboardShared {
            state: Mutex::new(OnboardState::Pending),
            called_back: Condvar::new(),
            progress,
            wake,
            _init: init,
        });
        let user_state = Arc::into_raw(Arc::clone(&shared))
            .cast_mut()
            .cast::<c_void>();
        let api = &runtime.library.api;
        let result = start(
            api,
            runtime.handle(),
            Some(onboard_result_trampoline),
            progress_callback,
            user_state,
            token.handle.as_ptr(),
        );
        if let Err(error) = check(api, function, result) {
            // The header: when the call itself returns an error, the callback never fires, so
            // the library's reference is reclaimed here.
            // SAFETY: the Arc::into_raw reference above, which the library did not take.
            drop(unsafe { Arc::from_raw(user_state.cast_const().cast::<OnboardShared>()) });
            return Err(error);
        }
        Ok(Self {
            call: Some(Call {
                shared,
                token,
                runtime: Rc::clone(runtime),
                function,
            }),
        })
    }

    /// Never blocks. `None` while the call runs; once it called back, the [`Onboarded`] or the
    /// error it delivered ([`crate::StatusCode::TIMEOUT`] for a cancelled or timed-out call).
    /// Polling again after that returns [`crate::Error::InvalidArgument`].
    pub fn poll(&mut self) -> Option<Result<Onboarded>> {
        let Some(call) = &self.call else {
            return Some(Err(already_taken()));
        };
        call.runtime.release_delivered_abandoned_onboards();
        let outcome = call.shared.take_delivered()?;
        let call = self.call.take()?;
        Some(call.into_onboarded(outcome))
    }

    /// Blocks until the library called back, which the header guarantees, and returns what
    /// [`PendingOnboard::poll`] would.
    pub fn wait(mut self) -> Result<Onboarded> {
        let Some(call) = &self.call else {
            return Err(already_taken());
        };
        call.shared.wait_for_callback();
        self.poll().unwrap_or_else(|| Err(already_taken()))
    }

    /// Cancels the call from any thread while it runs; the library then calls back with
    /// [`crate::StatusCode::TIMEOUT`] (or the outcome, when the call was just finishing).
    pub fn cancel_handle(&self) -> Option<CancelHandle> {
        self.call
            .as_ref()
            .map(|call| CancelHandle::for_token(&call.token))
    }
}

fn already_taken() -> crate::Error {
    crate::Error::InvalidArgument(
        "the outcome of this pending onboard was already taken".to_string(),
    )
}

impl Drop for PendingOnboard {
    fn drop(&mut self) {
        let Some(call) = self.call.take() else {
            return;
        };
        if let Some(outcome) = call.shared.take_delivered() {
            // An unwanted session is destroyed as the wrapper drops.
            drop(call.into_onboarded(outcome));
            return;
        }
        if let Err(error) = call.token.cancel() {
            log::warn!("cancelling an async onboard: {error}");
        }
        let runtime = Rc::clone(&call.runtime);
        runtime.abandon_onboard(AbandonedOnboard {
            shared: call.shared,
            _token: call.token,
        });
    }
}
