//! In-memory behaviour of the functions the safe layer uses, plus the
//! `qar_stub_*` control exports for tests.

use std::ffi::{CStr, c_char, c_void};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::ptr;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[allow(clippy::wildcard_imports)]
use qaros_sys::*;

use crate::Stub;
use crate::exports::{StubApi, StubReturn};
use crate::frame_stamp::{self, ViewArea};
use crate::state::{
    self, CreateCompletion, RemotePeer, ReportedAdapter, ScenarioAction, ScenarioStep, StubPanel,
    StubSubscription, StubVolume, SubscriptionKind, destroy_handle, handle_value, new_handle, note,
    success, write_c_string,
};

const RUNTIME: &str = "runtime";
const SESSION: &str = "session";
const CANCEL_TOKEN: &str = "cancel_token";
const INVITE: &str = "invite";
const PEER_SPEC: &str = "peer_spec";
const GUI_PANEL: &str = "gui_panel";
const APP_VOLUME: &str = "app_volume";
const RENDER_REQUEST: &str = "render_request";
const RENDER_SENDER: &str = "render_sender";
const FRAME_INFO: &str = "frame_info";

/// Peer id of the simulated visualizer that requests a render stream.
pub const VISUALIZER_PEER: [u8; 16] = [0x56; 16];
const VISUALIZER_DISPLAY_NAME: &str = "Stub Visualizer";
const INVITE_PREFIX: &[u8] = b"QAROS-STUB-INVITE:";
const EYE_OFFSET_METERS: f32 = 0.032;
const HEAD_HEIGHT_METERS: f32 = 1.6;
const VIEW_SIZE_PIXELS: u32 = 64;
/// Joins and leaves for runs without a test driver; see [`parse_scenario`].
const SCENARIO_VARIABLE: &str = "QAROS_STUB_SCENARIO";
/// Where `qar_library_destroy` writes the call log, pairing checks included.
const REPORT_VARIABLE: &str = "QAROS_STUB_REPORT";

struct RuntimeValue {
    storage: PathBuf,
}

struct SessionValue {
    runtime: usize,
    onboarding_id: [u8; 16],
    my_peer: [u8; 16],
    display_name: String,
    app_version: String,
    custom_info: String,
}

/// Not zero-sized, so every token gets its own heap address and the live-handle map tells
/// concurrent tokens apart.
#[derive(Default)]
struct TokenValue {
    _allocated: u8,
}

struct InviteValue {
    expires_unix: i64,
}

#[derive(Clone)]
struct PeerSpecValue {
    id: [u8; 16],
    display_name: String,
    app_version: String,
    custom_info: String,
    app_state: QarAppState,
    is_target_app: bool,
}

struct RenderRequestValue {
    target_peer: [u8; 16],
    stream_id: [u8; 16],
}

struct StubTexture {
    size: QarTextureSize,
    pitch: u32,
    bytes: Vec<u8>,
}

struct RenderSenderValue {
    session: usize,
    target_peer: [u8; 16],
    views: Vec<QarVideoFrameView>,
    textures: Vec<StubTexture>,
    /// Frames begun on this sender so far; the next frame gets this index.
    frames_begun: u64,
    /// Index of the frame whose info is alive, the one `show_frame` presents.
    open_frame: Option<u64>,
    /// Index of the last frame `show_frame` presented.
    shown_frame: Option<u64>,
    /// The Vulkan images and semaphore of a `QAR_GRAPHICS_API_VULKAN` sender.
    vulkan: Option<crate::vulkan::VulkanSender>,
    /// The stand-in shared textures and keyed mutexes of a `QAR_GRAPHICS_API_D3D11` sender.
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    d3d11: Option<crate::d3d11::D3D11Sender>,
    /// When the begin-frame wait `try_begin_frame` started began, while it
    /// has not completed. On the real runtime that wait holds the sender
    /// alive past its handle's destruction.
    begin_wait_started: Option<Instant>,
}

struct FrameInfoValue {
    sender: usize,
    target_peer: [u8; 16],
    frame_index: u64,
    views: Vec<QarVideoFrameViewEye>,
}

fn invalid(message: &str) -> QarResult {
    state::error(QarStatusCode::QAR_STATUS_ARGUMENT_NOT_SUPPORTED, message)
}

fn header_mismatch(
    header: *const QarStructureHeader,
    expected: QarStructureType,
    what: &str,
) -> Option<QarResult> {
    if header.is_null() {
        return Some(invalid(&format!("{what} is NULL")));
    }
    // SAFETY: every init structure starts with QarStructureHeader.
    let actual = unsafe { (*header).type_ };
    if actual == expected {
        None
    } else {
        note(format!(
            "ERROR {what} header type {} expected {}",
            actual.0, expected.0
        ));
        Some(state::error(
            QarStatusCode::QAR_STATUS_LOGIC_ERROR,
            format!(
                "{what} has structure type {}, expected {}",
                actual.0, expected.0
            ),
        ))
    }
}

/// # Safety
///
/// `text` must be null or a valid C string.
unsafe fn text_or_empty(text: *const c_char) -> String {
    if text.is_null() {
        String::new()
    } else {
        // SAFETY: forwarded to the caller.
        unsafe { CStr::from_ptr(text) }
            .to_string_lossy()
            .into_owned()
    }
}

fn derive_id(name: &str) -> [u8; 16] {
    let mut identifier = [0_u8; 16];
    for (half, seed) in [0xcbf2_9ce4_8422_2325_u64, 0x8422_2325_cbf2_9ce4]
        .into_iter()
        .enumerate()
    {
        let hash = name.bytes().fold(seed, |accumulated, byte| {
            (accumulated ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
        });
        identifier[half * 8..half * 8 + 8].copy_from_slice(&hash.to_le_bytes());
    }
    identifier
}

fn fresh_identity() -> [u8; 16] {
    let nanoseconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    let mut guard = state::lock();
    guard.next_identity += 1;
    let counter = guard.next_identity;
    drop(guard);
    let mut identifier = [0_u8; 16];
    identifier[..8].copy_from_slice(&nanoseconds.to_le_bytes()[..8]);
    identifier[8..].copy_from_slice(&counter.to_le_bytes());
    identifier
}

fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .fold(String::with_capacity(bytes.len() * 2), |mut text, byte| {
            let _ = write!(text, "{byte:02x}");
            text
        })
}

fn marker_path(storage: &Path, onboarding_id: &[u8; 16]) -> PathBuf {
    storage.join("qaros-stub").join(hex(onboarding_id))
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| {
            i64::try_from(elapsed.as_secs()).unwrap_or(i64::MAX)
        })
}

fn bytes_per_pixel(format: QarPixelFormat) -> u32 {
    if format == QarPixelFormat::QAR_PIXEL_FORMAT_R16G16B16A16 {
        8
    } else {
        4
    }
}

fn live_subscriptions(kind: SubscriptionKind, session: Option<usize>) -> Vec<StubSubscription> {
    let guard = state::lock();
    guard
        .subscriptions
        .iter()
        .filter(|subscription| subscription.kind == kind)
        .filter(|subscription| session.is_none_or(|address| subscription.session == address))
        .filter(|subscription| !guard.cancelled_tokens.contains(&subscription.token))
        .copied()
        .collect()
}

/// Runs a callback on a separate thread, as the real library does, and
/// waits for it.
fn run_on_library_thread(job: impl FnOnce() + Send) {
    std::thread::scope(|scope| {
        if scope.spawn(job).join().is_err() {
            note("ERROR a callback panicked".to_string());
        }
    });
}

/// Who releases a handle delivered to a callback.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Delivery {
    /// The library frees it after the callback (peer updates, app volumes).
    Borrowed,
    /// The callback's owner releases it with its `*_handle_destroy`, as the
    /// header says of render requests; one never released shows up as a live
    /// handle at teardown.
    Owned,
}

/// Delivers a new handle to each subscription; a borrowed one is freed after
/// the callback returns.
fn deliver_handles<Value, Handle>(
    subscriptions: &[StubSubscription],
    kind: &'static str,
    delivery: Delivery,
    make_value: impl Fn() -> Value,
) -> usize {
    for subscription in subscriptions {
        let handle: *mut Handle = new_handle(make_value(), kind);
        let handle_address = handle as usize;
        let callback_address = subscription.callback;
        let user_state = subscription.user_state;
        run_on_library_thread(move || {
            // SAFETY: callback_address was stored from a non-null C callback
            // of signature (Handle*, void*).
            let callback = unsafe {
                std::mem::transmute::<usize, unsafe extern "C" fn(*mut Handle, *mut c_void)>(
                    callback_address,
                )
            };
            // SAFETY: the handle is live for the call; user_state is the
            // subscriber's pointer.
            unsafe { callback(handle_address as *mut Handle, user_state as *mut c_void) };
        });
        if delivery == Delivery::Borrowed {
            // SAFETY: created above by new_handle with this Value type.
            unsafe { destroy_handle::<Value, Handle>(handle, kind) };
        }
    }
    subscriptions.len()
}

fn own_peer_spec(session: &SessionValue) -> PeerSpecValue {
    PeerSpecValue {
        id: session.my_peer,
        display_name: session.display_name.clone(),
        app_version: session.app_version.clone(),
        custom_info: session.custom_info.clone(),
        app_state: QarAppState::QAR_APP_STATE_RUNNING,
        is_target_app: false,
    }
}

/// The simulated visualizer and joined peers are viewers, so target apps.
fn remote_peer_spec(id: [u8; 16], display_name: &str, app_state: QarAppState) -> PeerSpecValue {
    PeerSpecValue {
        id,
        display_name: display_name.to_string(),
        app_version: "0".to_string(),
        custom_info: String::new(),
        app_state,
        is_target_app: true,
    }
}

/// This application's own peer first, then the visualizer and every joined peer still present.
fn peer_specs_for(session: &SessionValue) -> Vec<PeerSpecValue> {
    let guard = state::lock();
    let remote = std::iter::once((VISUALIZER_PEER, VISUALIZER_DISPLAY_NAME.to_string()))
        .chain(
            guard
                .remote_peers
                .iter()
                .map(|peer| (peer.id, peer.display_name.clone())),
        )
        .filter(|(id, _)| !guard.departed_peers.contains(id))
        .map(|(id, name)| remote_peer_spec(id, &name, QarAppState::QAR_APP_STATE_RUNNING))
        .collect::<Vec<_>>();
    drop(guard);
    std::iter::once(own_peer_spec(session))
        .chain(remote)
        .collect()
}

/// Where each peer's simulated head stands: the visualizer at the origin, joined peers half a
/// meter apart along room X in join order.
fn peer_slot(peer: [u8; 16]) -> u16 {
    state::lock()
        .remote_peers
        .iter()
        .position(|remote| remote.id == peer)
        .map_or(0, |index| u16::try_from(index + 1).unwrap_or(u16::MAX))
}

/// The pose the stub issues for one eye of one frame: distinct per peer (the slot) and per
/// frame (the head moves 1 mm back per frame), so a frame shown with another frame's or
/// another user's pose is detectable.
fn issued_eye_pose(peer: [u8; 16], frame_index: u64, eye: QarVideoFrameViewEye) -> QarPose {
    let eye_offset = match eye {
        QarVideoFrameViewEye::QAR_VIDEO_FRAME_VIEW_EYE_LEFT => -EYE_OFFSET_METERS,
        QarVideoFrameViewEye::QAR_VIDEO_FRAME_VIEW_EYE_RIGHT => EYE_OFFSET_METERS,
        _ => 0.0,
    };
    #[allow(clippy::cast_precision_loss)]
    let step_back = 0.001 * (frame_index % 10_000) as f32;
    let mut pose = defaults::qar_pose_default();
    pose.position.x = f32::from(peer_slot(peer)) * 0.5 + eye_offset;
    pose.position.y = HEAD_HEIGHT_METERS;
    pose.position.z = step_back;
    pose
}

fn short_hex(peer: [u8; 16]) -> String {
    hex(&peer[..4])
}

fn peer_departed(peer: [u8; 16]) -> bool {
    state::lock().departed_peers.contains(&peer)
}

/// The sender's receiver paused the stream, and its peer has not left.
///
/// # Safety
///
/// `stream` is null or a handle from this stub.
unsafe fn stream_is_paused(stream: *mut QarRenderSender) -> bool {
    if state::lock().paused_peers.is_empty() {
        return false;
    }
    // SAFETY: forwarded to the caller.
    let target_peer =
        unsafe { handle_value::<RenderSenderValue, QarRenderSender>(stream, RENDER_SENDER) }
            .map(|sender| sender.target_peer);
    let guard = state::lock();
    target_peer.is_some_and(|peer| {
        guard.paused_peers.contains(&peer) && !guard.departed_peers.contains(&peer)
    })
}

/// Whether the begin-frame wait of `try_begin_frame` has completed, starting it on the first
/// call: never while the receiver paused the stream, else once the `try_begin_frame` delay
/// has passed since the wait started. A handle that is not live completes at once, so the
/// caller reports it.
///
/// # Safety
///
/// `stream` is null or a handle from this stub.
unsafe fn begin_wait_completed(stream: *mut QarRenderSender) -> bool {
    // SAFETY: forwarded to the caller.
    let paused = unsafe { stream_is_paused(stream) };
    let delay = state::lock().try_begin_frame_delay;
    // SAFETY: forwarded to the caller.
    let Some(sender) =
        (unsafe { handle_value::<RenderSenderValue, QarRenderSender>(stream, RENDER_SENDER) })
    else {
        return true;
    };
    let started = *sender.begin_wait_started.get_or_insert_with(Instant::now);
    let completed = !paused && started.elapsed() >= delay;
    if completed {
        sender.begin_wait_started = None;
    }
    completed
}

fn stream_closed() -> QarResult {
    state::error(
        QarStatusCode::QAR_STATUS_RENDERING_PRODUCER_STREAM_IS_CLOSED,
        "the target peer left; the render stream is closed",
    )
}

/// Begins the next frame on a sender, shared by the blocking and the async `begin_frame`.
///
/// # Safety
///
/// `stream` must be null or a handle from this stub.
unsafe fn open_frame_info(
    stream: *mut QarRenderSender,
) -> Result<*mut QarRenderFrameInfo, QarResult> {
    // SAFETY: forwarded to the caller.
    let Some(sender) =
        (unsafe { handle_value::<RenderSenderValue, QarRenderSender>(stream, RENDER_SENDER) })
    else {
        return Err(invalid("render sender is not a live handle"));
    };
    if peer_departed(sender.target_peer) {
        return Err(stream_closed());
    }
    if let Some(open) = sender.open_frame {
        note(format!(
            "ERROR begin_frame while frame {open} is still open on the sender"
        ));
    }
    if let Some(vulkan) = sender.vulkan.as_mut() {
        match vulkan.settle_after_discard() {
            Ok(true) => note("vulkan discarded frame: semaphore replaced".to_string()),
            Ok(false) => {}
            Err(message) => note(format!("ERROR vulkan after a discarded frame: {message}")),
        }
    }
    let frame_index = sender.frames_begun;
    sender.frames_begun += 1;
    sender.open_frame = Some(frame_index);
    Ok(new_handle(
        FrameInfoValue {
            sender: stream as usize,
            target_peer: sender.target_peer,
            frame_index,
            views: sender.views.iter().map(|view| view.eye).collect(),
        },
        FRAME_INFO,
    ))
}

