//! (c) The Vulkan render path against the real runtime, on the extension's
//! test device: our Vulkan render sender (`create_vulkan_render_sender` with
//! the device's `VulkanStreamParams`) hands out the frame's images, the
//! extension uploads the known picture into them and writes back their
//! state while signalling the frame's semaphore, and the receiver checks it.
//!
//! `QAROS_E2E_RECEIVER=vulkan` lends receiver frames as Vulkan images on the
//! same device (default `cpu`: CPU buffers). Eye size and frame count as in
//! `real_cpu_render`.

#![cfg(feature = "e2e")]
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

mod common;

use std::time::Duration;

use ash::vk;
use ash::vk::Handle;
use common::pattern;
use common::scenario::{self, Lending, Scenario};
use qaros::BlockingOptions;
use qaros::vulkan::VulkanStreamParams;

#[test]
fn vulkan_frames_on_the_extension_device_reach_the_receiver() {
    let Some(bin) = common::package_bin() else {
        return;
    };
    let (eye_width, eye_height) = common::eye_size_from_environment();
    let frame_total = common::frame_count_from_environment();
    let vulkan_receiver = std::env::var("QAROS_E2E_RECEIVER").is_ok_and(|kind| kind == "vulkan");
    let world = Scenario::start(&bin, "vulkan-render");
    let device = world
        .extension
        .create_vulkan_device()
        .expect("create the extension's Vulkan device");
    let params = device.params().expect("device stream parameters");
    eprintln!(
        "extension device: physical {:?}, device {:?}",
        params.physical_device, params.device
    );

    let layout = world.layout(eye_width, eye_height);
    let lending = if vulkan_receiver {
        Lending::Vulkan {
            device: &device,
            frame: device.create_frame(&layout).expect("receiver frame images"),
        }
    } else {
        Lending::Cpu
    };
    let mut receiver = world
        .viewer
        .create_render_receiver(&layout, vulkan_receiver.then_some(&device))
        .expect("create the render receiver");

    let cancel = world.cancel_token(Duration::from_secs(30));
    let config = world.sender_config(world.viewer_id);
    let stream_params = VulkanStreamParams {
        physical_device: vk::PhysicalDevice::from_raw(params.physical_device as u64),
        device: vk::Device::from_raw(params.device as u64),
    };
    let mut sender = scenario::connect(&receiver, || {
        // SAFETY: the extension's device was created for streaming (it is
        // what its own tests pass to senders) and outlives the sender.
        unsafe {
            world.session.create_vulkan_render_sender(
                &config,
                stream_params,
                &BlockingOptions {
                    progress: None,
                    cancel: Some(&cancel),
                },
            )
        }
        .expect("create the Vulkan render sender")
    });

    let receiver_kind = if vulkan_receiver { "Vulkan" } else { "CPU" };
    let label = format!(
        "Vulkan render path (extension device, {receiver_kind} receiver), {eye_width}x{eye_height} per eye"
    );
    let stats = scenario::run_frames(
        &label,
        frame_total,
        &mut receiver,
        &lending,
        &layout,
        &mut sender,
        |frame, frame_number| {
            let mut vulkan_frame = frame.vulkan_frame().expect("Vulkan frame");
            if frame_number < 2 {
                for image in vulkan_frame.images() {
                    eprintln!(
                        "frame {frame_number} sender image: {:?} layout {:?} queue family {:#x} format features {:#x}",
                        image.size,
                        image.layout,
                        image.queue_family_index,
                        image.format_features.as_raw()
                    );
                }
            }
            let record = vulkan_frame.as_raw_mut();
            // SAFETY: the record is the open frame's, borrowed for this block.
            let snapshot = unsafe { *record };
            let views = &snapshot.texture_views[..snapshot.texture_views_count];
            let sizes: Vec<_> = snapshot.textures[..snapshot.textures_count]
                .iter()
                .map(|texture| texture.size)
                .collect();
            let packed = pattern::packed_textures(views, &sizes, frame_number);
            for (index, texels) in packed.iter().enumerate() {
                // SAFETY: the sender's images live on this device; the frame
                // is open and nothing else touches them.
                unsafe { device.write_texture(record, index, texels) }.expect("upload a texture");
            }
            // SAFETY: as above; this writes back layout and queue family
            // and signals the frame's semaphore before show.
            unsafe { device.release_frame(record) }.expect("release the frame");
        },
    );
    assert_eq!(stats.frames, frame_total);
    drop(sender);
    drop(receiver);
    drop(lending);
    drop(device);
}
