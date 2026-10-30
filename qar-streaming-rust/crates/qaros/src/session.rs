use std::cell::RefCell;
use std::ffi::c_void;
use std::ptr::{self, NonNull};
use std::rc::Rc;
use std::sync::mpsc::Sender;
use std::time::Duration;

use qaros_sys::{self as sys, QarApi, QarCancelToken, QarResult, defaults};

use crate::adapter::GraphicsAdapter;
use crate::callbacks::{BlockingOptions, EventCallback, Progress, deliver};
use crate::cancel::CancelTokenInner;
use crate::error::{Result, check};
use crate::invite::OnboardingInvite;
use crate::library::{Library, require_handle};
use crate::panels::GuiPanels;
use crate::peers::{PeerInfo, read_peer_info};
use crate::pending::{self, AbandonedCall, CallSpec, Destroy, Keep, PendingCall, RawOutcome};
use crate::pending_sender::PendingRenderSender;
use crate::render::{
    RenderRequest, RenderSender, RenderSenderConfig, SenderGraphics, read_render_request,
};
use crate::runtime::RuntimeInner;
use crate::strings;
use crate::types::{PeerId, SessionId, SessionLifecycle};
use crate::volumes::AppVolumes;

/// A `QarSession`, obtained by [`crate::Runtime::rejoin`] or
/// [`crate::Runtime::onboard`].
///
/// Dropping the last handle destroys the session (`qar_session_handle_destroy`)
/// and keeps the persisted identity, so a later rejoin works. Render senders
/// keep the session alive; the session keeps its runtime alive.
///
/// Subscription callback state is owned here and freed only after the
/// session handle is destroyed. The vendored header ("Ending a
/// subscription") guarantees that cancelling waits for a running callback,
/// so freeing at cancel would also be sound; freeing at session end keeps a
/// channel connected until then, which callers already rely on.
#[derive(Debug)]
pub struct Session {
    pub(crate) inner: Rc<SessionInner>,
}

pub(crate) struct SessionInner {
    pub(crate) handle: NonNull<sys::QarSession>,
    subscriptions: RefCell<Vec<SubscriptionSlot>>,
    /// Async invites and target-app calls whose [`PendingCall`] was dropped before the callback.
    abandoned_calls: RefCell<Vec<AbandonedCall>>,
    pub(crate) runtime: Rc<RuntimeInner>,
}

/// Callback state is handed to C as a raw pointer from `Box::into_raw` and
/// never moved again; `release_state` rebuilds and drops the box.
struct SubscriptionSlot {
    token: Rc<CancelTokenInner>,
    callback_state: *mut c_void,
    release_state: unsafe fn(*mut c_void),
}

/// # Safety
///
/// `callback_state` must come from `Box::into_raw` of a `Box<State>` and must
/// not be used afterwards.
unsafe fn release_callback_state<State>(callback_state: *mut c_void) {
    // SAFETY: forwarded to the caller.
    drop(unsafe { Box::from_raw(callback_state.cast::<State>()) });
}

impl Drop for SubscriptionSlot {
    fn drop(&mut self) {
        // SAFETY: the slot owns the state; slots are dropped only after the
        // session handle is destroyed.
        unsafe { (self.release_state)(self.callback_state) };
    }
}

impl std::fmt::Debug for SessionInner {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Session")
            .field("handle", &self.handle)
            .field("subscriptions", &self.subscriptions.borrow().len())
            .finish_non_exhaustive()
    }
}

/// Guard for one subscription. Dropping it cancels delivery (through the
/// subscription's cancel token); the callback itself stays allocated until
/// the session is destroyed. A receiver from [`crate::channel`] therefore
/// does not disconnect when the guard drops, only when the session does.
#[derive(Debug)]
#[must_use = "dropping the subscription cancels it; call detach() to keep it for the session's lifetime"]
pub struct Subscription {
    token: Option<Rc<CancelTokenInner>>,
}

impl Subscription {
    /// Keeps delivering until the session is destroyed.
    pub fn detach(mut self) {
        self.token = None;
    }

    /// Cancels delivery, as dropping does, and reports a failed cancel.
    pub fn cancel(mut self) -> Result<()> {
        match self.token.take() {
            Some(token) => token.cancel(),
            None => Ok(()),
        }
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        if let Some(token) = self.token.take()
            && let Err(error) = token.cancel()
        {
            log::warn!("cancelling a subscription: {error}");
        }
    }
}

/// State behind a subscription's `user_state` pointer. Read from library
/// threads; it holds only the API table pointer and a `Send + Sync` callback.
pub(crate) struct CallbackState<Event> {
    api: NonNull<QarApi>,
    callback: EventCallback<Event>,
}

// SAFETY: `api` points into the library's Rc allocation, which outlives the
// session and therefore every callback; QarApi is a table of function
// pointers plus the library handle, both safe to share. The callback is
// Send + Sync by its type.
unsafe impl<Event> Send for CallbackState<Event> {}
// SAFETY: as above.
unsafe impl<Event> Sync for CallbackState<Event> {}

