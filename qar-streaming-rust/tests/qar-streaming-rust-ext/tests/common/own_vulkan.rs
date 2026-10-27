//! Our own Vulkan device, as an application with its own renderer brings it:
//! picked by the session's adapter, created with the extensions QAROS needs,
//! and filling the sender's images on the GPU only. Each view gets its
//! texel through `vkCmdFillBuffer` and a buffer-to-image copy, so no pixel
//! crosses the CPU.

#![allow(dead_code)]

use ash::vk;
use qaros::vulkan::{SHOWN_LAYOUT, VulkanFrame, VulkanImage, VulkanStreamParams};
use qaros::{Eye, GraphicsAdapter, PixelFormat, ViewKind};

use super::pattern;

pub struct OwnDevice {
    _entry: ash::Entry,
    instance: ash::Instance,
    physical_device: vk::PhysicalDevice,
    device: ash::Device,
    queue_family: u32,
    queue: vk::Queue,
    command_pool: vk::CommandPool,
    command_buffer: vk::CommandBuffer,
    fence: vk::Fence,
    view_buffers: Vec<(vk::Buffer, vk::DeviceMemory, vk::DeviceSize)>,
    pub device_name: String,
}

impl OwnDevice {
    /// The physical device whose LUID or UUID matches `adapter` (the
    /// session's), else the first discrete GPU.
    pub fn create(adapter: Option<GraphicsAdapter>) -> Self {
        // SAFETY: loading the system's Vulkan loader has no preconditions.
        let entry = unsafe { ash::Entry::load() }.expect("Vulkan loader");
        let application = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_1);
        let instance_info = vk::InstanceCreateInfo::default().application_info(&application);
        // SAFETY: valid create info.
        let instance =
            unsafe { entry.create_instance(&instance_info, None) }.expect("vkCreateInstance");
        // SAFETY: live instance.
        let candidates =
            unsafe { instance.enumerate_physical_devices() }.expect("physical devices");
        let mut chosen = None;
        for candidate in candidates {
            let mut identifiers = vk::PhysicalDeviceIDProperties::default();
            let mut properties =
                vk::PhysicalDeviceProperties2::default().push_next(&mut identifiers);
            // SAFETY: live physical device; Vulkan 1.1 instance.
            unsafe { instance.get_physical_device_properties2(candidate, &mut properties) };
            let name = properties
                .properties
                .device_name_as_c_str()
                .map(|text| text.to_string_lossy().into_owned())
                .unwrap_or_default();
            let device_type = properties.properties.device_type;
            let luid =
                (identifiers.device_luid_valid == vk::TRUE).then_some(identifiers.device_luid);
            let matches_adapter =
                adapter.is_some_and(|wanted| wanted.matches(luid, identifiers.device_uuid));
            eprintln!(
                "Vulkan device {name:?} type {device_type:?} luid {luid:?} matches session adapter: {matches_adapter}"
            );
            let preferred = matches_adapter
                || (adapter.is_none() && device_type == vk::PhysicalDeviceType::DISCRETE_GPU);
            if preferred && chosen.is_none() {
                chosen = Some((candidate, name));
            }
        }
        let (physical_device, device_name) =
            chosen.expect("a Vulkan device on the session's adapter");
        // SAFETY: live physical device.
        let families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
        let queue_family = families
            .iter()
            .position(|family| family.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .and_then(|index| u32::try_from(index).ok())
            .expect("a graphics queue family");
        let extensions: Vec<*const std::ffi::c_char> = qaros::vulkan::required_device_extensions()
            .iter()
            .map(|name| name.as_ptr())
            .collect();
        let priorities = [1.0];
        let queue_info = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family)
            .queue_priorities(&priorities)];
        let device_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_info)
            .enabled_extension_names(&extensions);
        // SAFETY: valid create info for this physical device.
        let device = unsafe { instance.create_device(physical_device, &device_info, None) }
            .expect("vkCreateDevice with the external memory and semaphore extensions");
        // SAFETY: the device has queue 0 in this family.
        let queue = unsafe { device.get_device_queue(queue_family, 0) };
        let pool_info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(queue_family)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        // SAFETY: live device.
        let command_pool =
            unsafe { device.create_command_pool(&pool_info, None) }.expect("command pool");
        let allocate_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);
        // SAFETY: live pool.
        let command_buffer =
            unsafe { device.allocate_command_buffers(&allocate_info) }.expect("command buffer")[0];
        // SAFETY: live device.
        let fence =
            unsafe { device.create_fence(&vk::FenceCreateInfo::default(), None) }.expect("fence");
        Self {
            _entry: entry,
            instance,
            physical_device,
            device,
            queue_family,
            queue,
            command_pool,
            command_buffer,
            fence,
            view_buffers: Vec::new(),
            device_name,
        }
    }

    pub fn stream_params(&self) -> VulkanStreamParams {
        VulkanStreamParams {
            physical_device: self.physical_device,
            device: self.device.handle(),
        }
    }

    /// Fills every view of the frame with the pattern on the GPU, releases
    /// the images to the external family in the shown layout, signals the
    /// frame's semaphore, and writes the state back. Waits for the GPU, so
    /// the next frame can reuse the command buffer.
    pub fn render(&mut self, frame: &mut VulkanFrame<'_>, frame_number: u64) {
        let images = frame.images();
        let views = frame.views();
        for (index, view) in views.iter().enumerate() {
            let texel_count = u64::from(view.end_x.abs_diff(view.start_x))
                * u64::from(view.end_y.abs_diff(view.start_y));
            self.ensure_view_buffer(index, texel_count * 4);
        }
        self.begin_commands();
        let acquire: Vec<_> = images
            .iter()
            .map(|image| self.acquire(image, vk::ImageLayout::TRANSFER_DST_OPTIMAL))
            .collect();
        self.image_barrier(&acquire);
        for (index, view) in views.iter().enumerate() {
            let image = &images[view.texture_index as usize];
            let left_eye = view.eye != Eye::Right;
            let texel = match view.kind {
                ViewKind::Depth => pattern::texel_bytes(
                    pattern::raw_format(view.format),
                    [0; 3],
                    pattern::expected_depth(left_eye),
                ),
                _ => pattern::texel_bytes(
                    pattern::raw_format(view.format),
                    pattern::expected_rgb(left_eye, frame_number),
                    0.0,
                ),
            };
            let texel_word = u32::from_le_bytes([texel[0], texel[1], texel[2], texel[3]]);
            let (buffer, _, size) = self.view_buffers[index];
            let region = vk::BufferImageCopy::default()
                .image_subresource(vk::ImageSubresourceLayers {
                    aspect_mask: aspect_of(image),
                    mip_level: 0,
                    base_array_layer: view.array_layer_index,
                    layer_count: 1,
                })
                .image_offset(vk::Offset3D {
                    x: i32::try_from(view.start_x.min(view.end_x)).expect("offset"),
                    y: i32::try_from(view.start_y.min(view.end_y)).expect("offset"),
                    z: 0,
                })
                .image_extent(vk::Extent3D {
                    width: view.end_x.abs_diff(view.start_x),
                    height: view.end_y.abs_diff(view.start_y),
                    depth: 1,
                });
            let filled = [vk::BufferMemoryBarrier::default()
                .buffer(buffer)
                .offset(0)
                .size(vk::WHOLE_SIZE)
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(vk::AccessFlags::TRANSFER_READ)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)];
            // SAFETY: recording; the buffer and image are live; the image is
            // in TRANSFER_DST_OPTIMAL after the acquire barrier.
            unsafe {
                self.device
                    .cmd_fill_buffer(self.command_buffer, buffer, 0, size, texel_word);
                self.device.cmd_pipeline_barrier(
                    self.command_buffer,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::DependencyFlags::empty(),
                    &[],
                    &filled,
                    &[],
                );
                self.device.cmd_copy_buffer_to_image(
                    self.command_buffer,
                    buffer,
                    image.image,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &[region],
                );
            }
        }
        let release: Vec<_> = images
            .iter()
            .map(|image| {
                transition(
                    image,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    SHOWN_LAYOUT,
                    self.queue_family,
                    vk::QUEUE_FAMILY_EXTERNAL,
                )
            })
            .collect();
        self.image_barrier(&release);
        self.submit_and_wait(frame.semaphore());
        frame.release_to_external(SHOWN_LAYOUT);
        frame.set_fence(vk::Fence::null());
    }

    fn ensure_view_buffer(&mut self, index: usize, size: vk::DeviceSize) {
        if let Some(&(_, _, existing)) = self.view_buffers.get(index)
            && existing >= size
        {
            return;
        }
        let buffer_info = vk::BufferCreateInfo::default()
            .size(size)
            .usage(vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC);
        // SAFETY: live device and physical device; the old buffer (if any)
        // is idle because every submission is waited for.
        unsafe {
            let buffer = self
                .device
                .create_buffer(&buffer_info, None)
                .expect("view buffer");
            let requirements = self.device.get_buffer_memory_requirements(buffer);
            let properties = self
                .instance
                .get_physical_device_memory_properties(self.physical_device);
            let memory_type_index = (0..properties.memory_type_count)
                .find(|&type_index| {
                    requirements.memory_type_bits & (1 << type_index) != 0
                        && properties.memory_types[type_index as usize]
                            .property_flags
                            .contains(vk::MemoryPropertyFlags::DEVICE_LOCAL)
                })
                .expect("device-local memory");
            let memory = self
                .device
                .allocate_memory(
                    &vk::MemoryAllocateInfo::default()
                        .allocation_size(requirements.size)
                        .memory_type_index(memory_type_index),
                    None,
                )
                .expect("view buffer memory");
            self.device
                .bind_buffer_memory(buffer, memory, 0)
                .expect("bind view buffer");
            if index < self.view_buffers.len() {
                let (old_buffer, old_memory, _) = self.view_buffers[index];
                self.device.destroy_buffer(old_buffer, None);
                self.device.free_memory(old_memory, None);
                self.view_buffers[index] = (buffer, memory, size);
            } else {
                self.view_buffers.push((buffer, memory, size));
            }
        }
    }

    fn begin_commands(&self) {
        let begin = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        // SAFETY: the command buffer is idle (every submission is waited for).
        unsafe {
            self.device
                .reset_command_buffer(self.command_buffer, vk::CommandBufferResetFlags::empty())
                .expect("reset command buffer");
            self.device
                .begin_command_buffer(self.command_buffer, &begin)
                .expect("begin command buffer");
        }
    }

    fn submit_and_wait(&self, semaphore: vk::Semaphore) {
        let command_buffers = [self.command_buffer];
        let signal_semaphores = [semaphore];
        let submit = vk::SubmitInfo::default()
            .command_buffers(&command_buffers)
            .signal_semaphores(&signal_semaphores);
        // SAFETY: recorded command buffer, live semaphore and fence.
        unsafe {
            self.device
                .end_command_buffer(self.command_buffer)
                .expect("end command buffer");
            self.device
                .queue_submit(self.queue, &[submit], self.fence)
                .expect("submit");
            self.device
                .wait_for_fences(&[self.fence], true, u64::MAX)
                .expect("wait");
            self.device
                .reset_fences(&[self.fence])
                .expect("reset fence");
        }
    }

    fn image_barrier(&self, barriers: &[vk::ImageMemoryBarrier<'_>]) {
        // SAFETY: recording command buffer; the barriers name live images.
        unsafe {
            self.device.cmd_pipeline_barrier(
                self.command_buffer,
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                barriers,
            );
        }
    }

    /// From the reported state onto our queue family; an ownership transfer
    /// only when another family (the external one) owns the image.
    fn acquire(
        &self,
        image: &VulkanImage,
        new_layout: vk::ImageLayout,
    ) -> vk::ImageMemoryBarrier<'static> {
        if image.queue_family_index == vk::QUEUE_FAMILY_IGNORED
            || image.queue_family_index == self.queue_family
        {
            transition(
                image,
                image.layout,
                new_layout,
                vk::QUEUE_FAMILY_IGNORED,
                vk::QUEUE_FAMILY_IGNORED,
            )
        } else {
            transition(
                image,
                image.layout,
                new_layout,
                image.queue_family_index,
                self.queue_family,
            )
        }
    }
}

