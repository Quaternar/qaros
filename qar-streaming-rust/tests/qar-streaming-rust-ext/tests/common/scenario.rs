//! One render scenario: the application session onboarded into an extension
//! hub, extension viewer peers as targets, and the frame loop that requests,
//! renders, shows, receives and checks frames.

#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use qar_streaming_rust_ext::{Extension, Hub, Peer, RenderReceiver, VulkanDevice, VulkanTestFrame};
use qaros::qaros_sys::{QarVideoFrameLayout, QarVideoFrameVulkan};
use qaros::{
    BlockingOptions, CancelToken, Frame, Library, PeerId, RenderSender, RenderSenderConfig,
    Runtime, Session, ShowParams,
};

use super::pattern;

/// Field order is drop order: peers, hub, session, runtime, extension, and
/// the base library last. Receivers, senders and devices made from it are
/// locals declared after it, so they drop first.
pub struct Scenario {
    pub viewer: Peer,
    pub viewer_id: PeerId,
    pub hub: Hub,
    pub session: Session,
    pub runtime: Runtime,
    pub extension: Extension,
    pub library: Library,
    pub scratch: PathBuf,
}

impl Scenario {
    pub fn start(bin: &Path, test_name: &str) -> Self {
        let loaded = super::load(bin, test_name);
        let runtime = super::create_runtime(&loaded.library, bin, &loaded.scratch, "application");
        let hub = loaded.extension.create_hub(false).expect("create a hub");
        let session = super::onboard_into(&hub, &loaded.library, &runtime, super::APPLICATION_NAME);
        eprintln!(
            "session adapter: {:?}",
            session.graphics_adapter().expect("session adapter")
        );
        let (viewer, viewer_id) = add_viewer(&hub, &session, "e2e-viewer");
        Self {
            viewer,
            viewer_id,
            hub,
            session,
            runtime,
            extension: loaded.extension,
            library: loaded.library,
            scratch: loaded.scratch,
        }
    }

    /// Sender options for `target`: the shared app volume and the stereo
    /// color and depth views.
    pub fn sender_config(&self, target: PeerId) -> RenderSenderConfig {
        let mut config = RenderSenderConfig::new(target);
        config.app_volume = Some(super::create_app_volume(&self.session, self.viewer_id));
        config.views = super::stereo_color_and_depth_views();
        config
    }

    pub fn cancel_token(&self, budget: Duration) -> CancelToken {
        self.library
            .create_cancel_token(Some(super::scaled(budget)))
            .expect("cancel token")
    }

    pub fn layout(&self, eye_width: u32, eye_height: u32) -> QarVideoFrameLayout {
        let layout = self
            .extension
            .layout_side_by_side(eye_width, eye_height)
            .expect("receiver layout");
        super::print_layout("receiver layout", &layout);
        layout
    }
}

/// A further extension peer, once the session lists it as a target app.
pub fn add_viewer(hub: &Hub, session: &Session, display_name: &str) -> (Peer, PeerId) {
    let viewer = hub
        .create_peer(Some(display_name))
        .expect("create a viewer peer");
    let viewer_id = PeerId(viewer.id().expect("viewer id").data);
    let listed = super::wait_for_target_app(session, viewer_id);
    eprintln!("the session lists the viewer as a target app: {listed:?}");
    (viewer, viewer_id)
}

/// Creates the sender with `create` and waits until `receiver` connects.
pub fn connect(receiver: &RenderReceiver, create: impl FnOnce() -> RenderSender) -> RenderSender {
    let started = Instant::now();
    let sender = create();
    eprintln!("render sender created in {:?}", started.elapsed());
    let connected = receiver
        .wait_connected(super::scaled(Duration::from_secs(15)))
        .expect("wait for the connection");
    assert!(connected, "the receiver connects once the sender exists");
    eprintln!(
        "receiver connected {:?} after sender creation began",
        started.elapsed()
    );
    eprintln!(
        "sender layout: {:?}",
        sender.layout().expect("sender layout")
    );
    if let Some(source_layout) = receiver.source_layout().expect("source layout") {
        super::print_layout("stream record's source layout", &source_layout);
    }
    sender
}

/// How the receiver gets the memory a frame is written into.
pub enum Lending<'device> {
    /// The receiver's own CPU buffers.
    Cpu,
    /// Exportable images on the extension's Vulkan device.
    Vulkan {
        device: &'device VulkanDevice,
        frame: VulkanTestFrame,
    },
}

/// What a frame loop measured.
#[derive(Debug)]
pub struct FrameStats {
    pub frames: u64,
    pub largest_color_error: u8,
    pub largest_depth_error: f32,
}

