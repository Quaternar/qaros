//! QuaternAR's `vulkan_cube` source app in Rust: a rotating cube, one flat color per face, with
//! stereo color and depth, rendered with `ash` on the example's own Vulkan device and handed to
//! QAROS through the Vulkan render sender (feature `vulkan`).
//!
//! ```text
//! cargo run -p qaros-examples --features vulkan --bin vulkan_cube
//! ```
//!
//! Like the C sample it takes no options and looks for no file itself. It loads the runtime with
//! the default lookup (`Library::load_default`, the C sample's `qar_library_load(NULL)`: the
//! runtime beside the executable, the runtime of the QAROS instance `QAR_INSTANCE` names, the
//! installed QAROS, `PATH`) and joins the QAROS on this PC as a local app (`OnboardMethod::LocalApp`): no code,
//! the first run waits until someone clicks Allow in QAROS. It then creates its device on the
//! session's GPU (`Session::graphics_adapter`; the first supported GPU when the session names
//! none), renders for every running target app in the peer list with one sender and camera each,
//! inside the "Vulkan Cube" app volume, and hands each frame over as `docs/vulkan-api.md`
//! describes: acquire every image from its reported layout and queue family, render both eyes
//! from the frame's view poses and fields of view, transition to `TRANSFER_SRC_OPTIMAL` and
//! release to `VK_QUEUE_FAMILY_EXTERNAL`, signal the frame's semaphore, write the state back,
//! show.
//!
//! Ctrl+C stops it, and so does the end of stdin when stdin is a pipe: QAROS stops an app it
//! started by closing that pipe ("Launched by QAROS"). A stop while it waits for the approval
//! cancels the wait. Exit codes as in the C sample: 0 after a stop, 2 when no runtime loads, 3
//! when the library fails to initialize, 4 on any other failure.
//!
//! One deliberate difference: the C sample serves each target on a thread of its own, paced by
//! the blocking begin-frame. `qaros` handles are not `Send`, since the SDK is designed for
//! single-threaded use per runtime, so this port serves all targets from one loop with the
//! non-blocking `try_begin_owned_frame`, as the CPU sample does. Each target keeps two frames in
//! flight, so the loop waits for the GPU only when a target reuses a frame slot whose work has not
//! finished. Creating a sender blocks that loop, so a target whose sender cannot be created is
//! tried again after a delay that doubles from the C sample's 3 s, instead of every 3 s.
//!
//! The shaders are QuaternAR's GLSL (`examples/shaders/cube.vert`, `cube.frag`), compiled once
//! with `glslangValidator -V --target-env vulkan1.1` and checked in as SPIR-V, so building needs
//! no shader compiler.

use std::collections::HashMap;
use std::ffi::{CStr, c_char};
use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use ash::vk;
use glam::{Mat4, Vec4};
use qaros::vulkan::{SHOWN_LAYOUT, VulkanImage, VulkanStreamParams};
use qaros::{
    AppVolumeConfig, AppVolumeId, AppVolumeSize, BlockingOptions, Eye, FrameLayout, FrameViewSpec,
    GraphicsAdapter, Library, LibraryConfig, OnboardMethod, OwnedFrame, PeerId, PeerInfo,
    PeerPresentation, PixelFormat, Pose, Progress, RenderSender, RenderSenderConfig, Runtime,
    RuntimeConfig, Session, ShowParams, StatusCode, Vec3, VideoFrameView, ViewKind, ViewPose,
};
use qaros_examples::{ExampleResult, describe_loaded_runtime, is_running_target, is_waiting};

const USAGE: &str = "Usage: vulkan_cube

Loads the QAROS runtime (the default lookup: the runtime beside this executable,
the runtime of the QAROS instance QAR_INSTANCE names, the installed QAROS, PATH)
and joins the QAROS on this PC as a local app. The first run waits until you click Allow in QAROS.
The cube renders on the GPU its session was created on.
Stop with Ctrl+C, or by closing stdin when it is a pipe.
";
/// The near and far planes of the cube's projection and of `show`, in app units.
const NEAR_PLANE: f32 = 0.1;
const FAR_PLANE: f32 = 10.0;
/// The cube's corners are at +-1, scaled to a 60 cm cube.
const CUBE_SCALE: f32 = 0.3;
const SPIN_RADIANS_PER_SECOND: f32 = 0.8;
const CLEAR_COLOR: [f32; 4] = [0.08, 0.08, 0.1, 1.0];
const FRAMES_IN_FLIGHT: usize = 2;
const COLOR_FORMAT: vk::Format = vk::Format::R8G8B8A8_UNORM;
const DEPTH_FORMAT: vk::Format = vk::Format::D32_SFLOAT;
const VERTEX_SHADER: &[u8] = include_bytes!("../../shaders/cube.vert.spv");
const FRAGMENT_SHADER: &[u8] = include_bytes!("../../shaders/cube.frag.spv");
/// How often the peer list is read without a peer update.
const PEER_LIST_INTERVAL: Duration = Duration::from_secs(2);
/// How long creating a sender may block before the target is tried again later.
const SENDER_CREATION_TIMEOUT: Duration = Duration::from_secs(10);
/// The C sample retries a failed target after 3 s.
const FIRST_RETRY_DELAY: Duration = Duration::from_secs(3);
const LONGEST_RETRY_DELAY: Duration = Duration::from_secs(60);
const IDLE_WAIT: Duration = Duration::from_millis(2);
/// How often the stop flag is checked while the app waits for the approval.
const STOP_POLL_INTERVAL: Duration = Duration::from_millis(50);

// ANCHOR: frame_views
/// The C sample's views: per eye a color and a depth texture of their own.
const FRAME_VIEWS: [FrameViewSpec; 4] = [
    FrameViewSpec {
        kind: ViewKind::Color,
        eye: Eye::Left,
        texture_index: 0,
    },
    FrameViewSpec {
        kind: ViewKind::Depth,
        eye: Eye::Left,
        texture_index: 1,
    },
    FrameViewSpec {
        kind: ViewKind::Color,
        eye: Eye::Right,
        texture_index: 2,
    },
    FrameViewSpec {
        kind: ViewKind::Depth,
        eye: Eye::Right,
        texture_index: 3,
    },
];
// ANCHOR_END: frame_views

// ANCHOR: main
fn main() -> ExitCode {
    if let Some(argument) = std::env::args().nth(1) {
        if argument == "--help" || argument == "-h" {
            print!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        eprint!("{USAGE}");
        return ExitCode::from(1);
    }
    stop::watch_for_stop_requests();
    // SAFETY: the runtime found is the installed QAROS, that of the instance the user chose with
    // QAR_INSTANCE, or one put beside this executable, which matches these bindings.
    let library = match unsafe { Library::load_default(&LibraryConfig::default()) } {
        Ok(library) => library,
        Err(qaros::Error::Load(load_error)) => {
            eprintln!(
                "Could not load QAROS. Install QAROS on this PC, then start this app again.\n{load_error}"
            );
            return ExitCode::from(2);
        }
        Err(error) => {
            eprintln!("The QAROS runtime failed to initialize: {error}");
            return ExitCode::from(3);
        }
    };
    println!("Loaded {}", describe_loaded_runtime(&library));
    let outcome = run_app(&library);
    // The wrapper logs a failing qar_library_destroy; the library is never unloaded.
    drop(library);
    match outcome {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(4)
        }
    }
}
// ANCHOR_END: main

// ANCHOR: run_app
/// Everything between loading the library and destroying it. A stop request is not a failure.
fn run_app(library: &Library) -> ExampleResult {
    // runtime_binaries_directory stays empty: the runtime uses the folder it was loaded from.
    let runtime = library.create_runtime(&RuntimeConfig::default())?;
    println!(
        "Joining the QAROS on this PC. On the first run, click Allow in the QAROS approval prompt."
    );
    let Some(session) = join_as_local_app(library, &runtime)? else {
        return Ok(());
    };
    // Onboard first: the session decides the GPU, and the cube creates its device there.
    let gpu = CubeDevice::create(session.graphics_adapter()?)?;
    let pipeline = CubePipeline::create(&gpu)?;
    println!("Connected. Waiting for rendering targets. Ctrl+C to quit.");
    let served = serve_targets(library, &session, &gpu, &pipeline);
    // The session ends before the device its senders streamed from, as in the C sample, whose
    // device outlives the session; destroying the device first crashes the runtime's teardown.
    drop(pipeline);
    drop(session);
    drop(gpu);
    served
}
// ANCHOR_END: run_app

