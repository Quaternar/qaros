//! The Vulkan render sender path (feature `vulkan`).
//!
//! The application creates its own `VkInstance` and `VkDevice` on the
//! adapter the session uses ([`crate::GraphicsAdapter`]) and hands both to
//! QAROS ([`VulkanStreamParams`], chained into `QarRenderSenderInit`). QAROS
//! then owns the frame images: it creates them on that device and reports,
//! per frame, each image's handle, memory, size, format features, current
//! layout and queue family, plus a binary semaphore to signal.
//!
//! One frame, in the order of QuaternAR's `vulkan_cube` example:
//!
//! 1. Begin a frame ([`crate::RenderSender::begin_owned_frame`],
//!    [`crate::RenderSender::try_begin_owned_frame`], ...).
//! 2. [`crate::OwnedFrame::vulkan_frame`]: find each eye's color and depth
//!    view ([`VulkanFrameState::view_index`]) and read the image records.
//! 3. Record a barrier per image from its reported `layout` and
//!    `queue_family_index` to the attachment layout on the application's
//!    queue family (no ownership transfer when the reported family is
//!    `VK_QUEUE_FAMILY_IGNORED` or already the application's).
//! 4. Render each eye with the frame's view pose and field of view.
//! 5. Record a barrier per image to [`SHOWN_LAYOUT`]
//!    (`VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL`) that releases ownership to
//!    `VK_QUEUE_FAMILY_EXTERNAL`.
//! 6. Submit, signalling [`VulkanFrameState::semaphore`].
//! 7. Write the new state back: [`VulkanFrame::release_to_external`] (or
//!    [`VulkanFrame::set_image_state`] per image) and
//!    [`VulkanFrame::set_fence`] with a null fence.
//! 8. [`crate::OwnedFrame::show`].
//!
//! A frame that is begun but not rendered must still signal the semaphore
//! (an empty submission will do) before it is dropped, so that QAROS can
//! consume it.
//!
//! # Safety rules
//!
//! These come from the header (`qar_render_sender_frame_vulkan`,
//! `QarStreamParamsVulkan`) and the cube example; the wrapper enforces what a
//! borrow can express and documents the rest.
//!
//! - **Ownership.** QAROS owns the images, their memory and the semaphore.
//!   Never destroy, free or re-bind them. They live at most as long as the
//!   render sender; the handles may change between frames (after a layout
//!   change, for example), so read the records every frame and rebuild image
//!   views when a handle changes.
//! - **Frame pointer validity.** The library's frame record stays valid until
//!   the next `qar_render_sender_frame_vulkan` call or sender destruction.
//!   [`VulkanFrameState`] and [`VulkanFrame`] borrow the sender or the open
//!   frame mutably, which rules out both while they live. Handles copied out
//!   of them stay valid after the borrow ends, as long as the sender lives.
//! - **Writes only inside a frame cycle.** Only [`VulkanFrame`], from an open
//!   frame, can write back; [`VulkanFrameState`] (from the sender, outside a
//!   cycle) is read-only.
//! - **Order.** Submit the work that signals the semaphore, then write back,
//!   then show. The write-back cannot outlive the frame (the borrow ends
//!   before `show` consumes it), but the wrapper cannot see the GPU
//!   submission: a binary semaphore wait needs its signal submitted first,
//!   so showing before submitting is invalid Vulkan usage on the QAROS side.
//! - **Device.** The device must be created from the physical device on the
//!   session's adapter, enable [`required_device_extensions`], and outlive
//!   every render sender created with it. Queue submissions and the
//!   images' layouts are the application's responsibility; QAROS reads the
//!   written-back records to know them.

use std::ffi::CStr;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;

use ash::vk::{self, Handle};
use qaros_sys::{self as sys, defaults};

use crate::error::{Error, Result};
use crate::render::{TextureSize, VideoFrameView};
use crate::types::{Eye, PixelFormat, ViewKind};

/// The layout every image is left in, and written back as, before
/// `show_frame` (the cube's final transition).
pub const SHOWN_LAYOUT: vk::ImageLayout = vk::ImageLayout::TRANSFER_SRC_OPTIMAL;

