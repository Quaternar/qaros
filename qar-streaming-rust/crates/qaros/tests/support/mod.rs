//! Fixtures shared by the stub test binaries: the stub's control exports,
//! a serialized fixture per test, and session helpers.

#![allow(
    dead_code,
    unused_imports,
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic
)]

use std::ffi::c_char;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use qaros::{
    AppState, AppVolumeConfig, BlockingOptions, Error, Eye, FrameViewSpec, GraphicsAdapter,
    GuiPanelConfig, GuiPanelState, HubAddress, Library, LibraryConfig, OnboardMethod, OwnedFrame,
    PeerId, PeerPresentation, Pose, RenderSenderConfig, RuntimeConfig, ShowParams, StatusCode,
    Vec3, ViewKind,
};
use qaros_stub::frame_stamp;

pub(crate) const VISUALIZER_PEER: PeerId = PeerId([0x56; 16]);

/// The stub's call log entry of the runtime release these bindings use
/// (`qaros_sys::RUNTIME_RELEASE_SYMBOL`).
pub(crate) fn runtime_release_call() -> &'static str {
    qaros::qaros_sys::RUNTIME_RELEASE_SYMBOL.trim_start_matches("qar_impl_")
}

/// Every runtime release in the call log, by either name a header declares.
pub(crate) fn runtime_releases(calls: &[String]) -> usize {
    calls
        .iter()
        .filter(|call| *call == "runtime_destroy" || *call == "runtime_handle_destroy")
        .count()
}

pub(crate) static SERIAL: Mutex<()> = Mutex::new(());

pub(crate) fn stub_path() -> PathBuf {
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
    .unwrap_or_else(|| panic!("{file_name} not found; build it with `cargo build -p qaros-stub`"))
}

/// The stub's `qar_stub_*` control exports, opened from the same file as the
/// library under test so they share its state.
pub(crate) struct StubControl {
    pub(crate) library: libloading::Library,
}

impl StubControl {
    pub(crate) fn open() -> Self {
        // SAFETY: the stub has no initialization side effects.
        let library = unsafe { libloading::Library::new(stub_path()) }.expect("open qaros-stub");
        Self { library }
    }

    pub(crate) fn symbol<T: Copy>(&self, name: &str) -> T {
        // SAFETY: the caller names a stub export with this signature.
        unsafe { *self.library.get::<T>(name.as_bytes()).expect(name) }
    }

    pub(crate) fn reset(&self) {
        let reset: unsafe extern "C" fn() = self.symbol("qar_stub_reset");
        // SAFETY: no arguments.
        unsafe { reset() };
    }

    pub(crate) fn calls(&self) -> Vec<String> {
        let call_log: unsafe extern "C" fn(*mut c_char, usize) -> usize =
            self.symbol("qar_stub_call_log");
        // SAFETY: a null buffer only queries the length.
        let length = unsafe { call_log(std::ptr::null_mut(), 0) };
        let mut buffer = vec![0 as c_char; length + 1];
        // SAFETY: the buffer holds length + 1 bytes.
        unsafe { call_log(buffer.as_mut_ptr(), buffer.len()) };
        let bytes: Vec<u8> = buffer
            .iter()
            .take_while(|&&character| character != 0)
            .map(|&character| character.to_ne_bytes()[0])
            .collect();
        String::from_utf8(bytes)
            .expect("UTF-8 log")
            .lines()
            .map(str::to_string)
            .collect()
    }

    pub(crate) fn fail_next(&self, function: &str, code: i64) {
        let fail_next: unsafe extern "C" fn(*const c_char, i64) = self.symbol("qar_stub_fail_next");
        let name = std::ffi::CString::new(function).expect("function name");
        // SAFETY: name is NUL-terminated.
        unsafe { fail_next(name.as_ptr(), code) };
    }

