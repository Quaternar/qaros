//! Global in-memory state of the stub, guarded by one mutex. Callbacks are
//! never invoked while the mutex is held.

use std::collections::{HashMap, HashSet};
use std::ffi::{c_char, c_void};
use std::sync::{LazyLock, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

use qaros_sys::{QarResult, QarStatusCode};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SubscriptionKind {
    RenderRequests,
    PeerUpdates,
    GuiPanels,
    AppVolumes,
    SessionLifecycle,
}

/// A registered C callback. Pointers are stored as addresses so the state is
/// `Send`; the stub calls them only while the subscription is live.
#[derive(Clone, Copy, Debug)]
pub struct StubSubscription {
    pub session: usize,
    pub kind: SubscriptionKind,
    pub callback: usize,
    pub user_state: usize,
    pub token: usize,
    /// For per-object subscriptions (one GUI panel): the object's id.
    pub filter: Option<[u8; 16]>,
}

#[derive(Clone, Debug)]
pub struct StubPanel {
    pub id: [u8; 16],
    pub display_name: String,
    pub pose: qaros_sys::QarPose,
    pub size: qaros_sys::QarGuiPanelSize,
    pub state: qaros_sys::QarGuiPanelState,
    pub content_uri: String,
    pub visible_to: Vec<[u8; 16]>,
}

#[derive(Clone, Debug)]
pub struct StubVolume {
    pub id: [u8; 16],
    pub display_name: String,
    pub pose: qaros_sys::QarPose,
    pub size: qaros_sys::QarAppVolumeSize,
    pub app_pose: qaros_sys::QarPose,
    pub app_scale: f32,
    pub closed: bool,
}

#[derive(Default)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "independent test switches, not a state machine"
)]
pub struct StubState {
    pub calls: Vec<String>,
    pub messages: HashMap<u32, String>,
    pub next_error_handle: u32,
    pub library_initialized: bool,
    /// Address of each live handle and its kind, to detect double destroys.
    pub live_handles: HashMap<usize, &'static str>,
    pub cancelled_tokens: HashSet<usize>,
    /// When each token made by `qar_cancel_token_create_with_timeout` times out.
    pub token_deadlines: HashMap<usize, Instant>,
    pub subscriptions: Vec<StubSubscription>,
    pub injected_failures: HashMap<String, i64>,
    pub disable_auto_render_request: bool,
    pub panels: Vec<StubPanel>,
    pub volumes: Vec<StubVolume>,
    pub next_identity: u64,
    /// Peers that joined through `qar_stub_peer_join`, in join order.
    pub remote_peers: Vec<RemotePeer>,
    /// Peers that left; their render senders report a closed stream.
    pub departed_peers: HashSet<[u8; 16]>,
    /// Peers whose receivers paused their streams: `try_begin_frame` reports
    /// no frame ready for their render senders, and the senders stay valid.
    pub paused_peers: HashSet<[u8; 16]>,
    /// Peers whose receivers are gone while the peer list still shows them
    /// running: an async render sender creation for one of them creates
    /// nothing and reports a timeout after this long, or earlier when its
    /// token is cancelled or times out.
    pub unanswering_peers: HashMap<[u8; 16], Duration>,
    /// Version ids set through `qar_stub_set_peer_version_id`, as a restarted app is listed;
    /// every other peer is listed with `qaros-stub`.
    pub peer_version_ids: HashMap<[u8; 16], String>,
    /// How long the begin-frame wait that `try_begin_frame` starts takes to
    /// complete; zero completes it within the call.
    pub try_begin_frame_delay: Duration,
    /// How long an async render sender creation waits before it creates the
    /// sender; cancellation and timeout end the wait early.
    pub render_sender_create_delay: Duration,
    /// How long a cancelled async render sender creation takes to call back
    /// once its token is cancelled.
    pub render_sender_create_cancel_delay: Duration,
    pub render_sender_create_completion: CreateCompletion,
    /// How long an async onboard or rejoin waits before it creates its session.
    pub onboard_async_delay: Duration,
    /// An async onboard or rejoin ignores its cancel token (not the runtime's destroy).
    pub onboard_async_ignores_cancel: bool,
    /// Async calls whose callback has not returned, per owner address (a runtime, session or
    /// render sender).
    pub pending_calls: HashMap<usize, Vec<PendingStubCall>>,
    /// Owners whose destroy is waiting for their async calls, which end at once.
    pub destroying_owners: HashSet<usize>,
    pub next_call_id: u64,
    /// How long an async begin-frame, invite request or target-app call waits before it
    /// completes.
    pub async_call_delay: Duration,
    /// The stub's hub joined a primary (`qar_runtime_join_hub_as_member`): only then does it
    /// redeem remote-app grants.
    pub member_of_primary: bool,
    /// Target apps the stub's hub started, by peer id.
    pub started_target_apps: HashMap<[u8; 16], StartedTargetApp>,
    /// Every successful `show_frame`, across all senders.
    pub frames_shown: u64,
    pub frames_shown_by_peer: HashMap<[u8; 16], u64>,
    /// Joins and leaves keyed to `frames_shown`, from `QAROS_STUB_SCENARIO`.
    pub scenario: Vec<ScenarioStep>,
    /// What `qar_session_get_graphics_device_id` reports, set through
    /// `qar_stub_set_reported_adapter`; `None` reports no adapter.
    pub reported_adapter: Option<ReportedAdapter>,
}

