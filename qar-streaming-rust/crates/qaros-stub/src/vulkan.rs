//! The Vulkan render sender, emulated on the caller's own device.
//!
//! The real runtime creates the frame images on the application's device
//! with Win32 external memory and imports them into its own device. The stub
//! creates plain images, memory and a binary semaphore on the caller's
//! device instead (so it runs on drivers without Win32 external memory), reports their
//! state through the `QarVideoFrameVulkan` record it owns, checks the
//! write-back at `show_frame` and consumes the semaphore with a wait on one
//! of the device's queues.

use std::ffi::{CStr, c_char, c_void};
use std::time::Duration;

use ash::vk::{self, Handle};
#[allow(clippy::wildcard_imports)]
use qaros_sys::*;

const LIBRARY_VARIABLE: &str = "QAROS_STUB_VULKAN_LIBRARY";
const QUEUE_FAMILY_VARIABLE: &str = "QAROS_STUB_VULKAN_QUEUE_FAMILY";
const SEMAPHORE_WAIT_LIMIT: Duration = Duration::from_secs(5);

/// The name `ash::Entry::load` opens, so the dynamic linker returns the
/// loader the caller (wgpu-hal, a test) already has.
const DEFAULT_LIBRARY: &str = if cfg!(target_os = "windows") {
    "vulkan-1.dll"
} else if cfg!(target_os = "macos") {
    "libvulkan.dylib"
} else {
    "libvulkan.so.1"
};

const SHOWN_LAYOUT: vk::ImageLayout = vk::ImageLayout::TRANSFER_SRC_OPTIMAL;

/// What the stub creates per frame texture: the format it assumes for the
/// QAROS pixel format, and usage and reported format features by kind.
fn image_description(
    format: QarPixelFormat,
) -> Result<(vk::Format, vk::ImageUsageFlags, vk::FormatFeatureFlags), String> {
    let color_usage = vk::ImageUsageFlags::COLOR_ATTACHMENT
        | vk::ImageUsageFlags::SAMPLED
        | vk::ImageUsageFlags::TRANSFER_SRC
        | vk::ImageUsageFlags::TRANSFER_DST;
    let color_features = vk::FormatFeatureFlags::COLOR_ATTACHMENT
        | vk::FormatFeatureFlags::SAMPLED_IMAGE
        | vk::FormatFeatureFlags::TRANSFER_SRC
        | vk::FormatFeatureFlags::TRANSFER_DST;
    let vulkan_format = match format {
        QarPixelFormat::QAR_PIXEL_FORMAT_R8G8B8A8 => vk::Format::R8G8B8A8_UNORM,
        QarPixelFormat::QAR_PIXEL_FORMAT_B8G8R8A8 | QarPixelFormat::QAR_PIXEL_FORMAT_B8G8R8X8 => {
            vk::Format::B8G8R8A8_UNORM
        }
        QarPixelFormat::QAR_PIXEL_FORMAT_R16G16B16A16 => vk::Format::R16G16B16A16_SFLOAT,
        QarPixelFormat::QAR_PIXEL_FORMAT_R32_FLOAT => vk::Format::R32_SFLOAT,
        QarPixelFormat::QAR_PIXEL_FORMAT_D32_FLOAT => {
            return Ok((
                vk::Format::D32_SFLOAT,
                vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT
                    | vk::ImageUsageFlags::SAMPLED
                    | vk::ImageUsageFlags::TRANSFER_SRC
                    | vk::ImageUsageFlags::TRANSFER_DST,
                vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT
                    | vk::FormatFeatureFlags::SAMPLED_IMAGE
                    | vk::FormatFeatureFlags::TRANSFER_SRC
                    | vk::FormatFeatureFlags::TRANSFER_DST,
            ));
        }
        other => {
            return Err(format!(
                "pixel format {} has no Vulkan format in the stub",
                other.0
            ));
        }
    };
    Ok((vulkan_format, color_usage, color_features))
}

struct StubImage {
    image: vk::Image,
    memory: vk::DeviceMemory,
}