// ANCHOR: join
/// Joins the QAROS on this PC as a local app: no code, the user approves the app once in QAROS.
/// No adapter is chosen: the session decides the GPU. A stop request cancels the call, which can
/// wait minutes for the approval; `None` then.
fn join_as_local_app(library: &Library, runtime: &Runtime) -> ExampleResult<Option<Session>> {
    let token = library.create_cancel_token(None)?;
    let cancel = token.cancel_handle();
    let joined = Arc::new(AtomicBool::new(false));
    let canceller = {
        let joined = Arc::clone(&joined);
        std::thread::spawn(move || {
            while !joined.load(Ordering::Acquire) && !stop::requested() {
                std::thread::sleep(STOP_POLL_INTERVAL);
            }
            if !joined.load(Ordering::Acquire)
                && let Err(error) = cancel.cancel()
            {
                eprintln!("Failed to cancel joining QAROS: {error}");
            }
        })
    };
    let (progress, progress_reports) = std::sync::mpsc::channel::<Progress>();
    let printer = std::thread::spawn(move || {
        for report in progress_reports {
            if !report.message.is_empty() {
                println!("{}", report.message);
            }
        }
    });
    let presentation = PeerPresentation {
        display_name: Some("Vulkan Cube Source".to_string()),
        ..PeerPresentation::default()
    };
    let onboarded = runtime.onboard(
        &OnboardMethod::LocalApp {
            approval_timeout: None,
            // Constant across versions: every start gets the same peer id back from the hub.
            app_tag: Some(APP_TAG),
        },
        &presentation,
        &BlockingOptions {
            progress: Some(progress),
            cancel: Some(&token),
        },
    );
    joined.store(true, Ordering::Release);
    if canceller.join().is_err() || printer.join().is_err() {
        eprintln!("a helper thread of the join panicked");
    }
    match onboarded {
        Ok(onboarded) if !stop::requested() => {
            print_my_peer_id(&onboarded.session);
            Ok(Some(onboarded.session))
        }
        Ok(_) => Ok(None),
        Err(_) if stop::requested() => Ok(None),
        Err(error) => {
            explain_onboarding_failure(&error);
            Err("could not join QAROS".into())
        }
    }
}
// ANCHOR_END: join

/// The cube's app tag: an identity key, printable ASCII and constant across versions, as in the
/// C cube.
const APP_TAG: &str = "vulkan-cube.examples.qaros";

/// Prints the peer id the hub gave this app. Thanks to the app tag it is the same on every
/// start, so the user (and a test) can see the restart kept it.
fn print_my_peer_id(session: &Session) {
    let peer = session
        .my_peer()
        .and_then(|me| session.library().uuid_to_string(&me.id.0));
    match peer {
        Ok(text) => println!("Joined QAROS as peer {text}"),
        Err(error) => eprintln!("could not read this app's peer id: {error}"),
    }
}

// ANCHOR: explain_failure
/// Tells the user, in plain words, why joining failed and what to do.
fn explain_onboarding_failure(error: &qaros::Error) {
    let explanations = [
        (
            StatusCode::ONBOARDING_APPROVAL_DENIED,
            "QAROS did not allow this app to join: the approval was denied, or revoked. Start the app again to be asked again.",
        ),
        (
            StatusCode::ONBOARDING_APPROVAL_TIMEOUT,
            "Nobody answered the approval prompt in QAROS in time. It opens in the QAROS Visualizer, or as a QAROS tray window. Start the app again.",
        ),
        (
            StatusCode::ONBOARDING_HUB_NOT_AUTHENTIC,
            "The program answering on this PC is not the installed QAROS, so this app does not trust it and stops. Reinstall QAROS or ask your administrator.",
        ),
        (
            StatusCode::ONBOARDING_APP_IDENTITY_MISMATCH,
            "This app no longer matches what QAROS approved: it was moved, copied or re-signed. In the QAROS Visualizer, revoke it under Connection > Approved apps, then start it again to approve it anew.",
        ),
        (
            StatusCode::ONBOARDING_HUB_UNREACHABLE,
            "QAROS is not running on this PC. Start QAROS from the Start menu, then start this app again.",
        ),
    ];
    let explanation = explanations
        .iter()
        .find(|(code, _)| error.has_code(*code))
        .map_or("Could not join QAROS.", |(_, explanation)| explanation);
    eprintln!("{explanation}");
    eprintln!("Details: {error}");
}
// ANCHOR_END: explain_failure

// ANCHOR: serve_targets
/// Follows the targets and renders for them until a stop request.
fn serve_targets(
    library: &Library,
    session: &Session,
    gpu: &CubeDevice,
    pipeline: &CubePipeline,
) -> ExampleResult {
    let volume = create_volume(session)?;
    // Subscribe first, then list, so a target joining in between is not missed.
    let (forward, peer_updates) = qaros::channel();
    let _peer_subscription = session.subscribe_peer_updates(forward)?;
    let started = Instant::now();
    let mut targets: Vec<Target> = Vec::new();
    let mut retries = RetryPlan::default();
    let mut last_listing: Option<Instant> = None;

    while !stop::requested() {
        let updates_arrived = peer_updates.try_iter().count() > 0;
        if updates_arrived || last_listing.is_none_or(|last| last.elapsed() >= PEER_LIST_INTERVAL) {
            last_listing = Some(Instant::now());
            follow_targets(
                library,
                session,
                volume,
                gpu,
                &session.peers()?,
                &mut targets,
                &mut retries,
            );
        }
        let seconds = started.elapsed().as_secs_f32();
        let mut showed_a_frame = false;
        targets.retain_mut(
            |target| match render_next_frame(target, pipeline, seconds) {
                Ok(showed) => {
                    showed_a_frame |= showed;
                    true
                }
                Err(error) => {
                    println!("Stopped streaming to {}: {error}", target.label);
                    false
                }
            },
        );
        if !showed_a_frame {
            std::thread::sleep(IDLE_WAIT);
        }
    }
    println!("Stopping.");
    // Cameras drain their GPU work and senders close before the pipeline and the device go.
    drop(targets);
    Ok(())
}
// ANCHOR_END: serve_targets

/// Stop requests: Ctrl+C (SIGINT), SIGTERM, and the end of stdin when it is a pipe. A signal
/// handler can safely write nothing but an atomic, so all SDK and GPU cleanup stays on the main
/// thread, which polls [`stop::requested`].
mod stop {
    use std::sync::atomic::{AtomicBool, Ordering};

    static STOP_REQUESTED: AtomicBool = AtomicBool::new(false);

    pub(crate) fn requested() -> bool {
        STOP_REQUESTED.load(Ordering::Acquire)
    }

    fn request() {
        STOP_REQUESTED.store(true, Ordering::Release);
    }

    /// Installs the signal handlers and, when stdin is a pipe, a thread that waits for its end.
    /// Whatever arrives on the pipe is ignored; its end is the stop request.
    pub(crate) fn watch_for_stop_requests() {
        install_signal_handlers();
        if stdin_is_pipe() {
            let spawned = std::thread::Builder::new()
                .name("stdin".to_string())
                .spawn(|| {
                    let drained = std::io::copy(&mut std::io::stdin().lock(), &mut std::io::sink());
                    if let Err(error) = drained {
                        eprintln!("Reading stdin failed: {error}");
                    }
                    request();
                });
            if let Err(error) = spawned {
                eprintln!("Cannot watch stdin ({error}); stop the app with Ctrl+C");
            }
        }
    }

    #[cfg(windows)]
    fn install_signal_handlers() {
        use windows::Win32::Foundation::{FALSE, TRUE};
        use windows::Win32::System::Console::{
            CTRL_BREAK_EVENT, CTRL_C_EVENT, SetConsoleCtrlHandler,
        };
        use windows::core::BOOL;

        unsafe extern "system" fn on_console_control(control_type: u32) -> BOOL {
            if control_type == CTRL_C_EVENT || control_type == CTRL_BREAK_EVENT {
                request();
                TRUE
            } else {
                FALSE
            }
        }
        // SAFETY: the handler only writes an atomic.
        if let Err(error) = unsafe { SetConsoleCtrlHandler(Some(on_console_control), true) } {
            eprintln!("Cannot handle Ctrl+C ({error}); it ends the app at once");
        }
    }

    #[cfg(windows)]
    fn stdin_is_pipe() -> bool {
        use windows::Win32::Storage::FileSystem::{FILE_TYPE_PIPE, GetFileType};
        use windows::Win32::System::Console::{GetStdHandle, STD_INPUT_HANDLE};

        // SAFETY: GetStdHandle and GetFileType only query the handle.
        unsafe { GetStdHandle(STD_INPUT_HANDLE) }
            .is_ok_and(|handle| unsafe { GetFileType(handle) } == FILE_TYPE_PIPE)
    }