/// Device extensions QAROS needs on the application's device, as the C Vulkan
/// cube enables them: Win32 external memory and semaphores on Windows, file
/// descriptor external memory on Linux (the cube also enables
/// `VK_KHR_external_semaphore_fd` where the device has it). None elsewhere,
/// where no QAROS runtime exists and the stub works with plain images.
pub fn required_device_extensions() -> &'static [&'static CStr] {
    if cfg!(target_os = "windows") {
        &[
            ash::khr::external_memory_win32::NAME,
            ash::khr::external_semaphore_win32::NAME,
        ]
    } else if cfg!(target_os = "linux") {
        &[ash::khr::external_memory_fd::NAME]
    } else {
        &[]
    }
}

/// The `VkFormat` assumed for a QAROS pixel format. The header reports only
/// the [`PixelFormat`], not the image's `VkFormat`; these are the UNORM and
/// SFLOAT formats `qaros-stub` creates. Whether QAROS uses the sRGB variants
/// for 8-bit color is not documented.
pub fn assumed_vk_format(format: PixelFormat) -> Option<vk::Format> {
    match format {
        PixelFormat::R8G8B8A8 => Some(vk::Format::R8G8B8A8_UNORM),
        PixelFormat::B8G8R8A8 | PixelFormat::B8G8R8X8 => Some(vk::Format::B8G8R8A8_UNORM),
        PixelFormat::R16G16B16A16 => Some(vk::Format::R16G16B16A16_SFLOAT),
        PixelFormat::R32Float => Some(vk::Format::R32_SFLOAT),
        PixelFormat::D32Float => Some(vk::Format::D32_SFLOAT),
        PixelFormat::Other(_) => None,
    }
}

/// `QarStreamParamsVulkan`: the application's physical device and device,
/// on which QAROS creates the frame images.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VulkanStreamParams {
    pub physical_device: vk::PhysicalDevice,
    pub device: vk::Device,
}

impl VulkanStreamParams {
    pub(crate) fn to_raw(self) -> sys::QarStreamParamsVulkan {
        let mut raw = defaults::qar_stream_params_vulkan_default();
        raw.physical_device = dispatchable_pointer(self.physical_device.as_raw());
        raw.device = dispatchable_pointer(self.device.as_raw());
        raw
    }
}

/// Dispatchable Vulkan handles are pointers; `ash` stores them as `u64`
/// through `Handle::as_raw`.
fn dispatchable_pointer<Target>(raw_handle: u64) -> *mut Target {
    usize::try_from(raw_handle).map_or(std::ptr::null_mut(), |address| address as *mut Target)
}

/// One sender-owned image of the frame (`QarVideoTextureVulkan`), copied out
/// of the library's record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VulkanImage {
    pub size: TextureSize,
    pub image: vk::Image,
    pub memory: vk::DeviceMemory,
    /// Format features of the image's format, as QAROS reports them. The
    /// image's usage flags are not reported.
    pub format_features: vk::FormatFeatureFlags,
    /// The layout the image is in now: transition from it.
    pub layout: vk::ImageLayout,
    /// The queue family that owns the image now; `VK_QUEUE_FAMILY_IGNORED`
    /// means no ownership transfer is needed.
    pub queue_family_index: u32,
}

impl VulkanImage {
    fn from_raw(raw: &sys::QarVideoTextureVulkan) -> Self {
        Self {
            size: TextureSize::from_raw(&raw.size),
            image: vk::Image::from_raw(raw.image),
            memory: vk::DeviceMemory::from_raw(raw.image_memory),
            format_features: vk::FormatFeatureFlags::from_raw(raw.format_features),
            layout: vk::ImageLayout::from_raw(raw.layout),
            queue_family_index: raw.queue_family_index,
        }
    }
}

/// Read access to the sender-owned Vulkan frame record. From
/// [`crate::RenderSender::vulkan_frame_state`] outside a frame cycle, or
/// through [`VulkanFrame`] inside one.
#[derive(Debug)]
pub struct VulkanFrameState<'borrow> {
    raw: NonNull<sys::QarVideoFrameVulkan>,
    _borrow: PhantomData<&'borrow mut sys::QarVideoFrameVulkan>,
}

