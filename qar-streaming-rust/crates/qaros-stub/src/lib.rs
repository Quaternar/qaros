//! In-memory stand-in for `qar-streaming-c`.
//!
//! Built as a cdylib, it exports every `qar_impl_*` symbol of the C
//! header (generated from the same function list as `qaros-sys`), so the
//! runtime-loading path loads it like the real DLL. Functions the safe layer
//! uses have simple in-memory behaviour; the rest return
//! `QAR_STATUS_NOT_IMPLEMENTED`.
//!
//! What it checks, through a call log read with `qar_stub_call_log`:
//! - structure headers are stamped with the expected type,
//! - handles are destroyed once, by the right function, in order,
//! - callback-delivered handles are not destroyed by the caller.
//!
//! Callbacks run on threads of their own, as the real library's do:
//! - subscription callbacks and the progress callbacks of blocking calls run
//!   on a spawned thread that is joined before the triggering call returns;
//! - the callback of `qar_render_sender_create_async` runs on a detached
//!   thread after the call has returned, honouring cancellation, and is
//!   shaped with `qar_stub_set_render_sender_create_delay_ms`,
//!   `qar_stub_set_render_sender_create_cancel_delay_ms`,
//!   `qar_stub_set_render_sender_create_never_calls_back`,
//!   `qar_stub_set_render_sender_create_ignores_cancel` and
//!   `qar_stub_set_peer_unanswering` (a dead peer listed as running).
//!
//! `try_begin_frame` starts a begin-frame wait on its first call and has the
//! frame ready once it completes: at once by default, after
//! `qar_stub_set_try_begin_frame_delay_ms`, and never while the target peer's
//! receiver paused its stream (`qar_stub_set_peer_paused`). A sender
//! destroyed while its wait is pending is noted in the call log.
//!
//! Graphics paths: CPU frames in memory; Vulkan on the caller's own device (`vulkan.rs`);
//! D3D11, where the bindings carry it (Windows and macOS), without any D3D11 object: the
//! shared textures and keyed mutexes are stand-in addresses (`d3d11.rs`).
//!
//! What it does not model: the real library's own threads and their timing.
//! Those are tested on Windows against the package.

#![allow(
    clippy::missing_safety_doc,
    clippy::too_many_arguments,
    clippy::not_unsafe_ptr_arg_deref
)]

// Where the bindings carry the D3D11 types: Windows, and macOS for the stub's sake.
#[cfg(any(target_os = "windows", target_os = "macos"))]
mod d3d11;
mod exports;
pub mod frame_stamp;
mod state;
mod vulkan;

pub use exports::StubApi;

/// The implementation behind the exported symbols.
#[derive(Debug)]
pub struct Stub;

mod behaviour;
