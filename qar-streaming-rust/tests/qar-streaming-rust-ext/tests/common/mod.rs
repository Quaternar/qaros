//! Shared set-up of the end-to-end tests: the real QAROS package from
//! `QAROS_PACKAGE_BIN`, the base library through `qaros`, the extension, an
//! extension hub, and an application session onboarded into that hub with a
//! minted invite. Nothing touches a Hub running on the machine: the runtime
//! stores its state in a fresh directory per test process.

#![allow(dead_code, clippy::expect_used)]

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use qar_streaming_rust_ext::{Extension, Hub, InviteMethod};
use qaros::qaros_sys::{QarPose, QarVideoFrameLayout, defaults};
use qaros::{
    AppVolumeConfig, AppVolumeId, BlockingOptions, Eye, FrameViewSpec, Library, LibraryConfig,
    LogSeverity, OnboardMethod, PeerId, PeerInfo, PeerPresentation, Runtime, RuntimeConfig,
    Session, ViewKind,
};

pub mod own_vulkan;
pub mod pattern;
pub mod scenario;

pub const PACKAGE_VARIABLE: &str = "QAROS_PACKAGE_BIN";
pub const APPLICATION_NAME: &str = "qaros-e2e-application";

/// The package's `bin` folder, or `None` (the test then passes as skipped).
pub fn package_bin() -> Option<PathBuf> {
    match std::env::var_os(PACKAGE_VARIABLE) {
        Some(value) if !value.is_empty() => Some(PathBuf::from(value)),
        _ => {
            eprintln!("skipped: set {PACKAGE_VARIABLE} to the QAROS package's bin folder");
            None
        }
    }
}

/// A fresh directory for this test process's runtime storage and logs.
pub fn scratch_directory(test_name: &str) -> PathBuf {
    let directory = std::env::temp_dir()
        .join("qaros-e2e")
        .join(format!("{test_name}-{}", std::process::id()));
    std::fs::create_dir_all(&directory).expect("create the scratch directory");
    directory
}

/// Base library and extension, loaded in that order.
pub struct Loaded {
    pub extension: Extension,
    pub library: Library,
    pub scratch: PathBuf,
}

pub fn load(bin: &Path, test_name: &str) -> Loaded {
    let scratch = scratch_directory(test_name);
    let library_path = bin.join(qaros::qaros_sys::LIBRARY_FILE_NAME);
    let config = LibraryConfig {
        console_logging: false,
        log_directory: Some(scratch.join("logs")),
        log_severity: LogSeverity::Info,
    };
    std::fs::create_dir_all(scratch.join("logs")).expect("create the log directory");
    let started = Instant::now();
    // SAFETY: the package's DLL, whose header the bindings were generated from.
    let library = unsafe { Library::load(&library_path, &config) }.expect("load and init the base");
    eprintln!(
        "base library loaded and initialized in {:?}",
        started.elapsed()
    );
    let started = Instant::now();
    // SAFETY: the base from the same folder is loaded and initialized.
    let extension = unsafe { Extension::load(bin) }.expect("load the extension");
    eprintln!(
        "extension loaded in {:?}; QAR_TIME_SCALE factor {}",
        started.elapsed(),
        extension.time_scale()
    );
    Loaded {
        extension,
        library,
        scratch,
    }
}

/// A runtime whose state lives in the test's scratch directory.
pub fn create_runtime(library: &Library, bin: &Path, scratch: &Path, name: &str) -> Runtime {
    let storage = scratch.join(format!("storage-{name}"));
    std::fs::create_dir_all(&storage).expect("create the storage directory");
    library
        .create_runtime(&RuntimeConfig {
            runtime_binaries_directory: Some(bin.to_path_buf()),
            storage_directory: Some(storage),
            ephemeral_storage: false,
        })
        .expect("create the runtime")
}