    pub(crate) fn set_reported_adapter(&self, adapter: Option<GraphicsAdapter>) {
        let set_reported: unsafe extern "C" fn(bool, *const u8) =
            self.symbol("qar_stub_set_reported_adapter");
        // SAFETY: null, or 8 (LUID) or 16 (UUID) readable bytes.
        unsafe {
            match &adapter {
                None => set_reported(false, std::ptr::null()),
                Some(GraphicsAdapter::Luid(luid)) => set_reported(true, luid.as_ptr()),
                Some(GraphicsAdapter::Uuid(uuid)) => set_reported(false, uuid.as_ptr()),
            }
        }
    }

    pub(crate) fn set_auto_render_request(&self, enabled: bool) {
        let set_auto: unsafe extern "C" fn(bool) = self.symbol("qar_stub_set_auto_render_request");
        // SAFETY: plain value argument.
        unsafe { set_auto(enabled) };
    }

    pub(crate) fn emit_render_request(&self, target_peer: PeerId) -> usize {
        let emit: unsafe extern "C" fn(*const u8) -> usize =
            self.symbol("qar_stub_emit_render_request");
        // SAFETY: points to 16 readable bytes.
        unsafe { emit(target_peer.0.as_ptr()) }
    }

    pub(crate) fn live_handle_count(&self) -> usize {
        let count: unsafe extern "C" fn() -> usize = self.symbol("qar_stub_live_handle_count");
        // SAFETY: no arguments.
        unsafe { count() }
    }

    pub(crate) fn peer_join(&self, peer: PeerId, display_name: &str) -> usize {
        let join: unsafe extern "C" fn(*const u8, *const c_char) -> usize =
            self.symbol("qar_stub_peer_join");
        let name = std::ffi::CString::new(display_name).expect("display name");
        // SAFETY: 16 readable bytes and a NUL-terminated name.
        unsafe { join(peer.0.as_ptr(), name.as_ptr()) }
    }

    pub(crate) fn peer_leave(&self, peer: PeerId) {
        let leave: unsafe extern "C" fn(*const u8) = self.symbol("qar_stub_peer_leave");
        // SAFETY: 16 readable bytes.
        unsafe { leave(peer.0.as_ptr()) };
    }

    pub(crate) fn peer_retire(&self, peer: PeerId) {
        let retire: unsafe extern "C" fn(*const u8) = self.symbol("qar_stub_peer_retire");
        // SAFETY: 16 readable bytes.
        unsafe { retire(peer.0.as_ptr()) };
    }

    pub(crate) fn set_peer_paused(&self, peer: PeerId, paused: bool) {
        let set_paused: unsafe extern "C" fn(*const u8, u8) =
            self.symbol("qar_stub_set_peer_paused");
        // SAFETY: 16 readable bytes and a plain flag.
        unsafe { set_paused(peer.0.as_ptr(), u8::from(paused)) };
    }

    pub(crate) fn set_peer_unanswering(&self, peer: PeerId, milliseconds: u64) {
        let set_unanswering: unsafe extern "C" fn(*const u8, u64) =
            self.symbol("qar_stub_set_peer_unanswering");
        // SAFETY: 16 readable bytes and a plain value.
        unsafe { set_unanswering(peer.0.as_ptr(), milliseconds) };
    }

    pub(crate) fn set_try_begin_frame_delay(&self, milliseconds: u64) {
        let set_delay: extern "C" fn(u64) = self.symbol("qar_stub_set_try_begin_frame_delay_ms");
        set_delay(milliseconds);
    }

    pub(crate) fn set_render_sender_create_delay(&self, milliseconds: u64) {
        let set_delay: unsafe extern "C" fn(u64) =
            self.symbol("qar_stub_set_render_sender_create_delay_ms");
        // SAFETY: plain value argument.
        unsafe { set_delay(milliseconds) };
    }

