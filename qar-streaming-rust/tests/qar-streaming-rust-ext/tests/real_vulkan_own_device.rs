//! (c) The Vulkan render path against the real runtime on our own device, as
//! an application with its own renderer runs it: a device on the session's adapter
//! (`Session::graphics_adapter`) with the Win32 external memory and
//! semaphore extensions, the sender's images filled on the GPU, released to
//! the external queue family in the shown layout, the frame's semaphore
//! signalled, the state written back, then `show`. The receiver lends CPU
//! buffers, or Vulkan images on the extension's device with
//! `QAROS_E2E_RECEIVER=vulkan`. Eye size and frame count as in
//! `real_cpu_render`.

#![cfg(feature = "e2e")]
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

mod common;

use std::time::Duration;

use common::own_vulkan::OwnDevice;
use common::scenario::{self, Lending, Scenario};
use qaros::BlockingOptions;

#[test]
fn vulkan_frames_rendered_on_our_own_device_reach_the_receiver() {
    let Some(bin) = common::native_bin() else {
        return;
    };
    let (eye_width, eye_height) = common::eye_size_from_environment();
    let frame_total = common::frame_count_from_environment();
    let vulkan_receiver = std::env::var("QAROS_E2E_RECEIVER").is_ok_and(|kind| kind == "vulkan");
    let world = Scenario::start(&bin, "vulkan-own-device");
    let adapter = world.session.graphics_adapter().expect("session adapter");
    let mut own_device = OwnDevice::create(adapter);
    eprintln!("rendering on {:?}", own_device.device_name);

    let receiver_device = vulkan_receiver.then(|| {
        world
            .extension
            .create_vulkan_device()
            .expect("the extension's Vulkan device for the receiver")
    });
    let layout = world.layout(eye_width, eye_height);
    let lending = match &receiver_device {
        Some(device) => Lending::Vulkan {
            device,
            frame: device.create_frame(&layout).expect("receiver frame images"),
        },
        None => Lending::Cpu,
    };
    let mut receiver = world
        .viewer
        .create_render_receiver(&layout, receiver_device.as_ref())
        .expect("create the render receiver");

    let cancel = world.cancel_token(Duration::from_secs(30));
    let config = world.sender_config(world.viewer_id);
    let stream_params = own_device.stream_params();
    let mut sender = scenario::connect(&receiver, || {
        // SAFETY: the device is on the session's adapter, has the required
        // extensions, and outlives the sender (dropped after it below).
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
        .expect("create the Vulkan render sender on our device")
    });

    let receiver_kind = if vulkan_receiver { "Vulkan" } else { "CPU" };
    let label = format!(
        "Vulkan render path (own device, GPU fill, {receiver_kind} receiver), {eye_width}x{eye_height} per eye"
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
            own_device.render(&mut vulkan_frame, frame_number);
        },
    );
    assert_eq!(stats.frames, frame_total);
    drop(sender);
    drop(receiver);
    drop(lending);
    drop(receiver_device);
    // The session ends before the device its sender streamed from, as in the C Vulkan cube.
    drop(world);
    drop(own_device);
}