/// Onboards a session into `hub` with a freshly minted PAKE invite and
/// checks that the hub lists it under `display_name`.
pub fn onboard_into(
    hub: &Hub,
    library: &Library,
    runtime: &Runtime,
    display_name: &str,
) -> Session {
    let invite_text = hub.mint_invite(InviteMethod::Pake).expect("mint an invite");
    eprintln!("minted invite: {} bytes of wire text", invite_text.len());
    let invite = library
        .deserialize_invite(invite_text.as_bytes())
        .expect("deserialize the minted invite");
    let cancel = library
        .create_cancel_token(Some(scaled(Duration::from_secs(30))))
        .expect("cancel token");
    let started = Instant::now();
    let onboarded = runtime
        .onboard(
            &OnboardMethod::Invite(&invite),
            &PeerPresentation {
                display_name: Some(display_name.to_string()),
                app_version: Some("e2e".to_string()),
                ..PeerPresentation::default()
            },
            &BlockingOptions {
                progress: None,
                cancel: Some(&cancel),
            },
        )
        .expect("onboard with the minted invite");
    eprintln!("onboarded in {:?}", started.elapsed());
    let my_peer = onboarded.session.my_peer().expect("my peer");
    let listed_name = wait_for(scaled(Duration::from_secs(10)), || {
        hub.find_peer_display_name(&qaros_sys_peer_id(my_peer.id.0))
            .expect("hub peer lookup")
    });
    assert_eq!(
        listed_name.as_deref(),
        Some(display_name),
        "the hub lists the onboarded session"
    );
    onboarded.session
}

pub fn qaros_sys_peer_id(bytes: [u8; 16]) -> qaros::qaros_sys::QarPeerId {
    qaros::qaros_sys::QarPeerId { data: bytes }
}

