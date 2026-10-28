//! Render sender creation without blocking (`qar_render_sender_create_async`).
//!
//! The vendored header documents it only as the async variant of
//! `qar_render_sender_create`. This module assumes the worst wherever it is
//! silent:
//!
//! - **Callback thread.** The callback may run on any library thread, during
//!   or after the start call. It only records the outcome under a mutex; the
//!   [`RenderSender`] (not `Send`) is built on the thread that polls.
//! - **Cancellation.** The header promises exactly one callback, with
//!   `QAR_STATUS_TIMEOUT` on cancel, and a session destroy that waits for it.
//!   A runtime older than that promise (0.9.0 and earlier) may never call back
//!   for a cancelled call, so this module still guards against it: dropping a
//!   pending creation never waits for the callback; session teardown waits at
//!   most [`CANCELLED_CREATION_WAIT`] for the callbacks still due. Everything
//!   the callback touches stays allocated until it comes, which on such a
//!   runtime may be never.
//! - **`init` lifetime.** The library may read `init` and what it points to
//!   until it calls back. They live inside the callback state, whose last
//!   reference the library's callback releases, so they are freed only after
//!   the callback, or leaked when it never comes.
//! - **Exactly once.** The callback is assumed to run at most once, per the
//!   async conventions (`api-conventions.mdx` §4); a second call would
//!   release the callback state twice.

use std::ffi::c_void;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::ptr::NonNull;
use std::rc::Rc;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

use qaros_sys::{self as sys, QarApi};

use crate::cancel::CancelTokenInner;
use crate::error::{Error, Result, check};
use crate::library::require_handle;
use crate::render::{
    GraphicsReferences, RenderSender, RenderSenderConfig, RenderSenderInitStorage, SenderGraphics,
};
use crate::session::SessionInner;

const CREATE_ASYNC: &str = "qar_render_sender_create_async";

/// How long session teardown waits, for all of them together, for the
/// callbacks of creations cancelled before they called back. A session with
/// such a creation still pending afterwards is leaked rather than destroyed
/// (`SessionInner::drop`). The real runtime ends a creation by its token's
/// timeout with a callback; this gives a cancel the same chance while staying
/// well inside the 1.3 s QAROS allows a process it stops.
pub const CANCELLED_CREATION_WAIT: Duration = Duration::from_millis(500);

/// What the creation callback delivered. The sender address is kept as an
/// integer so the state is `Send`.
#[derive(Clone, Copy, Debug)]
struct CreationOutcome {
    result: sys::QarResult,
    sender: usize,
}

impl CreationOutcome {
    fn sender_pointer(self) -> *mut sys::QarRenderSender {
        self.sender as *mut sys::QarRenderSender
    }
}

#[derive(Debug)]
enum CreationState {
    Pending,
    Delivered(CreationOutcome),
    /// The outcome went to its owner (a polled sender, or a release).
    Taken,
    /// The session was destroyed before the callback: a sender delivered
    /// now can no longer be destroyed safely and is leaked.
    Orphaned,
}

/// Owns the init storage the library may read until the creation calls back.
/// Held as a raw pointer, never as a `Box`, so that moving this value does
/// not invalidate the pointers handed to the library.
struct InitAllocation {
    storage: NonNull<RenderSenderInitStorage>,
}

// SAFETY: the storage is plain data (ids, formats, Vulkan handles, pointers
// into itself) that Rust never touches after `wired_init`; it is only freed,
// which is sound from any thread.
unsafe impl Send for InitAllocation {}
// SAFETY: as above; no `&self` method reads the storage.
unsafe impl Sync for InitAllocation {}

impl InitAllocation {
    fn new(storage: Box<RenderSenderInitStorage>) -> (Self, *mut sys::QarRenderSenderInit) {
        let storage = Box::into_raw(storage);
        // SAFETY: just allocated and not aliased.
        let init = unsafe { (*storage).wired_init() };
        let allocation = Self {
            // SAFETY: Box::into_raw never returns null.
            storage: unsafe { NonNull::new_unchecked(storage) },
        };
        (allocation, init)
    }
}

impl Drop for InitAllocation {
    fn drop(&mut self) {
        // SAFETY: from Box::into_raw in new, freed once; the creation state
        // that owns this is dropped only after the library's reference is.
        drop(unsafe { Box::from_raw(self.storage.as_ptr()) });
    }
}

/// The callback's state. One `Arc` reference belongs to the library from
/// the start call until the callback reclaims it.
struct CreationShared {
    state: Mutex<CreationState>,
    /// Notified once the callback has recorded its outcome.
    called_back: Condvar,
    wake: Box<dyn Fn() + Send + Sync>,
    _init: InitAllocation,
}