impl<Event> CallbackState<Event> {
    /// # Safety
    ///
    /// `user_state` must be the pointer registered for a live subscription
    /// of this `Event` type.
    pub(crate) unsafe fn from_user_state<'state>(user_state: *mut c_void) -> &'state Self {
        // SAFETY: forwarded to the caller; the state lives until the session
        // handle is destroyed.
        unsafe { &*user_state.cast::<Self>() }
    }

    pub(crate) fn api(&self) -> &QarApi {
        // SAFETY: see the Send/Sync justification above.
        unsafe { self.api.as_ref() }
    }

    pub(crate) fn deliver(&self, event: Event, source: &str) {
        deliver(&self.callback, event, source);
    }
}

impl Session {
    pub(crate) fn from_handle(handle: NonNull<sys::QarSession>, runtime: Rc<RuntimeInner>) -> Self {
        Self {
            inner: Rc::new(SessionInner {
                handle,
                subscriptions: RefCell::new(Vec::new()),
                abandoned_calls: RefCell::new(Vec::new()),
                runtime,
            }),
        }
    }

    /// The library this session belongs to.
    pub fn library(&self) -> Library {
        Library {
            inner: Rc::clone(&self.inner.runtime.library),
        }
    }

    /// The `QarSession` handle, for the functions only the raw table of
    /// [`Library::raw_api`] reaches (the "Raw only" column of the README's coverage table), as
    /// in `(library.raw_api().qar_impl_gui_panels_update_visible_to)(session.as_raw(), ...)`.
    ///
    /// Taking the pointer is safe; using it is `unsafe`, and the caller keeps these rules:
    ///
    /// - It is valid while this `Session` lives; never use it after the `Session` is dropped.
    /// - Never destroy it (`qar_session_handle_destroy`): the `Session` destroys it on drop.
    /// - The session's threading rules apply: call with it on the thread that uses this
    ///   `Session` (no handle type is `Send`). A callback registered through a raw call runs on
    ///   a library thread, and its state must stay alive until its subscription ends; end the
    ///   subscription (cancel its token) before the session drops.
    /// - A handle a raw call hands out (a peer spec, a panel, an invite) is the caller's to
    ///   destroy with its `_handle_destroy` function.
    pub fn as_raw(&self) -> *mut sys::QarSession {
        self.inner.handle.as_ptr()
    }

    /// The session's id (`qar_session_get_id`).
    pub fn id(&self) -> Result<SessionId> {
        let api = self.inner.api();
        let mut session_id = defaults::qar_session_id_default();
        // SAFETY: live session handle; valid out-pointer.
        let result = unsafe {
            (api.qar_impl_session_get_id)(self.inner.handle.as_ptr(), &raw mut session_id)
        };
        check(api, "qar_session_get_id", result)?;
        Ok(SessionId::from_raw(session_id))
    }

    /// The GPU adapter the session's graphics devices, and so every render
    /// sender's frame textures, live on (`qar_session_get_graphics_device_id`):
    /// the one chained at onboard or rejoin, else the one QAROS or the library
    /// selected. Create the renderer's device on it. `None` when the session
    /// knows no adapter.
    pub fn graphics_adapter(&self) -> Result<Option<GraphicsAdapter>> {
        let api = self.inner.api();
        let mut device_id = defaults::qar_graphics_device_id_default();
        let mut present = false;
        // SAFETY: live session handle; valid out-pointers.
        let result = unsafe {
            (api.qar_impl_session_get_graphics_device_id)(
                self.inner.handle.as_ptr(),
                &raw mut device_id,
                &raw mut present,
            )
        };
        check(api, "qar_session_get_graphics_device_id", result)?;
        if present {
            GraphicsAdapter::from_raw(&device_id).map(Some)
        } else {
            Ok(None)
        }
    }

    /// This application's own peer.
    pub fn my_peer(&self) -> Result<PeerInfo> {
        let api = self.inner.api();
        let mut spec: *mut sys::QarPeerSpec = ptr::null_mut();
        // SAFETY: live session handle; valid out-pointer.
        let result = unsafe {
            (api.qar_impl_session_get_my_spec)(self.inner.handle.as_ptr(), &raw mut spec)
        };
        check(api, "qar_session_get_my_spec", result)?;
        let spec = OwnedHandle::new(
            require_handle(spec, "qar_session_get_my_spec")?,
            api.qar_impl_peer_spec_handle_destroy,
        );
        read_peer_info(api, spec.as_ptr())
    }

    /// Snapshot of every known peer (count-then-fetch).
    pub fn peers(&self) -> Result<Vec<PeerInfo>> {
        let api = self.inner.api();
        let session = self.inner.handle.as_ptr();
        query_owned_handles(
            api,
            // SAFETY: live session handle; valid out-pointer.
            |count| unsafe { (api.qar_impl_query_peer_specs_count)(session, count) },
            // SAFETY: the buffer holds `capacity` handle slots.
            |handles, capacity, written| unsafe {
                (api.qar_impl_query_peer_specs)(session, handles, capacity, written)
            },
            api.qar_impl_peer_spec_handle_destroy,
            "qar_query_peer_specs",
            |handle| read_peer_info(api, handle),
        )
    }

