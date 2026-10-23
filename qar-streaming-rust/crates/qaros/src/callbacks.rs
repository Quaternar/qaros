//! Callback plumbing: the C API calls these trampolines on library threads.

use std::ffi::{CStr, c_char, c_void};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::mpsc::{self, Receiver, Sender};

use qaros_sys::QarActionSeverity;

use crate::cancel::CancelToken;
use crate::types::ActionSeverity;

/// A progress report from a blocking onboarding call.
#[derive(Clone, Debug, PartialEq)]
pub struct Progress {
    pub severity: ActionSeverity,
    pub percent: f32,
    pub message: String,
}

/// Optional progress channel and cancellation for the blocking calls.
#[derive(Clone, Debug, Default)]
pub struct BlockingOptions<'token> {
    /// Receives progress reports, sent from a library thread while the call
    /// blocks.
    pub progress: Option<Sender<Progress>>,
    /// Must stay alive until the call returns, which the borrow ensures.
    pub cancel: Option<&'token CancelToken>,
}

impl BlockingOptions<'_> {
    pub(crate) fn cancel_pointer(&self) -> *mut qaros_sys::QarCancelToken {
        self.cancel
            .map_or(std::ptr::null_mut(), CancelToken::as_ptr)
    }

    pub(crate) fn progress_state(&self) -> *mut c_void {
        self.progress
            .as_ref()
            .map_or(std::ptr::null_mut(), |sender| {
                std::ptr::from_ref(sender).cast_mut().cast()
            })
    }

    pub(crate) fn progress_callback(&self) -> qaros_sys::qar_progress_callback_t {
        self.progress
            .as_ref()
            .map(|_| progress_trampoline as unsafe extern "C" fn(_, _, _, _))
    }
}

unsafe extern "C" fn progress_trampoline(
    severity: QarActionSeverity,
    percent: f32,
    message: *const c_char,
    user_state: *mut c_void,
) {
    if user_state.is_null() {
        return;
    }
    // SAFETY: user_state is the &Sender<Progress> from BlockingOptions, which
    // outlives the blocking call; the docs guarantee no progress callback
    // after the call returns.
    let sender = unsafe { &*user_state.cast::<Sender<Progress>>() };
    let message = if message.is_null() {
        String::new()
    } else {
        // SAFETY: the docs guarantee message is a valid C string for the
        // duration of the callback.
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

/// A callback for subscription events. Called on library threads, possibly
/// concurrently; it must not call blocking QAROS functions.
pub type EventCallback<Event> = Box<dyn Fn(Event) + Send + Sync + 'static>;

/// A callback that forwards events into a channel, and the receiving end.
///
/// ```no_run
/// # fn demo(session: &qaros::Session) -> qaros::Result<()> {
/// let (forward, requests) = qaros::channel();
/// let _subscription = session.subscribe_render_requests(forward)?;
/// let first_request = requests.recv();
/// # Ok(()) }
/// ```
pub fn channel<Event: Send + 'static>() -> (impl Fn(Event) + Send + Sync + 'static, Receiver<Event>)
{
    let (sender, receiver) = mpsc::channel();
    let forward = move |event: Event| {
        // A dropped receiver means the application stopped listening.
        let _ = sender.send(event);
    };
    (forward, receiver)
}

/// Runs a user callback, stopping a panic at the FFI boundary.
pub(crate) fn deliver<Event>(callback: &EventCallback<Event>, event: Event, source: &str) {
    if catch_unwind(AssertUnwindSafe(|| callback(event))).is_err() {
        log::error!("a {source} callback panicked; the event was dropped");
    }
}
