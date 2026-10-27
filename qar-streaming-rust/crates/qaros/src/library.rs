use std::cell::RefCell;
use std::mem::ManuallyDrop;
use std::path::{Path, PathBuf};
use std::ptr::NonNull;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use qaros_sys::{QarApi, QarCancelToken, defaults};

use crate::cancel::CancelToken;
use crate::error::{Error, Result, check};
use crate::invite::OnboardingInvite;
use crate::runtime::{Runtime, RuntimeConfig};
use crate::strings;
use crate::types::LogSeverity;

static LIBRARY_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Options for `qar_library_init`; the default matches
/// `qar_library_init_default()`.
#[derive(Clone, Debug)]
pub struct LibraryConfig {
    pub console_logging: bool,
    pub log_directory: Option<PathBuf>,
    pub log_severity: LogSeverity,
}

impl Default for LibraryConfig {
    fn default() -> Self {
        Self {
            console_logging: true,
            log_directory: None,
            log_severity: LogSeverity::Info,
        }
    }
}

/// What the loaded `qar-streaming-c` reports about itself
/// (`qar_runtime_info`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeInfo {
    /// Product version, e.g. `0.9.0` or `0.9.0-rc.2`.
    pub version: String,
    /// `release` or `demo`.
    pub flavor: String,
    /// C API generation, `v0`.
    pub c_api: String,
}

/// The loaded and initialized C API library.
///
/// Process-wide: at most one may be alive at a time. Cloning shares the same
/// library. Runtimes, sessions and every other handle keep it alive, so it is
/// destroyed (`qar_library_destroy`) only after them, whatever order the Rust
/// values are dropped in.
///
/// The DLL is never unloaded: it stays mapped for the rest of the process.
/// Freeing it after `qar_library_destroy` runs its detach
/// code and unmaps code that its threads may still reach, which has crashed
/// processes at exit. After a drop whose `qar_library_destroy`
/// succeeded, a second [`Library::load`] re-initializes the mapped library.
/// After one whose `qar_library_destroy` failed, the process slot stays
/// reserved and every later load returns [`Error::LibraryAlreadyLoaded`].
///
/// Not `Send`: the SDK is designed for single-threaded use per runtime.
#[derive(Clone, Debug)]
pub struct Library {
    pub(crate) inner: Rc<LibraryInner>,
}

#[derive(Debug)]
pub(crate) struct LibraryInner {
    /// Never dropped: the table owns the opened DLL, which stays mapped for
    /// the rest of the process (see [`Library`]).
    pub(crate) api: ManuallyDrop<QarApi>,
    /// Cancel tokens of operations that may still call back (a cancelled
    /// sender creation whose callback never came): destroyed right before the
    /// library, once no session that could run the operation is left.
    pub(crate) tokens_destroyed_at_teardown: RefCell<Vec<NonNull<QarCancelToken>>>,
}

impl Library {
    /// Finds `qar-streaming-c` with the default lookup every QAROS binding follows and runs
    /// `qar_library_init`. This is the normal way to load the runtime: a distributed application
    /// runs against the QAROS installed on the PC and ships no copy of the runtime.
    ///
    /// The first match wins: a runtime already loaded in the process; one beside the executable
    /// (or library) this binding is compiled into; on Windows, when `QAR_INSTANCE` names a QAROS
    /// instance, that instance's runtime folder (`default` is the installed QAROS, any other name
    /// the `RuntimeFolder` its hub records under `HKCU\SOFTWARE\Quaternar\QAROS\Instances`); the
    /// installed QAROS (`<InstallFolder>\bin` from `HKLM\SOFTWARE\Quaternar\QAROS`); each
    /// folder on `PATH`. A set `QAR_INSTANCE` naming an invalid or unknown instance, and an
    /// instance or installed QAROS whose runtime is missing or fails to load, is an error, never
    /// a reason to load another runtime. Nothing found is
    /// [`Error::Load`] with [`qaros_sys::LoadError::NotFound`], naming every place looked at.
    /// See [`QarApi::load_default`] for the details; [`Library::library_path`] tells which file
    /// was loaded.
    ///
    /// # Safety
    ///
    /// The runtime found must be a `qar-streaming-c` build matching these bindings; see
    /// [`QarApi::load`].
    pub unsafe fn load_default(config: &LibraryConfig) -> Result<Self> {
        // SAFETY: forwarded to the caller.
        Self::load_with(config, || unsafe { QarApi::load_default() })
    }