impl Drop for OwnDevice {
    fn drop(&mut self) {
        // SAFETY: every submission was waited for; senders on this device
        // are dropped before it.
        unsafe {
            let _ = self.device.device_wait_idle();
            for &(buffer, memory, _) in &self.view_buffers {
                self.device.destroy_buffer(buffer, None);
                self.device.free_memory(memory, None);
            }
            self.device.destroy_fence(self.fence, None);
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

fn aspect_of(image: &VulkanImage) -> vk::ImageAspectFlags {
    if image.size.format == PixelFormat::D32Float {
        vk::ImageAspectFlags::DEPTH
    } else {
        vk::ImageAspectFlags::COLOR
    }
}

fn transition(
    image: &VulkanImage,
    old_layout: vk::ImageLayout,
    new_layout: vk::ImageLayout,
    source_family: u32,
    destination_family: u32,
) -> vk::ImageMemoryBarrier<'static> {
    vk::ImageMemoryBarrier::default()
        .image(image.image)
        .old_layout(old_layout)
        .new_layout(new_layout)
        .src_queue_family_index(source_family)
        .dst_queue_family_index(destination_family)
        .src_access_mask(vk::AccessFlags::MEMORY_WRITE)
        .dst_access_mask(vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE)
        .subresource_range(vk::ImageSubresourceRange {
            aspect_mask: aspect_of(image),
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: image.size.array_layers.max(1),
        })
}