    #[cfg(not(windows))]
    fn install_signal_handlers() {
        use std::ffi::c_int;

        const SIGINT: c_int = 2;
        const SIGTERM: c_int = 15;
        const SIG_ERR: usize = usize::MAX;

        unsafe extern "C" {
            fn signal(signal_number: c_int, handler: extern "C" fn(c_int)) -> usize;
        }

        extern "C" fn on_signal(_signal_number: c_int) {
            request();
        }

        for signal_number in [SIGINT, SIGTERM] {
            // SAFETY: the handler only writes an atomic, which is async-signal-safe.
            let previous = unsafe { signal(signal_number, on_signal) };
            if previous == SIG_ERR {
                eprintln!("Cannot handle signal {signal_number}; it ends the app at once");
            }
        }
    }

    #[cfg(not(windows))]
    fn stdin_is_pipe() -> bool {
        use std::os::unix::fs::FileTypeExt as _;

        std::fs::metadata("/dev/stdin").is_ok_and(|metadata| {
            let file_type = metadata.file_type();
            file_type.is_fifo() || file_type.is_socket()
        })
    }
}

// ANCHOR: create_volume
fn create_volume(session: &Session) -> qaros::Result<AppVolumeId> {
    let mut volume_config = AppVolumeConfig::new("vulkan-cube");
    volume_config.display_name = Some("Vulkan Cube".to_string());
    volume_config.pose = Pose::from_position(Vec3::new(0.0, 0.0, -1.5));
    volume_config.size = AppVolumeSize {
        width: 1.2,
        length: 1.2,
        height: 1.2,
    };
    session.app_volumes().get_or_create(&volume_config)
}
// ANCHOR_END: create_volume

/// When to try a target app again whose sender could not be created. Creating a sender blocks
/// the loop until the target asks for the stream (or [`SENDER_CREATION_TIMEOUT`]), so a target
/// that never asks would stall every other target at each listing; the delay doubles from
/// [`FIRST_RETRY_DELAY`] up to [`LONGEST_RETRY_DELAY`] instead.
#[derive(Default)]
struct RetryPlan {
    next_attempts: HashMap<PeerId, (Instant, Duration)>,
}

impl RetryPlan {
    fn is_due(&self, peer: PeerId) -> bool {
        self.next_attempts
            .get(&peer)
            .is_none_or(|(next_attempt, _)| Instant::now() >= *next_attempt)
    }

    fn failed(&mut self, peer: PeerId) -> Duration {
        let delay = self
            .next_attempts
            .get(&peer)
            .map_or(FIRST_RETRY_DELAY, |(_, delay)| {
                (*delay * 2).min(LONGEST_RETRY_DELAY)
            });
        self.next_attempts
            .insert(peer, (Instant::now() + delay, delay));
        delay
    }

    fn succeeded(&mut self, peer: PeerId) {
        self.next_attempts.remove(&peer);
    }
}

// ANCHOR: target
/// One target app: its camera and the sender rendering for it. The camera is declared first so
/// that it drains its GPU work before the sender's images go.
struct Target {
    peer: PeerId,
    label: String,
    camera: Camera,
    sender: RenderSender,
}

impl Target {
    fn create(
        library: &Library,
        session: &Session,
        volume: AppVolumeId,
        gpu: &CubeDevice,
        peer: PeerId,
        label: String,
    ) -> ExampleResult<Self> {
        let config = RenderSenderConfig {
            views: FRAME_VIEWS.to_vec(),
            app_volume: Some(volume),
            color_format: PixelFormat::R8G8B8A8,
            depth_format: PixelFormat::D32Float,
            layout: FrameLayout::SeparatedTextures,
            ..RenderSenderConfig::new(peer)
        };
        let timeout = library.create_cancel_token(Some(SENDER_CREATION_TIMEOUT))?;
        // SAFETY: the device was created on the session's adapter with
        // required_device_extensions(), and `run` drops every target before the device.
        let sender = unsafe {
            session.create_vulkan_render_sender(
                &config,
                gpu.stream_params(),
                &BlockingOptions {
                    cancel: Some(&timeout),
                    ..BlockingOptions::default()
                },
            )
        }?;
        Ok(Self {
            peer,
            label,
            camera: Camera::create(gpu)?,
            sender,
        })
    }
}
// ANCHOR_END: target

/// Creates a target for every running target app the list shows and drops the targets it no
/// longer shows running.
fn follow_targets(
    library: &Library,
    session: &Session,
    volume: AppVolumeId,
    gpu: &CubeDevice,
    listed: &[PeerInfo],
    targets: &mut Vec<Target>,
    retries: &mut RetryPlan,
) {
    targets.retain(|target| {
        let still_running = listed
            .iter()
            .any(|peer| peer.id == target.peer && is_running_target(peer));
        if !still_running {
            println!("{} left", target.label);
        }
        still_running
    });
    for peer in listed.iter().filter(|peer| is_running_target(peer)) {
        if targets.iter().any(|target| target.peer == peer.id) || !retries.is_due(peer.id) {
            continue;
        }
        let label = if peer.display_name.is_empty() {
            peer.id.to_string()
        } else {
            peer.display_name.clone()
        };
        match Target::create(library, session, volume, gpu, peer.id, label.clone()) {
            Ok(target) => {
                println!("Rendering for target {label} (peer {})", peer.id);
                retries.succeeded(peer.id);
                targets.push(target);
            }
            Err(error) => {
                let delay = retries.failed(peer.id);
                println!(
                    "No sender for {label} yet ({error}); trying again in {} s",
                    delay.as_secs()
                );
            }
        }
    }
}

// ANCHOR: render_next_frame
/// Renders and shows the target's next frame when its sender has one; false while none is
/// ready or the stream is waiting.
fn render_next_frame(
    target: &mut Target,
    pipeline: &CubePipeline,
    seconds: f32,
) -> ExampleResult<bool> {
    let mut frame = match target.sender.try_begin_owned_frame() {
        Ok(Some(frame)) => frame,
        Ok(None) => return Ok(false),
        Err(error) if is_waiting(&error) => return Ok(false),
        Err(error) => return Err(error.into()),
    };
    let semaphore = frame.vulkan_frame()?.semaphore();
    let rendered = eye_views(&mut frame)
        .and_then(|eyes| render_frame(&mut frame, &mut target.camera, pipeline, &eyes, seconds));
    if let Err(error) = rendered {
        // A begun frame that is not rendered still signals its semaphore before it is dropped.
        if let Err(discard_error) = target.camera.discard(semaphore) {
            eprintln!("Failed to discard the camera frame: {discard_error}");
        }
        return Err(error);
    }
    match frame.show(&ShowParams {
        near: NEAR_PLANE,
        far: FAR_PLANE,
        depth_scale: 1.0,
    }) {
        Ok(()) => Ok(true),
        Err(error) if is_waiting(&error) => Ok(false),
        Err(error) => Err(error.into()),
    }
}
// ANCHOR_END: render_next_frame

// ANCHOR: eye_views
/// Each eye's pose and field of view, from the frame view showing that eye's color.
fn eye_views(frame: &mut OwnedFrame) -> ExampleResult<[ViewPose; 2]> {
    let color_views = {
        let vulkan_frame = frame.vulkan_frame()?;
        [Eye::Left, Eye::Right].map(|eye| vulkan_frame.view_index(eye, ViewKind::Color))
    };
    let [Some(left), Some(right)] = color_views else {
        return Err("the frame lacks a color view for an eye".into());
    };
    Ok([frame.view(left)?, frame.view(right)?])
}
// ANCHOR_END: eye_views

// ANCHOR: render_frame
/// Renders both eyes into the frame's images, signals its semaphore, and writes the images'
/// new state back. The frame is then ready to show.
fn render_frame(
    frame: &mut OwnedFrame,
    camera: &mut Camera,
    pipeline: &CubePipeline,
    eyes: &[ViewPose; 2],
    seconds: f32,
) -> ExampleResult {
    let (views, images, semaphore) = {
        let vulkan_frame = frame.vulkan_frame()?;
        (
            vulkan_frame.views(),
            vulkan_frame.images(),
            vulkan_frame.semaphore(),
        )
    };
    camera.render(pipeline, &views, &images, semaphore, eyes, seconds)?;
    let mut vulkan_frame = frame.vulkan_frame()?;
    vulkan_frame.release_to_external(SHOWN_LAYOUT);
    vulkan_frame.set_fence(vk::Fence::null());
    Ok(())
}
// ANCHOR_END: render_frame