/// Reads the frame stamp from every color view and compares it with the frame the sender
/// issued last: the check that each shown image was rendered for the pose of its own frame.
fn pairing_report(sender: &RenderSenderValue, frame_index: u64) -> String {
    let peer = short_hex(sender.target_peer);
    let expected = frame_stamp::frame_id_bits(frame_index);
    let mut stamps = Vec::new();
    for view in sender
        .views
        .iter()
        .filter(|view| view.data_type == QarVideoFrameViewType::QAR_VIDEO_FRAME_VIEW_TYPE_COLOR)
    {
        let Some(texture) = sender.textures.get(view.texture_index as usize) else {
            continue;
        };
        let area = ViewArea {
            start_x: view.start_x.min(view.end_x),
            start_y: view.start_y.min(view.end_y),
            width: view.start_x.abs_diff(view.end_x),
            height: view.start_y.abs_diff(view.end_y),
            pitch: texture.pitch as usize,
            bytes_per_pixel: bytes_per_pixel(view.texture_format) as usize,
        };
        let right_eye = view.eye == QarVideoFrameViewEye::QAR_VIDEO_FRAME_VIEW_EYE_RIGHT;
        stamps.push((right_eye, frame_stamp::read(&texture.bytes, area)));
    }
    if stamps.iter().all(|(_, stamp)| stamp.is_none()) {
        return format!("paired peer={peer} frame={frame_index} unstamped");
    }
    let all_match = stamps.iter().all(|(right_eye, stamp)| {
        stamp.is_some_and(|stamp| stamp.frame_id == expected && stamp.right_eye == *right_eye)
    });
    if all_match {
        format!("paired peer={peer} frame={frame_index} stamped={frame_index}")
    } else {
        format!(
            "ERROR pairing peer={peer} issued frame {frame_index}, color views stamped {stamps:?}"
        )
    }
}

/// Counts the shown frame and runs the scenario steps that became due.
fn record_shown_frame(peer: [u8; 16]) {
    let due = {
        let mut guard = state::lock();
        guard.frames_shown += 1;
        *guard.frames_shown_by_peer.entry(peer).or_default() += 1;
        let frames_shown = guard.frames_shown;
        let (due, later): (Vec<ScenarioStep>, Vec<ScenarioStep>) =
            std::mem::take(&mut guard.scenario)
                .into_iter()
                .partition(|step| step.after_frames_shown <= frames_shown);
        guard.scenario = later;
        due
    };
    for step in due {
        match step.action {
            ScenarioAction::Join { peer, name } => {
                peer_join(peer, &name);
            }
            ScenarioAction::Leave { peer } => {
                peer_leave(peer, QarAppState::QAR_APP_STATE_SHUTTING_DOWN);
            }
        }
    }
}

/// Adds a peer, announces it through peer updates, and sends its render stream request.
fn peer_join(peer: [u8; 16], display_name: &str) -> usize {
    {
        let mut guard = state::lock();
        guard.departed_peers.remove(&peer);
        if peer != VISUALIZER_PEER && !guard.remote_peers.iter().any(|remote| remote.id == peer) {
            guard.remote_peers.push(RemotePeer {
                id: peer,
                display_name: display_name.to_string(),
            });
        }
    }
    note(format!("peer joined {}", short_hex(peer)));
    fire_peer_updates(
        None,
        &remote_peer_spec(peer, display_name, QarAppState::QAR_APP_STATE_RUNNING),
    );
    fire_render_requests(None, peer)
}

/// Marks a peer as gone: it leaves the peer list, a last update reports it in
/// `departed_state` (shutting down, or retired for good), and its render senders report a
/// closed stream from then on.
fn peer_leave(peer: [u8; 16], departed_state: QarAppState) {
    let display_name = {
        let mut guard = state::lock();
        guard.departed_peers.insert(peer);
        remote_display_name(&guard, peer)
    };
    note(format!("peer left {}", short_hex(peer)));
    fire_peer_updates(None, &remote_peer_spec(peer, &display_name, departed_state));
}

/// A joined peer's display name; any other peer is the simulated visualizer.
fn remote_display_name(state: &state::StubState, peer: [u8; 16]) -> String {
    state
        .remote_peers
        .iter()
        .find(|remote| remote.id == peer)
        .map_or_else(
            || VISUALIZER_DISPLAY_NAME.to_string(),
            |remote| remote.display_name.clone(),
        )
}

fn scenario_peer(name: &str) -> [u8; 16] {
    if name == "visualizer" {
        VISUALIZER_PEER
    } else {
        derive_id(name)
    }
}

/// Parses `QAROS_STUB_SCENARIO`: comma-separated `join:NAME@N` and `leave:NAME@N`, where `N`
/// counts shown frames across all senders and `visualizer` names the default peer.
fn parse_scenario(text: &str) -> Result<Vec<ScenarioStep>, String> {
    text.split(',')
        .map(str::trim)
        .filter(|step| !step.is_empty())
        .map(|step| {
            let (verb, rest) = step
                .split_once(':')
                .ok_or_else(|| format!("{step:?}: expected join:NAME@N or leave:NAME@N"))?;
            let (name, count) = rest
                .split_once('@')
                .ok_or_else(|| format!("{step:?}: missing @N"))?;
            let after_frames_shown = count
                .parse()
                .map_err(|_| format!("{step:?}: {count:?} is not a frame count"))?;
            let peer = scenario_peer(name);
            let action = match verb {
                "join" => ScenarioAction::Join {
                    peer,
                    name: name.to_string(),
                },
                "leave" => ScenarioAction::Leave { peer },
                other => return Err(format!("{step:?}: unknown action {other:?}")),
            };
            Ok(ScenarioStep {
                after_frames_shown,
                action,
            })
        })
        .collect()
}

fn fire_panel_updates(session: usize, panel_id: [u8; 16]) {
    let Some(panel) = state::lock()
        .panels
        .iter()
        .find(|panel| panel.id == panel_id)
        .cloned()
    else {
        return;
    };
    let subscriptions: Vec<StubSubscription> =
        live_subscriptions(SubscriptionKind::GuiPanels, Some(session))
            .into_iter()
            .filter(|subscription| subscription.filter.is_none_or(|filter| filter == panel_id))
            .collect();
    deliver_handles::<StubPanel, QarGuiPanel>(
        &subscriptions,
        GUI_PANEL,
        Delivery::Borrowed,
        || panel.clone(),
    );
}

fn fire_volume_updates(session: usize, volume_id: [u8; 16]) {
    let Some(volume) = state::lock()
        .volumes
        .iter()
        .find(|volume| volume.id == volume_id)
        .cloned()
    else {
        return;
    };
    let subscriptions = live_subscriptions(SubscriptionKind::AppVolumes, Some(session));
    deliver_handles::<StubVolume, QarAppVolume>(
        &subscriptions,
        APP_VOLUME,
        Delivery::Borrowed,
        || volume.clone(),
    );
}

fn fire_peer_updates(session: Option<usize>, peer: &PeerSpecValue) {
    let subscriptions = live_subscriptions(SubscriptionKind::PeerUpdates, session);
    deliver_handles::<PeerSpecValue, QarPeerSpec>(
        &subscriptions,
        PEER_SPEC,
        Delivery::Borrowed,
        || peer.clone(),
    );
}

fn fire_render_requests(session: Option<usize>, target_peer: [u8; 16]) -> usize {
    let subscriptions = live_subscriptions(SubscriptionKind::RenderRequests, session);
    deliver_handles::<RenderRequestValue, QarRenderStreamRequest>(
        &subscriptions,
        RENDER_REQUEST,
        Delivery::Owned,
        || RenderRequestValue {
            target_peer,
            stream_id: derive_id(&format!("stub-stream-{}", hex(&target_peer))),
        },
    )
}

/// # Safety
///
/// `session` must be null or a handle from this stub.
unsafe fn session_value<'value>(session: *const QarSession) -> Option<&'value mut SessionValue> {
    // SAFETY: forwarded to the caller.
    unsafe { handle_value::<SessionValue, QarSession>(session, SESSION) }
}

fn register_subscription(
    session: *mut QarSession,
    kind: SubscriptionKind,
    callback: usize,
    user_state: *mut c_void,
    token: *mut QarCancelToken,
    filter: Option<[u8; 16]>,
) -> QarResult {
    // SAFETY: the caller passes a session handle from this stub.
    if unsafe { session_value(session) }.is_none() {
        return invalid("session is not a live handle");
    }
    if callback == 0 {
        return invalid("callback is NULL");
    }
    state::lock().subscriptions.push(StubSubscription {
        session: session as usize,
        kind,
        callback,
        user_state: state::user_state_address(user_state),
        token: token as usize,
        filter,
    });
    success()
}

/// # Safety
///
/// `onboarding_id` must be null or writable.
unsafe fn create_session(
    runtime: *mut QarRuntime,
    onboarding_id: [u8; 16],
    presentation: &QarPeerPresentation,
) -> *mut QarSession {
    // SAFETY: presentation strings are valid C strings or NULL.
    let (display_name, app_version, custom_info) = unsafe {
        (
            text_or_empty(presentation.display_name),
            text_or_empty(presentation.app_version),
            text_or_empty(presentation.app_custom_peer_info),
        )
    };
    new_handle(
        SessionValue {
            runtime: runtime as usize,
            onboarding_id,
            my_peer: derive_id(&hex(&onboarding_id)),
            display_name,
            app_version,
            custom_info,
        },
        SESSION,
    )
}

/// # Safety
///
/// `runtime` must be null or a handle from this stub.
unsafe fn runtime_storage(runtime: *mut QarRuntime) -> Option<PathBuf> {
    // SAFETY: forwarded to the caller.
    unsafe { handle_value::<RuntimeValue, QarRuntime>(runtime, RUNTIME) }
        .map(|value| value.storage.clone())
}

fn session_still_active(onboarding_id: &[u8; 16]) -> bool {
    let sessions: Vec<usize> = state::lock()
        .live_handles
        .iter()
        .filter(|(_, kind)| **kind == SESSION)
        .map(|(address, _)| *address)
        .collect();
    sessions.into_iter().any(|address| {
        // SAFETY: address is a live session handle created by this stub.
        unsafe { session_value(address as *const QarSession) }
            .is_some_and(|session| &session.onboarding_id == onboarding_id)
    })
}

fn live_handles_of(kind: &str, owner_matches: impl Fn(usize) -> bool) -> usize {
    let addresses: Vec<usize> = state::lock()
        .live_handles
        .iter()
        .filter(|(_, handle_kind)| **handle_kind == kind)
        .map(|(address, _)| *address)
        .collect();
    addresses
        .into_iter()
        .filter(|address| owner_matches(*address))
        .count()
}

fn token_timed_out(state: &state::StubState, token_address: usize) -> bool {
    state
        .token_deadlines
        .get(&token_address)
        .is_some_and(|deadline| Instant::now() >= *deadline)
}

enum CancelReason {
    Cancelled,
    TimedOut,
}

/// Waits `delay`, ending early when the token (0 for none) is cancelled or
/// times out.
fn wait_unless_cancelled(delay: Duration, token_address: usize) -> Option<CancelReason> {
    let deadline = Instant::now() + delay;
    loop {
        if token_address != 0 {
            let guard = state::lock();
            if guard.cancelled_tokens.contains(&token_address) {
                return Some(CancelReason::Cancelled);
            }
            if token_timed_out(&guard, token_address) {
                return Some(CancelReason::TimedOut);
            }
        }
        let now = Instant::now();
        if now >= deadline {
            return None;
        }
        std::thread::sleep((deadline - now).min(Duration::from_millis(1)));
    }
}

/// The checks and value of `qar_render_sender_create`, shared by the
/// blocking and the async form.
///
/// # Safety
///
/// `session` is null or a handle from this stub; `init` is null or a stamped
/// `QarRenderSenderInit` whose extension chain is valid.
unsafe fn create_sender_value(
    session: *mut QarSession,
    init: *const QarRenderSenderInit,
) -> Result<RenderSenderValue, QarResult> {
    if let Some(failure) = header_mismatch(
        init.cast(),
        QarStructureType::QAR_STRUCTURE_TYPE_RENDERING_STREAM_SENDER_INIT,
        "QarRenderSenderInit",
    ) {
        return Err(failure);
    }
    // SAFETY: session comes from this stub.
    if unsafe { session_value(session) }.is_none() {
        return Err(invalid("session is not a live handle"));
    }
    // SAFETY: header checked.
    let init = unsafe { &*init };
    if init.peer_id.data == [0; 16] {
        return Err(invalid("peer_id is zero"));
    }
    let mut sender = build_sender(session as usize, init);
    if init.graphics_api == QarGraphicsAPI::QAR_GRAPHICS_API_VULKAN {
        // SAFETY: the chain is the caller's stamped extension structures.
        sender.vulkan = Some(unsafe { create_vulkan_sender(init, &sender) }?);
    } else if init.graphics_api == QarGraphicsAPI::QAR_GRAPHICS_API_D3D11 {
        // SAFETY: the chain is the caller's stamped extension structures.
        unsafe { attach_d3d11_sender(init, &mut sender) }?;
    } else if init.graphics_api != QarGraphicsAPI::QAR_GRAPHICS_API_CPU {
        return Err(invalid(
            "qaros-stub supports only the CPU, Vulkan and D3D11 graphics APIs",
        ));
    } else if !init.header.next.is_null() {
        return Err(invalid(
            "QAR_GRAPHICS_API_CPU requires header.next to be NULL",
        ));
    }
    Ok(sender)
}

/// The deferred half of `qar_render_sender_create_async`: the outcome its
/// callback reports.
///
/// # Safety
///
/// As [`create_sender_value`], with `init` still valid now, after the delay.
unsafe fn create_sender_for_callback(
    session_address: usize,
    init_address: usize,
) -> (QarResult, *mut QarRenderSender) {
    let session_live = state::lock().live_handles.get(&session_address) == Some(&SESSION);
    if !session_live {
        note("render_sender_create_async after its session was destroyed".to_string());
        return (
            state::error(
                QarStatusCode::QAR_STATUS_LOGIC_ERROR,
                "the session was destroyed during render_sender_create",
            ),
            ptr::null_mut(),
        );
    }
    if let Some(code) = state::take_injected_failure("render_sender_create_async_callback")
        && let Some(failure) = <QarResult as StubReturn>::injected(code)
    {
        note("render_sender_create_async failed".to_string());
        return (failure, ptr::null_mut());
    }
    // SAFETY: forwarded to the caller.
    match unsafe {
        create_sender_value(
            session_address as *mut QarSession,
            init_address as *const QarRenderSenderInit,
        )
    } {
        Ok(sender) => {
            note("render_sender_create_async delivered".to_string());
            (success(), new_handle(sender, RENDER_SENDER))
        }
        Err(failure) => {
            note("render_sender_create_async failed".to_string());
            (failure, ptr::null_mut())
        }
    }
}

