//! The Vulkan render sender against `qaros-stub`, on a real Vulkan device.
//! Needs a Vulkan loader and driver the stub can open by leaf name
//! (`QAROS_STUB_VULKAN_LIBRARY` names another path).
//! Without a loader or a device each test prints why it is skipped and passes, unless
//! `QAROS_REQUIRE_VULKAN` is set, which turns the skip into a failure.

use ash::vk;
use qaros::vulkan::{SHOWN_LAYOUT, VulkanFrame, VulkanImage, VulkanStreamParams};
use qaros::{FrameLayout, FrameViewSpec, PixelFormat, RenderSender, RenderSenderConfig};

use super::*;

const QUEUE_FAMILY: u32 = 0;
const CLEAR_MAGENTA: [f32; 4] = [1.0, 0.0, 1.0, 1.0];
const MAGENTA_BYTES: [u8; 4] = [255, 0, 255, 255];

struct TestDevice {
    _entry: ash::Entry,
    instance: ash::Instance,
    physical_device: vk::PhysicalDevice,
    device: ash::Device,
    queue: vk::Queue,
    command_pool: vk::CommandPool,
    command_buffer: vk::CommandBuffer,
    fence: vk::Fence,
}

/// The skip of a test that needs Vulkan: `None` after printing why, or a failure when
/// `QAROS_REQUIRE_VULKAN` is set.
fn skipped(reason: &str) -> Option<TestDevice> {
    assert!(
        std::env::var_os("QAROS_REQUIRE_VULKAN").is_none(),
        "{reason}"
    );
    eprintln!("skipped: {reason}");
    None
}

