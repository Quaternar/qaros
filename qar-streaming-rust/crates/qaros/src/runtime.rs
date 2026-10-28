use std::cell::RefCell;
use std::ffi::CString;
use std::path::PathBuf;
use std::ptr::{self, NonNull};
use std::rc::Rc;
use std::sync::mpsc::Sender;
use std::time::Duration;

use qaros_sys::{self as sys, defaults};

use crate::adapter::GraphicsAdapter;
use crate::callbacks::{BlockingOptions, Progress};
use crate::device::DeviceIdentity;
#[cfg(qaros_device_identity)]
use crate::device::DeviceIdentityStorage;
use crate::error::{Error, Result, check};
use crate::invite::OnboardingInvite;
use crate::library::{LibraryInner, require_handle};
use crate::member_hub::MemberHubInvitation;
use crate::pending::{self, AbandonedCall, CallSpec, PendingOnboard, RawOutcome};
use crate::session::Session;
use crate::strings;
use crate::types::{OnboardingId, PeerId};

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

/// How this peer presents itself to others (`QarPeerPresentation`). `None` keeps the persisted
/// value (or a generated default on first onboard).
#[derive(Clone, Debug, Default)]
pub struct PeerPresentation {
    /// The human label advertised to peers; it also feeds the sanitized debug label of the
    /// certificate and the storage slot.
    pub display_name: Option<String>,
    pub app_version: Option<String>,
    /// App-defined peer metadata.
    pub custom_info: Option<String>,
    /// The device this application describes itself as (`QarPeerDeviceIdentity`, chained on the
    /// presentation of the onboard or rejoin init). `None`: a desktop PC with the fixed id of the
    /// host OS. Where the bindings lack the structure (cfg `qaros_device_identity` unset), a set
    /// identity is [`Error::InvalidArgument`] before the call.
    pub device_identity: Option<DeviceIdentity>,
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
    /// ([`crate::Session::request_onboarding_invite`]), or one the hub's operators minted. The
    /// invite is borrowed for the call (for [`Runtime::start_onboard`], for the start call,
    /// during which the library copies it).
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
    ///
    /// A member hub's remote-app grant is redeemed with
    /// [`OnboardExtensions::remote_app_grant`] next to this mode.
    LocalApp {
        approval_timeout: Option<Duration>,
        app_tag: Option<&'a str>,
    },
}

/// The optional extensions of [`Runtime::onboard_with`] and [`Runtime::start_onboard`], chained
/// after the mode extension in this order: the remote-app grant, the adapter, the planned peer id.
/// The re-enrolled onboarding id goes into the init itself.
#[derive(Clone, Copy, Debug, Default)]
pub struct OnboardExtensions<'a> {
    /// Creates the session's GPU devices, and with them every render sender's frame images, on
    /// this adapter (`QarGraphicsDeviceId`). `None`: the GPU QAROS selected for an app it
    /// launched (carried in the launch invite), else one the library picks.
    pub adapter: Option<GraphicsAdapter>,
    /// Joins under this peer id (`QarOnboardPeerIdExt`), for a process whose peer id someone
    /// else planned and announced to the session. With [`OnboardMethod::Code`] or
    /// [`OnboardMethod::Invite`] only: it must equal any peer id the hub bound to the code or
    /// invite (a co-process invite always carries one), else onboarding fails. `None` takes that
    /// bound peer id, else a fresh one. The id is persisted with the slot, so a later rejoin keeps
    /// it. [`PeerId::ZERO`] and a local app ([`OnboardMethod::LocalApp`], whose peer id the hub
    /// assigns) are refused with [`Error::InvalidArgument`] before the call; the C API refuses
    /// both with [`crate::StatusCode::ARGUMENT_NOT_SUPPORTED`].
    pub peer_id: Option<PeerId>,
    /// Re-enrolls this persisted slot (`QarOnboardInit::onboarding_id`) instead of creating a new
    /// identity: the slot is reused when it survives and recreated under the same id after a
    /// wipe, so a persisted id never goes stale, and [`Onboarded::onboarding_id`] echoes it.
    /// [`OnboardingId::ZERO`] and a local app (stateless) are refused with
    /// [`Error::InvalidArgument`] before the call. A runtime with
    /// [`RuntimeConfig::ephemeral_storage`] refuses it with
    /// [`crate::StatusCode::ARGUMENT_NOT_SUPPORTED`], and a slot with a live session with
    /// [`crate::StatusCode::ONBOARDING_SESSION_STILL_ACTIVE`].
    pub onboarding_id: Option<OnboardingId>,
    /// Redeems a member hub's remote-app grant (`QarOnboardRemoteAppGrantExt`): the invitation
    /// the primary's app created with [`crate::MemberHubInvitationConfig::app_grant_tag`]. With
    /// [`OnboardMethod::LocalApp`] only, else [`Error::InvalidArgument`] before the call (the C
    /// API refuses it with [`crate::StatusCode::ARGUMENT_NOT_SUPPORTED`]). Only a member hub
    /// redeems a grant: join the hub on this PC to the invitation's primary first
    /// (`Runtime::join_hub_as_member`). A hub that is not a member ignores the grant, with no
    /// error of its own, and asks its operator as for any local app: the approval prompt, then
    /// [`crate::StatusCode::ONBOARDING_APPROVAL_DENIED`] or
    /// [`crate::StatusCode::ONBOARDING_APPROVAL_TIMEOUT`] when nobody allows it. On a member hub,
    /// when no approval admits this executable yet, the primary records it as the app the grant
    /// admits, without a prompt; once another executable redeemed the grant, or when the primary
    /// refuses it, onboarding fails with [`crate::StatusCode::ONBOARDING_APPROVAL_DENIED`]. When an approval already admits
    /// it, the invitation is not used. The grant's app tag applies; the method's `app_tag` is not
    /// needed. The invitation is borrowed for the call (for [`Runtime::start_onboard`], for the
    /// start call, during which the library copies it). Always `None` where the bindings lack
    /// member hubs, since no [`MemberHubInvitation`] exists there.
    pub remote_app_grant: Option<&'a MemberHubInvitation>,
}

