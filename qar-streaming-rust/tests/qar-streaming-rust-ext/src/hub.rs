use std::ffi::{CString, c_char};
use std::ptr::{self, NonNull};
use std::rc::Rc;
use std::sync::Arc;

use qaros_sys::{QarPeerId, QarVideoFrameLayout, defaults};

use crate::ffi::{ExtHub, ExtHubInit, ExtInviteMethod, ExtMediaStreamRecord, ExtPeer, ExtPeerInit};
use crate::loader::text_from_buffer;
use crate::receiver::RenderReceiver;
use crate::{Error, ExtApi, Result, VulkanDevice, stamped_header, structure_types};

/// How a minted invite authenticates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InviteMethod {
    /// Single-use PAKE code served by the hub's onboarding service.
    Pake,
    /// Persistent shared password (hub created with password onboarding).
    Password,
}

const NAME_BUFFER_SIZE: usize = 1024;
const INVITE_BUFFER_SIZE: usize = 16 * 1024;

/// An in-process hub: an authority session with its onboarding servers and
/// no stream register. Peers keep it alive, so it is destroyed after them.
#[derive(Debug)]
pub struct Hub {
    inner: Rc<HubInner>,
}

#[derive(Debug)]
pub(crate) struct HubInner {
    handle: NonNull<ExtHub>,
    pub(crate) api: Arc<ExtApi>,
}

impl Hub {
    pub(crate) fn create(api: &Arc<ExtApi>, enable_password_onboarding: bool) -> Result<Self> {
        let init = ExtHubInit {
            header: stamped_header(structure_types::HUB_INIT),
            enable_password_onboarding,
        };
        let mut handle: *mut ExtHub = ptr::null_mut();
        // SAFETY: init is stamped; valid out-pointer.
        let result = unsafe { (api.qar_impl_ext_hub_create)(&raw const init, &raw mut handle) };
        api.check("qar_ext_hub_create", result)?;
        let handle = NonNull::new(handle).ok_or(Error::MissingHandle("qar_ext_hub_create"))?;
        Ok(Self {
            inner: Rc::new(HubInner {
                handle,
                api: Arc::clone(api),
            }),
        })
    }

    /// The hub session's own peer id.
    pub fn peer_id(&self) -> Result<QarPeerId> {
        let api = &self.inner.api;
        let mut peer_id = defaults::qar_peer_id_default();
        // SAFETY: live hub; valid out-pointer.
        let result = unsafe {
            (api.qar_impl_ext_hub_get_peer_id)(self.inner.handle.as_ptr(), &raw mut peer_id)
        };
        api.check("qar_ext_hub_get_peer_id", result)?;
        Ok(peer_id)
    }

    /// Mints a fresh invite as the wire string
    /// `qar_onboarding_invite_deserialize` accepts. A too-small buffer
    /// discards the minted invite, so a generous buffer is tried first and
    /// the reported size only on a retry.
    pub fn mint_invite(&self, method: InviteMethod) -> Result<String> {
        let api = &self.inner.api;
        let raw_method = match method {
            InviteMethod::Pake => ExtInviteMethod::PAKE,
            InviteMethod::Password => ExtInviteMethod::PASSWORD,
        };
        let mint = |buffer: &mut Vec<c_char>, required_size: &mut usize| {
            // SAFETY: live hub; the buffer is writable for its length.
            unsafe {
                (api.qar_impl_ext_hub_mint_invite)(
                    self.inner.handle.as_ptr(),
                    raw_method,
                    buffer.as_mut_ptr(),
                    buffer.len(),
                    required_size,
                )
            }
        };
        let mut buffer = vec![0 as c_char; INVITE_BUFFER_SIZE];
        let mut required_size = 0_usize;
        let first_result = mint(&mut buffer, &mut required_size);
        if api.check("qar_ext_hub_mint_invite", first_result).is_err()
            && required_size >= buffer.len()
        {
            buffer = vec![0 as c_char; required_size + 1];
            let retried_result = mint(&mut buffer, &mut required_size);
            api.check("qar_ext_hub_mint_invite", retried_result)?;
        } else {
            api.check("qar_ext_hub_mint_invite", first_result)?;
        }
        Ok(text_from_buffer(&buffer))
    }

    /// The display name the hub lists for `peer_id`, `None` when the hub
    /// does not list it.
    pub fn find_peer_display_name(&self, peer_id: &QarPeerId) -> Result<Option<String>> {
        let api = &self.inner.api;
        let mut buffer = vec![0 as c_char; NAME_BUFFER_SIZE];
        let mut required_size = 0_usize;
        let mut found = false;
        // SAFETY: live hub; the buffer is writable for its length.
        let result = unsafe {
            (api.qar_impl_ext_hub_find_peer_display_name)(
                self.inner.handle.as_ptr(),
                peer_id,
                buffer.as_mut_ptr(),
                buffer.len(),
                &raw mut required_size,
                &raw mut found,
            )
        };
        api.check("qar_ext_hub_find_peer_display_name", result)?;
        Ok(found.then(|| text_from_buffer(&buffer)))
    }