impl StubApi for Stub {
    unsafe fn qar_impl_result_message(
        result: QarResult,
        out_buffer: *mut c_char,
        buffer_size: usize,
    ) {
        let message = state::lock()
            .messages
            .get(&result.error_handle)
            .cloned()
            .unwrap_or_default();
        // SAFETY: the caller provides buffer_size writable bytes.
        unsafe { write_c_string(&message, out_buffer, buffer_size) };
    }

    unsafe fn qar_impl_result_error(code: QarStatusCode, message: *const c_char) -> QarResult {
        // SAFETY: message is a C string or NULL.
        state::error(code, unsafe { text_or_empty(message) })
    }

    unsafe fn qar_impl_result_success() -> QarResult {
        success()
    }

    /// The stub never wraps a result, so its own code is the only one it carries.
    unsafe fn qar_impl_result_has_code(result: QarResult, code: QarStatusCode) -> bool {
        result.code == code
    }

    unsafe fn qar_impl_library_init(init: *const QarLibraryInit) -> QarResult {
        if let Some(failure) = header_mismatch(
            init.cast(),
            QarStructureType::QAR_STRUCTURE_TYPE_LIBRARY_INIT,
            "QarLibraryInit",
        ) {
            return failure;
        }
        let mut guard = state::lock();
        if guard.library_initialized {
            drop(guard);
            return state::error(
                QarStatusCode::QAR_STATUS_LOGIC_ERROR,
                "library already initialized",
            );
        }
        guard.library_initialized = true;
        drop(guard);
        if let Ok(text) = std::env::var(SCENARIO_VARIABLE) {
            match parse_scenario(&text) {
                Ok(steps) => state::lock().scenario = steps,
                Err(error) => note(format!("ERROR {SCENARIO_VARIABLE}: {error}")),
            }
        }
        success()
    }

    unsafe fn qar_impl_library_destroy() -> QarResult {
        let live: Vec<&'static str> = state::lock().live_handles.values().copied().collect();
        if !live.is_empty() {
            note(format!(
                "ERROR library destroyed with live handles: {live:?}"
            ));
        }
        state::lock().library_initialized = false;
        if let Some(path) = std::env::var_os(REPORT_VARIABLE) {
            let mut report = state::lock().calls.join("\n");
            report.push('\n');
            if let Err(error) = std::fs::write(&path, report) {
                eprintln!(
                    "qaros-stub: cannot write {REPORT_VARIABLE} {}: {error}",
                    Path::new(&path).display()
                );
            }
        }
        success()
    }

    unsafe fn qar_impl_runtime_info(out_info: *mut QarRuntimeInfo) -> QarResult {
        if out_info.is_null() {
            return state::error(
                QarStatusCode::QAR_STATUS_LOGIC_ERROR,
                "out_info must not be NULL",
            );
        }
        // SAFETY: out_info checked non-null; the strings are static.
        unsafe {
            *out_info = QarRuntimeInfo {
                version: c"0.9.0-stub".as_ptr(),
                flavor: c"stub".as_ptr(),
                c_api: c"v0".as_ptr(),
            };
        }
        success()
    }

    unsafe fn qar_impl_runtime_create(
        init: *const QarRuntimeInit,
        out_runtime: *mut *mut QarRuntime,
    ) -> QarResult {
        if let Some(failure) = header_mismatch(
            init.cast(),
            QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_INIT,
            "QarRuntimeInit",
        ) {
            return failure;
        }
        if out_runtime.is_null() {
            return invalid("out_runtime is NULL");
        }
        // SAFETY: header checked; strings are C strings or NULL.
        let storage = unsafe { text_or_empty((*init).storage_folder_path) };
        let storage = if storage.is_empty() {
            std::env::temp_dir().join("qaros-stub-storage")
        } else {
            PathBuf::from(storage)
        };
        // SAFETY: out_runtime is writable.
        unsafe { *out_runtime = new_handle(RuntimeValue { storage }, RUNTIME) };
        success()
    }

    unsafe fn qar_impl_runtime_handle_destroy(runtime: *mut QarRuntime) {
        let runtime_address = runtime as usize;
        let sessions = live_handles_of(SESSION, |address| {
            // SAFETY: live session handle created by this stub.
            unsafe { session_value(address as *const QarSession) }
                .is_some_and(|session| session.runtime == runtime_address)
        });
        if sessions > 0 {
            note("ERROR runtime destroyed before its sessions".to_string());
        }
        // SAFETY: runtime handles come from qar_impl_runtime_create.
        unsafe { destroy_handle::<RuntimeValue, QarRuntime>(runtime, RUNTIME) };
    }

    unsafe fn qar_impl_cancel_token_create(token: *mut *mut QarCancelToken) -> QarResult {
        if token.is_null() {
            return invalid("token is NULL");
        }
        // SAFETY: token is writable.
        unsafe { *token = new_handle(TokenValue::default(), CANCEL_TOKEN) };
        success()
    }

    /// Only an async render sender creation honours the deadline; the
    /// stub's blocking calls return at once and never wait on a token.
    unsafe fn qar_impl_cancel_token_create_with_timeout(
        token: *mut *mut QarCancelToken,
        timeout_ms: u32,
    ) -> QarResult {
        // SAFETY: as qar_impl_cancel_token_create.
        let result = unsafe { Self::qar_impl_cancel_token_create(token) };
        if result.code == QarStatusCode::QAR_STATUS_SUCCESS {
            // SAFETY: just written by qar_impl_cancel_token_create.
            let token_address = unsafe { *token } as usize;
            let deadline = Instant::now() + Duration::from_millis(u64::from(timeout_ms));
            state::lock()
                .token_deadlines
                .insert(token_address, deadline);
        }
        result
    }

    unsafe fn qar_impl_cancel_token_cancel(token: *mut QarCancelToken) -> QarResult {
        if !state::lock().live_handles.contains_key(&(token as usize)) {
            note("ERROR cancel on a dead token".to_string());
            return invalid("token is not a live handle");
        }
        state::lock().cancelled_tokens.insert(token as usize);
        success()
    }

    unsafe fn qar_impl_cancel_token_is_cancelled(token: *const QarCancelToken) -> bool {
        state::lock().cancelled_tokens.contains(&(token as usize))
    }

    unsafe fn qar_impl_cancel_token_is_timeout(token: *const QarCancelToken) -> bool {
        token_timed_out(&state::lock(), token as usize)
    }

    unsafe fn qar_impl_cancel_token_handle_destroy(token: *mut QarCancelToken) {
        let token_address = token as usize;
        let in_use = state::lock()
            .subscriptions
            .iter()
            .any(|subscription| subscription.token == token_address);
        if in_use {
            note(
                "ERROR cancel token destroyed while its subscription's session is alive"
                    .to_string(),
            );
        }
        {
            let mut guard = state::lock();
            guard.cancelled_tokens.remove(&token_address);
            guard.token_deadlines.remove(&token_address);
        }
        // SAFETY: token handles come from qar_impl_cancel_token_create.
        unsafe { destroy_handle::<TokenValue, QarCancelToken>(token, CANCEL_TOKEN) };
    }

    unsafe fn qar_impl_runtime_rejoin(
        runtime: *mut QarRuntime,
        init: *const QarRejoinInit,
        _on_progress: qar_progress_callback_t,
        _progress_state: *mut c_void,
        _cancel: *mut QarCancelToken,
        out_session: *mut *mut QarSession,
    ) -> QarResult {
        if let Some(failure) = header_mismatch(
            init.cast(),
            QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_REJOIN_INIT,
            "QarRejoinInit",
        ) {
            return failure;
        }
        // SAFETY: header checked.
        let init = unsafe { &*init };
        if let Some(failure) = header_mismatch(
            &raw const init.presentation.header,
            QarStructureType::QAR_STRUCTURE_TYPE_PEER_PRESENTATION,
            "QarPeerPresentation",
        ) {
            return failure;
        }
        if init.onboarding_id.data == [0; 16] {
            return invalid("onboarding_id is zero");
        }
        // SAFETY: the chain consists of stamped extension structures.
        if let Err(failure) = unsafe {
            validate_optional_extensions(
                init.header.next.cast_const().cast(),
                OptionalExtensions::Rejoin,
            )
        } {
            return failure;
        }
        // SAFETY: runtime comes from this stub.
        let Some(storage) = (unsafe { runtime_storage(runtime) }) else {
            return invalid("runtime is not a live handle");
        };
        if !marker_path(&storage, &init.onboarding_id.data).exists() {
            return state::error(
                QarStatusCode::QAR_STATUS_ONBOARDING_SESSION_NOT_FOUND,
                "no persisted slot for this onboarding id",
            );
        }
        // SAFETY: out_session is writable; presentation strings are valid.
        unsafe {
            *out_session = create_session(runtime, init.onboarding_id.data, &init.presentation);
        };
        success()
    }

    unsafe fn qar_impl_runtime_onboard(
        runtime: *mut QarRuntime,
        init: *const QarOnboardInit,
        on_progress: qar_progress_callback_t,
        progress_state: *mut c_void,
        _cancel: *mut QarCancelToken,
        out_onboarding_id: *mut QarOnboardingId,
        out_session: *mut *mut QarSession,
    ) -> QarResult {
        if let Some(failure) = header_mismatch(
            init.cast(),
            QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_ONBOARD_INIT,
            "QarOnboardInit",
        ) {
            return failure;
        }
        // SAFETY: header checked.
        let init = unsafe { &*init };
        if out_onboarding_id.is_null() || out_session.is_null() {
            return invalid("out parameters must not be NULL");
        }
        // SAFETY: runtime comes from this stub.
        let Some(storage) = (unsafe { runtime_storage(runtime) }) else {
            return invalid("runtime is not a live handle");
        };
        // SAFETY: the chain consists of stamped extension structures.
        if let Err(failure) =
            unsafe { validate_onboard_chain(init.header.next.cast_const().cast()) }
        {
            return failure;
        }

        if let Some(progress) = on_progress {
            let state_address = progress_state as usize;
            let progress_address = progress as usize;
            run_on_library_thread(move || {
                // SAFETY: the caller's progress callback and state, valid
                // while the blocking call runs.
                let progress = unsafe {
                    std::mem::transmute::<
                        usize,
                        unsafe extern "C" fn(QarActionSeverity, f32, *const c_char, *mut c_void),
                    >(progress_address)
                };
                // SAFETY: as above; the message is a static C string.
                unsafe {
                    progress(
                        QarActionSeverity::QAR_ACTION_SEVERITY_DONE,
                        100.0,
                        c"stub onboarding complete".as_ptr(),
                        state_address as *mut c_void,
                    );
                }
            });
        }

        // SAFETY: validated as a stamped chain above.
        let mode = unsafe { (*init.header.next.cast_const().cast::<QarStructureHeader>()).type_ };
        if mode == QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_ONBOARD_LOCAL_APP_EXT {
            // SAFETY: out-pointers checked non-null; presentation strings valid.
            unsafe {
                (*out_onboarding_id).data = [0; 16];
                *out_session = create_session(runtime, [0; 16], &init.presentation);
            }
            return success();
        }

        let onboarding_id = fresh_identity();
        let marker = marker_path(&storage, &onboarding_id);
        let persisted = marker
            .parent()
            .map_or(Ok(()), std::fs::create_dir_all)
            .and_then(|()| std::fs::write(&marker, b"qaros-stub onboarding slot\n"));
        if let Err(io_error) = persisted {
            return state::error(
                QarStatusCode::QAR_STATUS_ONBOARDING_FAILED,
                format!("cannot persist {}: {io_error}", marker.display()),
            );
        }
        // SAFETY: out-pointers checked non-null; presentation strings valid.
        unsafe {
            (*out_onboarding_id).data = onboarding_id;
            *out_session = create_session(runtime, onboarding_id, &init.presentation);
        }
        success()
    }

    unsafe fn qar_impl_runtime_forget(
        runtime: *mut QarRuntime,
        init: *const QarForgetInit,
    ) -> QarResult {
        if let Some(failure) = header_mismatch(
            init.cast(),
            QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_FORGET_INIT,
            "QarForgetInit",
        ) {
            return failure;
        }
        // SAFETY: header checked.
        let onboarding_id = unsafe { (*init).onboarding_id.data };
        if onboarding_id == [0; 16] {
            return invalid("onboarding_id is zero");
        }
        if session_still_active(&onboarding_id) {
            return state::error(
                QarStatusCode::QAR_STATUS_ONBOARDING_SESSION_STILL_ACTIVE,
                "destroy the active session handle first",
            );
        }
        // SAFETY: runtime comes from this stub.
        let Some(storage) = (unsafe { runtime_storage(runtime) }) else {
            return invalid("runtime is not a live handle");
        };
        match std::fs::remove_file(marker_path(&storage, &onboarding_id)) {
            Ok(()) => success(),
            Err(_) => state::error(
                QarStatusCode::QAR_STATUS_ONBOARDING_SESSION_NOT_FOUND,
                "no persisted slot for this onboarding id",
            ),
        }
    }

    unsafe fn qar_impl_session_request_onboarding_invite(
        session: *mut QarSession,
        init: *const QarRequestInviteInit,
        _on_progress: qar_progress_callback_t,
        _progress_state: *mut c_void,
        _cancel: *mut QarCancelToken,
        out_invite: *mut *mut QarOnboardingInvite,
    ) -> QarResult {
        if let Some(failure) = header_mismatch(
            init.cast(),
            QarStructureType::QAR_STRUCTURE_TYPE_SESSION_REQUEST_INVITE_INIT,
            "QarRequestInviteInit",
        ) {
            return failure;
        }
        // SAFETY: session comes from this stub.
        if unsafe { session_value(session) }.is_none() || out_invite.is_null() {
            return invalid("session or out_invite invalid");
        }
        // SAFETY: out_invite is writable.
        unsafe {
            *out_invite = new_handle(
                InviteValue {
                    expires_unix: unix_now() + 60,
                },
                INVITE,
            );
        }
        success()
    }

    unsafe fn qar_impl_onboarding_invite_serialized_size(
        invite: *const QarOnboardingInvite,
        out_size: *mut usize,
    ) -> QarResult {
        // SAFETY: invite comes from this stub.
        if unsafe { handle_value::<InviteValue, QarOnboardingInvite>(invite, INVITE) }.is_none() {
            return invalid("invite is not a live handle");
        }
        // SAFETY: out_size is writable.
        unsafe { *out_size = INVITE_PREFIX.len() + 8 };
        success()
    }

