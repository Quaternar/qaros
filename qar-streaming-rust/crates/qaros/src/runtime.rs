use std::ffi::CString;
use std::path::PathBuf;
use std::ptr::{self, NonNull};
use std::rc::Rc;
use std::time::Duration;

use qaros_sys::{self as sys, defaults};

use crate::adapter::GraphicsAdapter;
use crate::callbacks::BlockingOptions;
use crate::error::{Error, Result, check};
use crate::invite::OnboardingInvite;
use crate::library::{LibraryInner, require_handle};
use crate::session::Session;
use crate::strings;
use crate::types::OnboardingId;

/// Options for `qar_runtime_create`; `None` keeps the library default.
#[derive(Clone, Debug, Default)]
pub struct RuntimeConfig {
    /// Directory holding the runtime binaries (the package's `bin/`).
    pub runtime_binaries_directory: Option<PathBuf>,
    /// Root for device certificates and session state; the library default
    /// on Windows is `%LOCALAPPDATA%\Quaternar\QarOS`. Give each runtime
    /// instance in one application its own directory.
    pub storage_directory: Option<PathBuf>,
    /// Keep the identity in memory only (`QarRuntimeEphemeralStorageExt`), for a co-process
    /// that joins with an invite from its parent app: nothing is written under the storage
    /// directory, the returned onboarding id is zero, and [`Runtime::rejoin`] and
    /// [`Runtime::forget`] fail with [`crate::StatusCode::ARGUMENT_NOT_SUPPORTED`]. A runtime
    /// older than the extension skips it and stores the identity on disk: check
    /// [`crate::Library::runtime_info`] before relying on it.
    pub ephemeral_storage: bool,
}

/// How this peer presents itself to others. `None` keeps the persisted value
/// (or a generated default on first onboard).
#[derive(Clone, Debug, Default)]
pub struct PeerPresentation {
    pub display_name: Option<String>,
    pub app_version: Option<String>,
    pub custom_info: Option<String>,
}

/// Where the hub runs, for onboarding with a code against a remote hub.
#[derive(Clone, Copy, Debug)]
pub struct HubAddress<'a> {
    /// Plain host name or IP address, no URL.
    pub hostname: &'a str,
    /// The hub's onboarding port. Required: hubs claim it from a port range,
    /// and `0` fails with [`crate::StatusCode::ARGUMENT_NOT_SUPPORTED`].
    pub port: u16,
}

/// The onboarding path, chained as exactly one mode extension.
#[derive(Clone, Copy, Debug)]
pub enum OnboardMethod<'a> {
    /// The short code shown on the hub's onboarding screen (it lives about
    /// ten seconds). Without an address the library finds only this user's
    /// hub: through the `hub-endpoint.json` the hub writes to the QAROS data
    /// root, then a discovery beacon whose owner is this user (up to 6 s), else
    /// [`crate::StatusCode::ONBOARDING_HUB_UNREACHABLE`]. Any other hub needs
    /// its host and port in `hub`.
    Code {
        code: &'a str,
        hub: Option<HubAddress<'a>>,
    },
    /// An invite: a co-process invite from the local app that started this process
    /// ([`crate::Session::request_onboarding_invite`]), or one the hub's operators minted.
    Invite(&'a OnboardingInvite),
    /// A source app joining the QAROS hub installed on this PC, with no code
    /// (`QarOnboardLocalAppExt`, Windows only). The first start of an app
    /// waits until a person allows it in the hub, at most `approval_timeout`
    /// (`None`: the hub's default of 120 s). The C API reads a zero timeout as
    /// that default, so a duration under 1 ms (`Duration::ZERO` included) or
    /// over `u32::MAX` milliseconds fails with [`crate::Error::InvalidArgument`]
    /// before the call; pass `None` for the default. A refusal ends in one of the
    /// `ONBOARDING_APPROVAL_*`, `ONBOARDING_HUB_NOT_AUTHENTIC` or
    /// `ONBOARDING_APP_IDENTITY_MISMATCH` codes, see
    /// [`crate::StatusCode::user_action`]. Stateless: the returned onboarding
    /// id is the zero id, and rejoin and forget do not apply.
    ///
    /// `app_tag` (`QarOnboardLocalAppTagExt`) names this instance of the app: the hub remembers
    /// one peer id per approval and tag, so the next start with the same tag gets the same peer
    /// id back while no other live process holds it. An identity key: printable ASCII, no space
    /// at either end, at most `QAR_MAX_APP_TAG_LENGTH` bytes, never starting with `member/`, and
    /// constant across versions of the app; anything else fails with
    /// [`crate::StatusCode::ARGUMENT_NOT_SUPPORTED`]. `None` is the default tag, remembered the
    /// same way. Give instances that run together different tags.
    LocalApp {
        approval_timeout: Option<Duration>,
        app_tag: Option<&'a str>,
    },
}