    /// Loads `qar-streaming-c` from the library file at `path` and runs `qar_library_init`.
    /// Only that file is tried. Explicit paths are for development builds of QAROS;
    /// a distributed application uses [`Library::load_default`], and `QAR_INSTANCE` points a run
    /// at another QAROS instance's runtime without code.
    ///
    /// # Safety
    ///
    /// `path` must name a `qar-streaming-c` build matching these bindings; see
    /// [`QarApi::load`].
    pub unsafe fn load(path: impl AsRef<Path>, config: &LibraryConfig) -> Result<Self> {
        // SAFETY: forwarded to the caller.
        Self::load_with(config, || unsafe { QarApi::load(path.as_ref()) })
    }

    fn load_with(
        config: &LibraryConfig,
        load_table: impl FnOnce() -> std::result::Result<QarApi, qaros_sys::LoadError>,
    ) -> Result<Self> {
        Self::reserve_process_slot()?;
        match load_table() {
            Ok(api) => Self::initialize(api, config),
            Err(load_error) => {
                LIBRARY_INITIALIZED.store(false, Ordering::Release);
                Err(Error::Load(load_error))
            }
        }
    }

    /// Uses the import-library symbols (feature `link`) and runs
    /// `qar_library_init`.
    #[cfg(feature = "link")]
    pub fn linked(config: &LibraryConfig) -> Result<Self> {
        Self::reserve_process_slot()?;
        Self::initialize(QarApi::linked(), config)
    }

    fn reserve_process_slot() -> Result<()> {
        if LIBRARY_INITIALIZED.swap(true, Ordering::AcqRel) {
            Err(Error::LibraryAlreadyLoaded)
        } else {
            Ok(())
        }
    }

    fn initialize(api: QarApi, config: &LibraryConfig) -> Result<Self> {
        let initialized = Self::call_library_init(&api, config);
        if let Err(error) = initialized {
            LIBRARY_INITIALIZED.store(false, Ordering::Release);
            return Err(error);
        }
        Ok(Self {
            inner: Rc::new(LibraryInner {
                api: ManuallyDrop::new(api),
                tokens_destroyed_at_teardown: RefCell::new(Vec::new()),
            }),
        })
    }

    fn call_library_init(api: &QarApi, config: &LibraryConfig) -> Result<()> {
        let log_directory = config
            .log_directory
            .as_deref()
            .map(|directory| strings::path_to_c_string(directory, "log directory"))
            .transpose()?;
        let mut init = defaults::qar_library_init_default();
        init.enable_console_logging = config.console_logging;
        init.log_folder_path = strings::pointer_or_null(log_directory.as_ref());
        init.log_severity = config.log_severity.to_raw();
        // SAFETY: init is stamped by its default and its strings outlive the call.
        let result = unsafe { (api.qar_impl_library_init)(&raw const init) };
        check(api, "qar_library_init", result)
    }

    /// `qar_runtime_create`.
    pub fn create_runtime(&self, config: &RuntimeConfig) -> Result<Runtime> {
        Runtime::create(&self.inner, config)
    }

    /// Creates a cancellation token, optionally cancelling itself after
    /// `timeout`.
    pub fn create_cancel_token(&self, timeout: Option<Duration>) -> Result<CancelToken> {
        CancelToken::create(&self.inner, timeout)
    }

    /// Parses an invite serialized by [`OnboardingInvite::serialize`].
    pub fn deserialize_invite(&self, wire_bytes: &[u8]) -> Result<OnboardingInvite> {
        OnboardingInvite::deserialize(&self.inner, wire_bytes)
    }

