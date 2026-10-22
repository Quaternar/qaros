//! The Rust layer of `qar-streaming-c-ext`, the never-shipped library of the C API's
//! counterparts, for the integration tests: an in-process hub that mints invites, in-process
//! peers, render and video receivers that collect frames, and a Vulkan test device. It lives in
//! the tests only, as the C# tests' `Ext` layer does: the API under test is the binding of the
//! unchanged `qar-streaming-c`.
//!
//! `qar-streaming-c-ext` is built only in `Debug` and `RelWithDebInfo` SDK builds and never ships in
//! a release package. This crate is not a default member of the workspace, is never published,
//! and no shipped crate depends on it. Its FFI layer, [`ffi`], mirrors the
//! extension's structures and functions by hand.
//!
//! Loading order: load and initialize the base library first (with `qaros`),
//! then [`Extension::load`] from the same `bin` folder, so the extension binds
//! that very `qar-streaming-c.dll`. Drop every extension object before the
//! `qaros::Library`: `qar_library_destroy` ends the shared runtime.

#![allow(clippy::missing_safety_doc)]

mod error;
#[allow(missing_debug_implementations)]
pub mod ffi;
mod hub;
mod loader;
mod receiver;
mod vulkan;

pub use error::{Error, Result};
pub use ffi::*;
pub use hub::{Hub, InviteMethod, Peer};
pub use loader::{EXTENSION_FILE_NAME, ExtApi};
pub use receiver::{CpuFrameBuffers, RenderReceiver};
pub use vulkan::{VulkanDevice, VulkanTestFrame};

use std::path::Path;
use std::sync::Arc;

use qaros_sys::{QarStructureType, QarVideoFrameLayout, defaults};

/// The loaded extension. Cloning shares it.
#[derive(Clone, Debug)]
pub struct Extension {
    api: Arc<ExtApi>,
}

impl Extension {
    /// Loads `qar-streaming-c-ext` from `bin_directory`, the folder the base
    /// library was loaded from, and resolves every symbol.
    ///
    /// # Safety
    ///
    /// The base `qar-streaming-c` from the same folder must be loaded and
    /// initialized in this process, and the extension must match the
    /// layouts mirrored in this crate. Every object made from the extension
    /// (hub, peer, receiver, device, frame) must be dropped before the
    /// `qaros::Library`: `qar_library_destroy` ends the runtime they use.
    pub unsafe fn load(bin_directory: &Path) -> Result<Self> {
        // SAFETY: forwarded to the caller.
        let api = unsafe { ExtApi::load(bin_directory) }?;
        Ok(Self { api: Arc::new(api) })
    }

    /// The factor every test time budget is scaled by (`QAR_TIME_SCALE`).
    pub fn time_scale(&self) -> f64 {
        // SAFETY: no arguments.
        unsafe { (self.api.qar_impl_ext_time_scale)() }
    }

    /// The stereo side-by-side layout receivers usually lend: one color and
    /// one depth texture, each two eyes wide.
    pub fn layout_side_by_side(
        &self,
        eye_width: u32,
        eye_height: u32,
    ) -> Result<QarVideoFrameLayout> {
        let mut layout = defaults::qar_video_frame_layout_default();
        // SAFETY: valid out-pointer.
        let result = unsafe {
            (self.api.qar_impl_ext_layout_side_by_side)(eye_width, eye_height, &raw mut layout)
        };
        self.api.check("qar_ext_layout_side_by_side", result)?;
        Ok(layout)
    }

    /// Starts an in-process hub. `enable_password_onboarding` also allows
    /// password invites.
    pub fn create_hub(&self, enable_password_onboarding: bool) -> Result<Hub> {
        Hub::create(&self.api, enable_password_onboarding)
    }

    /// Creates the extension's own Vulkan instance and device.
    pub fn create_vulkan_device(&self) -> Result<VulkanDevice> {
        VulkanDevice::create(&self.api)
    }
}

pub(crate) fn stamped_header(structure_type: QarStructureType) -> qaros_sys::QarStructureHeader {
    qaros_sys::QarStructureHeader {
        type_: structure_type,
        next: std::ptr::null_mut(),
    }
}