/// Polls `probe` until it returns `Some`, or the budget runs out.
pub fn wait_for<Value>(
    budget: Duration,
    mut probe: impl FnMut() -> Option<Value>,
) -> Option<Value> {
    let deadline = Instant::now() + budget;
    loop {
        if let Some(value) = probe() {
            return Some(value);
        }
        if Instant::now() >= deadline {
            return None;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}

/// `QAROS_E2E_EYE=<width>x<height>`, default 960x1000.
pub fn eye_size_from_environment() -> (u32, u32) {
    let text = std::env::var("QAROS_E2E_EYE").unwrap_or_else(|_| "960x1000".to_string());
    let (width, height) = text
        .split_once('x')
        .unwrap_or_else(|| panic!("QAROS_E2E_EYE {text:?} is not <width>x<height>"));
    (
        width.parse().expect("QAROS_E2E_EYE width"),
        height.parse().expect("QAROS_E2E_EYE height"),
    )
}

/// `QAROS_E2E_FRAMES`, default 60.
pub fn frame_count_from_environment() -> u64 {
    std::env::var("QAROS_E2E_FRAMES")
        .ok()
        .map_or(60, |text| text.parse().expect("QAROS_E2E_FRAMES"))
}

/// Waits until the session's peer list holds `target` as a running target
/// app: the discovery the header's "Basic flow" step 5 describes.
pub fn wait_for_target_app(session: &Session, target: PeerId) -> PeerInfo {
    let found = wait_for(scaled(Duration::from_secs(10)), || {
        session
            .peers()
            .expect("peer list")
            .into_iter()
            .find(|peer| peer.id == target)
    });
    let peer = found.unwrap_or_else(|| panic!("the session lists peer {target}"));
    assert!(
        peer.is_target_app,
        "an extension peer is a target app: {peer:?}"
    );
    peer
}

/// The app volume a render sender needs (`qar_render_sender_create` refuses a
/// sender without one), shown to `viewer`.
pub fn create_app_volume(session: &Session, viewer: PeerId) -> AppVolumeId {
    let mut config = AppVolumeConfig::new("qaros-e2e-volume");
    config.display_name = Some("qaros e2e volume".to_string());
    config.initial_peers = vec![viewer];
    session
        .app_volumes()
        .get_or_create(&config)
        .expect("create the app volume")
}

/// Stereo color in texture 0 and depth in texture 1, the side-by-side
/// receiver layout's views. The frame's view poses follow this order: a
/// sender declared without views has none (`View index out of range`).
pub fn stereo_color_and_depth_views() -> Vec<FrameViewSpec> {
    vec![
        FrameViewSpec {
            kind: ViewKind::Color,
            eye: Eye::Left,
            texture_index: 0,
        },
        FrameViewSpec {
            kind: ViewKind::Color,
            eye: Eye::Right,
            texture_index: 0,
        },
        FrameViewSpec {
            kind: ViewKind::Depth,
            eye: Eye::Left,
            texture_index: 1,
        },
        FrameViewSpec {
            kind: ViewKind::Depth,
            eye: Eye::Right,
            texture_index: 1,
        },
    ]
}

/// A head pose 1.6 m up that moves 1 mm back per frame, so frames differ.
pub fn head_pose(frame_number: u64) -> QarPose {
    let mut pose = defaults::qar_pose_default();
    pose.position.y = 1.6;
    #[allow(clippy::cast_precision_loss)]
    let offset = frame_number as f32 * 0.001;
    pose.position.z = offset;
    pose
}

pub fn print_layout(label: &str, layout: &QarVideoFrameLayout) {
    let views = &layout.views[..layout.views_count.min(layout.views.len())];
    let textures = &layout.textures[..layout.textures_count.min(layout.textures.len())];
    eprintln!("{label}:");
    for view in views {
        eprintln!(
            "  view type {} eye {} texture {} layer {} rect ({}, {})-({}, {}) format {}",
            view.data_type.0,
            view.eye.0,
            view.texture_index,
            view.array_layer_index,
            view.start_x,
            view.start_y,
            view.end_x,
            view.end_y,
            view.texture_format.0
        );
    }
    for (index, texture) in textures.iter().enumerate() {
        eprintln!(
            "  texture {index}: {}x{} layers {} format {}",
            texture.width, texture.height, texture.array_layers, texture.format.0
        );
    }
}

/// Durations of the named phases of each frame, for medians at the end.
#[derive(Debug, Default)]
pub struct Phases {
    samples: Vec<(&'static str, Vec<Duration>)>,
}

impl Phases {
    /// Records the time since `since` under `name` and returns now.
    pub fn lap(&mut self, name: &'static str, since: Instant) -> Instant {
        let now = Instant::now();
        let elapsed = now - since;
        match self.samples.iter_mut().find(|(known, _)| *known == name) {
            Some((_, durations)) => durations.push(elapsed),
            None => self.samples.push((name, vec![elapsed])),
        }
        now
    }

    pub fn report(&self, label: &str) {
        let medians: Vec<String> = self
            .samples
            .iter()
            .map(|(name, durations)| {
                let mut sorted = durations.clone();
                sorted.sort();
                let median = sorted.get(sorted.len() / 2).copied().unwrap_or_default();
                format!("{name} {:.2} ms", median.as_secs_f64() * 1000.0)
            })
            .collect();
        eprintln!("PHASES {label} (medians): {}", medians.join(", "));
    }
}

/// Prints frames per second over the summed round trips (the pixel checks
/// between frames excluded) and the round-trip spread
/// (request, render, show, receive; one frame in flight).
pub fn report_rate(label: &str, round_trips: &[Duration], total: Duration) {
    let mut sorted = round_trips.to_vec();
    sorted.sort();
    let median = sorted.get(sorted.len() / 2).copied().unwrap_or_default();
    let slowest = sorted.last().copied().unwrap_or_default();
    let rendering: Duration = round_trips.iter().sum();
    #[allow(clippy::cast_precision_loss)]
    let rate = round_trips.len() as f64 / rendering.as_secs_f64();
    eprintln!(
        "RATE {label}: {} frames, {rate:.1} frames/s over the round trips ({:.2} s; {:.2} s with the checks); round trip median {:.2} ms, slowest {:.2} ms",
        round_trips.len(),
        rendering.as_secs_f64(),
        total.as_secs_f64(),
        median.as_secs_f64() * 1000.0,
        slowest.as_secs_f64() * 1000.0
    );
}

/// Scales a test budget like the extension's own tests (`QAR_TIME_SCALE`).
pub fn scaled(budget: Duration) -> Duration {
    let factor = std::env::var("QAR_TIME_SCALE")
        .ok()
        .and_then(|value| value.parse::<f64>().ok())
        .filter(|factor| *factor > 0.0)
        .unwrap_or(1.0);
    budget.mul_f64(factor)
}