    unsafe fn qar_impl_onboarding_invite_serialize(
        invite: *const QarOnboardingInvite,
        out_buffer: *mut u8,
        buffer_size: usize,
        out_bytes_written: *mut usize,
    ) -> QarResult {
        // SAFETY: invite comes from this stub.
        let Some(value) =
            (unsafe { handle_value::<InviteValue, QarOnboardingInvite>(invite, INVITE) })
        else {
            return invalid("invite is not a live handle");
        };
        let mut wire = INVITE_PREFIX.to_vec();
        wire.extend_from_slice(&value.expires_unix.to_le_bytes());
        // SAFETY: out_bytes_written is writable.
        unsafe { *out_bytes_written = wire.len() };
        if buffer_size < wire.len() {
            return invalid("buffer too small");
        }
        // SAFETY: the buffer holds buffer_size >= wire.len() bytes.
        unsafe { ptr::copy_nonoverlapping(wire.as_ptr(), out_buffer, wire.len()) };
        success()
    }

    unsafe fn qar_impl_onboarding_invite_deserialize(
        wire_data: *const u8,
        wire_data_size: usize,
        out_invite: *mut *mut QarOnboardingInvite,
    ) -> QarResult {
        if wire_data.is_null() || out_invite.is_null() {
            return invalid("NULL argument");
        }
        // SAFETY: wire_data is readable for wire_data_size bytes.
        let wire = unsafe { std::slice::from_raw_parts(wire_data, wire_data_size) };
        let Some(expiry_bytes) = wire
            .strip_prefix(INVITE_PREFIX)
            .and_then(|rest| <[u8; 8]>::try_from(rest).ok())
        else {
            return state::error(
                QarStatusCode::QAR_STATUS_PAKE_ERROR,
                "not a qaros-stub invite",
            );
        };
        // SAFETY: out_invite is writable.
        unsafe {
            *out_invite = new_handle(
                InviteValue {
                    expires_unix: i64::from_le_bytes(expiry_bytes),
                },
                INVITE,
            );
        }
        success()
    }

    unsafe fn qar_impl_onboarding_invite_get_expires_unix(
        invite: *const QarOnboardingInvite,
        out_expires_unix: *mut i64,
    ) -> QarResult {
        // SAFETY: invite comes from this stub.
        let Some(value) =
            (unsafe { handle_value::<InviteValue, QarOnboardingInvite>(invite, INVITE) })
        else {
            return invalid("invite is not a live handle");
        };
        // SAFETY: out_expires_unix is writable.
        unsafe { *out_expires_unix = value.expires_unix };
        success()
    }

    unsafe fn qar_impl_onboarding_invite_get_method(
        invite: *const QarOnboardingInvite,
        out_method: *mut QarOnboardingMethod,
    ) -> QarResult {
        // SAFETY: invite comes from this stub.
        if unsafe { handle_value::<InviteValue, QarOnboardingInvite>(invite, INVITE) }.is_none() {
            return invalid("invite is not a live handle");
        }
        // SAFETY: out_method is writable.
        unsafe { *out_method = QarOnboardingMethod::QAR_ONBOARDING_METHOD_PAKE };
        success()
    }

    unsafe fn qar_impl_onboarding_invite_handle_is_valid(
        invite: *const QarOnboardingInvite,
    ) -> bool {
        state::lock().live_handles.get(&(invite as usize)) == Some(&INVITE)
    }

    unsafe fn qar_impl_onboarding_invite_handle_destroy(invite: *mut QarOnboardingInvite) {
        // SAFETY: invite handles come from this stub.
        unsafe { destroy_handle::<InviteValue, QarOnboardingInvite>(invite, INVITE) };
    }

    unsafe fn qar_impl_session_handle_destroy(session: *mut QarSession) {
        let session_address = session as usize;
        let senders = live_handles_of(RENDER_SENDER, |address| {
            // SAFETY: live render sender created by this stub.
            unsafe {
                handle_value::<RenderSenderValue, QarRenderSender>(
                    address as *const QarRenderSender,
                    RENDER_SENDER,
                )
            }
            .is_some_and(|sender| sender.session == session_address)
        });
        if senders > 0 {
            note("ERROR session destroyed before its render senders".to_string());
        }
        state::lock()
            .subscriptions
            .retain(|subscription| subscription.session != session_address);
        // SAFETY: session handles come from this stub.
        unsafe { destroy_handle::<SessionValue, QarSession>(session, SESSION) };
    }

    unsafe fn qar_impl_session_get_id(
        session: *const QarSession,
        out_session_id: *mut QarSessionId,
    ) -> QarResult {
        // SAFETY: session comes from this stub.
        let Some(value) = (unsafe { session_value(session) }) else {
            return invalid("session is not a live handle");
        };
        // SAFETY: out_session_id is writable.
        unsafe {
            (*out_session_id).data = derive_id(&format!("session-{}", hex(&value.onboarding_id)));
        };
        success()
    }

    /// The stub creates no GPU devices: it reports no adapter unless a test set one with
    /// `qar_stub_set_reported_adapter`, as a session onboarded with a launch invite that
    /// carries the target's GPU would.
    unsafe fn qar_impl_session_get_graphics_device_id(
        session: *const QarSession,
        out_device_id: *mut QarGraphicsDeviceId,
        out_present: *mut bool,
    ) -> QarResult {
        // SAFETY: session comes from this stub.
        if unsafe { session_value(session) }.is_none() {
            return invalid("session is not a live handle");
        }
        if out_device_id.is_null() || out_present.is_null() {
            return invalid("out parameters must not be NULL");
        }
        let reported = state::lock().reported_adapter;
        let Some(reported) = reported else {
            // SAFETY: out_present is writable.
            unsafe { *out_present = false };
            return success();
        };
        let mut device_id = defaults::qar_graphics_device_id_default();
        match reported {
            ReportedAdapter::Luid(luid) => {
                device_id.id_type = QarGpuDeviceIdType::QAR_GPU_DEVICE_ID_TYPE_LUID;
                // SAFETY: QarLuid is 8 plain bytes on every target (a `LUID` on Windows).
                device_id.luid = unsafe { std::mem::transmute_copy::<[u8; 8], QarLuid>(&luid) };
            }
            ReportedAdapter::Uuid(uuid) => {
                device_id.id_type = QarGpuDeviceIdType::QAR_GPU_DEVICE_ID_TYPE_UUID;
                device_id.uuid = uuid;
            }
        }
        // SAFETY: both out-pointers are writable.
        unsafe {
            *out_device_id = device_id;
            *out_present = true;
        }
        success()
    }

    unsafe fn qar_impl_session_get_my_spec(
        session: *const QarSession,
        out_handle: *mut *mut QarPeerSpec,
    ) -> QarResult {
        // SAFETY: session comes from this stub.
        let Some(value) = (unsafe { session_value(session) }) else {
            return invalid("session is not a live handle");
        };
        // SAFETY: out_handle is writable.
        unsafe { *out_handle = new_handle(own_peer_spec(value), PEER_SPEC) };
        success()
    }

    unsafe fn qar_impl_query_peer_specs_count(
        session: *mut QarSession,
        out_count: *mut usize,
    ) -> QarResult {
        // SAFETY: session comes from this stub.
        let Some(value) = (unsafe { session_value(session) }) else {
            return invalid("session is not a live handle");
        };
        let count = peer_specs_for(value).len();
        // SAFETY: out_count is writable.
        unsafe { *out_count = count };
        success()
    }

    unsafe fn qar_impl_query_peer_specs(
        session: *mut QarSession,
        out_handles: *mut *mut QarPeerSpec,
        handles_buffer_size: usize,
        out_handles_written: *mut usize,
    ) -> QarResult {
        // SAFETY: session comes from this stub.
        let Some(value) = (unsafe { session_value(session) }) else {
            return invalid("session is not a live handle");
        };
        let specs = peer_specs_for(value);
        let written = specs.len().min(handles_buffer_size);
        for (index, spec) in specs.into_iter().take(written).enumerate() {
            // SAFETY: the buffer holds handles_buffer_size slots.
            unsafe { *out_handles.add(index) = new_handle(spec, PEER_SPEC) };
        }
        // SAFETY: out_handles_written is writable.
        unsafe { *out_handles_written = written };
        success()
    }

    unsafe fn qar_impl_peer_spec_handle_is_valid(handle: *mut QarPeerSpec) -> bool {
        state::lock().live_handles.get(&(handle as usize)) == Some(&PEER_SPEC)
    }

    unsafe fn qar_impl_peer_spec_handle_destroy(handle: *mut QarPeerSpec) {
        // SAFETY: peer spec handles come from this stub.
        unsafe { destroy_handle::<PeerSpecValue, QarPeerSpec>(handle, PEER_SPEC) };
    }

    unsafe fn qar_impl_peer_spec_get_id(
        handle: *mut QarPeerSpec,
        out_id: *mut QarPeerId,
    ) -> QarResult {
        // SAFETY: handle comes from this stub.
        let Some(spec) = (unsafe { handle_value::<PeerSpecValue, QarPeerSpec>(handle, PEER_SPEC) })
        else {
            return invalid("peer spec is not a live handle");
        };
        // SAFETY: out_id is writable.
        unsafe { (*out_id).data = spec.id };
        success()
    }

    unsafe fn qar_impl_peer_spec_get_app_state(
        handle: *mut QarPeerSpec,
        out_state: *mut QarAppState,
    ) -> QarResult {
        // SAFETY: handle comes from this stub.
        let Some(spec) = (unsafe { handle_value::<PeerSpecValue, QarPeerSpec>(handle, PEER_SPEC) })
        else {
            return invalid("peer spec is not a live handle");
        };
        // SAFETY: out_state is writable.
        unsafe { *out_state = spec.app_state };
        success()
    }

    unsafe fn qar_impl_peer_spec_is_target_app(
        handle: *mut QarPeerSpec,
        out_is_target_app: *mut bool,
    ) -> QarResult {
        // SAFETY: handle comes from this stub.
        let Some(spec) = (unsafe { handle_value::<PeerSpecValue, QarPeerSpec>(handle, PEER_SPEC) })
        else {
            return invalid("peer spec is not a live handle");
        };
        if out_is_target_app.is_null() {
            return invalid("out_is_target_app must not be NULL");
        }
        // SAFETY: out_is_target_app is writable.
        unsafe { *out_is_target_app = spec.is_target_app };
        success()
    }

    unsafe fn qar_impl_peer_spec_get_display_name(
        handle: *mut QarPeerSpec,
        out_buffer: *mut c_char,
        buffer_size: usize,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe {
            peer_text(handle, out_buffer, buffer_size, |spec| {
                spec.display_name.clone()
            })
        }
    }

    unsafe fn qar_impl_peer_spec_get_app_version(
        handle: *mut QarPeerSpec,
        out_buffer: *mut c_char,
        buffer_size: usize,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe {
            peer_text(handle, out_buffer, buffer_size, |spec| {
                spec.app_version.clone()
            })
        }
    }

    unsafe fn qar_impl_peer_spec_get_app_custom_peer_info(
        handle: *mut QarPeerSpec,
        out_buffer: *mut c_char,
        buffer_size: usize,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe {
            peer_text(handle, out_buffer, buffer_size, |spec| {
                spec.custom_info.clone()
            })
        }
    }

    unsafe fn qar_impl_peer_spec_get_version_id(
        handle: *mut QarPeerSpec,
        out_buffer: *mut c_char,
        buffer_size: usize,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe {
            peer_text(handle, out_buffer, buffer_size, |spec| {
                state::lock()
                    .peer_version_ids
                    .get(&spec.id)
                    .cloned()
                    .unwrap_or_else(|| "qaros-stub".to_string())
            })
        }
    }

    unsafe fn qar_impl_peer_spec_get_room_tag(
        handle: *mut QarPeerSpec,
        out_buffer: *mut c_char,
        buffer_size: usize,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe { peer_text(handle, out_buffer, buffer_size, |_| "stub-room".to_string()) }
    }

    unsafe fn qar_impl_peer_update_display_name(
        session: *mut QarSession,
        name: *const c_char,
    ) -> QarResult {
        // SAFETY: session comes from this stub.
        let Some(value) = (unsafe { session_value(session) }) else {
            return invalid("session is not a live handle");
        };
        // SAFETY: name is a C string or NULL.
        value.display_name = unsafe { text_or_empty(name) };
        fire_peer_updates(Some(session as usize), &own_peer_spec(value));
        success()
    }

    unsafe fn qar_impl_peer_subscribe_updates(
        session: *mut QarSession,
        callback: qar_peer_update_callback_t,
        user_state: *mut c_void,
        token: *mut QarCancelToken,
    ) -> QarResult {
        register_subscription(
            session,
            SubscriptionKind::PeerUpdates,
            state::callback_address(callback),
            user_state,
            token,
            None,
        )
    }

    unsafe fn qar_impl_render_sender_subscribe_requests(
        session: *mut QarSession,
        callback: qar_render_sender_request_callback_t,
        user_state: *mut c_void,
        token: *mut QarCancelToken,
    ) -> QarResult {
        let registered = register_subscription(
            session,
            SubscriptionKind::RenderRequests,
            state::callback_address(callback),
            user_state,
            token,
            None,
        );
        let auto_request = !state::lock().disable_auto_render_request;
        if registered.code == QarStatusCode::QAR_STATUS_SUCCESS && auto_request {
            fire_render_requests(Some(session as usize), VISUALIZER_PEER);
        }
        registered
    }

    unsafe fn qar_impl_render_request_handle_destroy(request: *mut QarRenderStreamRequest) {
        // SAFETY: request handles come from this stub.
        unsafe {
            destroy_handle::<RenderRequestValue, QarRenderStreamRequest>(request, RENDER_REQUEST);
        };
    }

    unsafe fn qar_impl_render_request_get_target_peer_id(
        request: *mut QarRenderStreamRequest,
        out_peer_id: *mut QarPeerId,
    ) -> QarResult {
        // SAFETY: request comes from this stub.
        let Some(value) = (unsafe {
            handle_value::<RenderRequestValue, QarRenderStreamRequest>(request, RENDER_REQUEST)
        }) else {
            return invalid("render request is not a live handle");
        };
        // SAFETY: out_peer_id is writable.
        unsafe { (*out_peer_id).data = value.target_peer };
        success()
    }

    unsafe fn qar_impl_render_request_get_stream_id(
        request: *mut QarRenderStreamRequest,
        out_stream_id: *mut QarStreamId,
    ) -> QarResult {
        // SAFETY: request comes from this stub.
        let Some(value) = (unsafe {
            handle_value::<RenderRequestValue, QarRenderStreamRequest>(request, RENDER_REQUEST)
        }) else {
            return invalid("render request is not a live handle");
        };
        // SAFETY: out_stream_id is writable.
        unsafe { (*out_stream_id).data = value.stream_id };
        success()
    }