/// The result of a successful onboarding.
#[derive(Debug)]
pub struct Onboarded {
    /// Persist this; it is the ticket for later rejoin and forget calls. Zero
    /// for a local app ([`OnboardMethod::LocalApp`]); nothing to persist.
    pub onboarding_id: OnboardingId,
    pub session: Session,
}

/// A `QarRuntime`. Sessions keep it alive; it is released after the last of them, with the call
/// the bindings' header releases a runtime with ([`qaros_sys::RUNTIME_RELEASE_SYMBOL`]:
/// `qar_runtime_handle_destroy` under the public header, `qar_runtime_destroy` under the header of
/// the runtimes up to 0.9.0-dev.2916).
#[derive(Debug)]
pub struct Runtime {
    pub(crate) inner: Rc<RuntimeInner>,
}

#[derive(Debug)]
pub(crate) struct RuntimeInner {
    handle: NonNull<sys::QarRuntime>,
    pub(crate) library: Rc<LibraryInner>,
}

struct PresentationStrings {
    display_name: Option<CString>,
    app_version: Option<CString>,
    custom_info: Option<CString>,
}

impl PresentationStrings {
    fn new(presentation: &PeerPresentation) -> Result<Self> {
        Ok(Self {
            display_name: strings::optional_c_string(
                presentation.display_name.as_deref(),
                "display name",
            )?,
            app_version: strings::optional_c_string(
                presentation.app_version.as_deref(),
                "app version",
            )?,
            custom_info: strings::optional_c_string(
                presentation.custom_info.as_deref(),
                "custom peer info",
            )?,
        })
    }

    /// The returned structure borrows these strings.
    fn to_raw(&self) -> sys::QarPeerPresentation {
        let mut raw = defaults::qar_peer_presentation_default();
        raw.display_name = strings::pointer_or_null(self.display_name.as_ref());
        raw.app_version = strings::pointer_or_null(self.app_version.as_ref());
        raw.app_custom_peer_info = strings::pointer_or_null(self.custom_info.as_ref());
        raw
    }
}

impl Runtime {
    pub(crate) fn create(library: &Rc<LibraryInner>, config: &RuntimeConfig) -> Result<Self> {
        let binaries_directory = config
            .runtime_binaries_directory
            .as_deref()
            .map(|directory| strings::path_to_c_string(directory, "runtime binaries directory"))
            .transpose()?;
        let storage_directory = config
            .storage_directory
            .as_deref()
            .map(|directory| strings::path_to_c_string(directory, "storage directory"))
            .transpose()?;

        let mut init = defaults::qar_runtime_init_default();
        init.runtime_binaries_folder_path = strings::pointer_or_null(binaries_directory.as_ref());
        init.storage_folder_path = strings::pointer_or_null(storage_directory.as_ref());
        let mut ephemeral_extension = defaults::qar_runtime_ephemeral_storage_ext_default();
        if config.ephemeral_storage {
            init.header.next = (&raw mut ephemeral_extension).cast();
        }

        let api = &library.api;
        let mut handle: *mut sys::QarRuntime = ptr::null_mut();
        // SAFETY: init and its chained extension are stamped and, with their strings, outlive
        // the call; handle is a valid out-pointer.
        let result = unsafe { (api.qar_impl_runtime_create)(&raw const init, &raw mut handle) };
        check(api, "qar_runtime_create", result)?;
        let handle = require_handle(handle, "qar_runtime_create")?;
        Ok(Self {
            inner: Rc::new(RuntimeInner {
                handle,
                library: Rc::clone(library),
            }),
        })
    }

    /// The `QarRuntime` handle, for the functions only the raw table of
    /// [`crate::Library::raw_api`] reaches, such as the asynchronous onboard and rejoin, or an
    /// onboard with an extension the safe API does not chain.
    ///
    /// Taking the pointer is safe; using it is `unsafe`, under the rules of
    /// [`crate::Session::as_raw`] for a runtime: valid while this `Runtime` lives, never
    /// destroyed by the caller (the last of the `Runtime` and its sessions releases it), used on
    /// the thread that uses this `Runtime`. A session a raw onboard or rejoin hands out is not a
    /// [`crate::Session`]: the caller destroys it with `qar_session_handle_destroy` before this
    /// `Runtime` drops.
    pub fn as_raw(&self) -> *mut sys::QarRuntime {
        self.inner.handle.as_ptr()
    }