/// The Vulkan side of one render sender.
pub struct VulkanSender {
    device: ash::Device,
    images: Vec<StubImage>,
    semaphore: vk::Semaphore,
    consume_fence: vk::Fence,
    queue: vk::Queue,
    /// Owned here so its address is stable; the library hands out a pointer
    /// to it, valid until the sender is destroyed.
    record: Box<QarVideoFrameVulkan>,
    /// A frame was dropped unshown: whether the caller signalled the
    /// semaphore is unknown, so it is replaced before the next frame.
    semaphore_state_unknown: bool,
    /// Keeps the loader the device functions came from loaded.
    _loader: libloading::Library,
}

/// Function pointers resolve through the loader's `vkGetDeviceProcAddr`.
type GetDeviceProcAddr =
    unsafe extern "system" fn(vk::Device, *const c_char) -> vk::PFN_vkVoidFunction;

fn open_loader() -> Result<(libloading::Library, GetDeviceProcAddr), String> {
    let library_path =
        std::env::var(LIBRARY_VARIABLE).unwrap_or_else(|_| DEFAULT_LIBRARY.to_string());
    // SAFETY: the Vulkan loader's initializers have no preconditions.
    let loader = unsafe { libloading::Library::new(&library_path) }
        .map_err(|error| format!("cannot open the Vulkan loader {library_path}: {error}"))?;
    // SAFETY: vkGetDeviceProcAddr has this signature in every loader.
    let get_device_proc_addr = unsafe { loader.get::<GetDeviceProcAddr>(b"vkGetDeviceProcAddr\0") }
        .map(|symbol| *symbol)
        .map_err(|error| format!("{library_path} lacks vkGetDeviceProcAddr: {error}"))?;
    Ok((loader, get_device_proc_addr))
}

fn queue_family() -> u32 {
    std::env::var(QUEUE_FAMILY_VARIABLE)
        .ok()
        .and_then(|value| value.trim().parse().ok())
        .unwrap_or(0)
}

impl VulkanSender {
    /// Creates one image per layout texture on the caller's device.
    ///
    /// # Safety
    ///
    /// The parameters name a live `VkDevice` created through the loader the
    /// stub opens, with a queue in the stub's queue family, alive until the
    /// sender is destroyed.
    pub unsafe fn create(
        params: &QarStreamParamsVulkan,
        views: &[QarVideoFrameView],
        textures: &[QarTextureSize],
    ) -> Result<Self, String> {
        if params.physical_device.is_null() || params.device.is_null() {
            return Err("QarStreamParamsVulkan needs a physical device and a device".to_string());
        }
        let device_handle = vk::Device::from_raw(params.device as u64);
        let (loader, get_device_proc_addr) = open_loader()?;
        // SAFETY: the device is live (caller's contract); unknown names
        // resolve to null, which ash reports when the function is called.
        let device = unsafe {
            ash::Device::load_with(
                |name: &CStr| {
                    get_device_proc_addr(device_handle, name.as_ptr())
                        .map_or(std::ptr::null(), |function| function as *const c_void)
                },
                device_handle,
            )
        };
        // SAFETY: the stub's queue family has queue 0 (caller's contract).
        let queue = unsafe { device.get_device_queue(queue_family(), 0) };

        let mut sender = Self {
            device,
            images: Vec::new(),
            semaphore: vk::Semaphore::null(),
            consume_fence: vk::Fence::null(),
            queue,
            record: Box::new(defaults::qar_video_frame_vulkan_default()),
            semaphore_state_unknown: false,
            _loader: loader,
        };
        // On an early return, Drop releases what was created so far.
        sender.semaphore = sender.create_semaphore()?;
        // SAFETY: live device.
        sender.consume_fence = unsafe {
            sender
                .device
                .create_fence(&vk::FenceCreateInfo::default(), None)
        }
        .map_err(|error| format!("vkCreateFence: {error}"))?;

        for (slot, view) in sender.record.texture_views.iter_mut().zip(views) {
            *slot = *view;
        }
        sender.record.texture_views_count = views.len().min(sender.record.texture_views.len());
        for (index, size) in textures
            .iter()
            .take(sender.record.textures.len())
            .enumerate()
        {
            let (vulkan_format, usage, format_features) = image_description(size.format)?;
            let created = sender.create_image(size, vulkan_format, usage)?;
            sender.record.textures[index] = QarVideoTextureVulkan {
                size: *size,
                image: created.image.as_raw(),
                image_memory: created.memory.as_raw(),
                format_features: format_features.as_raw(),
                layout: vk::ImageLayout::UNDEFINED.as_raw(),
                queue_family_index: vk::QUEUE_FAMILY_IGNORED,
            };
            sender.images.push(created);
            sender.record.textures_count = sender.images.len();
        }
        sender.record.synchronization.semaphore = sender.semaphore.as_raw();
        Ok(sender)
    }