    /// `qar_peer_update_display_name`.
    pub fn set_display_name(&self, display_name: &str) -> Result<()> {
        let api = self.inner.api();
        let display_name = strings::to_c_string(display_name, "display name")?;
        // SAFETY: live session handle; NUL-terminated string.
        let result = unsafe {
            (api.qar_impl_peer_update_display_name)(
                self.inner.handle.as_ptr(),
                display_name.as_ptr(),
            )
        };
        check(api, "qar_peer_update_display_name", result)
    }

    /// Calls `callback` with a snapshot whenever a peer joins, leaves or
    /// changes. Runs on library threads.
    pub fn subscribe_peer_updates(
        &self,
        callback: impl Fn(PeerInfo) + Send + Sync + 'static,
    ) -> Result<Subscription> {
        let session = self.inner.handle.as_ptr();
        self.inner.subscribe(
            Box::new(callback),
            "qar_peer_subscribe_updates",
            |api, user_state, token| {
                // SAFETY: live session; user_state stays valid until the
                // session handle is destroyed.
                unsafe {
                    (api.qar_impl_peer_subscribe_updates)(
                        session,
                        Some(peer_update_trampoline),
                        user_state,
                        token,
                    )
                }
            },
        )
    }

    /// The session's current lifecycle state (`qar_session_get_lifecycle_state`): the session
    /// id it is in now and, after a move, the one it moved from and why. `reason` is empty here.
    pub fn lifecycle(&self) -> Result<SessionLifecycle> {
        let api = self.inner.api();
        let mut info = defaults::qar_session_lifecycle_info_default();
        // SAFETY: live session; info is stamped and a valid out-pointer.
        let result = unsafe {
            (api.qar_impl_session_get_lifecycle_state)(self.inner.handle.as_ptr(), &raw mut info)
        };
        check(api, "qar_session_get_lifecycle_state", result)?;
        Ok(SessionLifecycle::from_raw(&info))
    }

    /// Calls `callback` with each lifecycle state of the session
    /// (`qar_session_subscribe_lifecycle`): first the current one, then every change. Runs on
    /// library threads; hand the snapshot to the app's own thread. On a new session id, get or
    /// create again what the app keys by it (app volumes, panels, senders); on
    /// [`crate::SessionLifecycleState::RePairRequired`], drop the session and onboard again.
    pub fn subscribe_lifecycle(
        &self,
        callback: impl Fn(SessionLifecycle) + Send + Sync + 'static,
    ) -> Result<Subscription> {
        let session = self.inner.handle.as_ptr();
        self.inner.subscribe(
            Box::new(callback),
            "qar_session_subscribe_lifecycle",
            |api, user_state, token| {
                // SAFETY: as in subscribe_peer_updates.
                unsafe {
                    (api.qar_impl_session_subscribe_lifecycle)(
                        session,
                        Some(session_lifecycle_trampoline),
                        user_state,
                        token,
                    )
                }
            },
        )
    }

    /// Calls `callback` for each incoming render stream request (for example
    /// from a visualizer). Runs on library threads; create the sender on the
    /// session's thread.
    pub fn subscribe_render_requests(
        &self,
        callback: impl Fn(RenderRequest) + Send + Sync + 'static,
    ) -> Result<Subscription> {
        let session = self.inner.handle.as_ptr();
        self.inner.subscribe(
            Box::new(callback),
            "qar_render_sender_subscribe_requests",
            |api, user_state, token| {
                // SAFETY: as in subscribe_peer_updates.
                unsafe {
                    (api.qar_impl_render_sender_subscribe_requests)(
                        session,
                        Some(render_request_trampoline),
                        user_state,
                        token,
                    )
                }
            },
        )
    }

    /// `qar_render_sender_create` with the CPU graphics API.
    pub fn create_render_sender(
        &self,
        config: &RenderSenderConfig,
        options: &BlockingOptions<'_>,
    ) -> Result<RenderSender> {
        RenderSender::create(&self.inner, config, &SenderGraphics::Cpu, options)
    }

    /// Starts `qar_render_sender_create_async` with the CPU graphics API and
    /// returns at once; [`PendingRenderSender::poll`] picks the sender up on
    /// a later frame. `timeout` cancels the creation through its cancel
    /// token. `wake` runs on a library thread right after the library called
    /// back, for example to request a repaint that then polls. See
    /// [`PendingRenderSender`] for dropping a creation before it completes.
    pub fn start_render_sender(
        &self,
        config: &RenderSenderConfig,
        timeout: Option<Duration>,
        wake: impl Fn() + Send + Sync + 'static,
    ) -> Result<PendingRenderSender> {
        crate::pending_sender::start(
            &self.inner,
            config,
            &SenderGraphics::Cpu,
            timeout,
            Box::new(wake),
        )
    }