/// Runs `frame_total` frames, one in flight: lend, request (with the frame
/// number), begin, `fill`, show, receive, check the picture. Prints the rate
/// and the phase medians under `label`.
pub fn run_frames(
    label: &str,
    frame_total: u64,
    receiver: &mut RenderReceiver,
    lending: &Lending<'_>,
    receiver_layout: &QarVideoFrameLayout,
    sender: &mut RenderSender,
    mut fill: impl FnMut(&mut Frame<'_>, u64),
) -> FrameStats {
    let frame_options = BlockingOptions::default();
    let mut stats = FrameStats {
        frames: 0,
        largest_color_error: 0,
        largest_depth_error: 0.0,
    };
    let mut round_trips = Vec::new();
    let mut phases = super::Phases::default();
    let loop_started = Instant::now();
    for frame_number in 0..frame_total {
        let frame_started = Instant::now();
        lend(receiver, lending, frame_number);
        let head_pose = super::head_pose(frame_number);
        receiver
            .request_render(head_pose, Some(frame_number))
            .expect("request a render");
        let lap = phases.lap("lend and request", frame_started);

        let mut frame = sender.begin_frame(&frame_options).expect("begin a frame");
        let lap = phases.lap("begin", lap);
        if frame_number == 0 || frame_number + 1 == frame_total {
            eprintln!("frame {frame_number} requested head pose: {head_pose:?}");
            eprintln!(
                "frame {frame_number} view 0: {:?}",
                frame.view(0).expect("view 0")
            );
            eprintln!(
                "frame {frame_number} view 1: {:?}",
                frame.view(1).expect("view 1")
            );
        }
        fill(&mut frame, frame_number);
        let lap = phases.lap("fill", lap);
        frame.show(&ShowParams::default()).expect("show the frame");
        let lap = phases.lap("show", lap);

        let info = receiver
            .receive_frame(super::scaled(Duration::from_secs(5)))
            .expect("receive")
            .unwrap_or_else(|| panic!("{label}: frame {frame_number} arrives within the budget"));
        let lap = phases.lap("receive", lap);
        round_trips.push(frame_started.elapsed());
        assert!(info.valid_frame, "frame {frame_number} holds content");
        assert_eq!(
            info.sequence_number, frame_number,
            "one frame per request: the sender's sequence counts the frames shown"
        );

        let textures = received_textures(receiver, lending, receiver_layout);
        let borrowed: Vec<(&[u8], usize)> = textures
            .iter()
            .map(|(bytes, pitch)| (bytes.as_slice(), *pitch))
            .collect();
        let verdict = pattern::check_frame(receiver_layout, &borrowed, frame_number);
        phases.lap("read back and check", lap);
        if frame_number == 0 || frame_number + 1 == frame_total {
            eprintln!(
                "frame {frame_number}: sequence {} creation {} ns; {} color and {} depth samples",
                info.sequence_number,
                info.creation_time_ns,
                verdict.color_samples,
                verdict.depth_samples
            );
        }
        assert!(
            verdict.mismatches.is_empty(),
            "{label}: frame {frame_number}: {:?}",
            verdict.mismatches
        );
        stats.frames += 1;
        stats.largest_color_error = stats.largest_color_error.max(verdict.largest_color_error);
        stats.largest_depth_error = stats.largest_depth_error.max(verdict.largest_depth_error);
    }
    super::report_rate(label, &round_trips, loop_started.elapsed());
    phases.report(label);
    eprintln!(
        "{label}: largest color error {}, largest depth error {}",
        stats.largest_color_error, stats.largest_depth_error
    );
    stats
}

fn lend(receiver: &mut RenderReceiver, lending: &Lending<'_>, frame_number: u64) {
    let accepted = super::wait_for(super::scaled(Duration::from_secs(5)), || match lending {
        Lending::Cpu => receiver
            .provide_cpu_frame()
            .expect("lend CPU buffers")
            .then_some(()),
        Lending::Vulkan { frame, .. } => {
            let lent = frame.lend().expect("describe the Vulkan frame");
            // SAFETY: the frame's images live in `lending`, which outlives
            // the loop and so every lend.
            unsafe { receiver.provide_vulkan_frame(&lent) }
                .expect("lend the Vulkan frame")
                .then_some(())
        }
    });
    assert!(
        accepted.is_some(),
        "frame {frame_number}: the receiver accepts the lent frame"
    );
}

/// Every texture of the received frame as (bytes, pitch).
fn received_textures(
    receiver: &RenderReceiver,
    lending: &Lending<'_>,
    layout: &QarVideoFrameLayout,
) -> Vec<(Vec<u8>, usize)> {
    match lending {
        Lending::Cpu => {
            let buffers = receiver.cpu_buffers().expect("CPU receiver buffers");
            (0..layout.textures_count)
                .map(|index| {
                    (
                        buffers.texture(index).unwrap_or_default().to_vec(),
                        buffers.pitch(index).unwrap_or_default() as usize,
                    )
                })
                .collect()
        }
        Lending::Vulkan { device, .. } => {
            let mut delivered_frame: QarVideoFrameVulkan =
                receiver.last_frame_vulkan().expect("last Vulkan frame");
            (0..delivered_frame.textures_count)
                .map(|index| {
                    let size = delivered_frame.textures[index].size;
                    // SAFETY: the record describes images of the lent frame on
                    // this device; the first read waits for its semaphore.
                    let bytes =
                        unsafe { device.read_texture(&raw mut delivered_frame, index, index == 0) }
                            .expect("read the received texture");
                    let pitch = bytes.len() / size.height.max(1) as usize;
                    (bytes, pitch)
                })
                .collect()
        }
    }
}
