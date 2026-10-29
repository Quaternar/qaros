//! (b) The CPU render path against the real runtime: an extension hub mints
//! an invite, the application session onboards with it, an extension peer
//! is the target, the application's CPU render sender streams to the peer's
//! render receiver, and the receiver's frames carry the known picture.
//!
//! `QAROS_E2E_EYE=<width>x<height>` sets the eye size (default 960x1000),
//! `QAROS_E2E_FRAMES` the frame count (default 60).

#![cfg(feature = "e2e")]
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

mod common;

use std::time::Duration;

use common::pattern;
use common::scenario::{self, Lending, Scenario};
use qaros::BlockingOptions;

#[test]
fn cpu_frames_reach_the_extension_receiver_with_the_known_picture() {
    let Some(bin) = common::native_bin() else {
        return;
    };
    let (eye_width, eye_height) = common::eye_size_from_environment();
    let frame_total = common::frame_count_from_environment();
    let world = Scenario::start(&bin, "cpu-render");
    let layout = world.layout(eye_width, eye_height);
    let mut receiver = world
        .viewer
        .create_render_receiver(&layout, None)
        .expect("create the CPU render receiver");

    let cancel = world.cancel_token(Duration::from_secs(30));
    let config = world.sender_config(world.viewer_id);
    let mut sender = scenario::connect(&receiver, || {
        world
            .session
            .create_render_sender(
                &config,
                &BlockingOptions {
                    progress: None,
                    cancel: Some(&cancel),
                },
            )
            .expect("create the CPU render sender")
    });

    let label = format!("CPU render path, {eye_width}x{eye_height} per eye");
    let stats = scenario::run_frames(
        &label,
        frame_total,
        &mut receiver,
        &Lending::Cpu,
        &layout,
        &mut sender,
        |frame, frame_number| {
            let mut cpu_frame = frame.cpu_frame().expect("CPU frame");
            pattern::fill_cpu_frame(&mut cpu_frame, frame_number);
        },
    );
    assert_eq!(stats.frames, frame_total);

    let streams = world.hub.media_streams().expect("hub media streams");
    eprintln!("hub media-stream records: {}", streams.len());
    drop(sender);
    drop(receiver);
}