/// The cube's model matrix after `seconds`: spinning about the (1, 1, 0) diagonal.
fn cube_model(seconds: f32) -> Mat4 {
    let axis = glam::Vec3::new(1.0, 1.0, 0.0).normalize();
    Mat4::from_axis_angle(axis, seconds * SPIN_RADIANS_PER_SECOND)
        * Mat4::from_scale(glam::Vec3::splat(CUBE_SCALE))
}

// ANCHOR: view_projection
/// Projection times view for one eye: OpenXR field-of-view signs, forward Vulkan depth (0 at
/// near, 1 at far). The pose is already in app content space.
fn view_projection(eye: &ViewPose) -> Mat4 {
    let left = eye.fov.angle_left.tan();
    let right = eye.fov.angle_right.tan();
    let down = eye.fov.angle_down.tan();
    let up = eye.fov.angle_up.tan();
    let projection = Mat4::from_cols(
        Vec4::new(2.0 / (right - left), 0.0, 0.0, 0.0),
        Vec4::new(0.0, 2.0 / (up - down), 0.0, 0.0),
        Vec4::new(
            (right + left) / (right - left),
            (up + down) / (up - down),
            -FAR_PLANE / (FAR_PLANE - NEAR_PLANE),
            -1.0,
        ),
        Vec4::new(
            0.0,
            0.0,
            -FAR_PLANE * NEAR_PLANE / (FAR_PLANE - NEAR_PLANE),
            0.0,
        ),
    );
    let orientation = eye.pose.orientation;
    let position = eye.pose.position;
    let camera = Mat4::from_rotation_translation(
        glam::Quat::from_xyzw(orientation.x, orientation.y, orientation.z, orientation.w),
        glam::Vec3::new(position.x, position.y, position.z),
    );
    projection * camera.inverse()
}
// ANCHOR_END: view_projection

fn failed(operation: &'static str) -> impl Fn(vk::Result) -> String {
    move |result| format!("{operation} failed: {result}")
}

/// The application's own Vulkan instance and device on the session's GPU. Every pipeline,
/// camera and render sender must be dropped before it.
struct CubeDevice {
    _entry: ash::Entry,
    instance: ash::Instance,
    physical_device: vk::PhysicalDevice,
    device: ash::Device,
    queue_family: u32,
    queue: vk::Queue,
}

/// A physical device the cube can render on, with what creating the device needs.
struct Candidate {
    physical_device: vk::PhysicalDevice,
    name: String,
    luid: Option<[u8; 8]>,
    queue_family: u32,
    extensions: Vec<*const c_char>,
}

impl CubeDevice {
    /// The device on `adapter`, or on the first supported GPU when the session names none. A
    /// named adapter that matches no supported GPU is an error.
    fn create(adapter: Option<GraphicsAdapter>) -> ExampleResult<Self> {
        // SAFETY: loading the system's Vulkan loader has no preconditions.
        let entry = unsafe { ash::Entry::load() }?;
        let instance = create_instance(&entry)?;
        let chosen = match pick_physical_device(&instance, adapter) {
            Ok(Some(candidate)) => candidate,
            Ok(None) => {
                // SAFETY: nothing was created from the instance.
                unsafe { instance.destroy_instance(None) };
                return Err(match adapter {
                    Some(adapter) => format!(
                        "the session's GPU ({adapter}) names no graphics GPU with the external memory and semaphore support QAROS needs"
                    ),
                    None => "no graphics GPU with the external memory and semaphore support QAROS needs".to_string(),
                }
                .into());
            }
            Err(error) => {
                // SAFETY: nothing was created from the instance.
                unsafe { instance.destroy_instance(None) };
                return Err(error);
            }
        };
        let luid = chosen.luid.map_or_else(
            || "none".to_string(),
            |luid| GraphicsAdapter::Luid(luid).to_string(),
        );
        println!(
            "Rendering on {} (LUID {luid}), {}",
            chosen.name,
            if adapter.is_some() {
                "the session's GPU"
            } else {
                "the first supported GPU"
            }
        );
        let priorities = [1.0];
        let queue_info = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(chosen.queue_family)
            .queue_priorities(&priorities)];
        let device_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_info)
            .enabled_extension_names(&chosen.extensions);
        // SAFETY: valid create info for this physical device; the extensions are supported.
        let device =
            match unsafe { instance.create_device(chosen.physical_device, &device_info, None) } {
                Ok(device) => device,
                Err(error) => {
                    // SAFETY: nothing else was created from the instance.
                    unsafe { instance.destroy_instance(None) };
                    return Err(failed("vkCreateDevice")(error).into());
                }
            };
        // SAFETY: the device has queue 0 in this family.
        let queue = unsafe { device.get_device_queue(chosen.queue_family, 0) };
        Ok(Self {
            _entry: entry,
            instance,
            physical_device: chosen.physical_device,
            device,
            queue_family: chosen.queue_family,
            queue,
        })
    }

    // ANCHOR: stream_params
    fn stream_params(&self) -> VulkanStreamParams {
        VulkanStreamParams {
            physical_device: self.physical_device,
            device: self.device.handle(),
        }
    }
    // ANCHOR_END: stream_params
}