    fn create_semaphore(&self) -> Result<vk::Semaphore, String> {
        // SAFETY: live device.
        unsafe {
            self.device
                .create_semaphore(&vk::SemaphoreCreateInfo::default(), None)
        }
        .map_err(|error| format!("vkCreateSemaphore: {error}"))
    }

    fn create_image(
        &self,
        size: &QarTextureSize,
        vulkan_format: vk::Format,
        usage: vk::ImageUsageFlags,
    ) -> Result<StubImage, String> {
        let image_info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::TYPE_2D)
            .format(vulkan_format)
            .extent(vk::Extent3D {
                width: size.width,
                height: size.height,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(size.array_layers.max(1))
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .initial_layout(vk::ImageLayout::UNDEFINED);
        // SAFETY: live device, valid create info.
        let image = unsafe { self.device.create_image(&image_info, None) }
            .map_err(|error| format!("vkCreateImage: {error}"))?;
        // SAFETY: the image was just created on this device.
        let requirements = unsafe { self.device.get_image_memory_requirements(image) };
        // Any type the image allows is valid; the lowest one avoids needing
        // the instance-level memory properties query.
        let memory_type_index = requirements.memory_type_bits.trailing_zeros();
        let allocate_info = vk::MemoryAllocateInfo::default()
            .allocation_size(requirements.size)
            .memory_type_index(memory_type_index);
        // SAFETY: live device, a memory type the image accepts.
        let memory = match unsafe { self.device.allocate_memory(&allocate_info, None) } {
            Ok(memory) => memory,
            Err(error) => {
                // SAFETY: unused image of this device.
                unsafe { self.device.destroy_image(image, None) };
                return Err(format!("vkAllocateMemory: {error}"));
            }
        };
        // SAFETY: fresh image and memory of sufficient size.
        if let Err(error) = unsafe { self.device.bind_image_memory(image, memory, 0) } {
            // SAFETY: unused objects of this device.
            unsafe {
                self.device.destroy_image(image, None);
                self.device.free_memory(memory, None);
            }
            return Err(format!("vkBindImageMemory: {error}"));
        }
        Ok(StubImage { image, memory })
    }

    pub fn record_pointer(&mut self) -> *mut QarVideoFrameVulkan {
        std::ptr::from_mut(self.record.as_mut())
    }

    /// The write-back the cube does before `show_frame`: every image left in
    /// `TRANSFER_SRC_OPTIMAL` and released to `VK_QUEUE_FAMILY_EXTERNAL`.
    pub fn write_back_report(&self) -> String {
        let count = self.record.textures_count.min(self.record.textures.len());
        let wrong: Vec<String> = self.record.textures[..count]
            .iter()
            .enumerate()
            .filter(|(_, texture)| {
                texture.layout != SHOWN_LAYOUT.as_raw()
                    || texture.queue_family_index != vk::QUEUE_FAMILY_EXTERNAL
            })
            .map(|(index, texture)| {
                format!(
                    "image {index} layout={} queue_family={:#x}",
                    texture.layout, texture.queue_family_index
                )
            })
            .collect();
        let fence = if self.record.synchronization.fence == 0 {
            "null"
        } else {
            "set"
        };
        if wrong.is_empty() {
            format!("vulkan write-back ok images={count} fence={fence}")
        } else {
            format!(
                "ERROR vulkan write-back expected TRANSFER_SRC_OPTIMAL on VK_QUEUE_FAMILY_EXTERNAL: {}",
                wrong.join(", ")
            )
        }
    }

    /// Waits on the semaphore with an empty submission, as the runtime's
    /// own queue would, and waits for that submission to finish so the
    /// semaphore is unsignalled for the next frame.
    pub fn consume_semaphore(&mut self) -> Result<(), String> {
        let wait_semaphores = [self.semaphore];
        let wait_stages = [vk::PipelineStageFlags::ALL_COMMANDS];
        let submit = vk::SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages);
        // SAFETY: live device objects; the fence is unsignalled (reset after
        // every wait below).
        unsafe {
            self.device
                .queue_submit(self.queue, &[submit], self.consume_fence)
        }
        .map_err(|error| format!("vkQueueSubmit of the semaphore wait: {error}"))?;
        let limit_nanoseconds = u64::try_from(SEMAPHORE_WAIT_LIMIT.as_nanos()).unwrap_or(u64::MAX);
        // SAFETY: live fence of this device.
        let waited = unsafe {
            self.device
                .wait_for_fences(&[self.consume_fence], true, limit_nanoseconds)
        };
        match waited {
            Ok(()) => {
                // SAFETY: the fence is signalled and no longer in use.
                unsafe { self.device.reset_fences(&[self.consume_fence]) }
                    .map_err(|error| format!("vkResetFences: {error}"))
            }
            Err(vk::Result::TIMEOUT) => {
                // The submission stays pending; later frames reuse nothing of
                // it, but the queue may stay blocked.
                self.semaphore_state_unknown = true;
                Err(format!(
                    "vulkan semaphore not signalled within {SEMAPHORE_WAIT_LIMIT:?}"
                ))
            }
            Err(error) => Err(format!("vkWaitForFences: {error}")),
        }
    }