/// The result of a successful onboarding or rejoin.
#[derive(Debug)]
pub struct Onboarded {
    /// Persist this; it is the ticket for later rejoin and forget calls. Zero
    /// for a local app ([`OnboardMethod::LocalApp`]) and on a runtime with
    /// [`RuntimeConfig::ephemeral_storage`]: nothing to persist. A rejoin echoes its id.
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
    /// Async onboards and rejoins whose [`PendingOnboard`] was dropped before the callback.
    abandoned_onboards: RefCell<Vec<AbandonedCall>>,
    pub(crate) library: Rc<LibraryInner>,
}

/// A [`PeerPresentation`] as C strings and the device identity structure, which the
/// presentation's `header.next` points at. Lives inside a boxed init storage, so its address is
/// stable while the library may read it.
#[derive(Debug)]
struct PresentationStorage {
    display_name: Option<CString>,
    app_version: Option<CString>,
    custom_info: Option<CString>,
    #[cfg(qaros_device_identity)]
    device_identity: Option<DeviceIdentityStorage>,
}

impl PresentationStorage {
    fn new(presentation: &PeerPresentation) -> Result<Self> {
        #[cfg(not(qaros_device_identity))]
        if presentation.device_identity.is_some() {
            return Err(Error::InvalidArgument(
                "these bindings come from a header without QarPeerDeviceIdentity".to_string(),
            ));
        }
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
            #[cfg(qaros_device_identity)]
            device_identity: presentation
                .device_identity
                .as_ref()
                .map(DeviceIdentityStorage::new)
                .transpose()?,
        })
    }

    /// Fills `raw` with pointers into this storage, which must not move while they are used.
    fn wire(&mut self, raw: &mut sys::QarPeerPresentation) {
        raw.display_name = strings::pointer_or_null(self.display_name.as_ref());
        raw.app_version = strings::pointer_or_null(self.app_version.as_ref());
        raw.app_custom_peer_info = strings::pointer_or_null(self.custom_info.as_ref());
        #[cfg(qaros_device_identity)]
        if let Some(identity) = self.device_identity.as_mut() {
            raw.header.next = (&raw mut identity.raw).cast();
        }
    }
}

/// Points `first` at the first header of `chain` and each header at the next.
///
/// # Safety
///
/// Every pointer is a valid, stamped structure header that outlives the chain's use.
unsafe fn link_chain(first: &mut *mut std::ffi::c_void, chain: &[*mut sys::QarStructureHeader]) {
    let mut next: *mut std::ffi::c_void = ptr::null_mut();
    for &header in chain.iter().rev() {
        // SAFETY: forwarded to the caller.
        unsafe { (*header).next = next };
        next = header.cast();
    }
    *first = next;
}