    /// `qar_render_sender_create` with the Vulkan graphics API: QAROS creates
    /// the frame images on `vulkan.device` and hands them out per frame
    /// through [`crate::OwnedFrame::vulkan_frame`]. Use
    /// [`crate::FrameLayout::SeparatedTextures`] for one image per eye and
    /// kind, as QuaternAR's cube does. See [`crate::vulkan`] for the frame
    /// protocol.
    ///
    /// # Safety
    ///
    /// - `vulkan.physical_device` and `vulkan.device` are valid handles, the
    ///   device was created from that physical device, on the adapter this
    ///   session was onboarded or rejoined with, and enables
    ///   [`crate::vulkan::required_device_extensions`].
    /// - The device outlives the returned sender and every frame and image
    ///   handle taken from it.
    #[cfg(feature = "vulkan")]
    pub unsafe fn create_vulkan_render_sender(
        &self,
        config: &RenderSenderConfig,
        vulkan: crate::vulkan::VulkanStreamParams,
        options: &BlockingOptions<'_>,
    ) -> Result<RenderSender> {
        RenderSender::create(
            &self.inner,
            config,
            &SenderGraphics::Vulkan(vulkan),
            options,
        )
    }

    /// [`Session::start_render_sender`] with the Vulkan graphics API, the
    /// non-blocking form of [`Session::create_vulkan_render_sender`].
    ///
    /// # Safety
    ///
    /// As for [`Session::create_vulkan_render_sender`], and the device must
    /// also stay valid for as long as the library may still create a sender
    /// on it: until this creation's sender is taken and dropped; if the
    /// [`PendingRenderSender`] is dropped first, until the session is
    /// dropped; and if that creation had not called back by then, until the
    /// library is dropped.
    #[cfg(feature = "vulkan")]
    pub unsafe fn start_vulkan_render_sender(
        &self,
        config: &RenderSenderConfig,
        vulkan: crate::vulkan::VulkanStreamParams,
        timeout: Option<Duration>,
        wake: impl Fn() + Send + Sync + 'static,
    ) -> Result<PendingRenderSender> {
        crate::pending_sender::start(
            &self.inner,
            config,
            &SenderGraphics::Vulkan(vulkan),
            timeout,
            Box::new(wake),
        )
    }

    /// `qar_render_sender_create` with the D3D11 graphics API: QAROS creates the frame's
    /// shared textures on the device in `d3d11` and hands them out per frame through
    /// [`crate::OwnedFrame::d3d11_frame`]. See [`crate::d3d11`] for the frame protocol. The
    /// sender keeps the references `D3D11StreamParams::new` (Windows) took on the device
    /// and context until it is destroyed, and its session with it when it held the last
    /// reference.
    ///
    /// # Safety
    ///
    /// - The immediate context in `d3d11` is used only on the thread that drives this session
    ///   and its senders, or the device has multithread protection on: QAROS uses the context
    ///   at times and on threads the header does not document, and a D3D11 immediate context
    ///   is not thread-safe.
    /// - Parameters from [`crate::d3d11::D3D11StreamParams::from_raw`] hold valid pointers as
    ///   long as that function requires.
    ///
    /// The device should be on the adapter this session was onboarded or rejoined with
    /// ([`Session::graphics_adapter`]); the header creates the session's devices there.
    #[cfg(qaros_d3d11)]
    pub unsafe fn create_d3d11_render_sender(
        &self,
        config: &RenderSenderConfig,
        d3d11: crate::d3d11::D3D11StreamParams,
        options: &BlockingOptions<'_>,
    ) -> Result<RenderSender> {
        RenderSender::create(&self.inner, config, &SenderGraphics::D3D11(d3d11), options)
    }

    /// [`Session::start_render_sender`] with the D3D11 graphics API, the non-blocking form of
    /// [`Session::create_d3d11_render_sender`]. The device references stay with the creation:
    /// with the sender once taken; with the session when the [`PendingRenderSender`] is
    /// dropped first, until its callback comes; and for good when it never comes.
    ///
    /// # Safety
    ///
    /// As for [`Session::create_d3d11_render_sender`]. Parameters from
    /// [`crate::d3d11::D3D11StreamParams::from_raw`] must also stay valid for as long as the
    /// library may still create a sender on them: until this creation's sender is taken and
    /// its session dropped; if the [`PendingRenderSender`] is dropped first, until the session
    /// is dropped; and if that creation had not called back by then, until the process ends.
    #[cfg(qaros_d3d11)]
    pub unsafe fn start_d3d11_render_sender(
        &self,
        config: &RenderSenderConfig,
        d3d11: crate::d3d11::D3D11StreamParams,
        timeout: Option<Duration>,
        wake: impl Fn() + Send + Sync + 'static,
    ) -> Result<PendingRenderSender> {
        crate::pending_sender::start(
            &self.inner,
            config,
            &SenderGraphics::D3D11(d3d11),
            timeout,
            Box::new(wake),
        )
    }

    /// Destroys what async calls dropped before their callback delivered since: a render
    /// sender, an invite. Every `start_*` and every poll of a [`crate::PendingCall`] of this
    /// session do it too; call this regularly (once per frame) so such a sender does not stay
    /// connected to its target while no other call runs.
    pub fn release_abandoned_creations(&self) {
        self.inner.release_delivered_abandoned_calls();
    }