// ANCHOR: device_drop
impl Drop for CubeDevice {
    fn drop(&mut self) {
        // SAFETY: pipelines, cameras and senders on this device are dropped before it.
        unsafe {
            if let Err(error) = self.device.device_wait_idle() {
                eprintln!("vkDeviceWaitIdle failed at teardown: {error}");
            }
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}
// ANCHOR_END: device_drop

/// A Vulkan 1.1 instance, enumerating portability drivers where the loader offers
/// them.
fn create_instance(entry: &ash::Entry) -> ExampleResult<ash::Instance> {
    // SAFETY: valid entry.
    let available = unsafe { entry.enumerate_instance_extension_properties(None) }?;
    let mut extensions = Vec::new();
    let mut flags = vk::InstanceCreateFlags::empty();
    if available.iter().any(|extension| {
        extension.extension_name_as_c_str() == Ok(ash::khr::portability_enumeration::NAME)
    }) {
        extensions.push(ash::khr::portability_enumeration::NAME.as_ptr());
        flags |= vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR;
    }
    let application = vk::ApplicationInfo::default()
        .application_name(c"QAROS Vulkan cube")
        .api_version(vk::API_VERSION_1_1);
    let instance_info = vk::InstanceCreateInfo::default()
        .application_info(&application)
        .enabled_extension_names(&extensions)
        .flags(flags);
    // SAFETY: valid create info.
    let instance = unsafe { entry.create_instance(&instance_info, None) }
        .map_err(failed("vkCreateInstance"))?;
    Ok(instance)
}

// ANCHOR: pick_device
/// The first physical device that is on `adapter` (when given), supports the extensions QAROS
/// needs, and has a graphics queue family.
fn pick_physical_device(
    instance: &ash::Instance,
    adapter: Option<GraphicsAdapter>,
) -> ExampleResult<Option<Candidate>> {
    // SAFETY: live instance.
    let physical_devices =
        unsafe { instance.enumerate_physical_devices() }.map_err(failed("enumerate GPUs"))?;
    for physical_device in physical_devices {
        let mut identifiers = vk::PhysicalDeviceIDProperties::default();
        let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut identifiers);
        // SAFETY: live physical device of a Vulkan 1.1 instance.
        unsafe { instance.get_physical_device_properties2(physical_device, &mut properties) };
        let name = properties
            .properties
            .device_name_as_c_str()
            .map(|text| text.to_string_lossy().into_owned())
            .unwrap_or_default();
        let luid = (identifiers.device_luid_valid == vk::TRUE).then_some(identifiers.device_luid);
        if adapter.is_some_and(|wanted| !wanted.matches(luid, identifiers.device_uuid)) {
            continue;
        }
        let Some(extensions) = device_extensions(instance, physical_device)? else {
            continue;
        };
        let Some(queue_family) = graphics_queue_family(instance, physical_device) else {
            continue;
        };
        return Ok(Some(Candidate {
            physical_device,
            name,
            luid,
            queue_family,
            extensions,
        }));
    }
    Ok(None)
}

/// The device extensions to enable: QAROS's, plus `VK_KHR_portability_subset` where the device
/// has it (the Vulkan spec requires enabling it then) and, on Linux, `VK_KHR_external_semaphore_fd`
/// where the device has it, as the C sample does. `None` when QAROS's are missing.
fn device_extensions(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
) -> ExampleResult<Option<Vec<*const c_char>>> {
    // SAFETY: live physical device.
    let available = unsafe { instance.enumerate_device_extension_properties(physical_device) }
        .map_err(failed("enumerate extensions"))?;
    let has = |wanted: &CStr| {
        available
            .iter()
            .any(|extension| extension.extension_name_as_c_str() == Ok(wanted))
    };
    let required = qaros::vulkan::required_device_extensions();
    if !required.iter().all(|name| has(name)) {
        return Ok(None);
    }
    let mut extensions: Vec<*const c_char> = required.iter().map(|name| name.as_ptr()).collect();
    if has(ash::khr::portability_subset::NAME) {
        extensions.push(ash::khr::portability_subset::NAME.as_ptr());
    }
    if cfg!(target_os = "linux") && has(ash::khr::external_semaphore_fd::NAME) {
        extensions.push(ash::khr::external_semaphore_fd::NAME.as_ptr());
    }
    Ok(Some(extensions))
}
// ANCHOR_END: pick_device

fn graphics_queue_family(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
) -> Option<u32> {
    // SAFETY: live physical device.
    let families = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
    families
        .iter()
        .position(|family| family.queue_flags.contains(vk::QueueFlags::GRAPHICS))
        .and_then(|index| u32::try_from(index).ok())
}

/// The render pass and pipeline every camera draws the cube with.
struct CubePipeline {
    device: ash::Device,
    render_pass: vk::RenderPass,
    layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
}

impl CubePipeline {
    fn create(gpu: &CubeDevice) -> ExampleResult<Self> {
        let mut created = Self {
            device: gpu.device.clone(),
            render_pass: vk::RenderPass::null(),
            layout: vk::PipelineLayout::null(),
            pipeline: vk::Pipeline::null(),
        };
        created.render_pass = create_render_pass(&created.device)?;
        let push_constants = [vk::PushConstantRange::default()
            .stage_flags(vk::ShaderStageFlags::VERTEX)
            .size(u32::try_from(size_of::<Mat4>())?)];
        let layout_info =
            vk::PipelineLayoutCreateInfo::default().push_constant_ranges(&push_constants);
        // SAFETY: live device; valid create info.
        created.layout = unsafe { created.device.create_pipeline_layout(&layout_info, None) }
            .map_err(failed("create cube layout"))?;
        created.pipeline =
            create_graphics_pipeline(&created.device, created.render_pass, created.layout)?;
        Ok(created)
    }
}

impl Drop for CubePipeline {
    fn drop(&mut self) {
        // SAFETY: every camera drawing with this pipeline was drained and dropped before it;
        // destroying null handles is a no-op.
        unsafe {
            self.device.destroy_pipeline(self.pipeline, None);
            self.device.destroy_pipeline_layout(self.layout, None);
            self.device.destroy_render_pass(self.render_pass, None);
        }
    }
}

/// One subpass with a color and a depth attachment, both cleared and stored, entered and left
/// in their attachment layouts (the frame's barriers do the transitions).
fn create_render_pass(device: &ash::Device) -> ExampleResult<vk::RenderPass> {
    let attachment = |format, layout| {
        vk::AttachmentDescription::default()
            .format(format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(layout)
            .final_layout(layout)
    };
    let attachments = [
        attachment(COLOR_FORMAT, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL),
        attachment(
            DEPTH_FORMAT,
            vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        ),
    ];
    let color_reference = [vk::AttachmentReference::default()
        .attachment(0)
        .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
    let depth_reference = vk::AttachmentReference::default()
        .attachment(1)
        .layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);
    let subpasses = [vk::SubpassDescription::default()
        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
        .color_attachments(&color_reference)
        .depth_stencil_attachment(&depth_reference)];
    let render_pass_info = vk::RenderPassCreateInfo::default()
        .attachments(&attachments)
        .subpasses(&subpasses);
    // SAFETY: live device; valid create info.
    let render_pass = unsafe { device.create_render_pass(&render_pass_info, None) }
        .map_err(failed("create cube render pass"))?;
    Ok(render_pass)
}

fn create_shader_module(device: &ash::Device, spirv: &[u8]) -> ExampleResult<vk::ShaderModule> {
    let words = ash::util::read_spv(&mut std::io::Cursor::new(spirv))?;
    let shader_info = vk::ShaderModuleCreateInfo::default().code(&words);
    // SAFETY: live device; the words are SPIR-V validated at check-in (spirv-val).
    let module = unsafe { device.create_shader_module(&shader_info, None) }
        .map_err(failed("create cube shader"))?;
    Ok(module)
}

/// The C sample's pipeline: no vertex input (the vertex shader indexes its own corners), no
/// culling, depth test less, dynamic viewport and scissor.
fn create_graphics_pipeline(
    device: &ash::Device,
    render_pass: vk::RenderPass,
    layout: vk::PipelineLayout,
) -> ExampleResult<vk::Pipeline> {
    let vertex_module = create_shader_module(device, VERTEX_SHADER)?;
    let fragment_module = match create_shader_module(device, FRAGMENT_SHADER) {
        Ok(module) => module,
        Err(error) => {
            // SAFETY: the module is not in use.
            unsafe { device.destroy_shader_module(vertex_module, None) };
            return Err(error);
        }
    };
    let stages = [
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vertex_module)
            .name(c"main"),
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(fragment_module)
            .name(c"main"),
    ];
    let vertex_input = vk::PipelineVertexInputStateCreateInfo::default();
    let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
        .topology(vk::PrimitiveTopology::TRIANGLE_LIST);
    let viewport = vk::PipelineViewportStateCreateInfo::default()
        .viewport_count(1)
        .scissor_count(1);
    let rasterization = vk::PipelineRasterizationStateCreateInfo::default()
        .polygon_mode(vk::PolygonMode::FILL)
        .cull_mode(vk::CullModeFlags::NONE)
        .line_width(1.0);
    let multisample = vk::PipelineMultisampleStateCreateInfo::default()
        .rasterization_samples(vk::SampleCountFlags::TYPE_1);
    let depth_stencil = vk::PipelineDepthStencilStateCreateInfo::default()
        .depth_test_enable(true)
        .depth_write_enable(true)
        .depth_compare_op(vk::CompareOp::LESS);
    let blend_attachments = [vk::PipelineColorBlendAttachmentState::default()
        .color_write_mask(vk::ColorComponentFlags::RGBA)];
    let color_blend =
        vk::PipelineColorBlendStateCreateInfo::default().attachments(&blend_attachments);
    let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
    let dynamic = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);
    let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
        .stages(&stages)
        .vertex_input_state(&vertex_input)
        .input_assembly_state(&input_assembly)
        .viewport_state(&viewport)
        .rasterization_state(&rasterization)
        .multisample_state(&multisample)
        .depth_stencil_state(&depth_stencil)
        .color_blend_state(&color_blend)
        .dynamic_state(&dynamic)
        .layout(layout)
        .render_pass(render_pass);
    // SAFETY: live device; every referenced object is alive for the call.
    let created = unsafe {
        device.create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None)
    };
    // SAFETY: the pipeline is created (or not); the modules are no longer needed.
    unsafe {
        device.destroy_shader_module(vertex_module, None);
        device.destroy_shader_module(fragment_module, None);
    }
    let pipelines = created.map_err(|(_, result)| failed("create cube pipeline")(result))?;
    Ok(pipelines
        .into_iter()
        .next()
        .ok_or("vkCreateGraphicsPipelines returned no pipeline")?)
}

/// One frame slot: a command buffer and the fence telling when its last submission finished.
struct Slot {
    commands: vk::CommandBuffer,
    fence: vk::Fence,
}

/// One eye's attachments for a pair of sender images, made the first time the pair is seen.
/// The sender's images live as long as the sender, so after the first frame nothing is created
/// per frame.
struct EyeTarget {
    views: [vk::ImageView; 2],
    framebuffer: vk::Framebuffer,
}

/// One target's camera. It keeps up to [`FRAMES_IN_FLIGHT`] frames on the GPU and waits only
/// when it reuses the oldest slot, never for the frame just submitted: that frame's semaphore
/// hands the work to the sender, which waits for it on the GPU.
struct Camera {
    device: ash::Device,
    queue: vk::Queue,
    queue_family: u32,
    pool: vk::CommandPool,
    slots: Vec<Slot>,
    next_slot: usize,
    eye_targets: HashMap<(vk::Image, vk::Image), EyeTarget>,
}