/// A `QarOnboardInit`, its extensions and the strings they point to, in one heap allocation the
/// library may read while the call runs (for the async call, until it calls back).
#[derive(Debug)]
pub(crate) struct OnboardInitStorage {
    init: sys::QarOnboardInit,
    code: sys::QarOnboardCodeExt,
    host: sys::QarOnboardHostExt,
    invite: sys::QarOnboardInviteExt,
    local_app: sys::QarOnboardLocalAppExt,
    app_tag: sys::QarOnboardLocalAppTagExt,
    #[cfg(qaros_member_hubs)]
    remote_app_grant: sys::QarOnboardRemoteAppGrantExt,
    adapter: sys::QarGraphicsDeviceId,
    peer_id: sys::QarOnboardPeerIdExt,
    presentation: PresentationStorage,
    code_text: Option<CString>,
    hostname_text: Option<CString>,
    app_tag_text: Option<CString>,
}

// SAFETY: the storage is plain data and pointers into itself (or to an invite or invitation
// handle the library copies during the start call); Rust never reads through them after building
// it and only frees it, which is sound from any thread.
unsafe impl Send for OnboardInitStorage {}
// SAFETY: as above; no `&self` method reads through the pointers.
unsafe impl Sync for OnboardInitStorage {}

impl OnboardInitStorage {
    pub(crate) fn new(
        method: &OnboardMethod<'_>,
        presentation: &PeerPresentation,
        extensions: &OnboardExtensions<'_>,
    ) -> Result<Box<Self>> {
        validate_extensions(method, extensions)?;
        let mut storage = Box::new(Self {
            init: defaults::qar_onboard_init_default(),
            code: defaults::qar_onboard_code_ext_default(),
            host: defaults::qar_onboard_host_ext_default(),
            invite: defaults::qar_onboard_invite_ext_default(),
            local_app: defaults::qar_onboard_local_app_ext_default(),
            app_tag: defaults::qar_onboard_local_app_tag_ext_default(),
            #[cfg(qaros_member_hubs)]
            remote_app_grant: defaults::qar_onboard_remote_app_grant_ext_default(),
            adapter: defaults::qar_graphics_device_id_default(),
            peer_id: defaults::qar_onboard_peer_id_ext_default(),
            presentation: PresentationStorage::new(presentation)?,
            code_text: None,
            hostname_text: None,
            app_tag_text: None,
        });
        // Everything below points into the box, whose contents never move.
        let this = &mut *storage;
        if let Some(onboarding_id) = extensions.onboarding_id {
            this.init.onboarding_id = onboarding_id.to_raw();
        }
        this.presentation.wire(&mut this.init.presentation);
        let mut chain: Vec<*mut sys::QarStructureHeader> = Vec::new();
        match method {
            OnboardMethod::Code { code, hub } => {
                let code = this
                    .code_text
                    .insert(strings::to_c_string(code, "pairing code")?);
                this.code.code = code.as_ptr();
                chain.push(&raw mut this.code.header);
                if let Some(address) = hub {
                    let hostname = this
                        .hostname_text
                        .insert(strings::to_c_string(address.hostname, "hub hostname")?);
                    this.host.hostname = hostname.as_ptr();
                    this.host.port = address.port;
                    chain.push(&raw mut this.host.header);
                }
            }
            OnboardMethod::Invite(invite) => {
                this.invite.invite = invite.as_ptr();
                chain.push(&raw mut this.invite.header);
            }
            OnboardMethod::LocalApp {
                approval_timeout,
                app_tag,
            } => {
                this.local_app.approval_timeout_ms =
                    optional_milliseconds(*approval_timeout, "approval timeout")?;
                chain.push(&raw mut this.local_app.header);
                if let Some(tag) = app_tag {
                    let tag = this
                        .app_tag_text
                        .insert(strings::to_c_string(tag, "app tag")?);
                    this.app_tag.app_tag = tag.as_ptr();
                    chain.push(&raw mut this.app_tag.header);
                }
            }
        }
        #[cfg(qaros_member_hubs)]
        if let Some(invitation) = extensions.remote_app_grant {
            this.remote_app_grant.invitation = invitation.as_ptr();
            chain.push(&raw mut this.remote_app_grant.header);
        }
        if let Some(adapter) = extensions.adapter {
            this.adapter = adapter.to_raw();
            chain.push(&raw mut this.adapter.header);
        }
        if let Some(peer_id) = extensions.peer_id {
            this.peer_id.peer_id = peer_id.to_raw();
            chain.push(&raw mut this.peer_id.header);
        }
        // SAFETY: every header is a stamped structure inside the box.
        unsafe { link_chain(&mut this.init.header.next, &chain) };
        Ok(storage)
    }

