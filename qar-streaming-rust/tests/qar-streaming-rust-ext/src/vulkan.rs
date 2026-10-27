use std::ptr::{self, NonNull};
use std::rc::Rc;
use std::sync::Arc;

use qaros_sys::{QarStreamParamsVulkan, QarVideoFrameLayout, QarVideoFrameVulkan, defaults};

use crate::ffi::{ExtVulkanDevice, ExtVulkanFrame};
use crate::{Error, ExtApi, Result};

/// The extension's own Vulkan instance and device, so a test needs no Vulkan
/// setup of its own. Frames keep it alive; senders and receivers on it must
/// be dropped before it.
#[derive(Debug)]
pub struct VulkanDevice {
    pub(crate) inner: Rc<DeviceInner>,
}

#[derive(Debug)]
pub(crate) struct DeviceInner {
    handle: NonNull<ExtVulkanDevice>,
    api: Arc<ExtApi>,
}

impl VulkanDevice {
    pub(crate) fn create(api: &Arc<ExtApi>) -> Result<Self> {
        let mut handle: *mut ExtVulkanDevice = ptr::null_mut();
        // SAFETY: valid out-pointer.
        let result = unsafe { (api.qar_impl_ext_vulkan_device_create)(&raw mut handle) };
        api.check("qar_ext_vulkan_device_create", result)?;
        let handle =
            NonNull::new(handle).ok_or(Error::MissingHandle("qar_ext_vulkan_device_create"))?;
        Ok(Self {
            inner: Rc::new(DeviceInner {
                handle,
                api: Arc::clone(api),
            }),
        })
    }

    /// The stream parameters naming this device, header stamped.
    pub fn params(&self) -> Result<QarStreamParamsVulkan> {
        let api = &self.inner.api;
        let mut params = defaults::qar_stream_params_vulkan_default();
        // SAFETY: live device; valid out-pointer.
        let result = unsafe {
            (api.qar_impl_ext_vulkan_device_params)(self.inner.handle.as_ptr(), &raw mut params)
        };
        api.check("qar_ext_vulkan_device_params", result)?;
        Ok(params)
    }

    /// Exportable images for one frame of `layout`, to lend to a receiver.
    pub fn create_frame(&self, layout: &QarVideoFrameLayout) -> Result<VulkanTestFrame> {
        let api = &self.inner.api;
        let mut handle: *mut ExtVulkanFrame = ptr::null_mut();
        // SAFETY: live device; the layout is read during the call.
        let result = unsafe {
            (api.qar_impl_ext_vulkan_frame_create)(
                self.inner.handle.as_ptr(),
                layout,
                &raw mut handle,
            )
        };
        api.check("qar_ext_vulkan_frame_create", result)?;
        let handle =
            NonNull::new(handle).ok_or(Error::MissingHandle("qar_ext_vulkan_frame_create"))?;
        Ok(VulkanTestFrame {
            handle,
            device: Rc::clone(&self.inner),
        })
    }

    /// Uploads tightly packed texels into texture `texture_index` of `frame`,
    /// for example a frame a base render sender handed out.
    ///
    /// # Safety
    ///
    /// `frame` must describe live images on this device, not in use by the
    /// GPU or another thread during the call.
    pub unsafe fn write_texture(
        &self,
        frame: *mut QarVideoFrameVulkan,
        texture_index: usize,
        texels: &[u8],
    ) -> Result<()> {
        let api = &self.inner.api;
        // SAFETY: forwarded to the caller; texels is readable for its length.
        let result = unsafe {
            (api.qar_impl_ext_vulkan_write_texture)(
                self.inner.handle.as_ptr(),
                frame,
                texture_index,
                texels.as_ptr(),
                texels.len(),
            )
        };
        api.check("qar_ext_vulkan_write_texture", result)
    }

    /// Hands every written texture to the external consumer and signals the
    /// frame's semaphore, updating each texture's layout and queue family in
    /// `frame`: the write-back a sender's frame needs before `show`.
    ///
    /// # Safety
    ///
    /// As for [`Self::write_texture`].
    pub unsafe fn release_frame(&self, frame: *mut QarVideoFrameVulkan) -> Result<()> {
        let api = &self.inner.api;
        // SAFETY: forwarded to the caller.
        let result =
            unsafe { (api.qar_impl_ext_vulkan_release_frame)(self.inner.handle.as_ptr(), frame) };
        api.check("qar_ext_vulkan_release_frame", result)
    }

    /// Reads texture `texture_index` back to tightly packed texels, after
    /// waiting for (and consuming) the frame's semaphore when asked.
    ///
    /// # Safety
    ///
    /// As for [`Self::write_texture`].
    pub unsafe fn read_texture(
        &self,
        frame: *mut QarVideoFrameVulkan,
        texture_index: usize,
        wait_frame_semaphore: bool,
    ) -> Result<Vec<u8>> {
        let api = &self.inner.api;
        let device = self.inner.handle.as_ptr();
        let mut required_size = 0_usize;
        // SAFETY: forwarded to the caller; a null buffer queries the size
        // and waits for nothing.
        let result = unsafe {
            (api.qar_impl_ext_vulkan_read_texture)(
                device,
                frame,
                texture_index,
                false,
                ptr::null_mut(),
                0,
                &raw mut required_size,
            )
        };
        api.check("qar_ext_vulkan_read_texture (size)", result)?;
        let mut texels = vec![0_u8; required_size];
        // SAFETY: forwarded to the caller; texels is writable for its length.
        let result = unsafe {
            (api.qar_impl_ext_vulkan_read_texture)(
                device,
                frame,
                texture_index,
                wait_frame_semaphore,
                texels.as_mut_ptr(),
                texels.len(),
                &raw mut required_size,
            )
        };
        api.check("qar_ext_vulkan_read_texture", result)?;
        Ok(texels)
    }
}

impl Drop for DeviceInner {
    fn drop(&mut self) {
        // SAFETY: every frame holds an Rc to this value, so none is alive.
        unsafe { (self.api.qar_impl_ext_vulkan_device_handle_destroy)(self.handle.as_ptr()) };
    }
}

/// Images for one frame, owned by a [`VulkanDevice`].
#[derive(Debug)]
pub struct VulkanTestFrame {
    handle: NonNull<ExtVulkanFrame>,
    device: Rc<DeviceInner>,
}

impl VulkanTestFrame {
    /// Describes the frame for one lend to a receiver, with a fresh
    /// completion semaphore the receiver signals.
    pub fn lend(&self) -> Result<QarVideoFrameVulkan> {
        let api = &self.device.api;
        // SAFETY: an all-zero frame record is a valid empty value.
        let mut frame: QarVideoFrameVulkan = unsafe { std::mem::zeroed() };
        // SAFETY: live frame; valid out-pointer.
        let result =
            unsafe { (api.qar_impl_ext_vulkan_frame_lend)(self.handle.as_ptr(), &raw mut frame) };
        api.check("qar_ext_vulkan_frame_lend", result)?;
        Ok(frame)
    }
}

impl Drop for VulkanTestFrame {
    fn drop(&mut self) {
        // SAFETY: owned handle; a receiver holding it lent must be gone.
        unsafe { (self.device.api.qar_impl_ext_vulkan_frame_handle_destroy)(self.handle.as_ptr()) };
    }
}