    unsafe fn qar_impl_render_sender_create(
        session: *mut QarSession,
        init: *mut QarRenderSenderInit,
        _cancel: *mut QarCancelToken,
        out_stream: *mut *mut QarRenderSender,
    ) -> QarResult {
        if out_stream.is_null() {
            return invalid("out_stream is NULL");
        }
        // SAFETY: forwarded from the caller.
        match unsafe { create_sender_value(session, init) } {
            Ok(sender) => {
                // SAFETY: out_stream is writable.
                unsafe { *out_stream = new_handle(sender, RENDER_SENDER) };
                success()
            }
            Err(failure) => failure,
        }
    }

    /// Validates synchronously only what needs no `init`; the creation itself
    /// runs on a detached thread that reads `init` after the delay set with
    /// `qar_stub_set_render_sender_create_delay_ms`, so a caller that frees
    /// `init` when this call returns reads freed memory there. A cancelled
    /// or timed-out creation calls back with an error and no sender, and so
    /// does every creation for a peer set with `qar_stub_set_peer_unanswering`
    /// once its time has passed. Besides
    /// `qar_stub_fail_next("render_sender_create_async", ...)`, which fails
    /// the call itself, `"render_sender_create_async_callback"` fails the
    /// next creation through its callback.
    unsafe fn qar_impl_render_sender_create_async(
        session: *mut QarSession,
        init: *mut QarRenderSenderInit,
        callback: qar_render_sender_create_callback_t,
        user_state: *mut c_void,
        token: *mut QarCancelToken,
    ) -> QarResult {
        let Some(callback) = callback else {
            return invalid("callback is NULL");
        };
        if init.is_null() {
            return invalid("init is NULL");
        }
        // SAFETY: session comes from this stub.
        if unsafe { session_value(session) }.is_none() {
            return invalid("session is not a live handle");
        }
        // SAFETY: init checked non-null; the caller keeps it alive until the callback.
        let target_peer = unsafe { (*init).peer_id.data };
        let (delay, cancel_delay, completion, unanswered_after) = {
            let guard = state::lock();
            (
                guard.render_sender_create_delay,
                guard.render_sender_create_cancel_delay,
                guard.render_sender_create_completion,
                guard.unanswering_peers.get(&target_peer).copied(),
            )
        };
        let session_address = session as usize;
        let init_address = init as usize;
        let user_state_address = user_state as usize;
        let token_address = if completion == CreateCompletion::IgnoresCancel {
            0
        } else {
            token as usize
        };
        // Not joined: the caller picks the sender up later.
        std::thread::spawn(move || {
            if completion == CreateCompletion::NeverCallsBack {
                note("render_sender_create_async never calls back".to_string());
                return;
            }
            let waited = match unanswered_after {
                Some(after) => Some(
                    wait_unless_cancelled(after, token_address).unwrap_or(CancelReason::TimedOut),
                ),
                None => wait_unless_cancelled(delay, token_address),
            };
            let (result, sender) = match waited {
                Some(CancelReason::Cancelled) => {
                    std::thread::sleep(cancel_delay);
                    note("render_sender_create_async cancelled".to_string());
                    (
                        state::error(
                            QarStatusCode::QAR_STATUS_UNCLASSIFIED,
                            "render_sender_create cancelled",
                        ),
                        ptr::null_mut(),
                    )
                }
                Some(CancelReason::TimedOut) => {
                    note("render_sender_create_async timed out".to_string());
                    (
                        state::error(
                            QarStatusCode::QAR_STATUS_TIMEOUT,
                            "render_sender_create timed out",
                        ),
                        ptr::null_mut(),
                    )
                }
                // SAFETY: the caller keeps init and what it points to alive
                // until this callback has run.
                None => unsafe { create_sender_for_callback(session_address, init_address) },
            };
            // SAFETY: the callback and user state come from the caller; called once.
            unsafe { callback(result, sender, user_state_address as *mut c_void) };
        });
        success()
    }

    unsafe fn qar_impl_render_stream_handle_destroy(handle: *mut QarRenderSender) {
        let sender_address = handle as usize;
        let frames = live_handles_of(FRAME_INFO, |address| {
            // SAFETY: live frame info created by this stub.
            unsafe {
                handle_value::<FrameInfoValue, QarRenderFrameInfo>(
                    address as *const QarRenderFrameInfo,
                    FRAME_INFO,
                )
            }
            .is_some_and(|frame| frame.sender == sender_address)
        });
        if frames > 0 {
            note("ERROR render sender destroyed with live frame infos".to_string());
        }
        // SAFETY: sender handles come from this stub.
        let begin_waiting =
            unsafe { handle_value::<RenderSenderValue, QarRenderSender>(handle, RENDER_SENDER) }
                .is_some_and(|sender| sender.begin_wait_started.is_some());
        if begin_waiting {
            // Legal, but the real runtime's wait keeps such a sender alive past this call,
            // into the session's destruction, where it can crash the process.
            note("render sender destroyed with a begin-frame wait pending".to_string());
        }
        // SAFETY: sender handles come from this stub.
        unsafe { destroy_handle::<RenderSenderValue, QarRenderSender>(handle, RENDER_SENDER) };
    }

    unsafe fn qar_impl_render_sender_layout(
        stream: *mut QarRenderSender,
        out_layout: *mut QarVideoFrameLayout,
    ) -> QarResult {
        // SAFETY: stream comes from this stub.
        let Some(sender) =
            (unsafe { handle_value::<RenderSenderValue, QarRenderSender>(stream, RENDER_SENDER) })
        else {
            return invalid("render sender is not a live handle");
        };
        // SAFETY: out_layout is writable.
        let layout = unsafe { &mut *out_layout };
        for (slot, view) in layout.views.iter_mut().zip(&sender.views) {
            *slot = *view;
        }
        layout.views_count = sender.views.len();
        for (slot, texture) in layout.textures.iter_mut().zip(&sender.textures) {
            *slot = texture.size;
        }
        layout.textures_count = sender.textures.len();
        success()
    }

    unsafe fn qar_impl_render_sender_begin_frame(
        stream: *mut QarRenderSender,
        _token: *mut QarCancelToken,
        out_frame_info: *mut *mut QarRenderFrameInfo,
    ) -> QarResult {
        // SAFETY: stream comes from this stub.
        match unsafe { open_frame_info(stream) } {
            Ok(frame_info) => {
                // SAFETY: out_frame_info is writable.
                unsafe { *out_frame_info = frame_info };
                success()
            }
            Err(failure) => failure,
        }
    }

    unsafe fn qar_impl_render_frame_info_handle_destroy(handle: *mut QarRenderFrameInfo) {
        // SAFETY: frame info handles come from this stub.
        if let Some(frame) =
            unsafe { handle_value::<FrameInfoValue, QarRenderFrameInfo>(handle, FRAME_INFO) }
        {
            let (sender_address, frame_index) = (frame.sender, frame.frame_index);
            // SAFETY: the sender outlives its frame infos (checked on sender destroy).
            if let Some(sender) = unsafe {
                handle_value::<RenderSenderValue, QarRenderSender>(
                    sender_address as *const QarRenderSender,
                    RENDER_SENDER,
                )
            } && sender.open_frame == Some(frame_index)
            {
                sender.open_frame = None;
                if sender.shown_frame != Some(frame_index)
                    && let Some(vulkan) = sender.vulkan.as_mut()
                {
                    vulkan.note_discarded_frame();
                    note(format!("vulkan frame {frame_index} dropped unshown"));
                }
            }
        }
        // SAFETY: frame info handles come from this stub.
        unsafe { destroy_handle::<FrameInfoValue, QarRenderFrameInfo>(handle, FRAME_INFO) };
    }

    unsafe fn qar_impl_render_frame_info_get_view_pose(
        handle: *mut QarRenderFrameInfo,
        view_index: usize,
        out_pose: *mut QarPose,
    ) -> QarResult {
        // SAFETY: handle comes from this stub.
        let Some(frame) =
            (unsafe { handle_value::<FrameInfoValue, QarRenderFrameInfo>(handle, FRAME_INFO) })
        else {
            return invalid("frame info is not a live handle");
        };
        let Some(eye) = frame.views.get(view_index) else {
            return invalid("view index out of range");
        };
        let pose = issued_eye_pose(frame.target_peer, frame.frame_index, *eye);
        // SAFETY: out_pose is writable.
        unsafe { *out_pose = pose };
        success()
    }

    unsafe fn qar_impl_render_frame_info_get_view_fov(
        handle: *mut QarRenderFrameInfo,
        view_index: usize,
        out_fov: *mut QarFov,
    ) -> QarResult {
        // SAFETY: handle comes from this stub.
        let Some(frame) =
            (unsafe { handle_value::<FrameInfoValue, QarRenderFrameInfo>(handle, FRAME_INFO) })
        else {
            return invalid("frame info is not a live handle");
        };
        if view_index >= frame.views.len() {
            return invalid("view index out of range");
        }
        // SAFETY: out_fov is writable.
        unsafe { *out_fov = defaults::qar_fov_default() };
        success()
    }

    unsafe fn qar_impl_render_sender_frame_cpu(
        stream: *mut QarRenderSender,
        out_frame: *mut QarVideoFrameCpu,
    ) -> QarResult {
        // SAFETY: stream comes from this stub.
        let Some(sender) =
            (unsafe { handle_value::<RenderSenderValue, QarRenderSender>(stream, RENDER_SENDER) })
        else {
            return invalid("render sender is not a live handle");
        };
        // SAFETY: out_frame is writable.
        let frame = unsafe { &mut *out_frame };
        for (slot, view) in frame.texture_views.iter_mut().zip(&sender.views) {
            *slot = *view;
        }
        frame.texture_views_count = sender.views.len();
        for (slot, texture) in frame.textures.iter_mut().zip(sender.textures.iter_mut()) {
            slot.size = texture.size;
            slot.pitch = texture.pitch;
            slot.texture_data = texture.bytes.as_mut_ptr();
            slot.texture_data_size = texture.bytes.len();
        }
        frame.textures_count = sender.textures.len();
        success()
    }

    unsafe fn qar_impl_render_sender_show_frame(
        stream: *mut QarRenderSender,
        frame_show: *const QarRenderFrameShow,
    ) -> QarResult {
        if let Some(failure) = header_mismatch(
            frame_show.cast(),
            QarStructureType::QAR_STRUCTURE_TYPE_RENDERING_END_FRAME,
            "QarRenderFrameShow",
        ) {
            return failure;
        }
        // SAFETY: stream comes from this stub.
        let Some(sender) =
            (unsafe { handle_value::<RenderSenderValue, QarRenderSender>(stream, RENDER_SENDER) })
        else {
            return invalid("render sender is not a live handle");
        };
        if peer_departed(sender.target_peer) {
            return stream_closed();
        }
        let Some(frame_index) = sender.open_frame else {
            note("ERROR show_frame without an open frame".to_string());
            return invalid("no frame is open on this render sender");
        };
        // SAFETY: header checked.
        let show = unsafe { &*frame_show };
        note(format!(
            "shown near={} far={} depth_scale={}",
            show.rendered_near_far.near_plane, show.rendered_near_far.far_plane, show.depth_scale
        ));
        sender.shown_frame = Some(frame_index);
        if let Some(vulkan) = sender.vulkan.as_mut() {
            note(vulkan.write_back_report());
            match vulkan.consume_semaphore() {
                Ok(()) => note(format!("vulkan semaphore consumed frame={frame_index}")),
                Err(message) => note(format!("ERROR {message}")),
            }
        } else if is_d3d11(sender) {
            // No pixels to check: the stand-in textures hold none.
            note(format!("d3d11 frame {frame_index} shown"));
        } else {
            note(pairing_report(sender, frame_index));
        }
        record_shown_frame(sender.target_peer);
        success()
    }

    unsafe fn qar_impl_render_sender_try_begin_frame(
        stream: *mut QarRenderSender,
        out_frame_info: *mut *mut QarRenderFrameInfo,
        out_ready: *mut bool,
    ) -> QarResult {
        if out_frame_info.is_null() || out_ready.is_null() {
            return invalid("out parameters must not be NULL");
        }
        // SAFETY: stream comes from this stub.
        if !unsafe { begin_wait_completed(stream) } {
            // SAFETY: out-pointers checked non-null.
            unsafe {
                *out_frame_info = std::ptr::null_mut();
                *out_ready = false;
            }
            return success();
        }
        // SAFETY: stream comes from this stub.
        match unsafe { open_frame_info(stream) } {
            Ok(frame_info) => {
                // SAFETY: out-pointers checked non-null.
                unsafe {
                    *out_frame_info = frame_info;
                    *out_ready = true;
                }
                success()
            }
            Err(failure) => failure,
        }
    }

    unsafe fn qar_impl_render_sender_frame_vulkan(
        stream: *mut QarRenderSender,
        out_frame: *mut *mut QarVideoFrameVulkan,
    ) -> QarResult {
        // SAFETY: stream comes from this stub.
        let Some(sender) =
            (unsafe { handle_value::<RenderSenderValue, QarRenderSender>(stream, RENDER_SENDER) })
        else {
            return invalid("render sender is not a live handle");
        };
        let Some(vulkan) = sender.vulkan.as_mut() else {
            return invalid("the render sender was not created for Vulkan");
        };
        if out_frame.is_null() {
            return invalid("out_frame is NULL");
        }
        // SAFETY: out_frame checked non-null.
        unsafe { *out_frame = vulkan.record_pointer() };
        success()
    }

    /// The record of the frame open on the sender, or outside a cycle of the frame that begins
    /// next: the keyed-mutex keys follow that frame's index.
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    unsafe fn qar_impl_render_sender_frame_d3d11(
        stream: *mut QarRenderSender,
        out_frame: *mut QarVideoFrameD3D11,
    ) -> QarResult {
        // SAFETY: stream comes from this stub.
        let Some(sender) =
            (unsafe { handle_value::<RenderSenderValue, QarRenderSender>(stream, RENDER_SENDER) })
        else {
            return invalid("render sender is not a live handle");
        };
        let Some(d3d11) = sender.d3d11.as_ref() else {
            return invalid("the render sender was not created for D3D11");
        };
        if out_frame.is_null() {
            return invalid("out_frame is NULL");
        }
        let frame_index = sender.open_frame.unwrap_or(sender.frames_begun);
        let sizes: Vec<QarTextureSize> =
            sender.textures.iter().map(|texture| texture.size).collect();
        // SAFETY: out_frame checked non-null and writable.
        d3d11.fill_record(&sender.views, &sizes, frame_index, unsafe {
            &mut *out_frame
        });
        success()
    }