impl TestDevice {
    /// `None` (with the reason printed) when no Vulkan loader, driver or device is reachable.
    fn create() -> Option<Self> {
        // SAFETY: loading the Vulkan loader has no preconditions.
        let entry = match unsafe { ash::Entry::load() } {
            Ok(entry) => entry,
            Err(error) => return skipped(&format!("no Vulkan loader ({error})")),
        };
        // SAFETY: valid entry.
        let instance_extensions = unsafe { entry.enumerate_instance_extension_properties(None) }
            .expect("instance extensions");
        let has_portability_enumeration = instance_extensions.iter().any(|extension| {
            extension.extension_name_as_c_str() == Ok(ash::khr::portability_enumeration::NAME)
        });
        let mut enabled_instance_extensions = Vec::new();
        let mut instance_flags = vk::InstanceCreateFlags::empty();
        if has_portability_enumeration {
            enabled_instance_extensions.push(ash::khr::portability_enumeration::NAME.as_ptr());
            instance_flags |= vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR;
        }
        let application = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_1);
        let instance_info = vk::InstanceCreateInfo::default()
            .application_info(&application)
            .enabled_extension_names(&enabled_instance_extensions)
            .flags(instance_flags);
        // SAFETY: valid create info.
        let instance = match unsafe { entry.create_instance(&instance_info, None) } {
            Ok(instance) => instance,
            Err(error) => return skipped(&format!("no Vulkan driver (vkCreateInstance: {error})")),
        };
        // SAFETY: live instance.
        let physical_devices =
            unsafe { instance.enumerate_physical_devices() }.expect("physical devices");
        let Some(&physical_device) = physical_devices.first() else {
            // SAFETY: nothing was created from the instance.
            unsafe { instance.destroy_instance(None) };
            return skipped("no Vulkan physical device");
        };
        // SAFETY: live physical device.
        let families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
        assert!(
            families
                .first()
                .is_some_and(|family| family.queue_flags.contains(vk::QueueFlags::GRAPHICS)),
            "queue family {QUEUE_FAMILY}, the one the stub waits on, must be a graphics family"
        );
        // SAFETY: live physical device.
        let device_extensions =
            unsafe { instance.enumerate_device_extension_properties(physical_device) }
                .expect("device extensions");
        let mut enabled_device_extensions: Vec<*const std::ffi::c_char> =
            qaros::vulkan::required_device_extensions()
                .iter()
                .map(|name| name.as_ptr())
                .collect();
        if device_extensions.iter().any(|extension| {
            extension.extension_name_as_c_str() == Ok(ash::khr::portability_subset::NAME)
        }) {
            enabled_device_extensions.push(ash::khr::portability_subset::NAME.as_ptr());
        }
        let priorities = [1.0];
        let queue_info = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(QUEUE_FAMILY)
            .queue_priorities(&priorities)];
        let device_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_info)
            .enabled_extension_names(&enabled_device_extensions);
        // SAFETY: valid create info for this physical device.
        let device = unsafe { instance.create_device(physical_device, &device_info, None) }
            .expect("vkCreateDevice");
        // SAFETY: the device has queue 0 in QUEUE_FAMILY.
        let queue = unsafe { device.get_device_queue(QUEUE_FAMILY, 0) };
        let pool_info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(QUEUE_FAMILY)
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
        Some(Self {
            _entry: entry,
            instance,
            physical_device,
            device,
            queue,
            command_pool,
            command_buffer,
            fence,
        })
    }

    fn stream_params(&self) -> VulkanStreamParams {
        VulkanStreamParams {
            physical_device: self.physical_device,
            device: self.device.handle(),
        }
    }

    /// Records commands, submits them signalling `semaphore`, and waits.
    fn submit_and_wait(&self, semaphore: vk::Semaphore, record: impl FnOnce(vk::CommandBuffer)) {
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
        record(self.command_buffer);
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

    fn barrier(&self, command_buffer: vk::CommandBuffer, barriers: &[vk::ImageMemoryBarrier<'_>]) {
        // SAFETY: recording command buffer; barriers name live images.
        unsafe {
            self.device.cmd_pipeline_barrier(
                command_buffer,
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                barriers,
            );
        }
    }

    fn host_buffer(&self, size: vk::DeviceSize) -> (vk::Buffer, vk::DeviceMemory) {
        let buffer_info = vk::BufferCreateInfo::default()
            .size(size)
            .usage(vk::BufferUsageFlags::TRANSFER_DST);
        // SAFETY: live device and physical device.
        unsafe {
            let buffer = self
                .device
                .create_buffer(&buffer_info, None)
                .expect("buffer");
            let requirements = self.device.get_buffer_memory_requirements(buffer);
            let properties = self
                .instance
                .get_physical_device_memory_properties(self.physical_device);
            let wanted =
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
            let memory_type_index = (0..properties.memory_type_count)
                .find(|&index| {
                    requirements.memory_type_bits & (1 << index) != 0
                        && properties.memory_types[index as usize]
                            .property_flags
                            .contains(wanted)
                })
                .expect("host-visible memory");
            let allocate_info = vk::MemoryAllocateInfo::default()
                .allocation_size(requirements.size)
                .memory_type_index(memory_type_index);
            let memory = self
                .device
                .allocate_memory(&allocate_info, None)
                .expect("memory");
            self.device
                .bind_buffer_memory(buffer, memory, 0)
                .expect("bind");
            (buffer, memory)
        }
    }
}

impl Drop for TestDevice {
    fn drop(&mut self) {
        // SAFETY: every submission was waited for; the render senders using
        // this device are dropped before it (see the tests).
        unsafe {
            let _ = self.device.device_wait_idle();
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

/// A layout transition with an optional ownership transfer, over the whole
/// image.
fn transition(
    image: &VulkanImage,
    new_layout: vk::ImageLayout,
    source_family: u32,
    destination_family: u32,
) -> vk::ImageMemoryBarrier<'static> {
    vk::ImageMemoryBarrier::default()
        .image(image.image)
        .old_layout(image.layout)
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

/// The cube's acquire: from the reported state onto this queue family, with
/// no ownership transfer when the image is unowned or already ours.
fn acquire(image: &VulkanImage, new_layout: vk::ImageLayout) -> vk::ImageMemoryBarrier<'static> {
    if image.queue_family_index == vk::QUEUE_FAMILY_IGNORED
        || image.queue_family_index == QUEUE_FAMILY
    {
        transition(
            image,
            new_layout,
            vk::QUEUE_FAMILY_IGNORED,
            vk::QUEUE_FAMILY_IGNORED,
        )
    } else {
        transition(image, new_layout, image.queue_family_index, QUEUE_FAMILY)
    }
}

/// The cube's release: to the shown layout, owned by the external family.
fn release(
    image: &VulkanImage,
    current_layout: vk::ImageLayout,
) -> vk::ImageMemoryBarrier<'static> {
    let mut current = *image;
    current.layout = current_layout;
    transition(
        &current,
        SHOWN_LAYOUT,
        QUEUE_FAMILY,
        vk::QUEUE_FAMILY_EXTERNAL,
    )
}

fn separated_stereo(target_peer: PeerId) -> RenderSenderConfig {
    let mut config = RenderSenderConfig::new(target_peer);
    config.layout = FrameLayout::SeparatedTextures;
    config.color_format = PixelFormat::R8G8B8A8;
    config.views = [Eye::Left, Eye::Right]
        .into_iter()
        .flat_map(|eye| [(eye, ViewKind::Color), (eye, ViewKind::Depth)])
        .enumerate()
        .map(|(index, (eye, kind))| FrameViewSpec {
            kind,
            eye,
            texture_index: u32::try_from(index).expect("small index"),
        })
        .collect();
    config
}

fn vulkan_sender(session: &qaros::Session, gpu: &TestDevice) -> RenderSender {
    // SAFETY: the device is live and outlives the sender (dropped first in
    // every test); no Windows extensions are needed off Windows.
    unsafe {
        session.create_vulkan_render_sender(
            &separated_stereo(VISUALIZER_PEER),
            gpu.stream_params(),
            &BlockingOptions::default(),
        )
    }
    .expect("vulkan sender")
}

/// The cube's frame body: clears every image (magenta color, depth 0.25)
/// from its reported state and releases it to the external queue family,
/// signalling the frame's semaphore.
fn clear_and_release(gpu: &TestDevice, vulkan_frame: &VulkanFrame<'_>) {
    let images = vulkan_frame.images();
    gpu.submit_and_wait(vulkan_frame.semaphore(), |command_buffer| {
        let to_transfer: Vec<_> = images
            .iter()
            .map(|image| acquire(image, vk::ImageLayout::TRANSFER_DST_OPTIMAL))
            .collect();
        gpu.barrier(command_buffer, &to_transfer);
        for image in &images {
            let range = transition(image, image.layout, 0, 0).subresource_range;
            // SAFETY: recording; the image is in TRANSFER_DST_OPTIMAL.
            unsafe {
                if aspect_of(image) == vk::ImageAspectFlags::DEPTH {
                    gpu.device.cmd_clear_depth_stencil_image(
                        command_buffer,
                        image.image,
                        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                        &vk::ClearDepthStencilValue {
                            depth: 0.25,
                            stencil: 0,
                        },
                        &[range],
                    );
                } else {
                    gpu.device.cmd_clear_color_image(
                        command_buffer,
                        image.image,
                        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                        &vk::ClearColorValue {
                            float32: CLEAR_MAGENTA,
                        },
                        &[range],
                    );
                }
            }
        }
        let releases: Vec<_> = images
            .iter()
            .map(|image| release(image, vk::ImageLayout::TRANSFER_DST_OPTIMAL))
            .collect();
        gpu.barrier(command_buffer, &releases);
    });
}

/// Acquires one color image from the external family, copies it into a
/// host buffer, releases it again, and returns its bytes.
fn read_back(gpu: &TestDevice, vulkan_frame: &VulkanFrame<'_>, color: &VulkanImage) -> Vec<u8> {
    let width = color.size.width;
    let height = color.size.height;
    let byte_count = u64::from(width) * u64::from(height) * 4;
    let (buffer, memory) = gpu.host_buffer(byte_count);
    gpu.submit_and_wait(vulkan_frame.semaphore(), |command_buffer| {
        gpu.barrier(command_buffer, &[acquire(color, SHOWN_LAYOUT)]);
        let region = vk::BufferImageCopy::default()
            .image_subresource(vk::ImageSubresourceLayers {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                mip_level: 0,
                base_array_layer: 0,
                layer_count: 1,
            })
            .image_extent(vk::Extent3D {
                width,
                height,
                depth: 1,
            });
        // SAFETY: recording; the image is in TRANSFER_SRC_OPTIMAL and the
        // buffer holds the whole image.
        unsafe {
            gpu.device.cmd_copy_image_to_buffer(
                command_buffer,
                color.image,
                SHOWN_LAYOUT,
                buffer,
                &[region],
            );
        }
        gpu.barrier(command_buffer, &[release(color, SHOWN_LAYOUT)]);
    });
    // SAFETY: the copy finished (waited for); host-coherent memory.
    unsafe {
        let mapped = gpu
            .device
            .map_memory(memory, 0, byte_count, vk::MemoryMapFlags::empty())
            .expect("map");
        let bytes = std::slice::from_raw_parts(
            mapped.cast::<u8>(),
            usize::try_from(byte_count).expect("size"),
        )
        .to_vec();
        gpu.device.unmap_memory(memory);
        gpu.device.destroy_buffer(buffer, None);
        gpu.device.free_memory(memory, None);
        bytes
    }
}

#[test]
fn frames_round_trip_image_state_and_pixels_through_the_stub() {
    let Some(gpu) = TestDevice::create() else {
        return;
    };
    let fixture = Fixture::new();
    let library = load_stub();
    let (_runtime, session) = onboarded_session(&fixture, &library);
    let mut sender = vulkan_sender(&session, &gpu);

    let (left_color, left_color_image, initial_images) = {
        let state = sender
            .vulkan_frame_state()
            .expect("frame state before any frame");
        assert_ne!(state.semaphore(), vk::Semaphore::null());
        assert!(state.view_index(Eye::Right, ViewKind::Depth).is_some());
        let left_color = state
            .view_index(Eye::Left, ViewKind::Color)
            .expect("left color view");
        let left_color_image = state.views()[left_color].texture_index as usize;
        (left_color, left_color_image, state.images())
    };
    assert_eq!(initial_images.len(), 4);
    for image in &initial_images {
        assert_ne!(image.image, vk::Image::null());
        assert_ne!(image.memory, vk::DeviceMemory::null());
        assert_eq!(image.layout, vk::ImageLayout::UNDEFINED);
        assert_eq!(image.queue_family_index, vk::QUEUE_FAMILY_IGNORED);
    }
    assert_eq!(initial_images[1].size.format, PixelFormat::D32Float);

    let mut frame = sender
        .try_begin_owned_frame()
        .expect("try begin")
        .expect("ready");
    let refused = sender.vulkan_frame_state();
    assert!(
        matches!(&refused, Err(Error::InvalidArgument(_))),
        "{refused:?}"
    );
    frame.view(left_color).expect("left eye pose");
    let mut vulkan_frame = frame.vulkan_frame().expect("vulkan frame");
    clear_and_release(&gpu, &vulkan_frame);
    vulkan_frame.release_to_external(SHOWN_LAYOUT);
    vulkan_frame.set_fence(vk::Fence::null());
    show(frame);

    // The stub reports the written-back state on the next frame.
    let mut frame = sender
        .begin_owned_frame(&BlockingOptions::default())
        .expect("second frame");
    let mut vulkan_frame = frame.vulkan_frame().expect("vulkan frame");
    let images = vulkan_frame.images();
    for (image, initial) in images.iter().zip(&initial_images) {
        assert_eq!(image.layout, SHOWN_LAYOUT);
        assert_eq!(image.queue_family_index, vk::QUEUE_FAMILY_EXTERNAL);
        assert_eq!(image.image, initial.image);
    }
    let pixels = read_back(&gpu, &vulkan_frame, &images[left_color_image]);
    vulkan_frame.release_to_external(SHOWN_LAYOUT);
    vulkan_frame.set_fence(vk::Fence::null());
    show(frame);
    assert!(
        pixels
            .as_chunks::<4>()
            .0
            .iter()
            .all(|pixel| *pixel == MAGENTA_BYTES),
        "first pixels {:?}",
        &pixels[..8]
    );

    drop(sender);
    let calls = fixture.control.calls();
    assert!(calls.contains(&"vulkan sender created images=4 views=4".to_string()));
    assert_eq!(
        fixture.control.entries_starting_with("vulkan write-back"),
        vec!["vulkan write-back ok images=4 fence=null".to_string(); 2]
    );
    assert_eq!(
        fixture.control.entries_starting_with("vulkan semaphore"),
        vec![
            "vulkan semaphore consumed frame=0".to_string(),
            "vulkan semaphore consumed frame=1".to_string(),
        ]
    );
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    drop(session);
    drop(library);
}

#[test]
fn a_missing_write_back_is_flagged_and_a_discarded_frame_gets_a_fresh_semaphore() {
    let Some(gpu) = TestDevice::create() else {
        return;
    };
    let fixture = Fixture::new();
    let library = load_stub();
    let (_runtime, session) = onboarded_session(&fixture, &library);
    let mut sender = vulkan_sender(&session, &gpu);

    // Frame 0: signals the semaphore but writes nothing back.
    let mut frame = sender
        .begin_owned_frame(&BlockingOptions::default())
        .expect("frame 0");
    let semaphore = frame.vulkan_frame().expect("vulkan frame").semaphore();
    gpu.submit_and_wait(semaphore, |_| {});
    show(frame);
    let flagged = fixture
        .control
        .entries_starting_with("ERROR vulkan write-back");
    assert_eq!(flagged.len(), 1, "{:?}", fixture.control.calls());
    assert!(flagged[0].contains("image 0 layout=0 queue_family=0xffffffff"));

    // Frame 1: signalled, then dropped unshown, as the cube's Discard does.
    let mut frame = sender
        .begin_owned_frame(&BlockingOptions::default())
        .expect("frame 1");
    let semaphore = frame.vulkan_frame().expect("vulkan frame").semaphore();
    gpu.submit_and_wait(semaphore, |_| {});
    drop(frame);

    // Frame 2: shown properly on the replaced semaphore.
    let mut frame = sender
        .begin_owned_frame(&BlockingOptions::default())
        .expect("frame 2");
    let mut vulkan_frame = frame.vulkan_frame().expect("vulkan frame");
    gpu.submit_and_wait(vulkan_frame.semaphore(), |_| {});
    vulkan_frame.release_to_external(SHOWN_LAYOUT);
    show(frame);
    drop(sender);

    let calls = fixture.control.calls();
    let dropped = position_of(&calls, "vulkan frame 1 dropped unshown");
    let replaced = position_of(&calls, "vulkan discarded frame: semaphore replaced");
    let consumed = position_of(&calls, "vulkan semaphore consumed frame=2");
    assert!(dropped < replaced && replaced < consumed, "{calls:?}");
    assert_eq!(
        fixture.control.entries_starting_with("ERROR").len(),
        1,
        "only the missing write-back is an error: {calls:?}"
    );
    drop(session);
    drop(library);
}

#[test]
fn an_async_vulkan_sender_hands_out_its_images_once_polled() {
    let Some(gpu) = TestDevice::create() else {
        return;
    };
    let fixture = Fixture::new();
    fixture.control.set_render_sender_create_delay(20);
    let library = load_stub();
    let (_runtime, session) = onboarded_session(&fixture, &library);
    // SAFETY: the device is live and outlives the session (dropped last).
    let mut pending = unsafe {
        session.start_vulkan_render_sender(
            &separated_stereo(VISUALIZER_PEER),
            gpu.stream_params(),
            None,
            || {},
        )
    }
    .expect("start");
    assert!(pending.poll().is_none());
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut sender = loop {
        if let Some(outcome) = pending.poll() {
            break outcome.expect("vulkan sender");
        }
        assert!(Instant::now() < deadline, "no callback");
        std::thread::sleep(Duration::from_millis(1));
    };
    let images = sender.vulkan_frame_state().expect("frame state").images();
    assert_eq!(images.len(), 4);
    assert_eq!(images[1].size.format, PixelFormat::D32Float);
    assert!(images.iter().all(|image| image.image != vk::Image::null()));
    drop(sender);
    drop(pending);
    drop(session);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[test]
fn an_async_vulkan_sender_without_device_handles_fails_through_its_callback() {
    let fixture = Fixture::new();
    let library = load_stub();
    let (_runtime, session) = onboarded_session(&fixture, &library);
    let null_params = VulkanStreamParams {
        physical_device: vk::PhysicalDevice::null(),
        device: vk::Device::null(),
    };
    // SAFETY: deliberately invalid handles; the stub checks them before use.
    let mut pending = unsafe {
        session.start_vulkan_render_sender(
            &separated_stereo(VISUALIZER_PEER),
            null_params,
            None,
            || {},
        )
    }
    .expect("the call itself succeeds");
    let deadline = Instant::now() + Duration::from_secs(5);
    let refused = loop {
        if let Some(outcome) = pending.poll() {
            break outcome.expect_err("a sender without a device");
        }
        assert!(Instant::now() < deadline, "no callback");
        std::thread::sleep(Duration::from_millis(1));
    };
    assert_eq!(refused.code(), Some(StatusCode::UNCLASSIFIED), "{refused}");
    assert!(
        refused
            .to_string()
            .contains("needs a physical device and a device"),
        "{refused}"
    );
}

#[test]
fn a_vulkan_sender_without_device_handles_is_refused_by_the_stub() {
    let fixture = Fixture::new();
    let library = load_stub();
    let (_runtime, session) = onboarded_session(&fixture, &library);
    let null_params = VulkanStreamParams {
        physical_device: vk::PhysicalDevice::null(),
        device: vk::Device::null(),
    };
    // SAFETY: deliberately invalid handles; the stub checks them before use.
    let refused = unsafe {
        session.create_vulkan_render_sender(
            &separated_stereo(VISUALIZER_PEER),
            null_params,
            &BlockingOptions::default(),
        )
    };
    let Err(error) = refused else {
        panic!("a sender without a device was created");
    };
    assert!(
        error
            .to_string()
            .contains("needs a physical device and a device"),
        "{error}"
    );
}
