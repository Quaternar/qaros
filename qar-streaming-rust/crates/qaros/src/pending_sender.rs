//! Render sender creation without blocking (`qar_render_sender_create_async`), on the shared
//! [`PendingCall`] engine (`pending.rs`).
//!
//! - **Init.** The init and the extensions it points to live in the call's state until the
//!   callback; the header does not say the start copies them.
//! - **Device references.** A D3D11 creation keeps the references on its device and context
//!   until the callback: in the sender it delivers, or, when the pending creation was dropped,
//!   in the abandoned call its session keeps until the callback has come.
//! - **Teardown.** The header has the session's destroy end a creation still running with
//!   `QAR_STATUS_TIMEOUT` and return only after its callback, so a session never waits or leaks
//!   on its own account; a sender a dropped creation delivered is destroyed by the session's next
//!   start, poll or [`crate::Session::release_abandoned_creations`], or by its teardown.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use qaros_sys::{self as sys, QarApi};

use crate::error::{Result, check};
use crate::library::require_handle;
use crate::pending::{self, CallSpec, Keep, PendingCall, RawOutcome};
use crate::render::{
    GraphicsReferences, RenderSender, RenderSenderConfig, RenderSenderInitStorage, SenderGraphics,
};
use crate::session::SessionInner;

const CREATE_ASYNC: &str = "qar_render_sender_create_async";

/// A render sender being created by [`crate::Session::start_render_sender`],
/// `Session::start_vulkan_render_sender` (feature `vulkan`) or
/// `Session::start_d3d11_render_sender` (feature `d3d11`). Keeps its session alive, as a sender
/// does, so dropping the [`crate::Session`] meanwhile is safe. A D3D11 creation also keeps the
/// device references of its parameters.
///
/// [`PendingCall::poll`] never blocks: `None` while the library is still creating the sender,
/// then the sender or the error it delivered ([`crate::StatusCode::TIMEOUT`] for a cancelled or
/// timed-out creation). Dropping it before the sender is taken cancels the creation and returns
/// at once; the session keeps the creation until its callback, and destroys a sender the library
/// still delivers.
pub type PendingRenderSender = PendingCall<RenderSender>;

/// Destroys a sender a dropped creation delivered. Only called while its session lives.
fn release_unwanted_sender(api: &QarApi, sender: usize) {
    if sender != 0 {
        // SAFETY: delivered to us, used by nobody, and its session is alive.
        unsafe { (api.qar_impl_render_stream_handle_destroy)(sender as *mut sys::QarRenderSender) };
    }
}

/// Starts `qar_render_sender_create_async` on `session`.
pub(crate) fn start(
    session: &Rc<SessionInner>,
    config: &RenderSenderConfig,
    graphics: &SenderGraphics,
    timeout: Option<Duration>,
    wake: Box<dyn Fn() + Send + Sync>,
) -> Result<PendingRenderSender> {
    session.release_delivered_abandoned_calls();
    let mut storage = RenderSenderInitStorage::new(config, graphics)?;
    let graphics_api = storage.graphics_api();
    let init = storage.wired_init();
    // The sender takes the references on success; a dropped creation hands them to its session.
    let references = Rc::new(Cell::new(Some(graphics.references())));
    let finish_references = Rc::clone(&references);
    let finish_session = Rc::clone(session);
    let abandon_session = Rc::clone(session);
    let poll_session = Rc::clone(session);
    let spec = CallSpec {
        function: CREATE_ASYNC,
        progress: None,
        timeout,
        wake,
        keep: Some(Box::new(Keep(storage))),
        finish: Box::new(move |outcome: RawOutcome| {
            let api = finish_session.api();
            if let Err(error) = check(api, CREATE_ASYNC, outcome.result) {
                release_unwanted_sender(api, outcome.handle);
                return Err(error);
            }
            let handle = require_handle(outcome.handle as *mut sys::QarRenderSender, CREATE_ASYNC)?;
            Ok(RenderSender::from_created_handle(
                &finish_session,
                handle,
                graphics_api,
                finish_references.take().unwrap_or_default(),
            ))
        }),
        abandon: Box::new(move |mut call| {
            let kept: Option<GraphicsReferences> = references.take();
            call.keep_until_released(Box::new(kept));
            abandon_session.abandon_call(call);
        }),
        before_poll: Box::new(move || poll_session.release_delivered_abandoned_calls()),
        destroy: Some(release_unwanted_sender),
    };
    let api = session.api();
    let session_handle = session.handle.as_ptr();
    pending::start(session.library(), spec, |user_state, _progress, token| {
        // SAFETY: live session; init and what it points to live in the call's state until the
        // callback; the token outlives the call (the pending creation or its session keeps it).
        unsafe {
            (api.qar_impl_render_sender_create_async)(
                session_handle,
                init,
                Some(pending::handle_result_trampoline::<sys::QarRenderSender>),
                user_state,
                token,
            )
        }
    })
}