    pub(crate) fn init(&self) -> *const sys::QarOnboardInit {
        &raw const self.init
    }
}

/// What the C API refuses with `QAR_STATUS_ARGUMENT_NOT_SUPPORTED`, refused before the call.
fn validate_extensions(
    method: &OnboardMethod<'_>,
    extensions: &OnboardExtensions<'_>,
) -> Result<()> {
    let local_app = matches!(method, OnboardMethod::LocalApp { .. });
    let refuse = |message: &str| Err(Error::InvalidArgument(message.to_string()));
    match extensions.peer_id {
        Some(peer_id) if peer_id.is_zero() => return refuse("the zero peer id cannot be planned"),
        Some(_) if local_app => {
            return refuse("a local app gets its peer id from the hub; no planned peer id");
        }
        _ => {}
    }
    match extensions.onboarding_id {
        Some(id) if id.is_zero() => return refuse("the zero onboarding id names no slot"),
        Some(_) if local_app => return refuse("a local app is stateless; no slot to re-enroll"),
        _ => {}
    }
    if extensions.remote_app_grant.is_some() && !local_app {
        return refuse("a remote-app grant is redeemed only by a local app");
    }
    Ok(())
}

/// A `QarRejoinInit`, its adapter extension and the presentation strings, in one heap allocation.
#[derive(Debug)]
pub(crate) struct RejoinInitStorage {
    init: sys::QarRejoinInit,
    adapter: sys::QarGraphicsDeviceId,
    presentation: PresentationStorage,
}

// SAFETY: as for OnboardInitStorage.
unsafe impl Send for RejoinInitStorage {}
// SAFETY: as for OnboardInitStorage.
unsafe impl Sync for RejoinInitStorage {}

impl RejoinInitStorage {
    pub(crate) fn new(
        onboarding_id: OnboardingId,
        presentation: &PeerPresentation,
        adapter: Option<GraphicsAdapter>,
    ) -> Result<Box<Self>> {
        if onboarding_id.is_zero() {
            return Err(Error::InvalidArgument(
                "the zero onboarding id cannot be rejoined".to_string(),
            ));
        }
        let mut storage = Box::new(Self {
            init: defaults::qar_rejoin_init_default(),
            adapter: defaults::qar_graphics_device_id_default(),
            presentation: PresentationStorage::new(presentation)?,
        });
        let this = &mut *storage;
        this.init.onboarding_id = onboarding_id.to_raw();
        this.presentation.wire(&mut this.init.presentation);
        if let Some(adapter) = adapter {
            this.adapter = adapter.to_raw();
            this.init.header.next = (&raw mut this.adapter.header).cast();
        }
        Ok(storage)
    }