    /// The session's app volume operations.
    pub fn app_volumes(&self) -> AppVolumes<'_> {
        AppVolumes::new(&self.inner)
    }

    /// The session's GUI panel operations.
    pub fn gui_panels(&self) -> GuiPanels<'_> {
        GuiPanels::new(&self.inner)
    }

    /// Replaces the device this application describes itself as
    /// (`qar_peer_update_device_identity`), as [`crate::PeerPresentation::device_identity`] sets
    /// it at onboard or rejoin. A category other than the [`crate::SourceDeviceCategory`] values
    /// cannot be expressed; an empty device id fails with
    /// [`crate::StatusCode::ARGUMENT_NOT_SUPPORTED`].
    pub fn update_device_identity(&self, identity: &crate::DeviceIdentity) -> Result<()> {
        let storage = crate::device::DeviceIdentityStorage::new(identity)?;
        let api = self.inner.api();
        // SAFETY: live session; the identity and its strings live for the call.
        let result = unsafe {
            (api.qar_impl_peer_update_device_identity)(
                self.inner.handle.as_ptr(),
                &raw const storage.raw,
            )
        };
        check(api, "qar_peer_update_device_identity", result)
    }

    /// Starts [`Session::request_onboarding_invite`] without blocking
    /// (`qar_session_request_onboarding_invite_async`) and returns at once; the [`PendingCall`]
    /// delivers the invite. The tag is checked (and copied) before this returns: a refused tag
    /// is the error of this call, and no callback follows. `progress`, `timeout` and `wake` as
    /// for [`crate::Runtime::start_onboard`].
    pub fn start_onboarding_invite(
        &self,
        app_tag: &str,
        progress: Option<Sender<Progress>>,
        timeout: Option<Duration>,
        wake: impl Fn() + Send + Sync + 'static,
    ) -> Result<PendingCall<OnboardingInvite>> {
        const FUNCTION: &str = "qar_session_request_onboarding_invite_async";
        let app_tag_text = strings::to_c_string(app_tag, "app tag")?;
        let mut init = defaults::qar_request_invite_init_default();
        init.app_tag = app_tag_text.as_ptr();
        let library = Rc::clone(self.inner.library());
        let finish = move |outcome: RawOutcome| {
            let api = &library.api;
            if let Err(error) = check(api, FUNCTION, outcome.result) {
                release_invite(api, outcome.handle);
                return Err(error);
            }
            let handle = require_handle(outcome.handle as *mut sys::QarOnboardingInvite, FUNCTION)?;
            Ok(OnboardingInvite::from_handle(handle, &library))
        };
        let session = self.inner.handle.as_ptr();
        let api = self.inner.api();
        self.inner.start_call(
            FUNCTION,
            progress,
            timeout,
            Box::new(wake),
            None,
            Box::new(finish),
            Some(release_invite),
            |user_state, progress_callback, token| {
                // SAFETY: live session; init and its tag live for the start call, which copies
                // the tag; the token outlives the call.
                unsafe {
                    (api.qar_impl_session_request_onboarding_invite_async)(
                        session,
                        &raw const init,
                        Some(pending::handle_result_trampoline::<sys::QarOnboardingInvite>),
                        progress_callback,
                        user_state,
                        token,
                    )
                }
            },
        )
    }

    /// Asks this machine's hub to start the target app for one AR device
    /// (`qar_session_invite_target_app`) and returns its peer id once the target app's peer
    /// has joined the session. `connection` names the device kind and address:
    /// `"hololens:<host>"`, or a bare host, as `hololens:<host>`;
    /// [`crate::Library::target_app_connection_string`] builds it. An Android headset
    /// (`"headset:<host>"`, `"quest:"`, `"android:"`) is never invited: it pairs itself and joins
    /// the session on its own, and this call fails with
    /// [`crate::StatusCode::HUB_TARGET_APP_DEVICE_NOT_INVITABLE`] for it; find it with
    /// [`Session::find_render_target`].
    ///
    /// Only this machine's hub answers, and the invite is one-time: the hub remembers the device
    /// but never re-invites it by itself. A device another hub serves is stopped there first.
    /// Fails with [`crate::StatusCode::ARGUMENT_NOT_SUPPORTED`] for a malformed string, one
    /// without a host or of another kind (a `visualizer:` one, which
    /// [`Session::find_render_target`] resolves), and with
    /// [`crate::StatusCode::HUB_TARGET_APP_DEVICE_NOT_INVITABLE`] when the hub has no target app
    /// for the device family.
    pub fn invite_target_app(
        &self,
        connection: &str,
        options: &BlockingOptions<'_>,
    ) -> Result<PeerId> {
        let connection_text = strings::to_c_string(connection, "connection string")?;
        let mut init = defaults::qar_session_invite_target_app_init_default();
        init.connection_string = connection_text.as_ptr();
        let api = self.inner.api();
        let mut peer = defaults::qar_peer_id_default();
        // SAFETY: live session; init and its string live for the call; the progress state and
        // cancel token are borrowed from options for the call; valid out-pointer.
        let result = unsafe {
            (api.qar_impl_session_invite_target_app)(
                self.inner.handle.as_ptr(),
                &raw const init,
                options.progress_callback(),
                options.progress_state(),
                options.cancel_pointer(),
                &raw mut peer,
            )
        };
        check(api, "qar_session_invite_target_app", result)?;
        Ok(PeerId::from_raw(peer))
    }

    /// Starts [`Session::invite_target_app`] without blocking
    /// (`qar_session_invite_target_app_async`); the [`PendingCall`] delivers the target app's
    /// peer id. `progress`, `timeout` and `wake` as for [`crate::Runtime::start_onboard`].
    pub fn start_invite_target_app(
        &self,
        connection: &str,
        progress: Option<Sender<Progress>>,
        timeout: Option<Duration>,
        wake: impl Fn() + Send + Sync + 'static,
    ) -> Result<PendingCall<PeerId>> {
        const FUNCTION: &str = "qar_session_invite_target_app_async";
        let connection_text = strings::to_c_string(connection, "connection string")?;
        // The init and its string live in the call's state until the callback: the header does
        // not say the start copies them.
        let mut kept = Box::new(Keep((
            defaults::qar_session_invite_target_app_init_default(),
            connection_text,
        )));
        kept.0.0.connection_string = kept.0.1.as_ptr();
        let init: *const sys::QarSessionInviteTargetAppInit = &raw const kept.0.0;
        let library = Rc::clone(self.inner.library());
        let session = self.inner.handle.as_ptr();
        let api = self.inner.api();
        self.inner.start_call(
            FUNCTION,
            progress,
            timeout,
            Box::new(wake),
            Some(kept),
            Box::new(move |outcome: RawOutcome| {
                check(&library.api, FUNCTION, outcome.result)?;
                Ok(PeerId(outcome.id))
            }),
            None,
            |user_state, progress_callback, token| {
                // SAFETY: live session; init lives in the call's state until the callback; the
                // token outlives the call.
                unsafe {
                    (api.qar_impl_session_invite_target_app_async)(
                        session,
                        init,
                        Some(pending::peer_result_trampoline),
                        progress_callback,
                        user_state,
                        token,
                    )
                }
            },
        )
    }

    /// Asks this machine's hub to stop a target app it started (`qar_session_kill_target_app`):
    /// the target app and the whole pipeline feeding it. A target app this machine's hub did not
    /// start is served by no hub, so the call ends in [`crate::StatusCode::TIMEOUT`], not an
    /// authorization error.
    pub fn kill_target_app(&self, target_app: PeerId, options: &BlockingOptions<'_>) -> Result<()> {
        let peer = target_app.to_raw();
        let api = self.inner.api();
        // SAFETY: live session; the peer id lives for the call; the progress state and cancel
        // token are borrowed from options for the call.
        let result = unsafe {
            (api.qar_impl_session_kill_target_app)(
                self.inner.handle.as_ptr(),
                &raw const peer,
                options.progress_callback(),
                options.progress_state(),
                options.cancel_pointer(),
            )
        };
        check(api, "qar_session_kill_target_app", result)
    }

    /// Starts [`Session::kill_target_app`] without blocking
    /// (`qar_session_kill_target_app_async`). `progress`, `timeout` and `wake` as for
    /// [`crate::Runtime::start_onboard`].
    pub fn start_kill_target_app(
        &self,
        target_app: PeerId,
        progress: Option<Sender<Progress>>,
        timeout: Option<Duration>,
        wake: impl Fn() + Send + Sync + 'static,
    ) -> Result<PendingCall<()>> {
        const FUNCTION: &str = "qar_session_kill_target_app_async";
        let kept = Box::new(Keep(target_app.to_raw()));
        let peer: *const sys::QarPeerId = &raw const kept.0;
        let library = Rc::clone(self.inner.library());
        let session = self.inner.handle.as_ptr();
        let api = self.inner.api();
        self.inner.start_call(
            FUNCTION,
            progress,
            timeout,
            Box::new(wake),
            Some(kept),
            Box::new(move |outcome: RawOutcome| check(&library.api, FUNCTION, outcome.result)),
            None,
            |user_state, progress_callback, token| {
                // SAFETY: live session; the peer id lives in the call's state until the
                // callback; the token outlives the call.
                unsafe {
                    (api.qar_impl_session_kill_target_app_async)(
                        session,
                        peer,
                        Some(pending::peer_result_trampoline),
                        progress_callback,
                        user_state,
                        token,
                    )
                }
            },
        )
    }

    /// The peer id of an already-connected render target, from its connection string
    /// (`qar_session_find_render_target`), for example a `visualizer:` one.
    pub fn find_render_target(&self, connection: &str) -> Result<PeerId> {
        let connection_text = strings::to_c_string(connection, "connection string")?;
        let api = self.inner.api();
        let mut peer = defaults::qar_peer_id_default();
        // SAFETY: live session; the string lives for the call; valid out-pointer.
        let result = unsafe {
            (api.qar_impl_session_find_render_target)(
                self.inner.handle.as_ptr(),
                connection_text.as_ptr(),
                &raw mut peer,
            )
        };
        check(api, "qar_session_find_render_target", result)?;
        Ok(PeerId::from_raw(peer))
    }

    /// Mints a co-process invite (`qar_session_request_onboarding_invite`): a process this app
    /// starts onboards with it ([`crate::OnboardMethod::Invite`], with
    /// [`crate::RuntimeConfig::ephemeral_storage`]) and joins under the peer id the hub remembers
    /// for this app's approval and `app_tag`. `app_tag` is required, an identity key as for
    /// [`crate::OnboardMethod::LocalApp`]; give the co-process a tag of its own. Only a session
    /// that joined as a local app, or an app the QAROS launcher started, may request one; any
    /// other fails with [`crate::StatusCode::ONBOARDING_INVITE_REFUSED`], which is final. A tag
    /// that breaks the rules fails with [`crate::StatusCode::ARGUMENT_NOT_SUPPORTED`] before the
    /// hub is asked; [`crate::StatusCode::ONBOARDING_HUB_UNREACHABLE`] means the session names no
    /// hub to ask yet: retry. The invite is single use, valid for 60 s and redeemable only on the
    /// hub's PC.
    pub fn request_onboarding_invite(
        &self,
        app_tag: &str,
        options: &BlockingOptions<'_>,
    ) -> Result<OnboardingInvite> {
        OnboardingInvite::request(&self.inner, app_tag, options)
    }
}