impl VulkanFrameState<'_> {
    /// # Safety
    ///
    /// `raw` points to the library's frame record and stays valid, and is not
    /// accessed through another path, for the chosen lifetime.
    pub(crate) unsafe fn from_raw(raw: NonNull<sys::QarVideoFrameVulkan>) -> Self {
        Self {
            raw,
            _borrow: PhantomData,
        }
    }

    fn record(&self) -> &sys::QarVideoFrameVulkan {
        // SAFETY: valid and unaliased for the borrow (from_raw's contract).
        unsafe { self.raw.as_ref() }
    }

    /// The frame's views: which image, which rectangle, which eye, color or
    /// depth. QuaternAR's cube finds views by eye and kind rather than by the
    /// order they were declared in; [`Self::view_index`] does that.
    pub fn views(&self) -> Vec<VideoFrameView> {
        let record = self.record();
        let count = record.texture_views_count.min(record.texture_views.len());
        record.texture_views[..count]
            .iter()
            .map(VideoFrameView::from_raw)
            .collect()
    }

    /// Index of the first view showing `kind` for `eye`, the cube's
    /// `FindFrameView`. The same index selects the view's pose and field of
    /// view on the frame ([`crate::OwnedFrame::view`]).
    pub fn view_index(&self, eye: Eye, kind: ViewKind) -> Option<usize> {
        self.views()
            .iter()
            .position(|view| view.eye == eye && view.kind == kind)
    }

    /// Number of sender-owned images in the frame.
    pub fn image_count(&self) -> usize {
        let record = self.record();
        record.textures_count.min(record.textures.len())
    }

    /// The image a view's `texture_index` names.
    pub fn image(&self, index: usize) -> Option<VulkanImage> {
        let record = self.record();
        record.textures[..self.image_count()]
            .get(index)
            .map(VulkanImage::from_raw)
    }

    /// Every image of the frame, in `texture_index` order.
    pub fn images(&self) -> Vec<VulkanImage> {
        self.record().textures[..self.image_count()]
            .iter()
            .map(VulkanImage::from_raw)
            .collect()
    }

    /// The binary semaphore the frame's last submission must signal. Owned by
    /// QAROS; `VK_NULL_HANDLE` when the frame has none.
    pub fn semaphore(&self) -> vk::Semaphore {
        vk::Semaphore::from_raw(self.record().synchronization.semaphore)
    }

    /// The fence currently in the frame record. The cube writes null before
    /// showing; what QAROS does with a non-null fence is not documented.
    pub fn fence(&self) -> vk::Fence {
        vk::Fence::from_raw(self.record().synchronization.fence)
    }
}

/// The Vulkan frame of an open frame cycle, with write access to the state
/// QAROS reads at `show_frame`. From [`crate::OwnedFrame::vulkan_frame`] or
/// [`crate::Frame::vulkan_frame`]. Dereferences to [`VulkanFrameState`].
#[derive(Debug)]
pub struct VulkanFrame<'frame> {
    state: VulkanFrameState<'frame>,
}

impl<'frame> VulkanFrame<'frame> {
    pub(crate) fn new(state: VulkanFrameState<'frame>) -> Self {
        Self { state }
    }

    /// The library's `QarVideoFrameVulkan` record of this frame, for C helpers
    /// that take it (QuaternAR's test extension writes textures and the
    /// write-back through it). Valid, and not to be used through another
    /// path, while this `VulkanFrame` is borrowed.
    pub fn as_raw_mut(&mut self) -> *mut sys::QarVideoFrameVulkan {
        self.state.raw.as_ptr()
    }

    fn record_mut(&mut self) -> &mut sys::QarVideoFrameVulkan {
        // SAFETY: valid and unaliased for the borrow (from_raw's contract),
        // and this frame is inside a cycle, where the header allows writes.
        unsafe { self.state.raw.as_mut() }
    }

    /// Records the layout and owning queue family image `index` is left in
    /// by the work just submitted.
    pub fn set_image_state(
        &mut self,
        index: usize,
        layout: vk::ImageLayout,
        queue_family_index: u32,
    ) -> Result<()> {
        let count = self.state.image_count();
        let Some(texture) = self.record_mut().textures[..count].get_mut(index) else {
            return Err(Error::InvalidArgument(format!(
                "image {index} does not exist; the frame has {count}"
            )));
        };
        texture.layout = layout.as_raw();
        texture.queue_family_index = queue_family_index;
        Ok(())
    }