impl CreationShared {
    fn lock(&self) -> MutexGuard<'_, CreationState> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Blocks until the callback has run or `deadline` passes; true when it ran.
    fn wait_for_callback_until(&self, deadline: Instant) -> bool {
        let timeout = deadline.saturating_duration_since(Instant::now());
        let (state, _timed_out) = self
            .called_back
            .wait_timeout_while(self.lock(), timeout, |state| {
                matches!(*state, CreationState::Pending)
            })
            .unwrap_or_else(PoisonError::into_inner);
        !matches!(*state, CreationState::Pending)
    }

    /// The delivered outcome, marking it taken; `None` while pending.
    fn take_delivered(&self) -> Option<CreationOutcome> {
        let mut state = self.lock();
        let CreationState::Delivered(outcome) = *state else {
            return None;
        };
        *state = CreationState::Taken;
        Some(outcome)
    }
}

unsafe extern "C" fn render_sender_created_trampoline(
    status: sys::QarResult,
    sender: *mut sys::QarRenderSender,
    user_state: *mut c_void,
) {
    if user_state.is_null() {
        return;
    }
    // SAFETY: user_state is the Arc::into_raw reference handed over in
    // PendingRenderSender::start, reclaimed here once (module docs).
    let shared = unsafe { Arc::from_raw(user_state.cast_const().cast::<CreationShared>()) };
    let mut state = shared.lock();
    if matches!(*state, CreationState::Orphaned) {
        drop(state);
        if !sender.is_null() {
            log::warn!(
                "an async render sender creation delivered a sender after its session was destroyed; the sender is not released"
            );
        }
        return;
    }
    *state = CreationState::Delivered(CreationOutcome {
        result: status,
        sender: sender as usize,
    });
    drop(state);
    shared.called_back.notify_all();
    if catch_unwind(AssertUnwindSafe(|| (shared.wake)())).is_err() {
        log::error!("the wake callback of an async render sender creation panicked");
    }
}

/// Destroys a sender nobody will use. Only called with the session alive.
fn release_unwanted_sender(api: &QarApi, outcome: CreationOutcome) {
    let sender = outcome.sender_pointer();
    if !sender.is_null() {
        // SAFETY: delivered to us, used by nobody, and its session is alive.
        unsafe { (api.qar_impl_render_stream_handle_destroy)(sender) };
    }
}

/// A creation whose [`PendingRenderSender`] was dropped before the callback.
/// The session keeps it, so that a sender the callback delivers later is
/// destroyed on the session's thread while the session is still alive: in
/// the callback the session might be mid-destruction on another thread, and
/// the header does not say a library thread may destroy handles at all.
pub(crate) struct AbandonedCreation {
    shared: Arc<CreationShared>,
    /// Must stay valid while the library may still use it.
    token: Rc<CancelTokenInner>,
    /// The D3D11 device references the creation may still use; released with this value, or
    /// leaked when the creation is orphaned.
    graphics: GraphicsReferences,
}

impl std::fmt::Debug for AbandonedCreation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AbandonedCreation")
            .field("state", &*self.shared.lock())
            .finish_non_exhaustive()
    }
}

impl AbandonedCreation {
    /// Destroys a delivered sender; `false` while the callback is pending.
    pub(crate) fn release_if_delivered(&self, api: &QarApi) -> bool {
        match self.shared.take_delivered() {
            Some(outcome) => {
                release_unwanted_sender(api, outcome);
                true
            }
            None => false,
        }
    }

    pub(crate) fn has_called_back(&self) -> bool {
        !matches!(*self.shared.lock(), CreationState::Pending)
    }

    /// For session teardown: waits until the callback has run or `deadline`
    /// passes; true when it ran.
    pub(crate) fn wait_for_callback_until(&self, deadline: Instant) -> bool {
        self.shared.wait_for_callback_until(deadline)
    }

    /// For session teardown: releases a delivered sender and returns `false`,
    /// or marks the creation orphaned so a later callback leaks its sender
    /// instead of handing it to a destroyed session, and returns `true`. The
    /// token then stays until the library goes, and the graphics references
    /// for good, since the creation may still use them.
    pub(crate) fn release_or_orphan(self, api: &QarApi) -> bool {
        let mut state = self.shared.lock();
        if let CreationState::Delivered(outcome) = *state {
            *state = CreationState::Taken;
            drop(state);
            release_unwanted_sender(api, outcome);
            return false;
        }
        *state = CreationState::Orphaned;
        drop(state);
        self.token.destroy_at_library_teardown();
        self.graphics.keep_until_process_exit();
        true
    }
}

struct Creation {
    shared: Arc<CreationShared>,
    token: Rc<CancelTokenInner>,
    session: Rc<SessionInner>,
    graphics_api: sys::QarGraphicsAPI,
    graphics: GraphicsReferences,
}

impl Creation {
    fn into_sender(self, outcome: CreationOutcome) -> Result<RenderSender> {
        let api = self.session.api();
        if let Err(error) = check(api, CREATE_ASYNC, outcome.result) {
            release_unwanted_sender(api, outcome);
            return Err(error);
        }
        let handle = require_handle(outcome.sender_pointer(), CREATE_ASYNC)?;
        Ok(RenderSender::from_created_handle(
            &self.session,
            handle,
            self.graphics_api,
            self.graphics,
        ))
    }
}