    pub(crate) fn init(&self) -> *const sys::QarRejoinInit {
        &raw const self.init
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
                abandoned_onboards: RefCell::new(Vec::new()),
                library: Rc::clone(library),
            }),
        })
    }

    /// The `QarRuntime` handle, for the functions only the raw table of
    /// [`crate::Library::raw_api`] reaches.
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
    /// [`Error::InvalidArgument`]. The peer id is the one persisted by the onboard.
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
        self.inner.release_delivered_abandoned_onboards();
        let storage = RejoinInitStorage::new(onboarding_id, presentation, adapter)?;
        let api = &self.inner.library.api;
        let mut session: *mut sys::QarSession = ptr::null_mut();
        // SAFETY: the init storage outlives the call; the progress state and cancel token are
        // borrowed from options for the call; valid out-pointer.
        let result = unsafe {
            (api.qar_impl_runtime_rejoin)(
                self.inner.handle.as_ptr(),
                storage.init(),
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

    /// Full onboarding (`qar_runtime_onboard`), chaining the mode extension of `method`. The
    /// progress callback of `options` reports the steps (for a local app, "Waiting for approval
    /// in QAROS hub" while the prompt is open) and ends before the call returns.
    pub fn onboard(
        &self,
        method: &OnboardMethod<'_>,
        presentation: &PeerPresentation,
        options: &BlockingOptions<'_>,
    ) -> Result<Onboarded> {
        self.onboard_with(method, presentation, &OnboardExtensions::default(), options)
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
        let extensions = OnboardExtensions {
            adapter,
            ..OnboardExtensions::default()
        };
        self.onboard_with(method, presentation, &extensions, options)
    }

    /// [`Runtime::onboard`] with the optional extensions of `extensions`.
    pub fn onboard_with(
        &self,
        method: &OnboardMethod<'_>,
        presentation: &PeerPresentation,
        extensions: &OnboardExtensions<'_>,
        options: &BlockingOptions<'_>,
    ) -> Result<Onboarded> {
        self.inner.release_delivered_abandoned_onboards();
        let storage = OnboardInitStorage::new(method, presentation, extensions)?;
        let api = &self.inner.library.api;
        let mut onboarding_id = defaults::qar_onboarding_id_default();
        let mut session: *mut sys::QarSession = ptr::null_mut();
        // SAFETY: the init storage outlives the call; the progress state and cancel token are
        // borrowed from options for the call; out-pointers are valid.
        let result = unsafe {
            (api.qar_impl_runtime_onboard)(
                self.inner.handle.as_ptr(),
                storage.init(),
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

    /// Starts [`Runtime::onboard_with`] without blocking (`qar_runtime_onboard_async`) and
    /// returns at once; [`PendingOnboard::poll`] or [`PendingOnboard::wait`] picks up the
    /// [`Onboarded`]. `progress` receives the progress reports, sent from a library thread until
    /// the result arrives. `timeout` cancels the call through its cancel token, after which the
    /// library calls back with [`crate::StatusCode::TIMEOUT`]. `wake` runs on a library thread
    /// right after the result arrived, for example to request a repaint that then polls. See
    /// [`PendingOnboard`] for dropping it before the result.
    pub fn start_onboard(
        &self,
        method: &OnboardMethod<'_>,
        presentation: &PeerPresentation,
        extensions: &OnboardExtensions<'_>,
        progress: Option<Sender<Progress>>,
        timeout: Option<Duration>,
        wake: impl Fn() + Send + Sync + 'static,
    ) -> Result<PendingOnboard> {
        self.inner.release_delivered_abandoned_onboards();
        let storage = OnboardInitStorage::new(method, presentation, extensions)?;
        let init = storage.init();
        let runtime = self.inner.handle();
        let api = &self.inner.library.api;
        self.start_session_call(
            "qar_runtime_onboard_async",
            Box::new(storage),
            progress,
            timeout,
            Box::new(wake),
            |user_state, progress_callback, token| {
                // SAFETY: live runtime; init lives in the call's state until the callback; the
                // token outlives the call (the pending call or its owner keeps it).
                unsafe {
                    (api.qar_impl_runtime_onboard_async)(
                        runtime,
                        init,
                        Some(pending::onboard_result_trampoline),
                        progress_callback,
                        user_state,
                        token,
                    )
                }
            },
        )
    }

    /// Starts [`Runtime::rejoin_on_adapter`] without blocking (`qar_runtime_rejoin_async`), as
    /// [`Runtime::start_onboard`] does; the [`Onboarded`] it delivers echoes `onboarding_id`. The
    /// zero id is refused with [`Error::InvalidArgument`] before the call.
    pub fn start_rejoin(
        &self,
        onboarding_id: OnboardingId,
        presentation: &PeerPresentation,
        adapter: Option<GraphicsAdapter>,
        progress: Option<Sender<Progress>>,
        timeout: Option<Duration>,
        wake: impl Fn() + Send + Sync + 'static,
    ) -> Result<PendingOnboard> {
        self.inner.release_delivered_abandoned_onboards();
        let storage = RejoinInitStorage::new(onboarding_id, presentation, adapter)?;
        let init = storage.init();
        let runtime = self.inner.handle();
        let api = &self.inner.library.api;
        self.start_session_call(
            "qar_runtime_rejoin_async",
            Box::new(storage),
            progress,
            timeout,
            Box::new(wake),
            |user_state, progress_callback, token| {
                // SAFETY: as in start_onboard.
                unsafe {
                    (api.qar_impl_runtime_rejoin_async)(
                        runtime,
                        init,
                        Some(pending::onboard_result_trampoline),
                        progress_callback,
                        user_state,
                        token,
                    )
                }
            },
        )
    }

    /// Starts an async onboard or rejoin whose callback hands over a session.
    fn start_session_call(
        &self,
        function: &'static str,
        keep: Box<dyn std::any::Any + Send + Sync>,
        progress: Option<Sender<Progress>>,
        timeout: Option<Duration>,
        wake: Box<dyn Fn() + Send + Sync>,
        start_call: impl FnOnce(
            *mut std::ffi::c_void,
            sys::qar_progress_callback_t,
            *mut sys::QarCancelToken,
        ) -> sys::QarResult,
    ) -> Result<PendingOnboard> {
        let finish_runtime = Rc::clone(&self.inner);
        let abandon_runtime = Rc::clone(&self.inner);
        let poll_runtime = Rc::clone(&self.inner);
        let spec = CallSpec {
            function,
            progress,
            timeout,
            wake,
            keep: Some(keep),
            finish: Box::new(move |outcome: RawOutcome| {
                let api = &finish_runtime.library.api;
                if let Err(error) = check(api, function, outcome.result) {
                    release_session(api, outcome.handle);
                    return Err(error);
                }
                let session = require_handle(outcome.handle as *mut sys::QarSession, function)?;
                Ok(Onboarded {
                    onboarding_id: OnboardingId(outcome.id),
                    session: Session::from_handle(session, finish_runtime),
                })
            }),
            abandon: Box::new(move |call| abandon_runtime.abandon_onboard(call)),
            before_poll: Box::new(move || poll_runtime.release_delivered_abandoned_onboards()),
            destroy: Some(release_session),
        };
        pending::start(&self.inner.library, spec, start_call)
    }

    /// Erases a persisted identity slot (`qar_runtime_forget`): its certificate, persisted
    /// instance id and session state. Works with no active session. Fails with
    /// [`crate::StatusCode::ONBOARDING_SESSION_STILL_ACTIVE`] while a session of that slot is
    /// alive, and [`crate::StatusCode::ONBOARDING_SESSION_NOT_FOUND`] without a slot.
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

impl RuntimeInner {
    pub(crate) fn handle(&self) -> *mut sys::QarRuntime {
        self.handle.as_ptr()
    }

    pub(crate) fn abandon_onboard(&self, onboard: AbandonedCall) {
        self.abandoned_onboards.borrow_mut().push(onboard);
    }

    /// Destroys the sessions that abandoned onboards delivered since the last call.
    pub(crate) fn release_delivered_abandoned_onboards(&self) {
        let api = &self.library.api;
        self.abandoned_onboards
            .borrow_mut()
            .retain(|onboard| !onboard.release_if_delivered(api));
    }
}

impl Drop for RuntimeInner {
    fn drop(&mut self) {
        let api = &self.library.api;
        // A session an abandoned onboard delivered is destroyed while the runtime lives.
        self.abandoned_onboards
            .get_mut()
            .retain(|onboard| !onboard.release_if_delivered(api));
        // SAFETY: every session holds an Rc to this value, so all session handles are already
        // destroyed; the handle is not used again. Per the header, the destroy returns only after
        // the callbacks of the onboards still running have returned (with QAR_STATUS_TIMEOUT).
        unsafe { api.release_runtime(self.handle.as_ptr()) };
        for onboard in std::mem::take(self.abandoned_onboards.get_mut()) {
            onboard.leak_late_delivery();
        }
    }
}

/// Destroys a session an async onboard or rejoin delivered that nobody takes.
fn release_session(api: &sys::QarApi, session: usize) {
    if session != 0 {
        // SAFETY: delivered to us, used by nobody, and its runtime is alive.
        unsafe { (api.qar_impl_session_handle_destroy)(session as *mut sys::QarSession) };
    }
}

/// `None` is the C API's default, which it spells as `0`; a zero (or sub-millisecond) duration
/// would mean the same, so it is refused instead of silently becoming the default.
pub(crate) fn optional_milliseconds(value: Option<Duration>, what: &str) -> Result<u32> {
    let Some(duration) = value else {
        return Ok(0);
    };
    match u32::try_from(duration.as_millis()) {
        Ok(0) => Err(Error::InvalidArgument(format!(
            "a zero {what}; pass None for the default"
        ))),
        Ok(milliseconds) => Ok(milliseconds),
        Err(_) => Err(Error::InvalidArgument(format!(
            "{what} {duration:?} exceeds u32 milliseconds"
        ))),
    }
}