    unsafe fn qar_impl_gui_panels_get_or_create(
        session: *mut QarSession,
        init: *const QarGuiPanelInit,
        out_panel_id: *mut QarGuiPanelId,
    ) -> QarResult {
        if let Some(failure) = header_mismatch(
            init.cast(),
            QarStructureType::QAR_STRUCTURE_TYPE_GUI_PANEL_INIT,
            "QarGuiPanelInit",
        ) {
            return failure;
        }
        // SAFETY: header checked.
        let init = unsafe { &*init };
        if init.common_name.is_null() {
            return invalid("common_name is NULL");
        }
        // SAFETY: session comes from this stub.
        if unsafe { session_value(session) }.is_none() {
            return invalid("session is not a live handle");
        }
        // SAFETY: strings are C strings or NULL; the peer array holds the count.
        let (common_name, display_name, visible_to) = unsafe {
            (
                text_or_empty(init.common_name),
                text_or_empty(init.display_name),
                peer_array(init.visible_to_peers, init.visible_to_peer_count),
            )
        };
        let panel_id = derive_id(&common_name);
        let mut guard = state::lock();
        if !guard.panels.iter().any(|panel| panel.id == panel_id) {
            guard.panels.push(StubPanel {
                id: panel_id,
                display_name,
                pose: init.pose,
                size: init.size,
                state: QarGuiPanelState::QAR_GUI_PANEL_STATE_VISIBLE,
                content_uri: String::new(),
                visible_to,
            });
        }
        drop(guard);
        // SAFETY: out_panel_id is writable.
        unsafe { (*out_panel_id).data = panel_id };
        success()
    }

    unsafe fn qar_impl_gui_panels_update_pose(
        session: *mut QarSession,
        id: *const QarGuiPanelId,
        pose: *const QarPose,
    ) -> QarResult {
        // SAFETY: pose is readable.
        let pose = unsafe { *pose };
        // SAFETY: forwarded.
        unsafe { update_panel(session, id, |panel| panel.pose = pose) }
    }

    unsafe fn qar_impl_gui_panels_change_size(
        session: *mut QarSession,
        id: *const QarGuiPanelId,
        size: *const QarGuiPanelSize,
    ) -> QarResult {
        // SAFETY: size is readable.
        let size = unsafe { *size };
        // SAFETY: forwarded.
        unsafe { update_panel(session, id, |panel| panel.size = size) }
    }

    unsafe fn qar_impl_gui_panels_set_state(
        session: *mut QarSession,
        id: *const QarGuiPanelId,
        state: QarGuiPanelState,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe { update_panel(session, id, |panel| panel.state = state) }
    }

    unsafe fn qar_impl_gui_panels_navigate_to_uri(
        session: *mut QarSession,
        id: *const QarGuiPanelId,
        uri: *const c_char,
    ) -> QarResult {
        // SAFETY: uri is a C string or NULL.
        let uri = unsafe { text_or_empty(uri) };
        // SAFETY: forwarded.
        unsafe { update_panel(session, id, |panel| panel.content_uri.clone_from(&uri)) }
    }

    unsafe fn qar_impl_gui_panels_close_panel(
        session: *mut QarSession,
        id: *const QarGuiPanelId,
    ) -> QarResult {
        // SAFETY: forwarded.
        let closed = unsafe {
            update_panel(session, id, |panel| {
                panel.state = QarGuiPanelState::QAR_GUI_PANEL_STATE_CLOSED;
            })
        };
        if closed.code == QarStatusCode::QAR_STATUS_SUCCESS {
            // SAFETY: id checked by update_panel.
            let panel_id = unsafe { (*id).data };
            state::lock().panels.retain(|panel| panel.id != panel_id);
        }
        closed
    }

    unsafe fn qar_impl_gui_panels_subscribe_updates(
        session: *mut QarSession,
        callback: qar_gui_panel_update_callback_t,
        user_state: *mut c_void,
        token: *mut QarCancelToken,
    ) -> QarResult {
        register_subscription(
            session,
            SubscriptionKind::GuiPanels,
            state::callback_address(callback),
            user_state,
            token,
            None,
        )
    }

    unsafe fn qar_impl_gui_panels_subscribe_panel_updates(
        session: *mut QarSession,
        id: *const QarGuiPanelId,
        callback: qar_gui_panel_update_callback_t,
        user_state: *mut c_void,
        token: *mut QarCancelToken,
    ) -> QarResult {
        if id.is_null() {
            return invalid("panel id is NULL");
        }
        // SAFETY: id is readable.
        let panel_id = unsafe { (*id).data };
        register_subscription(
            session,
            SubscriptionKind::GuiPanels,
            state::callback_address(callback),
            user_state,
            token,
            Some(panel_id),
        )
    }

    unsafe fn qar_impl_query_gui_panels_count(
        session: *mut QarSession,
        out_count: *mut usize,
    ) -> QarResult {
        // SAFETY: session comes from this stub.
        if unsafe { session_value(session) }.is_none() {
            return invalid("session is not a live handle");
        }
        // SAFETY: out_count is writable.
        unsafe { *out_count = state::lock().panels.len() };
        success()
    }

    unsafe fn qar_impl_query_gui_panels(
        session: *mut QarSession,
        out_handles: *mut *mut QarGuiPanel,
        handles_buffer_size: usize,
        out_handles_written: *mut usize,
    ) -> QarResult {
        // SAFETY: session comes from this stub.
        if unsafe { session_value(session) }.is_none() {
            return invalid("session is not a live handle");
        }
        let panels = state::lock().panels.clone();
        let written = panels.len().min(handles_buffer_size);
        for (index, panel) in panels.into_iter().take(written).enumerate() {
            // SAFETY: the buffer holds handles_buffer_size slots.
            unsafe { *out_handles.add(index) = new_handle(panel, GUI_PANEL) };
        }
        // SAFETY: out_handles_written is writable.
        unsafe { *out_handles_written = written };
        success()
    }

    unsafe fn qar_impl_gui_panel_handle_is_valid(handle: *mut QarGuiPanel) -> bool {
        state::lock().live_handles.get(&(handle as usize)) == Some(&GUI_PANEL)
    }

    unsafe fn qar_impl_gui_panel_handle_destroy(handle: *mut QarGuiPanel) {
        // SAFETY: panel handles come from this stub.
        unsafe { destroy_handle::<StubPanel, QarGuiPanel>(handle, GUI_PANEL) };
    }

    unsafe fn qar_impl_gui_panel_get_id(
        handle: *mut QarGuiPanel,
        out_id: *mut QarGuiPanelId,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe { with_panel(handle, |panel| (*out_id).data = panel.id) }
    }

    unsafe fn qar_impl_gui_panel_get_display_name(
        handle: *mut QarGuiPanel,
        out_buffer: *mut c_char,
        buffer_size: usize,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe {
            with_panel(handle, |panel| {
                write_c_string(&panel.display_name, out_buffer, buffer_size);
            })
        }
    }

    unsafe fn qar_impl_gui_panel_get_pose(
        handle: *mut QarGuiPanel,
        out_pose: *mut QarPose,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe { with_panel(handle, |panel| *out_pose = panel.pose) }
    }

    unsafe fn qar_impl_gui_panel_get_size(
        handle: *mut QarGuiPanel,
        out_size: *mut QarGuiPanelSize,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe { with_panel(handle, |panel| *out_size = panel.size) }
    }

    unsafe fn qar_impl_gui_panel_get_content_uri(
        handle: *mut QarGuiPanel,
        out_uri: *mut c_char,
        buffer_size: usize,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe {
            with_panel(handle, |panel| {
                write_c_string(&panel.content_uri, out_uri, buffer_size);
            })
        }
    }

    unsafe fn qar_impl_gui_panel_get_state(
        handle: *mut QarGuiPanel,
        out_state: *mut QarGuiPanelState,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe { with_panel(handle, |panel| *out_state = panel.state) }
    }

    unsafe fn qar_impl_gui_panel_get_visible_to_peers_count(
        handle: *mut QarGuiPanel,
        out_count: *mut usize,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe { with_panel(handle, |panel| *out_count = panel.visible_to.len()) }
    }

    unsafe fn qar_impl_gui_panel_get_visible_to_peers(
        handle: *mut QarGuiPanel,
        out_peers: *mut QarPeerId,
        peers_buffer_size: usize,
        out_peers_written: *mut usize,
    ) -> QarResult {
        // SAFETY: forwarded; the buffer holds peers_buffer_size ids.
        unsafe {
            with_panel(handle, |panel| {
                let written = panel.visible_to.len().min(peers_buffer_size);
                for (index, peer) in panel.visible_to.iter().take(written).enumerate() {
                    (*out_peers.add(index)).data = *peer;
                }
                *out_peers_written = written;
            })
        }
    }

    unsafe fn qar_impl_app_volumes_get_or_create(
        session: *mut QarSession,
        init: *const QarAppVolumeInit,
        out_volume: *mut QarAppVolumeId,
    ) -> QarResult {
        if let Some(failure) = header_mismatch(
            init.cast(),
            QarStructureType::QAR_STRUCTURE_TYPE_APP_VOLUME_INIT,
            "QarAppVolumeInit",
        ) {
            return failure;
        }
        // SAFETY: header checked.
        let init = unsafe { &*init };
        if init.common_name.is_null() {
            return invalid("common_name is NULL");
        }
        // SAFETY: session comes from this stub.
        if unsafe { session_value(session) }.is_none() {
            return invalid("session is not a live handle");
        }
        // SAFETY: strings are C strings or NULL.
        let (common_name, display_name) = unsafe {
            (
                text_or_empty(init.common_name),
                text_or_empty(init.display_name),
            )
        };
        let volume_id = derive_id(&common_name);
        let mut guard = state::lock();
        if !guard.volumes.iter().any(|volume| volume.id == volume_id) {
            guard.volumes.push(StubVolume {
                id: volume_id,
                display_name,
                pose: init.pose,
                size: init.size,
                app_pose: init.app_pose,
                app_scale: init.app_scale,
                closed: false,
            });
        }
        drop(guard);
        // SAFETY: out_volume is writable.
        unsafe { (*out_volume).data = volume_id };
        success()
    }

    unsafe fn qar_impl_app_volumes_change_pose(
        session: *mut QarSession,
        volume_id: *const QarAppVolumeId,
        pose: *const QarPose,
    ) -> QarResult {
        // SAFETY: pose is readable.
        let pose = unsafe { *pose };
        // SAFETY: forwarded.
        unsafe { update_volume(session, volume_id, |volume| volume.pose = pose) }
    }

    unsafe fn qar_impl_app_volumes_change_size(
        session: *mut QarSession,
        volume_id: *const QarAppVolumeId,
        size: *const QarAppVolumeSize,
    ) -> QarResult {
        // SAFETY: size is readable.
        let size = unsafe { *size };
        // SAFETY: forwarded.
        unsafe { update_volume(session, volume_id, |volume| volume.size = size) }
    }

    unsafe fn qar_impl_app_volumes_change_app_pose(
        session: *mut QarSession,
        volume_id: *const QarAppVolumeId,
        app_pose: *const QarPose,
    ) -> QarResult {
        // SAFETY: app_pose is readable.
        let app_pose = unsafe { *app_pose };
        // SAFETY: forwarded.
        unsafe { update_volume(session, volume_id, |volume| volume.app_pose = app_pose) }
    }

    unsafe fn qar_impl_app_volumes_change_app_scale(
        session: *mut QarSession,
        volume_id: *const QarAppVolumeId,
        scale: f32,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe { update_volume(session, volume_id, |volume| volume.app_scale = scale) }
    }

    unsafe fn qar_impl_app_volumes_change_display_name(
        session: *mut QarSession,
        volume_id: *const QarAppVolumeId,
        display_name: *const c_char,
    ) -> QarResult {
        // SAFETY: display_name is a C string or NULL.
        let display_name = unsafe { text_or_empty(display_name) };
        // SAFETY: forwarded.
        unsafe {
            update_volume(session, volume_id, |volume| {
                volume.display_name.clone_from(&display_name);
            })
        }
    }

    unsafe fn qar_impl_app_volumes_close_volume(
        session: *mut QarSession,
        volume_id: *const QarAppVolumeId,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe { update_volume(session, volume_id, |volume| volume.closed = true) }
    }

    unsafe fn qar_impl_app_volume_get_latest_pose(
        session: *mut QarSession,
        volume_id: *const QarAppVolumeId,
        out_pose: *mut QarPose,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe { read_volume(session, volume_id, |volume| *out_pose = volume.pose) }
    }

    unsafe fn qar_impl_app_volume_get_latest_size(
        session: *mut QarSession,
        volume_id: *const QarAppVolumeId,
        out_size: *mut QarAppVolumeSize,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe { read_volume(session, volume_id, |volume| *out_size = volume.size) }
    }

    unsafe fn qar_impl_app_volume_get_latest_app_pose(
        session: *mut QarSession,
        volume_id: *const QarAppVolumeId,
        out_pose: *mut QarPose,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe { read_volume(session, volume_id, |volume| *out_pose = volume.app_pose) }
    }

    unsafe fn qar_impl_app_volume_get_latest_app_scale(
        session: *mut QarSession,
        volume_id: *const QarAppVolumeId,
        out_scale: *mut f32,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe { read_volume(session, volume_id, |volume| *out_scale = volume.app_scale) }
    }

    unsafe fn qar_impl_app_volumes_subscribe_updates(
        session: *mut QarSession,
        callback: qar_app_volume_update_callback_t,
        user_state: *mut c_void,
        token: *mut QarCancelToken,
    ) -> QarResult {
        register_subscription(
            session,
            SubscriptionKind::AppVolumes,
            state::callback_address(callback),
            user_state,
            token,
            None,
        )
    }

    unsafe fn qar_impl_query_app_volumes_count(
        session: *mut QarSession,
        out_count: *mut usize,
    ) -> QarResult {
        // SAFETY: session comes from this stub.
        if unsafe { session_value(session) }.is_none() {
            return invalid("session is not a live handle");
        }
        // SAFETY: out_count is writable.
        unsafe { *out_count = state::lock().volumes.len() };
        success()
    }

    unsafe fn qar_impl_query_app_volumes(
        session: *mut QarSession,
        out_handles: *mut *mut QarAppVolume,
        handles_buffer_size: usize,
        out_handles_written: *mut usize,
    ) -> QarResult {
        // SAFETY: session comes from this stub.
        if unsafe { session_value(session) }.is_none() {
            return invalid("session is not a live handle");
        }
        let volumes = state::lock().volumes.clone();
        let written = volumes.len().min(handles_buffer_size);
        for (index, volume) in volumes.into_iter().take(written).enumerate() {
            // SAFETY: the buffer holds handles_buffer_size slots.
            unsafe { *out_handles.add(index) = new_handle(volume, APP_VOLUME) };
        }
        // SAFETY: out_handles_written is writable.
        unsafe { *out_handles_written = written };
        success()
    }