    /// Restores the persisted session for `onboarding_id` without UI
    /// (`qar_runtime_rejoin`). [`crate::StatusCode::ONBOARDING_SESSION_NOT_FOUND`]
    /// means: onboard instead. The zero id (a local app's) is refused with
    /// [`Error::InvalidArgument`].
    pub fn rejoin(
        &self,
        onboarding_id: OnboardingId,
        presentation: &PeerPresentation,
        options: &BlockingOptions<'_>,
    ) -> Result<Session> {
        self.rejoin_on_adapter(onboarding_id, presentation, None, options)
    }

    /// [`Runtime::rejoin`] that creates the session's GPU devices, and with
    /// them every render sender's frame images, on `adapter`
    /// (`QarGraphicsDeviceId` chained into `QarRejoinInit`). `None` lets the
    /// library pick.
    pub fn rejoin_on_adapter(
        &self,
        onboarding_id: OnboardingId,
        presentation: &PeerPresentation,
        adapter: Option<GraphicsAdapter>,
        options: &BlockingOptions<'_>,
    ) -> Result<Session> {
        if onboarding_id.is_zero() {
            return Err(Error::InvalidArgument(
                "the zero onboarding id cannot be rejoined".to_string(),
            ));
        }
        let presentation_strings = PresentationStrings::new(presentation)?;
        let mut init = defaults::qar_rejoin_init_default();
        init.onboarding_id = onboarding_id.to_raw();
        init.presentation = presentation_strings.to_raw();
        let mut adapter_extension = adapter.map(GraphicsAdapter::to_raw);
        if let Some(extension) = adapter_extension.as_mut() {
            init.header.next = (&raw mut extension.header).cast();
        }

        let api = &self.inner.library.api;
        let mut session: *mut sys::QarSession = ptr::null_mut();
        // SAFETY: init, the chained adapter extension and the strings outlive
        // the call; the progress state and cancel token are borrowed from
        // options for the call.
        let result = unsafe {
            (api.qar_impl_runtime_rejoin)(
                self.inner.handle.as_ptr(),
                &raw const init,
                options.progress_callback(),
                options.progress_state(),
                options.cancel_pointer(),
                &raw mut session,
            )
        };
        check(api, "qar_runtime_rejoin", result)?;
        let session = require_handle(session, "qar_runtime_rejoin")?;
        Ok(Session::from_handle(session, Rc::clone(&self.inner)))
    }

    /// Full onboarding with a new identity (`qar_runtime_onboard`).
    pub fn onboard(
        &self,
        method: &OnboardMethod<'_>,
        presentation: &PeerPresentation,
        options: &BlockingOptions<'_>,
    ) -> Result<Onboarded> {
        self.onboard_on_adapter(method, presentation, None, options)
    }