impl Camera {
    fn create(gpu: &CubeDevice) -> ExampleResult<Self> {
        let pool_info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(gpu.queue_family)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        // SAFETY: live device.
        let pool = unsafe { gpu.device.create_command_pool(&pool_info, None) }
            .map_err(failed("create camera pool"))?;
        let mut camera = Self {
            device: gpu.device.clone(),
            queue: gpu.queue,
            queue_family: gpu.queue_family,
            pool,
            slots: Vec::with_capacity(FRAMES_IN_FLIGHT),
            next_slot: 0,
            eye_targets: HashMap::new(),
        };
        let allocate_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(u32::try_from(FRAMES_IN_FLIGHT)?);
        // SAFETY: live pool.
        let command_buffers = unsafe { camera.device.allocate_command_buffers(&allocate_info) }
            .map_err(failed("allocate camera commands"))?;
        // Created signaled: a slot that never ran is free to use.
        let fence_info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);
        for commands in command_buffers {
            // SAFETY: live device.
            let fence = unsafe { camera.device.create_fence(&fence_info, None) }
                .map_err(failed("create camera fence"))?;
            camera.slots.push(Slot { commands, fence });
        }
        Ok(camera)
    }

    /// The next slot, once its previous frame (submitted [`FRAMES_IN_FLIGHT`] frames ago) has
    /// finished on the GPU: the back-pressure a renderer should have.
    fn next_slot(&mut self) -> ExampleResult<usize> {
        let index = self.next_slot;
        self.next_slot = (index + 1) % self.slots.len();
        // SAFETY: live fence.
        unsafe {
            self.device
                .wait_for_fences(&[self.slots[index].fence], true, u64::MAX)
        }
        .map_err(failed("wait for camera slot"))?;
        Ok(index)
    }

    /// Submits the slot's work (none when `commands` is empty), signalling the frame's
    /// semaphore and the slot's fence. The fence is reset only here, so a frame that fails
    /// before its submission leaves the slot usable.
    fn submit(
        &self,
        slot: usize,
        commands: &[vk::CommandBuffer],
        semaphore: vk::Semaphore,
    ) -> ExampleResult {
        let semaphores = [semaphore];
        let signalled: &[vk::Semaphore] = if semaphore == vk::Semaphore::null() {
            &[]
        } else {
            &semaphores
        };
        let submit = vk::SubmitInfo::default()
            .command_buffers(commands)
            .signal_semaphores(signalled);
        let fence = self.slots[slot].fence;
        // SAFETY: the slot's fence is signalled (next_slot waited for it) and not in use; the
        // command buffers are recorded; the queue is used from this thread only.
        unsafe {
            self.device
                .reset_fences(&[fence])
                .map_err(failed("reset camera slot"))?;
            self.device
                .queue_submit(self.queue, &[submit], fence)
                .map_err(failed("submit cube frame"))?;
        }
        Ok(())
    }

    // ANCHOR: camera_render
    /// Records and submits one stereo frame into the sender's images and signals `semaphore`
    /// when the GPU is done with it, leaving every image in [`SHOWN_LAYOUT`], released to the
    /// external queue family.
    fn render(
        &mut self,
        pipeline: &CubePipeline,
        views: &[VideoFrameView],
        images: &[VulkanImage],
        semaphore: vk::Semaphore,
        eyes: &[ViewPose; 2],
        seconds: f32,
    ) -> ExampleResult {
        if views.len() != FRAME_VIEWS.len() || images.len() != FRAME_VIEWS.len() {
            return Err("expected the separated stereo color and depth layout".into());
        }
        let slot = self.next_slot()?;
        let commands = self.slots[slot].commands;
        let begin = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        // SAFETY: the slot's previous submission finished (next_slot).
        unsafe {
            self.device
                .reset_command_buffer(commands, vk::CommandBufferResetFlags::empty())
                .map_err(failed("reset camera commands"))?;
            self.device
                .begin_command_buffer(commands, &begin)
                .map_err(failed("begin cube commands"))?;
        }
        let acquire: Vec<_> = images.iter().map(|image| self.acquire(image)).collect();
        // SAFETY: recording; the barriers name live sender images.
        unsafe {
            self.device.cmd_pipeline_barrier(
                commands,
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
                    | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &acquire,
            );
        }
        let model = cube_model(seconds);
        for (eye, eye_view) in [Eye::Left, Eye::Right].into_iter().zip(eyes) {
            let mvp = view_projection(eye_view) * model;
            self.record_eye(commands, pipeline, views, images, eye, &mvp)?;
        }
        let release: Vec<_> = images.iter().map(|image| self.release(image)).collect();
        // SAFETY: recording; the barriers name live sender images.
        unsafe {
            self.device.cmd_pipeline_barrier(
                commands,
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
                    | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS
                    | vk::PipelineStageFlags::LATE_FRAGMENT_TESTS,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &release,
            );
            self.device
                .end_command_buffer(commands)
                .map_err(failed("end cube commands"))?;
        }
        self.submit(slot, &[commands], semaphore)
    }
    // ANCHOR_END: camera_render

    /// Draws the cube into one eye's color and depth views.
    #[allow(clippy::cast_precision_loss)]
    fn record_eye(
        &mut self,
        commands: vk::CommandBuffer,
        pipeline: &CubePipeline,
        views: &[VideoFrameView],
        images: &[VulkanImage],
        eye: Eye,
        mvp: &Mat4,
    ) -> ExampleResult {
        let find = |kind| {
            views
                .iter()
                .find(|view| view.eye == eye && view.kind == kind)
                .ok_or_else(|| format!("the frame lacks a {kind:?} view for the {eye:?} eye"))
        };
        let color_view = find(ViewKind::Color)?;
        let depth_view = find(ViewKind::Depth)?;
        let image_of = |view: &VideoFrameView| {
            images.get(view.texture_index as usize).ok_or_else(|| {
                format!(
                    "view names image {}, which the frame lacks",
                    view.texture_index
                )
            })
        };
        let color_image = image_of(color_view)?;
        let depth_image = image_of(depth_view)?;
        let framebuffer = self.eye_target(pipeline, color_image, depth_image)?;
        let offset = vk::Offset2D {
            x: i32::try_from(color_view.start_x.min(color_view.end_x))?,
            y: i32::try_from(color_view.start_y.min(color_view.end_y))?,
        };
        let extent = vk::Extent2D {
            width: color_view.end_x.abs_diff(color_view.start_x),
            height: color_view.end_y.abs_diff(color_view.start_y),
        };
        let area = vk::Rect2D { offset, extent };
        let clears = [
            vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: CLEAR_COLOR,
                },
            },
            vk::ClearValue {
                depth_stencil: vk::ClearDepthStencilValue {
                    depth: 1.0,
                    stencil: 0,
                },
            },
        ];
        let pass_begin = vk::RenderPassBeginInfo::default()
            .render_pass(pipeline.render_pass)
            .framebuffer(framebuffer)
            .render_area(area)
            .clear_values(&clears);
        // Flipped: Vulkan's framebuffer Y points down, the projection's up.
        let viewport = vk::Viewport {
            x: offset.x as f32,
            y: offset.y as f32 + extent.height as f32,
            width: extent.width as f32,
            height: -(extent.height as f32),
            min_depth: 0.0,
            max_depth: 1.0,
        };
        let push_constants: Vec<u8> = mvp
            .to_cols_array()
            .iter()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        // SAFETY: recording; the framebuffer's images are in their attachment layouts after
        // the acquire barrier; negative viewport heights are core in Vulkan 1.1.
        unsafe {
            self.device
                .cmd_begin_render_pass(commands, &pass_begin, vk::SubpassContents::INLINE);
            self.device.cmd_set_viewport(commands, 0, &[viewport]);
            self.device.cmd_set_scissor(commands, 0, &[area]);
            self.device.cmd_bind_pipeline(
                commands,
                vk::PipelineBindPoint::GRAPHICS,
                pipeline.pipeline,
            );
            self.device.cmd_push_constants(
                commands,
                pipeline.layout,
                vk::ShaderStageFlags::VERTEX,
                0,
                &push_constants,
            );
            self.device.cmd_draw(commands, 36, 1, 0, 0);
            self.device.cmd_end_render_pass(commands);
        }
        Ok(())
    }

    /// The framebuffer over a color and a depth image of the sender, created on first use.
    fn eye_target(
        &mut self,
        pipeline: &CubePipeline,
        color: &VulkanImage,
        depth: &VulkanImage,
    ) -> ExampleResult<vk::Framebuffer> {
        let key = (color.image, depth.image);
        if let Some(target) = self.eye_targets.get(&key) {
            return Ok(target.framebuffer);
        }
        let mut target = EyeTarget {
            views: [vk::ImageView::null(); 2],
            framebuffer: vk::Framebuffer::null(),
        };
        let created = self.fill_eye_target(&mut target, pipeline, color, depth);
        let framebuffer = target.framebuffer;
        // Kept even when incomplete, so that Drop destroys what was created.
        self.eye_targets.insert(key, target);
        created.map(|()| framebuffer)
    }

    fn fill_eye_target(
        &self,
        target: &mut EyeTarget,
        pipeline: &CubePipeline,
        color: &VulkanImage,
        depth: &VulkanImage,
    ) -> ExampleResult {
        for ((image, format, aspect), view) in [
            (color, COLOR_FORMAT, vk::ImageAspectFlags::COLOR),
            (depth, DEPTH_FORMAT, vk::ImageAspectFlags::DEPTH),
        ]
        .into_iter()
        .zip(&mut target.views)
        {
            if qaros::vulkan::assumed_vk_format(image.size.format) != Some(format) {
                return Err(format!(
                    "the sender reports {:?} where the cube renders {format:?}",
                    image.size.format
                )
                .into());
            }
            let view_info = vk::ImageViewCreateInfo::default()
                .image(image.image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(format)
                .subresource_range(whole_image(aspect));
            // SAFETY: live device; the sender's image lives as long as the sender, which
            // outlives this camera (Target drops the camera first).
            *view = unsafe { self.device.create_image_view(&view_info, None) }
                .map_err(failed("create camera image view"))?;
        }
        let framebuffer_info = vk::FramebufferCreateInfo::default()
            .render_pass(pipeline.render_pass)
            .attachments(&target.views)
            .width(color.size.width)
            .height(color.size.height)
            .layers(1);
        // SAFETY: live device, render pass and views.
        target.framebuffer = unsafe { self.device.create_framebuffer(&framebuffer_info, None) }
            .map_err(failed("create camera framebuffer"))?;
        Ok(())
    }

    // ANCHOR: acquire_release
    /// From the reported state to the attachment layout on this queue family, with an
    /// ownership transfer only when another family (the external one) owns the image.
    fn acquire(&self, image: &VulkanImage) -> vk::ImageMemoryBarrier<'static> {
        let (layout, access, aspect) = attachment_of(image);
        let unowned = image.queue_family_index == vk::QUEUE_FAMILY_IGNORED
            || image.queue_family_index == self.queue_family;
        let (source_family, destination_family) = if unowned {
            (vk::QUEUE_FAMILY_IGNORED, vk::QUEUE_FAMILY_IGNORED)
        } else {
            (image.queue_family_index, self.queue_family)
        };
        vk::ImageMemoryBarrier::default()
            .image(image.image)
            .old_layout(image.layout)
            .new_layout(layout)
            .src_queue_family_index(source_family)
            .dst_queue_family_index(destination_family)
            .dst_access_mask(access)
            .subresource_range(whole_image(aspect))
    }

    /// From the attachment layout to [`SHOWN_LAYOUT`], released to the external family.
    fn release(&self, image: &VulkanImage) -> vk::ImageMemoryBarrier<'static> {
        let (layout, access, aspect) = attachment_of(image);
        vk::ImageMemoryBarrier::default()
            .image(image.image)
            .old_layout(layout)
            .new_layout(SHOWN_LAYOUT)
            .src_queue_family_index(self.queue_family)
            .dst_queue_family_index(vk::QUEUE_FAMILY_EXTERNAL)
            .src_access_mask(access)
            .subresource_range(whole_image(aspect))
    }

    /// Signals the semaphore of a begun frame that could not be rendered, so that the sender
    /// can still consume it.
    fn discard(&mut self, semaphore: vk::Semaphore) -> ExampleResult {
        let slot = self.next_slot()?;
        self.submit(slot, &[], semaphore)
    }
    // ANCHOR_END: acquire_release

    /// Waits for every submitted frame. Teardown only.
    fn drain(&self) -> ExampleResult {
        let fences: Vec<vk::Fence> = self.slots.iter().map(|slot| slot.fence).collect();
        if fences.is_empty() {
            return Ok(());
        }
        // SAFETY: live fences.
        unsafe { self.device.wait_for_fences(&fences, true, u64::MAX) }
            .map_err(failed("drain camera submissions"))?;
        Ok(())
    }
}