impl SessionInner {
    pub(crate) fn api(&self) -> &QarApi {
        &self.runtime.library.api
    }

    pub(crate) fn library(&self) -> &Rc<crate::library::LibraryInner> {
        &self.runtime.library
    }

    /// Starts an async call owned by this session (see [`crate::PendingCall`]).
    #[allow(clippy::too_many_arguments)]
    fn start_call<T: 'static>(
        self: &Rc<Self>,
        function: &'static str,
        progress: Option<Sender<Progress>>,
        timeout: Option<Duration>,
        wake: Box<dyn Fn() + Send + Sync>,
        keep: Option<Box<dyn std::any::Any + Send + Sync>>,
        finish: Box<dyn FnOnce(RawOutcome) -> Result<T>>,
        destroy: Option<Destroy>,
        start_call: impl FnOnce(
            *mut c_void,
            sys::qar_progress_callback_t,
            *mut QarCancelToken,
        ) -> QarResult,
    ) -> Result<PendingCall<T>> {
        self.release_delivered_abandoned_calls();
        let abandon_session = Rc::clone(self);
        let poll_session = Rc::clone(self);
        // The finish closure keeps the session alive too, so a delivered handle is released
        // while it lives.
        let finish_session = Rc::clone(self);
        let spec = CallSpec {
            function,
            progress,
            timeout,
            wake,
            keep,
            finish: Box::new(move |outcome| {
                let value = finish(outcome);
                drop(finish_session);
                value
            }),
            abandon: Box::new(move |call| abandon_session.abandon_call(call)),
            before_poll: Box::new(move || poll_session.release_delivered_abandoned_calls()),
            destroy,
        };
        pending::start(self.library(), spec, start_call)
    }

    /// Destroys the handles that abandoned async calls delivered since the last call.
    pub(crate) fn release_delivered_abandoned_calls(&self) {
        let api = self.api();
        self.abandoned_calls
            .borrow_mut()
            .retain(|call| !call.release_if_delivered(api));
    }

    /// Keeps an async call dropped before its callback until the callback has come.
    pub(crate) fn abandon_call(&self, call: AbandonedCall) {
        self.abandoned_calls.borrow_mut().push(call);
    }

    /// Registers callback state and a cancel token, then subscribes. The
    /// state is owned by the session from here on.
    pub(crate) fn subscribe<Event: 'static>(
        &self,
        callback: EventCallback<Event>,
        function: &'static str,
        subscribe: impl FnOnce(&QarApi, *mut c_void, *mut QarCancelToken) -> QarResult,
    ) -> Result<Subscription> {
        let token = Rc::new(CancelTokenInner::create(self.library(), None)?);
        let slot = SubscriptionSlot {
            token: Rc::clone(&token),
            callback_state: Box::into_raw(Box::new(CallbackState {
                api: NonNull::from(self.api()),
                callback,
            }))
            .cast(),
            release_state: release_callback_state::<CallbackState<Event>>,
        };

        let result = subscribe(self.api(), slot.callback_state, token.handle.as_ptr());
        // On failure the library did not register the state; the slot frees it.
        check(self.api(), function, result)?;

        self.subscriptions.borrow_mut().push(slot);
        Ok(Subscription { token: Some(token) })
    }
}