    unsafe fn qar_impl_app_volume_handle_is_valid(handle: *mut QarAppVolume) -> bool {
        state::lock().live_handles.get(&(handle as usize)) == Some(&APP_VOLUME)
    }

    unsafe fn qar_impl_app_volume_handle_destroy(handle: *mut QarAppVolume) {
        // SAFETY: volume handles come from this stub.
        unsafe { destroy_handle::<StubVolume, QarAppVolume>(handle, APP_VOLUME) };
    }

    unsafe fn qar_impl_app_volume_get_id(
        handle: *mut QarAppVolume,
        out_id: *mut QarAppVolumeId,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe { with_volume(handle, |volume| (*out_id).data = volume.id) }
    }

    unsafe fn qar_impl_app_volume_get_display_name(
        handle: *mut QarAppVolume,
        out_buffer: *mut c_char,
        buffer_size: usize,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe {
            with_volume(handle, |volume| {
                write_c_string(&volume.display_name, out_buffer, buffer_size);
            })
        }
    }

    unsafe fn qar_impl_app_volume_get_pose(
        handle: *mut QarAppVolume,
        out_pose: *mut QarPose,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe { with_volume(handle, |volume| *out_pose = volume.pose) }
    }

    unsafe fn qar_impl_app_volume_get_size(
        handle: *mut QarAppVolume,
        out_size: *mut QarAppVolumeSize,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe { with_volume(handle, |volume| *out_size = volume.size) }
    }

    unsafe fn qar_impl_app_volume_get_lifetime_status(
        handle: *mut QarAppVolume,
        out_status: *mut QarAppVolumeLifetimeStatus,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe {
            with_volume(handle, |volume| {
                *out_status = if volume.closed {
                    QarAppVolumeLifetimeStatus::QAR_APP_VOLUME_CLOSED
                } else {
                    QarAppVolumeLifetimeStatus::QAR_APP_VOLUME_ACTIVE
                };
            })
        }
    }

    unsafe fn qar_impl_app_volume_get_app_pose(
        handle: *mut QarAppVolume,
        out_pose: *mut QarPose,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe { with_volume(handle, |volume| *out_pose = volume.app_pose) }
    }

    unsafe fn qar_impl_app_volume_get_app_scale(
        handle: *mut QarAppVolume,
        out_scale: *mut f32,
    ) -> QarResult {
        // SAFETY: forwarded.
        unsafe { with_volume(handle, |volume| *out_scale = volume.app_scale) }
    }

    unsafe fn qar_impl_uuid_to_string(
        uuid_bytes: *const u8,
        out_buffer: *mut c_char,
        buffer_size: usize,
    ) -> QarResult {
        if uuid_bytes.is_null() {
            return invalid("uuid_bytes is NULL");
        }
        if buffer_size < 37 {
            return invalid("buffer too small for a UUID");
        }
        // SAFETY: uuid_bytes points to 16 bytes.
        let bytes = unsafe { std::slice::from_raw_parts(uuid_bytes, 16) };
        let text = format!(
            "{}-{}-{}-{}-{}",
            hex(&bytes[0..4]),
            hex(&bytes[4..6]),
            hex(&bytes[6..8]),
            hex(&bytes[8..10]),
            hex(&bytes[10..16])
        );
        // SAFETY: the buffer holds buffer_size >= 37 bytes.
        unsafe { write_c_string(&text, out_buffer, buffer_size) };
        success()
    }

    unsafe fn qar_impl_uuid_from_string(text: *const c_char, out_uuid_bytes: *mut u8) -> QarResult {
        // SAFETY: text is a C string or NULL.
        let text = unsafe { text_or_empty(text) };
        let digits: String = text.chars().filter(|character| *character != '-').collect();
        if digits.len() != 32 {
            return invalid("not a UUID");
        }
        let mut bytes = [0_u8; 16];
        for (index, byte) in bytes.iter_mut().enumerate() {
            match digits
                .get(index * 2..index * 2 + 2)
                .and_then(|pair| u8::from_str_radix(pair, 16).ok())
            {
                Some(value) => *byte = value,
                None => return invalid("not a UUID"),
            }
        }
        // SAFETY: out_uuid_bytes is writable for 16 bytes.
        unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), out_uuid_bytes, 16) };
        success()
    }
}

/// Which extensions may follow in an init structure's chain.
#[derive(Clone, Copy, PartialEq, Eq)]
enum OptionalExtensions {
    AfterCodeMode,
    AfterInviteMode,
    AfterLocalAppMode,
    Rejoin,
}

/// Walks the rest of a chain: the adapter (`QarGraphicsDeviceId`, logged), a
/// planned peer id after an onboarding mode, and the hub address after a
/// code. Anything else is an error.
///
/// # Safety
///
/// `next` must be null or a chain of stamped extension structures.
unsafe fn validate_optional_extensions(
    mut next: *const QarStructureHeader,
    allowed: OptionalExtensions,
) -> Result<(), QarResult> {
    while !next.is_null() {
        // SAFETY: forwarded to the caller.
        let header = unsafe { &*next };
        let extension_type = header.type_;
        let is_hub_or_peer_id = extension_type
            == QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_ONBOARD_HOST_EXT
            || extension_type == QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_ONBOARD_PEER_ID_EXT;
        if is_hub_or_peer_id && allowed == OptionalExtensions::AfterLocalAppMode {
            return Err(invalid(
                "a local app enrollment takes no hub address and no peer id",
            ));
        }
        let is_adapter =
            extension_type == QarStructureType::QAR_STRUCTURE_TYPE_SESSION_GRAPHICS_DEVICE_ID;
        let accepted = is_adapter
            || (extension_type == QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_ONBOARD_HOST_EXT
                && allowed == OptionalExtensions::AfterCodeMode)
            || (extension_type == QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_ONBOARD_PEER_ID_EXT
                && allowed != OptionalExtensions::Rejoin);
        if !accepted {
            note(format!(
                "ERROR unexpected extension type {} in the init chain",
                extension_type.0
            ));
            return Err(state::error(
                QarStatusCode::QAR_STATUS_LOGIC_ERROR,
                "unexpected extension in the init chain",
            ));
        }
        if is_adapter {
            // SAFETY: the type identifies the structure.
            let adapter = unsafe { &*next.cast::<QarGraphicsDeviceId>() };
            note(adapter_report(adapter));
        }
        next = header.next.cast_const().cast();
    }
    Ok(())
}

fn adapter_report(adapter: &QarGraphicsDeviceId) -> String {
    if adapter.id_type == QarGpuDeviceIdType::QAR_GPU_DEVICE_ID_TYPE_LUID {
        // SAFETY: QarLuid is 8 plain bytes on every target.
        let luid: [u8; 8] = unsafe { std::mem::transmute_copy(&adapter.luid) };
        format!("adapter luid={}", hex(&luid))
    } else {
        format!("adapter uuid={}", hex(&adapter.uuid))
    }
}

/// # Safety
///
/// `next` must be null or the start of a chain of stamped structures.
unsafe fn validate_onboard_chain(next: *const QarStructureHeader) -> Result<(), QarResult> {
    if next.is_null() {
        return Err(invalid("QarOnboardInit needs a chained mode extension"));
    }
    // SAFETY: forwarded to the caller.
    let mode = unsafe { (*next).type_ };
    if mode == QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_ONBOARD_CODE_EXT {
        // SAFETY: the type identifies the structure.
        let code_extension = unsafe { &*next.cast::<QarOnboardCodeExt>() };
        // SAFETY: code is a C string or NULL.
        let code = unsafe { text_or_empty(code_extension.code) };
        // SAFETY: forwarded to the caller.
        unsafe {
            validate_optional_extensions(
                code_extension.header.next.cast_const().cast(),
                OptionalExtensions::AfterCodeMode,
            )
        }?;
        return match code.as_str() {
            "" => Err(invalid("pairing code is empty")),
            "WRONG" => Err(state::error(
                QarStatusCode::QAR_STATUS_PAKE_ERROR,
                "pairing code rejected",
            )),
            "UNREACHABLE" => Err(state::error(
                QarStatusCode::QAR_STATUS_ONBOARDING_HUB_UNREACHABLE,
                "no hub answered discovery",
            )),
            _ => Ok(()),
        };
    }
    if mode == QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_ONBOARD_INVITE_EXT {
        // SAFETY: the type identifies the structure.
        let invite_extension = unsafe { &*next.cast::<QarOnboardInviteExt>() };
        // SAFETY: forwarded to the caller.
        unsafe {
            validate_optional_extensions(
                invite_extension.header.next.cast_const().cast(),
                OptionalExtensions::AfterInviteMode,
            )
        }?;
        // SAFETY: the invite is a handle from this stub or NULL.
        let invite = unsafe {
            handle_value::<InviteValue, QarOnboardingInvite>(invite_extension.invite, INVITE)
        };
        return match invite {
            Some(value) if value.expires_unix >= unix_now() => Ok(()),
            Some(_) => Err(state::error(
                QarStatusCode::QAR_STATUS_PAKE_ERROR,
                "invite expired",
            )),
            None => Err(invalid("invite is not a live handle")),
        };
    }
    if mode == QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_ONBOARD_LOCAL_APP_EXT {
        // SAFETY: the type identifies the structure.
        let local_app_extension = unsafe { &*next.cast::<QarOnboardLocalAppExt>() };
        note(format!(
            "local app approval_timeout_ms={}",
            local_app_extension.approval_timeout_ms
        ));
        // SAFETY: forwarded to the caller.
        return unsafe {
            validate_optional_extensions(
                local_app_extension.header.next.cast_const().cast(),
                OptionalExtensions::AfterLocalAppMode,
            )
        };
    }
    note(format!(
        "ERROR unknown onboarding extension type {}",
        mode.0
    ));
    Err(state::error(
        QarStatusCode::QAR_STATUS_LOGIC_ERROR,
        "unknown onboarding extension",
    ))
}