    /// [`Runtime::onboard`] that creates the session's GPU devices, and with
    /// them every render sender's frame images, on `adapter`
    /// (`QarGraphicsDeviceId`, chained after the mode extension as in the
    /// Developer Guide's "Choosing the GPU"). `None` lets the library pick.
    pub fn onboard_on_adapter(
        &self,
        method: &OnboardMethod<'_>,
        presentation: &PeerPresentation,
        adapter: Option<GraphicsAdapter>,
        options: &BlockingOptions<'_>,
    ) -> Result<Onboarded> {
        let presentation_strings = PresentationStrings::new(presentation)?;
        let mut init = defaults::qar_onboard_init_default();
        init.presentation = presentation_strings.to_raw();

        let mut code_extension = defaults::qar_onboard_code_ext_default();
        let mut host_extension = defaults::qar_onboard_host_ext_default();
        let mut invite_extension = defaults::qar_onboard_invite_ext_default();
        let mut local_app_extension = defaults::qar_onboard_local_app_ext_default();
        let mut app_tag_extension = defaults::qar_onboard_local_app_tag_ext_default();
        let mut adapter_extension = adapter.map(GraphicsAdapter::to_raw);
        let adapter_pointer: *mut std::ffi::c_void = adapter_extension
            .as_mut()
            .map_or(ptr::null_mut(), |extension| {
                (&raw mut extension.header).cast()
            });
        let code_text: CString;
        let hostname_text: CString;
        let app_tag_text: CString;
        match method {
            OnboardMethod::Code { code, hub } => {
                code_text = strings::to_c_string(code, "pairing code")?;
                code_extension.code = code_text.as_ptr();
                code_extension.header.next = adapter_pointer;
                if let Some(address) = hub {
                    hostname_text = strings::to_c_string(address.hostname, "hub hostname")?;
                    host_extension.hostname = hostname_text.as_ptr();
                    host_extension.port = address.port;
                    host_extension.header.next = adapter_pointer;
                    code_extension.header.next = (&raw mut host_extension).cast();
                }
                init.header.next = (&raw mut code_extension).cast();
            }
            OnboardMethod::Invite(invite) => {
                invite_extension.invite = invite.as_ptr();
                invite_extension.header.next = adapter_pointer;
                init.header.next = (&raw mut invite_extension).cast();
            }
            OnboardMethod::LocalApp {
                approval_timeout,
                app_tag,
            } => {
                local_app_extension.approval_timeout_ms =
                    approval_timeout_milliseconds(*approval_timeout)?;
                local_app_extension.header.next = adapter_pointer;
                // The tag goes after the mode extension, before the adapter.
                if let Some(tag) = app_tag {
                    app_tag_text = strings::to_c_string(tag, "app tag")?;
                    app_tag_extension.app_tag = app_tag_text.as_ptr();
                    app_tag_extension.header.next = adapter_pointer;
                    local_app_extension.header.next = (&raw mut app_tag_extension).cast();
                }
                init.header.next = (&raw mut local_app_extension).cast();
            }
        }

        let api = &self.inner.library.api;
        let mut onboarding_id = defaults::qar_onboarding_id_default();
        let mut session: *mut sys::QarSession = ptr::null_mut();
        // SAFETY: init, the chained extensions and their strings live on this
        // stack frame for the whole call; out-pointers are valid.
        let result = unsafe {
            (api.qar_impl_runtime_onboard)(
                self.inner.handle.as_ptr(),
                &raw const init,
                options.progress_callback(),
                options.progress_state(),
                options.cancel_pointer(),
                &raw mut onboarding_id,
                &raw mut session,
            )
        };
        check(api, "qar_runtime_onboard", result)?;
        let session = require_handle(session, "qar_runtime_onboard")?;
        Ok(Onboarded {
            onboarding_id: OnboardingId::from_raw(onboarding_id),
            session: Session::from_handle(session, Rc::clone(&self.inner)),
        })
    }

    /// Erases a persisted identity slot (`qar_runtime_forget`). Fails with
    /// [`crate::StatusCode::ONBOARDING_SESSION_STILL_ACTIVE`] while a session
    /// of that slot is alive.
    pub fn forget(&self, onboarding_id: OnboardingId) -> Result<()> {
        if onboarding_id.is_zero() {
            return Err(Error::InvalidArgument(
                "the zero onboarding id cannot be forgotten".to_string(),
            ));
        }
        let mut init = defaults::qar_forget_init_default();
        init.onboarding_id = onboarding_id.to_raw();
        let api = &self.inner.library.api;
        // SAFETY: init is stamped and lives for the call.
        let result =
            unsafe { (api.qar_impl_runtime_forget)(self.inner.handle.as_ptr(), &raw const init) };
        check(api, "qar_runtime_forget", result)
    }
}

impl Drop for RuntimeInner {
    fn drop(&mut self) {
        // SAFETY: every session holds an Rc to this value, so all session
        // handles are already destroyed; the handle is not used again.
        unsafe { self.library.api.release_runtime(self.handle.as_ptr()) };
    }
}

/// `None` is the hub's default, which the C API spells as `0`; a zero
/// duration would mean the same, so it is refused instead of silently
/// becoming 120 s.
fn approval_timeout_milliseconds(approval_timeout: Option<Duration>) -> Result<u32> {
    let Some(timeout) = approval_timeout else {
        return Ok(0);
    };
    match u32::try_from(timeout.as_millis()) {
        Ok(0) => Err(Error::InvalidArgument(
            "a zero approval timeout; pass None for the hub's default".to_string(),
        )),
        Ok(milliseconds) => Ok(milliseconds),
        Err(_) => Err(Error::InvalidArgument(format!(
            "approval timeout {timeout:?} exceeds u32 milliseconds"
        ))),
    }
}