/// A render sender being created by [`crate::Session::start_render_sender`],
/// `Session::start_vulkan_render_sender` (feature `vulkan`) or
/// `Session::start_d3d11_render_sender` (feature `d3d11`). Keeps its session
/// alive, like a sender does, so dropping the [`crate::Session`] meanwhile is
/// safe. A D3D11 creation also keeps the device references of its parameters.
///
/// Dropping it before the sender is taken cancels the creation and returns at
/// once. The session then keeps the creation: a sender the library still
/// delivers is destroyed by the session's next [`PendingRenderSender::poll`],
/// start, or teardown, whichever comes first. Session teardown waits up to
/// [`CANCELLED_CREATION_WAIT`] for the callbacks of such creations, so the
/// session can still be destroyed when they come soon after the cancel. If
/// the callback never comes (a runtime older than the header's exactly-once
/// rule), its state stays allocated, its cancel token is
/// destroyed right before the library, and its device references are never
/// released.
pub struct PendingRenderSender {
    creation: Option<Creation>,
}

impl std::fmt::Debug for PendingRenderSender {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = self
            .creation
            .as_ref()
            .map(|creation| format!("{:?}", *creation.shared.lock()));
        formatter
            .debug_struct("PendingRenderSender")
            .field("state", &state)
            .finish()
    }
}

impl PendingRenderSender {
    pub(crate) fn start(
        session: &Rc<SessionInner>,
        config: &RenderSenderConfig,
        graphics: &SenderGraphics,
        timeout: Option<Duration>,
        wake: Box<dyn Fn() + Send + Sync>,
    ) -> Result<Self> {
        session.release_delivered_abandoned_creations();
        let storage = RenderSenderInitStorage::new(config, graphics)?;
        let token = Rc::new(CancelTokenInner::create(session.library(), timeout)?);
        let graphics_api = storage.graphics_api();
        let graphics = graphics.references();
        let (init_allocation, init) = InitAllocation::new(storage);
        let shared = Arc::new(CreationShared {
            state: Mutex::new(CreationState::Pending),
            called_back: Condvar::new(),
            wake,
            _init: init_allocation,
        });
        let user_state = Arc::into_raw(Arc::clone(&shared))
            .cast_mut()
            .cast::<c_void>();
        let api = session.api();
        // SAFETY: live session; init and what it points to live in the
        // callback state, which the library's reference keeps alive until the
        // callback; the token outlives the operation (see AbandonedCreation).
        let result = unsafe {
            (api.qar_impl_render_sender_create_async)(
                session.handle.as_ptr(),
                init,
                Some(render_sender_created_trampoline),
                user_state,
                token.handle.as_ptr(),
            )
        };
        if let Err(error) = check(api, CREATE_ASYNC, result) {
            // The header does not say whether a callback can still follow a
            // synchronous error, so the library's reference to the state (and
            // the init inside it) is leaked rather than freed, the token
            // kept until the library goes, and the device references kept for good.
            token.destroy_at_library_teardown();
            graphics.keep_until_process_exit();
            return Err(error);
        }
        Ok(Self {
            creation: Some(Creation {
                shared,
                token,
                session: Rc::clone(session),
                graphics_api,
                graphics,
            }),
        })
    }

    /// Never blocks. `None` while the library is still creating the sender;
    /// once it called back, the sender, or the error it delivered. Which
    /// status a cancelled or timed-out creation reports is not documented
    /// (`qaros-stub` uses `UNCLASSIFIED` and `TIMEOUT`). Polling again after
    /// that returns [`Error::InvalidArgument`].
    pub fn poll(&mut self) -> Option<Result<RenderSender>> {
        let Some(creation) = &self.creation else {
            return Some(Err(Error::InvalidArgument(
                "the render sender of this pending creation was already taken".to_string(),
            )));
        };
        creation.session.release_delivered_abandoned_creations();
        let outcome = creation.shared.take_delivered()?;
        let creation = self.creation.take()?;
        Some(creation.into_sender(outcome))
    }
}

impl Drop for PendingRenderSender {
    fn drop(&mut self) {
        let Some(creation) = self.creation.take() else {
            return;
        };
        if let Some(outcome) = creation.shared.take_delivered() {
            // An unwanted sender is destroyed as the wrapper drops.
            drop(creation.into_sender(outcome));
            return;
        }
        if let Err(error) = creation.token.cancel() {
            log::warn!("cancelling an async render sender creation: {error}");
        }
        let session = Rc::clone(&creation.session);
        session.abandon_creation(AbandonedCreation {
            shared: creation.shared,
            token: creation.token,
            graphics: creation.graphics,
        });
    }
}