impl Drop for SessionInner {
    fn drop(&mut self) {
        // Render senders and pending calls hold an Rc to this value, so only abandoned calls
        // can be left. What they delivered is destroyed while the session lives.
        let abandoned_calls = std::mem::take(self.abandoned_calls.get_mut());
        for call in &abandoned_calls {
            call.release_if_delivered(self.api());
        }
        let subscriptions = std::mem::take(self.subscriptions.get_mut());
        for slot in &subscriptions {
            if let Err(error) = slot.token.cancel() {
                log::warn!("cancelling a subscription during session teardown: {error}");
            }
        }
        // SAFETY: render senders hold an Rc to this value, so none is alive. Per the header,
        // the destroy ends the abandoned async calls still running with QAR_STATUS_TIMEOUT and
        // returns only after their callbacks have returned.
        unsafe { (self.api().qar_impl_session_handle_destroy)(self.handle.as_ptr()) };
        // Their tokens, init storage and device references are released only now, after the
        // callbacks; so are the subscriptions' callback state and tokens.
        for call in abandoned_calls {
            call.leak_late_delivery();
        }
        drop(subscriptions);
    }
}

/// An owned handle from a query or getter, destroyed on drop.
pub(crate) struct OwnedHandle<Handle> {
    handle: NonNull<Handle>,
    destroy: unsafe extern "C" fn(*mut Handle),
}