    pub fn note_discarded_frame(&mut self) {
        self.semaphore_state_unknown = true;
    }

    /// After a frame dropped unshown: waits for the device to go idle, so no
    /// signal is pending, and replaces the semaphore and fence.
    pub fn settle_after_discard(&mut self) -> Result<bool, String> {
        if !self.semaphore_state_unknown {
            return Ok(false);
        }
        // SAFETY: live device; the stub's own submissions are done or abandoned.
        unsafe { self.device.device_wait_idle() }
            .map_err(|error| format!("vkDeviceWaitIdle: {error}"))?;
        let replacement = self.create_semaphore()?;
        // SAFETY: the device is idle, so nothing uses the old semaphore or fence.
        let fence = unsafe {
            self.device.destroy_semaphore(self.semaphore, None);
            self.device.destroy_fence(self.consume_fence, None);
            self.device
                .create_fence(&vk::FenceCreateInfo::default(), None)
        }
        .map_err(|error| format!("vkCreateFence: {error}"))?;
        self.semaphore = replacement;
        self.consume_fence = fence;
        self.record.synchronization.semaphore = replacement.as_raw();
        self.semaphore_state_unknown = false;
        Ok(true)
    }
}

impl Drop for VulkanSender {
    fn drop(&mut self) {
        if self.semaphore_state_unknown {
            // SAFETY: live device; a discarded frame's signal may still run.
            if let Err(error) = unsafe { self.device.device_wait_idle() } {
                crate::state::note(format!("ERROR vkDeviceWaitIdle at sender destroy: {error}"));
            }
        }
        // SAFETY: every shown frame's semaphore was consumed with a fence
        // wait, and a discarded one waited for idle above, so the caller's
        // work on these objects is finished. Null handles are ignored.
        unsafe {
            for created in &self.images {
                self.device.destroy_image(created.image, None);
                self.device.free_memory(created.memory, None);
            }
            self.device.destroy_semaphore(self.semaphore, None);
            self.device.destroy_fence(self.consume_fence, None);
        }
    }
}