/// An async call in flight: its id and, while its result callback runs, that thread.
#[derive(Clone, Copy, Debug)]
pub struct PendingStubCall {
    pub id: u64,
    pub callback_thread: Option<std::thread::ThreadId>,
}

/// A target app the stub's hub started: its device family and host.
#[derive(Clone, Debug)]
pub struct StartedTargetApp {
    pub family: &'static str,
    pub host: String,
}

/// How an async render sender creation ends.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CreateCompletion {
    /// Calls back once: with the sender, or with an error when cancelled or
    /// timed out first.
    #[default]
    CallsBack,
    /// Never calls back and creates nothing, cancelled or not.
    NeverCallsBack,
    /// Ignores its cancel token and delivers a sender after the delay, as a
    /// creation that completes while being cancelled.
    IgnoresCancel,
}

#[derive(Clone, Copy, Debug)]
pub enum ReportedAdapter {
    Luid([u8; 8]),
    Uuid([u8; 16]),
}

#[derive(Clone, Debug)]
pub struct RemotePeer {
    pub id: [u8; 16],
    pub display_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScenarioAction {
    Join { peer: [u8; 16], name: String },
    Leave { peer: [u8; 16] },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScenarioStep {
    pub after_frames_shown: u64,
    pub action: ScenarioAction,
}

static STATE: LazyLock<Mutex<StubState>> = LazyLock::new(|| Mutex::new(StubState::default()));

pub fn lock() -> MutexGuard<'static, StubState> {
    STATE.lock().unwrap_or_else(PoisonError::into_inner)
}

pub fn reset() {
    *lock() = StubState::default();
}

pub fn record_call(function: &str) {
    lock()
        .calls
        .push(function.trim_start_matches("qar_impl_").to_string());
}

pub fn note(event: String) {
    lock().calls.push(event);
}

pub fn take_injected_failure(function: &str) -> Option<i64> {
    lock()
        .injected_failures
        .remove(function.trim_start_matches("qar_impl_"))
}

pub fn success() -> QarResult {
    QarResult {
        code: QarStatusCode::QAR_STATUS_SUCCESS,
        error_handle: 0,
    }
}

pub fn error(code: QarStatusCode, message: impl Into<String>) -> QarResult {
    let mut state = lock();
    state.next_error_handle += 1;
    let handle = state.next_error_handle;
    state.messages.insert(handle, message.into());
    QarResult {
        code,
        error_handle: handle,
    }
}

/// Moves a value to the heap and registers the address as a live handle.
pub fn new_handle<Value, Handle>(value: Value, kind: &'static str) -> *mut Handle {
    let pointer = Box::into_raw(Box::new(value));
    lock().live_handles.insert(pointer as usize, kind);
    pointer.cast()
}

/// Frees a handle created by [`new_handle`]; logs instead of freeing twice.
///
/// # Safety
///
/// `handle` must be null or come from `new_handle::<Value, _>`.
pub unsafe fn destroy_handle<Value, Handle>(handle: *mut Handle, kind: &'static str) {
    if handle.is_null() {
        return;
    }
    let registered = lock().live_handles.remove(&(handle as usize));
    match registered {
        Some(registered_kind) if registered_kind == kind => {
            // SAFETY: registered as a live handle of this kind by new_handle.
            drop(unsafe { Box::from_raw(handle.cast::<Value>()) });
        }
        Some(registered_kind) => {
            lock().live_handles.insert(handle as usize, registered_kind);
            note(format!(
                "ERROR destroy of {registered_kind} handle as {kind}"
            ));
        }
        None => note(format!("ERROR double or foreign destroy of {kind} handle")),
    }
}

/// Borrows a live handle's value, or `None` if the handle is not live.
///
/// # Safety
///
/// `handle` must be null or come from `new_handle::<Value, _>`, and no other
/// reference to the value may be active.
pub unsafe fn handle_value<'value, Value, Handle>(
    handle: *const Handle,
    kind: &'static str,
) -> Option<&'value mut Value> {
    if handle.is_null() {
        return None;
    }
    let live = lock().live_handles.get(&(handle as usize)) == Some(&kind);
    if !live {
        note(format!("ERROR use of a dead or foreign {kind} handle"));
        return None;
    }
    // SAFETY: live handle of this kind, per the caller's contract.
    Some(unsafe { &mut *handle.cast_mut().cast::<Value>() })
}

/// Writes `text` NUL-terminated into a C buffer, truncating to fit.
///
/// # Safety
///
/// `buffer` must be null or writable for `size` bytes.
pub unsafe fn write_c_string(text: &str, buffer: *mut c_char, size: usize) {
    if buffer.is_null() || size == 0 {
        return;
    }
    let bytes = text.as_bytes();
    let length = bytes.len().min(size - 1);
    // SAFETY: the caller guarantees size writable bytes; length < size.
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr().cast::<c_char>(), buffer, length);
        *buffer.add(length) = 0;
    }
}

pub fn callback_address<Callback>(callback: Option<Callback>) -> usize
where
    Callback: Copy,
{
    callback.map_or(0, |function| {
        // SAFETY: Callback is a function pointer type of pointer size.
        unsafe { std::mem::transmute_copy::<Callback, usize>(&function) }
    })
}

pub fn user_state_address(user_state: *mut c_void) -> usize {
    user_state as usize
}