    pub(crate) fn set_render_sender_create_cancel_delay(&self, milliseconds: u64) {
        let set_delay: unsafe extern "C" fn(u64) =
            self.symbol("qar_stub_set_render_sender_create_cancel_delay_ms");
        // SAFETY: plain value argument.
        unsafe { set_delay(milliseconds) };
    }

    pub(crate) fn set_render_sender_create_never_calls_back(&self, never: bool) {
        let set_never: unsafe extern "C" fn(u8) =
            self.symbol("qar_stub_set_render_sender_create_never_calls_back");
        // SAFETY: plain value argument.
        unsafe { set_never(u8::from(never)) };
    }

    pub(crate) fn set_render_sender_create_ignores_cancel(&self, ignores: bool) {
        let set_ignores: unsafe extern "C" fn(u8) =
            self.symbol("qar_stub_set_render_sender_create_ignores_cancel");
        // SAFETY: plain value argument.
        unsafe { set_ignores(u8::from(ignores)) };
    }

    pub(crate) fn frames_shown(&self, peer: PeerId) -> u64 {
        let shown: unsafe extern "C" fn(*const u8) -> u64 = self.symbol("qar_stub_frames_shown");
        // SAFETY: 16 readable bytes.
        unsafe { shown(peer.0.as_ptr()) }
    }

    pub(crate) fn entries_starting_with(&self, prefix: &str) -> Vec<String> {
        self.calls()
            .into_iter()
            .filter(|entry| entry.starts_with(prefix))
            .collect()
    }
}

pub(crate) struct Fixture {
    pub(crate) control: StubControl,
    pub(crate) storage: PathBuf,
    _serial: MutexGuard<'static, ()>,
}

impl Fixture {
    pub(crate) fn new() -> Self {
        let serial = SERIAL.lock().unwrap_or_else(PoisonError::into_inner);
        let control = StubControl::open();
        control.reset();
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let storage = std::env::temp_dir().join(format!("qaros-stub-test-{unique}"));
        Self {
            control,
            storage,
            _serial: serial,
        }
    }

    pub(crate) fn runtime_config(&self) -> RuntimeConfig {
        RuntimeConfig {
            runtime_binaries_directory: None,
            storage_directory: Some(self.storage.clone()),
        }
    }

    pub(crate) fn errors_in_log(&self) -> Vec<String> {
        self.control
            .calls()
            .into_iter()
            .filter(|entry| entry.starts_with("ERROR"))
            .collect()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.storage);
    }
}

pub(crate) fn load_stub() -> Library {
    // SAFETY: the stub matches the C header by construction.
    unsafe { Library::load(stub_path(), &quiet()) }.expect("load qaros-stub")
}

pub(crate) fn quiet() -> LibraryConfig {
    LibraryConfig {
        console_logging: false,
        ..LibraryConfig::default()
    }
}

pub(crate) fn code(pairing_code: &str) -> OnboardMethod<'_> {
    OnboardMethod::Code {
        code: pairing_code,
        hub: None,
    }
}

pub(crate) fn position_of(calls: &[String], entry: &str) -> usize {
    calls
        .iter()
        .position(|call| call == entry)
        .unwrap_or_else(|| panic!("{entry} not in call log: {calls:?}"))
}

pub(crate) fn last_position_of(calls: &[String], entry: &str) -> usize {
    calls
        .iter()
        .rposition(|call| call == entry)
        .unwrap_or_else(|| panic!("{entry} not in call log: {calls:?}"))
}

pub(crate) fn stereo_color_and_depth(target_peer: PeerId) -> RenderSenderConfig {
    let mut config = RenderSenderConfig::new(target_peer);
    config.views = vec![
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
    ];
    config
}

pub(crate) fn onboarded_session(
    fixture: &Fixture,
    library: &Library,
) -> (qaros::Runtime, qaros::Session) {
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    let session = runtime
        .onboard(
            &code("ABCD-EFGH"),
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect("onboard")
        .session;
    (runtime, session)
}