    /// Version, flavor and C API generation of the loaded runtime
    /// (`qar_runtime_info`), for diagnostics.
    pub fn runtime_info(&self) -> Result<RuntimeInfo> {
        let api = &self.inner.api;
        let mut raw_info = qaros_sys::QarRuntimeInfo {
            version: std::ptr::null(),
            flavor: std::ptr::null(),
            c_api: std::ptr::null(),
        };
        // SAFETY: raw_info is a valid out-pointer for the whole call.
        let result = unsafe { (api.qar_impl_runtime_info)(&raw mut raw_info) };
        check(api, "qar_runtime_info", result)?;
        // SAFETY: the runtime fills null or static NUL-terminated strings
        // that stay valid while the library is loaded, which `self` keeps.
        unsafe {
            Ok(RuntimeInfo {
                version: strings::from_returned_pointer(
                    raw_info.version,
                    "qar_runtime_info",
                    "a version",
                )?,
                flavor: strings::from_returned_pointer(
                    raw_info.flavor,
                    "qar_runtime_info",
                    "a flavor",
                )?,
                c_api: strings::from_returned_pointer(
                    raw_info.c_api,
                    "qar_runtime_info",
                    "a C API generation",
                )?,
            })
        }
    }

    /// The library's UUID text form, for persisting ids.
    pub fn uuid_to_string(&self, bytes: &[u8; 16]) -> Result<String> {
        let api = &self.inner.api;
        strings::read_string(|buffer, size| {
            // SAFETY: bytes holds 16 bytes; the buffer is writable for size bytes.
            let result = unsafe { (api.qar_impl_uuid_to_string)(bytes.as_ptr(), buffer, size) };
            check(api, "qar_uuid_to_string", result)
        })
    }

    /// Parses the library's UUID text form.
    pub fn uuid_from_string(&self, text: &str) -> Result<[u8; 16]> {
        let api = &self.inner.api;
        let text = strings::to_c_string(text.trim(), "UUID text")?;
        let mut bytes = [0_u8; 16];
        // SAFETY: text is NUL-terminated; bytes is writable for 16 bytes.
        let result = unsafe { (api.qar_impl_uuid_from_string)(text.as_ptr(), bytes.as_mut_ptr()) };
        check(api, "qar_uuid_from_string", result)?;
        Ok(bytes)
    }

    /// The file `qar-streaming-c` was loaded from, for diagnostics. `None` for the
    /// import-linked library or when the operating system does not name the module.
    pub fn library_path(&self) -> Option<&Path> {
        self.inner.api.library_path()
    }

    /// The step of the default lookup that found the runtime, for diagnostics; `None` after
    /// [`Library::load`] or for the import-linked library.
    pub fn lookup_step(&self) -> Option<qaros_sys::LookupStep> {
        self.inner.api.lookup_step()
    }

    /// The raw function table, for the parts this crate leaves to `qaros-sys`: hands, gestures,
    /// world anchors, video senders, target apps, the peer presence, role, category and device
    /// identity, and the other areas the README's coverage table lists as raw only. Call them
    /// with the `as_raw()` handle of a safe value ([`crate::Session::as_raw`] and the others).
    /// Valid while this `Library` (or any handle derived from it) is alive.
    pub fn raw_api(&self) -> &QarApi {
        &self.inner.api
    }
}

impl Drop for LibraryInner {
    fn drop(&mut self) {
        for token in self.tokens_destroyed_at_teardown.get_mut().drain(..) {
            // SAFETY: a live token handed over by its last owner; every
            // session is gone, so no operation still uses it.
            unsafe { (self.api.qar_impl_cancel_token_handle_destroy)(token.as_ptr()) };
        }
        // SAFETY: every runtime, session and handle holds an Rc to this
        // value, so none is alive any more.
        let result = unsafe { (self.api.qar_impl_library_destroy)() };
        // The table in `api` is left undropped, so the DLL is never unloaded:
        // freeing it after `qar_library_destroy` crashes the process at exit.
        match check(&self.api, "qar_library_destroy", result) {
            Ok(()) => LIBRARY_INITIALIZED.store(false, Ordering::Release),
            Err(error) => log::error!(
                "{error}; the library's global state may still be initialized, so no new Library can be loaded in this process"
            ),
        }
    }
}

/// Converts an owned C handle out-parameter to `NonNull`, treating a null
/// handle after a success result as an API error.
pub(crate) fn require_handle<T>(handle: *mut T, function: &'static str) -> Result<NonNull<T>> {
    NonNull::new(handle).ok_or_else(|| Error::Api {
        function,
        code: crate::error::StatusCode::UNCLASSIFIED,
        message: "returned success without a handle".to_string(),
        causes: Vec::new(),
    })
}