/// Checks the Vulkan extension chained into a render sender init and creates
/// the sender's images on the caller's device.
///
/// # Safety
///
/// `init.header.next` is null or a stamped extension structure.
unsafe fn create_vulkan_sender(
    init: &QarRenderSenderInit,
    sender: &RenderSenderValue,
) -> Result<crate::vulkan::VulkanSender, QarResult> {
    let extension = init.header.next.cast_const().cast::<QarStructureHeader>();
    if extension.is_null() {
        return Err(invalid(
            "QAR_GRAPHICS_API_VULKAN requires QarStreamParamsVulkan in header.next",
        ));
    }
    if let Some(failure) = header_mismatch(
        extension,
        QarStructureType::QAR_STRUCTURE_TYPE_STREAM_VULKAN_PARAMS_EXT,
        "QarStreamParamsVulkan",
    ) {
        return Err(failure);
    }
    // SAFETY: the type identifies the structure.
    let params = unsafe { &*extension.cast::<QarStreamParamsVulkan>() };
    if !params.header.next.is_null() {
        return Err(invalid("QarStreamParamsVulkan header.next must be NULL"));
    }
    let sizes: Vec<QarTextureSize> = sender.textures.iter().map(|texture| texture.size).collect();
    // SAFETY: the device handles are the caller's live device.
    unsafe { crate::vulkan::VulkanSender::create(params, &sender.views, &sizes) }
        .inspect(|_| {
            note(format!(
                "vulkan sender created images={} views={}",
                sizes.len(),
                sender.views.len()
            ));
        })
        .map_err(|message| {
            note(format!("ERROR vulkan sender: {message}"));
            state::error(QarStatusCode::QAR_STATUS_UNCLASSIFIED, message)
        })
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn is_d3d11(sender: &RenderSenderValue) -> bool {
    sender.d3d11.is_some()
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn is_d3d11(_sender: &RenderSenderValue) -> bool {
    false
}

/// Checks the chained `QarStreamParamsD3D11` and gives the sender its stand-in shared textures
/// and keyed mutexes, one per layout texture.
///
/// # Safety
///
/// `init.header.next` is null or a stamped extension structure.
#[cfg(any(target_os = "windows", target_os = "macos"))]
unsafe fn attach_d3d11_sender(
    init: &QarRenderSenderInit,
    sender: &mut RenderSenderValue,
) -> Result<(), QarResult> {
    let extension = init.header.next.cast_const().cast::<QarStructureHeader>();
    // SAFETY: forwarded to the caller.
    match unsafe { crate::d3d11::D3D11Sender::create(extension, sender.textures.len()) } {
        Ok((d3d11, report)) => {
            note(report);
            sender.d3d11 = Some(d3d11);
            Ok(())
        }
        Err(message) => {
            note(format!("ERROR d3d11 sender: {message}"));
            Err(state::error(QarStatusCode::QAR_STATUS_LOGIC_ERROR, message))
        }
    }
}

/// Where the bindings carry no D3D11 types, no D3D11 sender can be asked for through them.
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
unsafe fn attach_d3d11_sender(
    _init: &QarRenderSenderInit,
    _sender: &mut RenderSenderValue,
) -> Result<(), QarResult> {
    Err(invalid(
        "QAR_GRAPHICS_API_D3D11 is not available where the header has no D3D11",
    ))
}

fn build_sender(session: usize, init: &QarRenderSenderInit) -> RenderSenderValue {
    let requested = &init.frame_views[..init.frame_views_count.min(init.frame_views.len())];
    let fallback = [QarRenderFrameView {
        data_type: QarVideoFrameViewType::QAR_VIDEO_FRAME_VIEW_TYPE_COLOR,
        eye: QarVideoFrameViewEye::QAR_VIDEO_FRAME_VIEW_EYE_NONE,
        texture_index: 0,
    }];
    let requested = if requested.is_empty() {
        &fallback[..]
    } else {
        requested
    };

    let texture_count = requested
        .iter()
        .map(|view| view.texture_index as usize + 1)
        .max()
        .unwrap_or(1)
        .min(QAR_MAX_FRAME_TEXTURES as usize);
    let mut views_per_texture = vec![0_u32; texture_count];
    let mut views = Vec::new();
    for view in requested {
        let texture_index = (view.texture_index as usize).min(texture_count - 1);
        let column = views_per_texture[texture_index];
        views_per_texture[texture_index] += 1;
        let format = if view.data_type == QarVideoFrameViewType::QAR_VIDEO_FRAME_VIEW_TYPE_DEPTH {
            init.depth_format
        } else {
            init.color_format
        };
        views.push(QarVideoFrameView {
            start_x: column * VIEW_SIZE_PIXELS,
            start_y: 0,
            end_x: (column + 1) * VIEW_SIZE_PIXELS,
            end_y: VIEW_SIZE_PIXELS,
            texture_index: view.texture_index,
            array_layer_index: 0,
            texture_format: format,
            data_type: view.data_type,
            eye: view.eye,
        });
    }

    let textures = views_per_texture
        .iter()
        .enumerate()
        .map(|(texture_index, columns)| {
            let format = views
                .iter()
                .find(|view| view.texture_index as usize == texture_index)
                .map_or(init.color_format, |view| view.texture_format);
            let width = (*columns).max(1) * VIEW_SIZE_PIXELS;
            let pitch = width * bytes_per_pixel(format);
            StubTexture {
                size: QarTextureSize {
                    format,
                    width,
                    height: VIEW_SIZE_PIXELS,
                    array_layers: 1,
                },
                pitch,
                bytes: vec![0; (pitch * VIEW_SIZE_PIXELS) as usize],
            }
        })
        .collect();

    RenderSenderValue {
        session,
        target_peer: init.peer_id.data,
        views,
        textures,
        frames_begun: 0,
        open_frame: None,
        shown_frame: None,
        vulkan: None,
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        d3d11: None,
        begin_wait_started: None,
    }
}

/// # Safety
///
/// `peers` must be null or readable for `count` ids.
unsafe fn peer_array(peers: *const QarPeerId, count: usize) -> Vec<[u8; 16]> {
    if peers.is_null() {
        return Vec::new();
    }
    // SAFETY: forwarded to the caller.
    unsafe { std::slice::from_raw_parts(peers, count) }
        .iter()
        .map(|peer| peer.data)
        .collect()
}

/// # Safety
///
/// `handle` must come from this stub; the buffer must hold `buffer_size` bytes.
unsafe fn peer_text(
    handle: *mut QarPeerSpec,
    out_buffer: *mut c_char,
    buffer_size: usize,
    select: impl Fn(&PeerSpecValue) -> String,
) -> QarResult {
    // SAFETY: forwarded to the caller.
    let Some(spec) = (unsafe { handle_value::<PeerSpecValue, QarPeerSpec>(handle, PEER_SPEC) })
    else {
        return invalid("peer spec is not a live handle");
    };
    // SAFETY: forwarded to the caller.
    unsafe { write_c_string(&select(spec), out_buffer, buffer_size) };
    success()
}

/// # Safety
///
/// `session` and `id` must be valid; see the callers.
unsafe fn update_panel(
    session: *mut QarSession,
    id: *const QarGuiPanelId,
    change: impl FnOnce(&mut StubPanel),
) -> QarResult {
    // SAFETY: session comes from this stub.
    if unsafe { session_value(session) }.is_none() || id.is_null() {
        return invalid("session or panel id invalid");
    }
    // SAFETY: id is readable.
    let panel_id = unsafe { (*id).data };
    let mut guard = state::lock();
    let Some(panel) = guard.panels.iter_mut().find(|panel| panel.id == panel_id) else {
        drop(guard);
        return state::error(
            QarStatusCode::QAR_STATUS_GUI_PANEL_INVALID_ID,
            "no panel with this id",
        );
    };
    change(panel);
    drop(guard);
    fire_panel_updates(session as usize, panel_id);
    success()
}

/// # Safety
///
/// `handle` must come from this stub; `read` writes only to valid out-pointers.
unsafe fn with_panel(handle: *mut QarGuiPanel, read: impl FnOnce(&StubPanel)) -> QarResult {
    // SAFETY: forwarded to the caller.
    let Some(panel) = (unsafe { handle_value::<StubPanel, QarGuiPanel>(handle, GUI_PANEL) }) else {
        return invalid("GUI panel is not a live handle");
    };
    read(panel);
    success()
}

/// # Safety
///
/// `session` and `volume_id` must be valid; see the callers.
unsafe fn update_volume(
    session: *mut QarSession,
    volume_id: *const QarAppVolumeId,
    change: impl FnOnce(&mut StubVolume),
) -> QarResult {
    // SAFETY: session comes from this stub.
    if unsafe { session_value(session) }.is_none() || volume_id.is_null() {
        return invalid("session or volume id invalid");
    }
    // SAFETY: volume_id is readable.
    let volume_id = unsafe { (*volume_id).data };
    let mut guard = state::lock();
    let Some(volume) = guard
        .volumes
        .iter_mut()
        .find(|volume| volume.id == volume_id)
    else {
        drop(guard);
        return state::error(
            QarStatusCode::QAR_STATUS_APP_VOLUME_INVALID_ID,
            "no app volume with this id",
        );
    };
    change(volume);
    drop(guard);
    fire_volume_updates(session as usize, volume_id);
    success()
}

/// # Safety
///
/// `session` and `volume_id` must be valid; `read` writes only to valid
/// out-pointers.
unsafe fn read_volume(
    session: *mut QarSession,
    volume_id: *const QarAppVolumeId,
    read: impl FnOnce(&StubVolume),
) -> QarResult {
    // SAFETY: session comes from this stub.
    if unsafe { session_value(session) }.is_none() || volume_id.is_null() {
        return invalid("session or volume id invalid");
    }
    // SAFETY: volume_id is readable.
    let volume_id = unsafe { (*volume_id).data };
    let Some(volume) = state::lock()
        .volumes
        .iter()
        .find(|volume| volume.id == volume_id)
        .cloned()
    else {
        return state::error(
            QarStatusCode::QAR_STATUS_APP_VOLUME_INVALID_ID,
            "no app volume with this id",
        );
    };
    read(&volume);
    success()
}

/// # Safety
///
/// `handle` must come from this stub; `read` writes only to valid out-pointers.
unsafe fn with_volume(handle: *mut QarAppVolume, read: impl FnOnce(&StubVolume)) -> QarResult {
    // SAFETY: forwarded to the caller.
    let Some(volume) = (unsafe { handle_value::<StubVolume, QarAppVolume>(handle, APP_VOLUME) })
    else {
        return invalid("app volume is not a live handle");
    };
    read(volume);
    success()
}

/// Resets all stub state: call log, handles, subscriptions, panels, volumes.
#[unsafe(no_mangle)]
pub extern "C" fn qar_stub_reset() {
    state::reset();
}

/// Copies the call log, one entry per line, into `buffer` (truncated to fit,
/// NUL-terminated) and returns the full length in bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qar_stub_call_log(buffer: *mut c_char, size: usize) -> usize {
    let log = state::lock().calls.join("\n");
    // SAFETY: the caller provides size writable bytes.
    unsafe { write_c_string(&log, buffer, size) };
    log.len()
}

/// Makes the next call to `function` (public name without `qar_`, for example
/// `runtime_create`) fail with `code`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qar_stub_fail_next(function: *const c_char, code: i64) {
    // SAFETY: function is a C string or NULL.
    let name = unsafe { text_or_empty(function) };
    let name = name.trim_start_matches("qar_").to_string();
    state::lock().injected_failures.insert(name, code);
}

/// Makes `qar_session_get_graphics_device_id` report an adapter: a LUID (8 bytes
/// at `bytes`) when `is_luid`, else a UUID (16 bytes). A null `bytes` reports
/// none again, the default.
///
/// # Safety
///
/// `bytes` is null or points to 8 (LUID) or 16 (UUID) readable bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qar_stub_set_reported_adapter(is_luid: bool, bytes: *const u8) {
    let reported = if bytes.is_null() {
        None
    } else if is_luid {
        // SAFETY: the caller provides 8 readable bytes.
        Some(ReportedAdapter::Luid(unsafe {
            bytes.cast::<[u8; 8]>().read_unaligned()
        }))
    } else {
        // SAFETY: the caller provides 16 readable bytes.
        Some(ReportedAdapter::Uuid(unsafe {
            bytes.cast::<[u8; 16]>().read_unaligned()
        }))
    };
    state::lock().reported_adapter = reported;
}

/// When enabled (the default), subscribing to render requests immediately
/// delivers one request from the simulated visualizer peer.
#[unsafe(no_mangle)]
pub extern "C" fn qar_stub_set_auto_render_request(enabled: bool) {
    state::lock().disable_auto_render_request = !enabled;
}

/// Delivers a render request from `target_peer` (16 bytes) to every live
/// subscription; returns how many callbacks ran.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qar_stub_emit_render_request(target_peer: *const u8) -> usize {
    // SAFETY: target_peer points to 16 readable bytes or is null.
    let Some(peer) = (unsafe { read_peer(target_peer) }) else {
        return 0;
    };
    fire_render_requests(None, peer)
}

/// # Safety
///
/// `peer` must be null or point to 16 readable bytes.
unsafe fn read_peer(peer: *const u8) -> Option<[u8; 16]> {
    if peer.is_null() {
        return None;
    }
    let mut bytes = [0_u8; 16];
    // SAFETY: forwarded to the caller.
    unsafe { ptr::copy_nonoverlapping(peer, bytes.as_mut_ptr(), 16) };
    Some(bytes)
}

/// A new peer (16 bytes) named `display_name` joins: it appears in the peer list, peer
/// updates report it, and it requests a render stream. Returns how many render request
/// callbacks ran.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qar_stub_peer_join(peer: *const u8, display_name: *const c_char) -> usize {
    // SAFETY: peer points to 16 readable bytes or is null.
    let Some(peer) = (unsafe { read_peer(peer) }) else {
        return 0;
    };
    // SAFETY: display_name is a C string or NULL.
    let display_name = unsafe { text_or_empty(display_name) };
    peer_join(peer, &display_name)
}

/// The peer (16 bytes) leaves: it disappears from the peer list, a last peer update reports
/// it shutting down, and its render senders report a closed stream.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qar_stub_peer_leave(peer: *const u8) {
    // SAFETY: peer points to 16 readable bytes or is null.
    if let Some(peer) = unsafe { read_peer(peer) } {
        peer_leave(peer, QarAppState::QAR_APP_STATE_SHUTTING_DOWN);
    }
}

/// The peer (16 bytes) retires: as [`qar_stub_peer_leave`], but its last peer update reports
/// it retired, gone for good under its id.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qar_stub_peer_retire(peer: *const u8) {
    // SAFETY: peer points to 16 readable bytes or is null.
    if let Some(peer) = unsafe { read_peer(peer) } {
        peer_leave(peer, QarAppState::QAR_APP_STATE_RETIRED);
    }
}

/// Nonzero `paused`: the peer's (16 bytes) receiver pauses its stream, as a
/// headset taken off or a minimised visualizer does, so `try_begin_frame`
/// reports no frame ready for its render senders until it resumes (zero).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qar_stub_set_peer_paused(peer: *const u8, paused: u8) {
    // SAFETY: peer points to 16 readable bytes or is null.
    if let Some(peer) = unsafe { read_peer(peer) } {
        let mut guard = state::lock();
        if paused == 0 {
            guard.paused_peers.remove(&peer);
        } else {
            guard.paused_peers.insert(peer);
        }
    }
}

/// Nonzero `milliseconds`: the peer (16 bytes) is gone while the peer list still shows it
/// running, so every async render sender creation for it started from now on creates nothing
/// and reports `QAR_STATUS_TIMEOUT` after that long, or earlier when its token is cancelled
/// or times out. Zero: creations for the peer answer again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qar_stub_set_peer_unanswering(peer: *const u8, milliseconds: u64) {
    // SAFETY: peer points to 16 readable bytes or is null.
    if let Some(peer) = unsafe { read_peer(peer) } {
        let mut guard = state::lock();
        if milliseconds == 0 {
            guard.unanswering_peers.remove(&peer);
        } else {
            guard
                .unanswering_peers
                .insert(peer, Duration::from_millis(milliseconds));
        }
    }
}

/// The peer (16 bytes) is listed under `version_id` (a C string) from now on, as an app that
/// restarted under the same peer id, and a peer update reports it running.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qar_stub_set_peer_version_id(peer: *const u8, version_id: *const c_char) {
    // SAFETY: peer points to 16 readable bytes or is null.
    let Some(peer) = (unsafe { read_peer(peer) }) else {
        return;
    };
    // SAFETY: version_id is a C string or NULL.
    let version_id = unsafe { text_or_empty(version_id) };
    let display_name = {
        let mut guard = state::lock();
        guard.peer_version_ids.insert(peer, version_id.clone());
        remote_display_name(&guard, peer)
    };
    note(format!(
        "peer {} listed under version id {version_id}",
        short_hex(peer)
    ));
    fire_peer_updates(
        None,
        &remote_peer_spec(peer, &display_name, QarAppState::QAR_APP_STATE_RUNNING),
    );
}

/// How long the begin-frame wait that `try_begin_frame` starts on a sender takes to complete
/// (0 by default: within the call). Until then `try_begin_frame` reports no frame ready, and
/// destroying the sender notes that a begin-frame wait was pending.
#[unsafe(no_mangle)]
pub extern "C" fn qar_stub_set_try_begin_frame_delay_ms(milliseconds: u64) {
    state::lock().try_begin_frame_delay = Duration::from_millis(milliseconds);
}

/// Delay before an async render sender creation creates the sender and calls
/// back (0 by default); cancellation and timeout end it early.
#[unsafe(no_mangle)]
pub extern "C" fn qar_stub_set_render_sender_create_delay_ms(milliseconds: u64) {
    state::lock().render_sender_create_delay = Duration::from_millis(milliseconds);
}

/// How long an async render sender creation started from now on takes to
/// call back once its token is cancelled (0 by default), as a runtime that
/// ends a cancelled operation on a thread of its own.
#[unsafe(no_mangle)]
pub extern "C" fn qar_stub_set_render_sender_create_cancel_delay_ms(milliseconds: u64) {
    state::lock().render_sender_create_cancel_delay = Duration::from_millis(milliseconds);
}

/// Nonzero: async render sender creations started from now on never call
/// back and create nothing, cancelled or not (off by default).
#[unsafe(no_mangle)]
pub extern "C" fn qar_stub_set_render_sender_create_never_calls_back(never: u8) {
    set_create_completion(never, CreateCompletion::NeverCallsBack);
}

/// Nonzero: async render sender creations started from now on ignore their
/// cancel token and deliver a sender after the delay (off by default).
#[unsafe(no_mangle)]
pub extern "C" fn qar_stub_set_render_sender_create_ignores_cancel(ignores: u8) {
    set_create_completion(ignores, CreateCompletion::IgnoresCancel);
}

fn set_create_completion(enabled: u8, completion: CreateCompletion) {
    state::lock().render_sender_create_completion = if enabled != 0 {
        completion
    } else {
        CreateCompletion::CallsBack
    };
}

/// Frames shown to the peer (16 bytes); 0 for NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qar_stub_frames_shown(peer: *const u8) -> u64 {
    // SAFETY: peer points to 16 readable bytes or is null.
    let Some(peer) = (unsafe { read_peer(peer) }) else {
        return 0;
    };
    let guard = state::lock();
    guard.frames_shown_by_peer.get(&peer).copied().unwrap_or(0)
}

/// Number of live handles the stub has handed out and not yet seen destroyed.
#[unsafe(no_mangle)]
pub extern "C" fn qar_stub_live_handle_count() -> usize {
    state::lock().live_handles.len()
}