    /// Records every image as left in `layout` and released to
    /// `VK_QUEUE_FAMILY_EXTERNAL`, the cube's write-back with
    /// [`SHOWN_LAYOUT`].
    pub fn release_to_external(&mut self, layout: vk::ImageLayout) {
        let count = self.state.image_count();
        for texture in &mut self.record_mut().textures[..count] {
            texture.layout = layout.as_raw();
            texture.queue_family_index = vk::QUEUE_FAMILY_EXTERNAL;
        }
    }

    /// Writes the frame's fence; the cube writes `vk::Fence::null()`.
    pub fn set_fence(&mut self, fence: vk::Fence) {
        self.record_mut().synchronization.fence = fence.as_raw();
    }
}

impl<'frame> Deref for VulkanFrame<'frame> {
    type Target = VulkanFrameState<'frame>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record_with_two_images() -> sys::QarVideoFrameVulkan {
        let mut record = defaults::qar_video_frame_vulkan_default();
        record.textures_count = 2;
        record.textures[0].image = 11;
        record.textures[0].layout = vk::ImageLayout::UNDEFINED.as_raw();
        record.textures[0].queue_family_index = vk::QUEUE_FAMILY_IGNORED;
        record.textures[1].image = 12;
        record.texture_views_count = 2;
        record.texture_views[0].eye = sys::QarVideoFrameViewEye::QAR_VIDEO_FRAME_VIEW_EYE_LEFT;
        record.texture_views[0].data_type =
            sys::QarVideoFrameViewType::QAR_VIDEO_FRAME_VIEW_TYPE_DEPTH;
        record.texture_views[1].eye = sys::QarVideoFrameViewEye::QAR_VIDEO_FRAME_VIEW_EYE_LEFT;
        record.texture_views[1].data_type =
            sys::QarVideoFrameViewType::QAR_VIDEO_FRAME_VIEW_TYPE_COLOR;
        record.texture_views[1].texture_index = 1;
        record.synchronization.semaphore = 99;
        record.synchronization.fence = 98;
        record
    }

    #[test]
    fn views_are_found_by_eye_and_kind_and_images_read_back() {
        let mut record = record_with_two_images();
        // SAFETY: the record lives on this stack frame for the whole test.
        let state = unsafe { VulkanFrameState::from_raw(NonNull::from(&mut record)) };
        assert_eq!(state.view_index(Eye::Left, ViewKind::Color), Some(1));
        assert_eq!(state.view_index(Eye::Right, ViewKind::Color), None);
        let images = state.images();
        assert_eq!(images.len(), 2);
        assert_eq!(images[1].image, vk::Image::from_raw(12));
        assert_eq!(images[0].queue_family_index, vk::QUEUE_FAMILY_IGNORED);
        assert_eq!(state.semaphore(), vk::Semaphore::from_raw(99));
        assert_eq!(state.image(2), None);
    }

    #[test]
    fn the_write_back_lands_in_the_library_record() {
        let mut record = record_with_two_images();
        {
            // SAFETY: as above.
            let state = unsafe { VulkanFrameState::from_raw(NonNull::from(&mut record)) };
            let mut frame = VulkanFrame::new(state);
            frame.release_to_external(SHOWN_LAYOUT);
            frame.set_fence(vk::Fence::null());
            let refused = frame.set_image_state(2, SHOWN_LAYOUT, 0);
            assert!(refused.is_err());
        }
        for texture in &record.textures[..2] {
            assert_eq!(texture.layout, SHOWN_LAYOUT.as_raw());
            assert_eq!(texture.queue_family_index, vk::QUEUE_FAMILY_EXTERNAL);
        }
        assert_eq!(record.textures[2].layout, 0);
        assert_eq!(record.synchronization.fence, 0);
    }

    #[test]
    fn stream_params_carry_the_dispatchable_handles_as_pointers() {
        let params = VulkanStreamParams {
            physical_device: vk::PhysicalDevice::from_raw(0x1000),
            device: vk::Device::from_raw(0x2000),
        };
        let raw = params.to_raw();
        assert_eq!(
            raw.header.type_,
            sys::QarStructureType::QAR_STRUCTURE_TYPE_STREAM_VULKAN_PARAMS_EXT
        );
        assert_eq!(raw.physical_device as usize, 0x1000);
        assert_eq!(raw.device as usize, 0x2000);
    }
}