impl<Handle> OwnedHandle<Handle> {
    pub(crate) fn new(handle: NonNull<Handle>, destroy: unsafe extern "C" fn(*mut Handle)) -> Self {
        Self { handle, destroy }
    }

    pub(crate) fn as_ptr(&self) -> *mut Handle {
        self.handle.as_ptr()
    }
}

impl<Handle> Drop for OwnedHandle<Handle> {
    fn drop(&mut self) {
        // SAFETY: the handle was returned to us as owned and is destroyed once.
        unsafe { (self.destroy)(self.handle.as_ptr()) };
    }
}

/// Count-then-fetch enumeration of owned handles: reads each with
/// `read_item`, then destroys it.
pub(crate) fn query_owned_handles<Handle, Item>(
    api: &QarApi,
    query_count: impl FnOnce(*mut usize) -> QarResult,
    query_handles: impl FnOnce(*mut *mut Handle, usize, *mut usize) -> QarResult,
    destroy: unsafe extern "C" fn(*mut Handle),
    function: &'static str,
    mut read_item: impl FnMut(*mut Handle) -> Result<Item>,
) -> Result<Vec<Item>> {
    let mut count = 0_usize;
    check(api, function, query_count(&raw mut count))?;
    if count == 0 {
        return Ok(Vec::new());
    }
    let mut handles: Vec<*mut Handle> = vec![ptr::null_mut(); count];
    let mut written = 0_usize;
    let result = query_handles(handles.as_mut_ptr(), handles.len(), &raw mut written);
    // Handles written before an error are still ours to destroy.
    let owned: Vec<OwnedHandle<Handle>> = handles
        .into_iter()
        .take(written.min(count))
        .filter_map(NonNull::new)
        .map(|handle| OwnedHandle::new(handle, destroy))
        .collect();
    check(api, function, result)?;
    owned
        .iter()
        .map(|handle| read_item(handle.as_ptr()))
        .collect()
}

unsafe extern "C" fn peer_update_trampoline(
    handle: *mut sys::QarPeerSpec,
    user_state: *mut c_void,
) {
    // SAFETY: registered by subscribe_peer_updates with this event type.
    let state = unsafe { CallbackState::<PeerInfo>::from_user_state(user_state) };
    if handle.is_null() {
        return;
    }
    // The handle is borrowed for the duration of the callback: read, never destroy.
    match read_peer_info(state.api(), handle) {
        Ok(peer) => state.deliver(peer, "peer update"),
        Err(error) => log::warn!("reading a peer update: {error}"),
    }
}

unsafe extern "C" fn render_request_trampoline(
    request: *mut sys::QarRenderStreamRequest,
    user_state: *mut c_void,
) {
    // SAFETY: registered by subscribe_render_requests with this event type.
    let state = unsafe { CallbackState::<RenderRequest>::from_user_state(user_state) };
    if request.is_null() {
        return;
    }
    // Owned by the application: the header (`qar_render_sender_subscribe_requests`,
    // "Release each delivered request") and every sample destroy it here, which
    // takes precedence over api-conventions' general "borrowed" rule.
    let read = read_render_request(state.api(), request);
    // SAFETY: the request was delivered to this callback and is released exactly once,
    // after its getters ran.
    unsafe { (state.api().qar_impl_render_request_handle_destroy)(request) };
    match read {
        Ok(render_request) => state.deliver(render_request, "render request"),
        Err(error) => log::warn!("reading a render request: {error}"),
    }
}

unsafe extern "C" fn session_lifecycle_trampoline(
    info: *const sys::QarSessionLifecycleInfo,
    user_state: *mut c_void,
) {
    // SAFETY: registered by subscribe_lifecycle with this event type.
    let state = unsafe { CallbackState::<SessionLifecycle>::from_user_state(user_state) };
    // SAFETY: the library passes a valid info for the duration of the callback, or null.
    let Some(info) = (unsafe { info.as_ref() }) else {
        return;
    };
    state.deliver(SessionLifecycle::from_raw(info), "session lifecycle");
}

/// Destroys an invite an async invite request delivered that nobody takes.
fn release_invite(api: &QarApi, invite: usize) {
    if invite != 0 {
        // SAFETY: delivered to us and used by nobody.
        unsafe {
            (api.qar_impl_onboarding_invite_handle_destroy)(
                invite as *mut sys::QarOnboardingInvite,
            );
        };
    }
}