impl Drop for Camera {
    fn drop(&mut self) {
        // The camera's own work must finish before the attachments and command buffers it
        // refers to go away.
        if let Err(error) = self.drain() {
            eprintln!("Camera drain failed during teardown: {error}");
        }
        // SAFETY: no submission of this camera is pending any more; destroying null handles
        // is a no-op.
        unsafe {
            for target in self.eye_targets.values() {
                self.device.destroy_framebuffer(target.framebuffer, None);
                for view in target.views {
                    self.device.destroy_image_view(view, None);
                }
            }
            for slot in &self.slots {
                self.device.destroy_fence(slot.fence, None);
            }
            self.device.destroy_command_pool(self.pool, None);
        }
    }
}

/// The attachment layout, write access and aspect of a sender image.
fn attachment_of(image: &VulkanImage) -> (vk::ImageLayout, vk::AccessFlags, vk::ImageAspectFlags) {
    if image.size.format == PixelFormat::D32Float {
        (
            vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
            vk::ImageAspectFlags::DEPTH,
        )
    } else {
        (
            vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            vk::ImageAspectFlags::COLOR,
        )
    }
}

fn whole_image(aspect: vk::ImageAspectFlags) -> vk::ImageSubresourceRange {
    vk::ImageSubresourceRange {
        aspect_mask: aspect,
        base_mip_level: 0,
        level_count: 1,
        base_array_layer: 0,
        layer_count: 1,
    }
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::expect_used,
        clippy::unwrap_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use std::path::PathBuf;

    use qaros::{Fov, LibraryConfig, OnboardMethod, PeerPresentation, RuntimeConfig};

    use super::*;

    /// `colors[1]` of the vertex shader, the face at +Z, in `R8G8B8A8_UNORM`.
    const FRONT_FACE_BYTES: [u8; 4] = [51, 255, 51, 255];
    /// [`CLEAR_COLOR`] in `R8G8B8A8_UNORM`; 0.1 * 255 = 25.5 may round either way.
    const CLEAR_BYTES: [u8; 4] = [20, 20, 26, 255];

    /// An eye 1 m in front of the volume center at `x`, looking down -Z.
    fn eye_in_front(x: f32) -> ViewPose {
        ViewPose {
            pose: Pose::from_position(Vec3::new(x, 0.0, 1.0)),
            fov: Fov {
                angle_left: -0.6,
                angle_right: 0.6,
                angle_up: 0.6,
                angle_down: -0.6,
            },
        }
    }

    #[test]
    fn the_volume_center_projects_to_the_view_center_at_forward_depth() {
        let clip = view_projection(&eye_in_front(0.0)) * Vec4::new(0.0, 0.0, 0.0, 1.0);
        let device = clip / clip.w;
        let expected_depth = FAR_PLANE * (1.0 - NEAR_PLANE) / (FAR_PLANE - NEAR_PLANE);
        assert!(device.x.abs() < 1e-6 && device.y.abs() < 1e-6, "{device:?}");
        assert!((device.z - expected_depth).abs() < 1e-5, "{device:?}");
    }

    #[test]
    fn the_resting_cube_spans_the_scaled_unit_cube() {
        let corner = cube_model(0.0) * Vec4::new(1.0, 1.0, 1.0, 1.0);
        assert!(
            (corner - Vec4::new(0.3, 0.3, 0.3, 1.0)).length() < 1e-6,
            "{corner:?}"
        );
    }

    fn stub_path() -> PathBuf {
        if let Some(path) = std::env::var_os("QAROS_STUB_PATH") {
            return PathBuf::from(path);
        }
        let file_name = format!(
            "{}qaros_stub{}",
            std::env::consts::DLL_PREFIX,
            std::env::consts::DLL_SUFFIX
        );
        let executable = std::env::current_exe().expect("test executable path");
        let deps_directory = executable.parent().expect("deps directory");
        [
            deps_directory,
            deps_directory.parent().expect("profile directory"),
        ]
        .iter()
        .map(|directory| directory.join(&file_name))
        .find(|candidate| candidate.exists())
        .unwrap_or_else(|| {
            panic!("{file_name} not found; build it with `cargo build -p qaros-stub`")
        })
    }

    /// Copies a shown color image (released to the external family) into host memory and
    /// releases it again, so the frame's written-back state still holds.
    fn read_back(gpu: &CubeDevice, image: &VulkanImage) -> Vec<u8> {
        let device = &gpu.device;
        let byte_count = u64::from(image.size.width) * u64::from(image.size.height) * 4;
        // SAFETY: live device and physical device; every submission is waited for before the
        // objects are destroyed.
        unsafe {
            let buffer = device
                .create_buffer(
                    &vk::BufferCreateInfo::default()
                        .size(byte_count)
                        .usage(vk::BufferUsageFlags::TRANSFER_DST),
                    None,
                )
                .expect("buffer");
            let requirements = device.get_buffer_memory_requirements(buffer);
            let properties = gpu
                .instance
                .get_physical_device_memory_properties(gpu.physical_device);
            let host_visible =
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
            let memory_type_index = (0..properties.memory_type_count)
                .find(|&index| {
                    requirements.memory_type_bits & (1 << index) != 0
                        && properties.memory_types[index as usize]
                            .property_flags
                            .contains(host_visible)
                })
                .expect("host-visible memory");
            let memory = device
                .allocate_memory(
                    &vk::MemoryAllocateInfo::default()
                        .allocation_size(requirements.size)
                        .memory_type_index(memory_type_index),
                    None,
                )
                .expect("memory");
            device.bind_buffer_memory(buffer, memory, 0).expect("bind");
            let pool = device
                .create_command_pool(
                    &vk::CommandPoolCreateInfo::default().queue_family_index(gpu.queue_family),
                    None,
                )
                .expect("pool");
            let commands = device
                .allocate_command_buffers(
                    &vk::CommandBufferAllocateInfo::default()
                        .command_pool(pool)
                        .level(vk::CommandBufferLevel::PRIMARY)
                        .command_buffer_count(1),
                )
                .expect("command buffer")[0];
            let fence = device
                .create_fence(&vk::FenceCreateInfo::default(), None)
                .expect("fence");
            let ownership = |source, destination| {
                vk::ImageMemoryBarrier::default()
                    .image(image.image)
                    .old_layout(SHOWN_LAYOUT)
                    .new_layout(SHOWN_LAYOUT)
                    .src_queue_family_index(source)
                    .dst_queue_family_index(destination)
                    .src_access_mask(vk::AccessFlags::MEMORY_WRITE)
                    .dst_access_mask(vk::AccessFlags::MEMORY_READ)
                    .subresource_range(whole_image(vk::ImageAspectFlags::COLOR))
            };
            device
                .begin_command_buffer(commands, &vk::CommandBufferBeginInfo::default())
                .expect("begin");
            device.cmd_pipeline_barrier(
                commands,
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[ownership(vk::QUEUE_FAMILY_EXTERNAL, gpu.queue_family)],
            );
            let region = vk::BufferImageCopy::default()
                .image_subresource(vk::ImageSubresourceLayers {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    mip_level: 0,
                    base_array_layer: 0,
                    layer_count: 1,
                })
                .image_extent(vk::Extent3D {
                    width: image.size.width,
                    height: image.size.height,
                    depth: 1,
                });
            device.cmd_copy_image_to_buffer(commands, image.image, SHOWN_LAYOUT, buffer, &[region]);
            device.cmd_pipeline_barrier(
                commands,
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[ownership(gpu.queue_family, vk::QUEUE_FAMILY_EXTERNAL)],
            );
            device.end_command_buffer(commands).expect("end");
            let submitted = [commands];
            device
                .queue_submit(
                    gpu.queue,
                    &[vk::SubmitInfo::default().command_buffers(&submitted)],
                    fence,
                )
                .expect("submit");
            device
                .wait_for_fences(&[fence], true, u64::MAX)
                .expect("wait");
            let mapped = device
                .map_memory(memory, 0, byte_count, vk::MemoryMapFlags::empty())
                .expect("map");
            let bytes = std::slice::from_raw_parts(
                mapped.cast::<u8>(),
                usize::try_from(byte_count).expect("size"),
            )
            .to_vec();
            device.unmap_memory(memory);
            device.destroy_fence(fence, None);
            device.destroy_command_pool(pool, None);
            device.destroy_buffer(buffer, None);
            device.free_memory(memory, None);
            bytes
        }
    }

    fn assert_close(actual: &[u8], expected: [u8; 4], what: &str) {
        let close = actual
            .iter()
            .zip(expected)
            .all(|(&actual, expected)| actual.abs_diff(expected) <= 1);
        assert!(close, "{what}: {actual:?}, expected {expected:?}");
    }

    /// The stub's own eye poses stand 1.6 m above the volume center and look past the cube,
    /// so the frame is rendered from two eyes in front of it instead; the hand-off is the one
    /// `render_next_frame` performs. Needs a Vulkan loader and driver (see the
    /// README); without one the test is skipped unless `QAROS_REQUIRE_VULKAN` is set.
    #[test]
    fn both_eyes_show_the_green_front_face_on_the_clear_color_through_the_stub() {
        // SAFETY: loading the Vulkan loader has no preconditions.
        if let Err(error) = unsafe { ash::Entry::load() } {
            assert!(
                std::env::var_os("QAROS_REQUIRE_VULKAN").is_none(),
                "no Vulkan loader ({error})"
            );
            eprintln!("skipped: no Vulkan loader ({error})");
            return;
        }
        let storage =
            std::env::temp_dir().join(format!("qaros-vulkan-cube-test-{}", std::process::id()));
        // SAFETY: the stub implements the header's symbols.
        let library = unsafe { Library::load(stub_path(), &LibraryConfig::default()) }
            .expect("load the stub");
        let runtime = library
            .create_runtime(&RuntimeConfig {
                storage_directory: Some(storage.clone()),
                ..RuntimeConfig::default()
            })
            .expect("runtime");
        let session = runtime
            .onboard(
                &OnboardMethod::Code {
                    code: "ANY-CODE",
                    hub: None,
                },
                &PeerPresentation::default(),
                &BlockingOptions::default(),
            )
            .expect("onboard")
            .session;
        let gpu = CubeDevice::create(session.graphics_adapter().expect("adapter")).expect("device");
        let pipeline = CubePipeline::create(&gpu).expect("pipeline");
        let volume = create_volume(&session).expect("volume");
        let peer = session
            .peers()
            .expect("peers")
            .into_iter()
            .find(is_running_target)
            .expect("the stub's visualizer as a running target app");
        let mut target =
            Target::create(&library, &session, volume, &gpu, peer.id, peer.display_name)
                .expect("target");

        let mut frame = (0..100)
            .find_map(|_| target.sender.try_begin_owned_frame().expect("begin"))
            .expect("a frame");
        let eyes = [eye_in_front(-0.032), eye_in_front(0.032)];
        render_frame(&mut frame, &mut target.camera, &pipeline, &eyes, 0.0).expect("render");
        target.camera.drain().expect("drain");
        let (views, images) = {
            let vulkan_frame = frame.vulkan_frame().expect("vulkan frame");
            (vulkan_frame.views(), vulkan_frame.images())
        };
        for image in &images {
            assert_eq!(image.layout, SHOWN_LAYOUT);
            assert_eq!(image.queue_family_index, vk::QUEUE_FAMILY_EXTERNAL);
        }
        for eye in [Eye::Left, Eye::Right] {
            let view = views
                .iter()
                .find(|view| view.eye == eye && view.kind == ViewKind::Color)
                .expect("color view");
            let image = images[view.texture_index as usize];
            let pixels = read_back(&gpu, &image);
            let width = image.size.width as usize;
            let height = image.size.height as usize;
            let pixel = |x: usize, y: usize| &pixels[(y * width + x) * 4..(y * width + x) * 4 + 4];
            assert_eq!(
                pixel(width / 2, height / 2),
                FRONT_FACE_BYTES,
                "{eye:?} eye center"
            );
            assert_close(pixel(0, 0), CLEAR_BYTES, "top-left corner");
            assert_close(
                pixel(width - 1, height - 1),
                CLEAR_BYTES,
                "bottom-right corner",
            );
        }
        frame
            .show(&ShowParams {
                near: NEAR_PLANE,
                far: FAR_PLANE,
                depth_scale: 1.0,
            })
            .expect("show");

        // Later frames acquire from the written-back state, on the main path.
        let mut shown = 0;
        for _ in 0..1000 {
            if shown == 3 {
                break;
            }
            shown += usize::from(render_next_frame(&mut target, &pipeline, 0.5).expect("frame"));
        }
        assert_eq!(shown, 3);
        drop(target);
        drop(session);
        drop(runtime);
        if let Err(error) = std::fs::remove_dir_all(&storage) {
            eprintln!("cannot remove {}: {error}", storage.display());
        }
    }
}