    /// The session's media-stream records as the hub sees them.
    pub fn media_streams(&self) -> Result<Vec<ExtMediaStreamRecord>> {
        let api = &self.inner.api;
        let hub = self.inner.handle.as_ptr();
        query_records(
            api,
            "qar_ext_hub_query_media_streams",
            |records, capacity, count| {
                // SAFETY: live hub; the record buffer holds capacity entries.
                unsafe { (api.qar_impl_ext_hub_query_media_streams)(hub, records, capacity, count) }
            },
        )
    }

    /// Joins the hub's session as a new in-process visualizer peer.
    pub fn create_peer(&self, display_name: Option<&str>) -> Result<Peer> {
        Peer::create(&self.inner, display_name)
    }
}

impl Drop for HubInner {
    fn drop(&mut self) {
        // SAFETY: every peer holds an Rc to this value, so none is alive.
        unsafe { (self.api.qar_impl_ext_hub_handle_destroy)(self.handle.as_ptr()) };
    }
}

/// An in-process peer in a [`Hub`]'s session: the counterpart that receives
/// what the application under test sends. Receivers keep it alive.
#[derive(Debug)]
pub struct Peer {
    pub(crate) inner: Rc<PeerInner>,
}

#[derive(Debug)]
pub(crate) struct PeerInner {
    pub(crate) handle: NonNull<ExtPeer>,
    pub(crate) hub: Rc<HubInner>,
}

impl Peer {
    fn create(hub: &Rc<HubInner>, display_name: Option<&str>) -> Result<Self> {
        let api = &hub.api;
        let name_text = display_name
            .map(CString::new)
            .transpose()
            .map_err(|_| Error::InvalidArgument("display name contains a NUL byte".to_string()))?;
        let init = ExtPeerInit {
            header: stamped_header(structure_types::PEER_INIT),
            display_name: name_text.as_ref().map_or(ptr::null(), |text| text.as_ptr()),
        };
        let mut handle: *mut ExtPeer = ptr::null_mut();
        // SAFETY: live hub; init is stamped and its string outlives the call.
        let result = unsafe {
            (api.qar_impl_ext_peer_create)(hub.handle.as_ptr(), &raw const init, &raw mut handle)
        };
        api.check("qar_ext_peer_create", result)?;
        let handle = NonNull::new(handle).ok_or(Error::MissingHandle("qar_ext_peer_create"))?;
        Ok(Self {
            inner: Rc::new(PeerInner {
                handle,
                hub: Rc::clone(hub),
            }),
        })
    }

    pub fn id(&self) -> Result<QarPeerId> {
        let api = &self.inner.hub.api;
        let mut peer_id = defaults::qar_peer_id_default();
        // SAFETY: live peer; valid out-pointer.
        let result =
            unsafe { (api.qar_impl_ext_peer_get_id)(self.inner.handle.as_ptr(), &raw mut peer_id) };
        api.check("qar_ext_peer_get_id", result)?;
        Ok(peer_id)
    }

    /// A receiver for a stream rendered for this peer; it connects in the
    /// background once a sender answers. CPU frames without `vulkan`, Vulkan
    /// frames on that device with it.
    pub fn create_render_receiver(
        &self,
        layout: &QarVideoFrameLayout,
        vulkan: Option<&VulkanDevice>,
    ) -> Result<RenderReceiver> {
        RenderReceiver::create(&self.inner, layout, vulkan)
    }
}

impl Drop for PeerInner {
    fn drop(&mut self) {
        // SAFETY: every receiver holds an Rc to this value, so none is alive.
        unsafe { (self.hub.api.qar_impl_ext_peer_handle_destroy)(self.handle.as_ptr()) };
    }
}

fn query_records(
    api: &ExtApi,
    function: &'static str,
    query: impl Fn(*mut ExtMediaStreamRecord, usize, *mut usize) -> qaros_sys::QarResult,
) -> Result<Vec<ExtMediaStreamRecord>> {
    let mut count = 0_usize;
    api.check(function, query(ptr::null_mut(), 0, &raw mut count))?;
    if count == 0 {
        return Ok(Vec::new());
    }
    let mut records: Vec<std::mem::MaybeUninit<ExtMediaStreamRecord>> = (0..count)
        .map(|_| std::mem::MaybeUninit::zeroed())
        .collect();
    let mut written = 0_usize;
    api.check(
        function,
        query(records.as_mut_ptr().cast(), records.len(), &raw mut written),
    )?;
    Ok(records
        .into_iter()
        .take(written.min(count))
        // SAFETY: the library wrote the first `written` records; the rest
        // are not read.
        .map(|record| unsafe { record.assume_init() })
        .collect())
}
