use qaros_sys::{self as sys, QarApi, defaults};

use crate::device::PeerIdentity;
use crate::error::{Result, check};
use crate::strings;
use crate::types::{AppState, PeerId};

/// Snapshot of a `QarPeerSpec`.
#[derive(Clone, Debug, PartialEq)]
pub struct PeerInfo {
    pub id: PeerId,
    pub display_name: String,
    pub app_version: String,
    pub custom_info: String,
    /// `qar_peer_spec_get_app_state`. The public header deprecates it for the peer presence and
    /// derives it from the presence: an `ONLINE` peer, or one the hub has not reconciled
    /// (`UNKNOWN`), reads as what it claimed, which every current peer claims as
    /// [`AppState::Running`] once it joins; `OFFLINE` and `LEAVING` read as
    /// [`AppState::ShuttingDown`], and the peer may come back as `Running`; `RETIRED` reads as
    /// [`AppState::Retired`]. [`PeerInfo::identity`] carries the presence itself.
    pub app_state: AppState,
    pub version_id: String,
    pub room_tag: String,
    /// A device or viewer that shows rendered content (headset, Android
    /// streamer, visualizer): a peer to render for once it is
    /// [`AppState::Running`] (header "Basic flow", step 5).
    pub is_target_app: bool,
    /// The presence, role and device identity.
    pub identity: PeerIdentity,
}

pub(crate) fn read_peer_info(api: &QarApi, handle: *mut sys::QarPeerSpec) -> Result<PeerInfo> {
    let mut peer_id = defaults::qar_peer_id_default();
    // SAFETY: handle is live (owned or borrowed for the callback); valid out-pointer.
    let result = unsafe { (api.qar_impl_peer_spec_get_id)(handle, &raw mut peer_id) };
    check(api, "qar_peer_spec_get_id", result)?;

    let mut app_state = sys::QarAppState::QAR_APP_STATE_UNKNOWN;
    // SAFETY: as above.
    let result = unsafe { (api.qar_impl_peer_spec_get_app_state)(handle, &raw mut app_state) };
    check(api, "qar_peer_spec_get_app_state", result)?;

    let mut is_target_app = false;
    // SAFETY: as above.
    let result = unsafe { (api.qar_impl_peer_spec_is_target_app)(handle, &raw mut is_target_app) };
    check(api, "qar_peer_spec_is_target_app", result)?;

    let read_text = |getter: unsafe extern "C" fn(
        *mut sys::QarPeerSpec,
        *mut std::ffi::c_char,
        usize,
    ) -> sys::QarResult,
                     function: &'static str| {
        strings::read_string(|buffer, size| {
            // SAFETY: handle is live; the buffer is writable for size bytes.
            let result = unsafe { getter(handle, buffer, size) };
            check(api, function, result)
        })
    };

    Ok(PeerInfo {
        id: PeerId::from_raw(peer_id),
        display_name: read_text(
            api.qar_impl_peer_spec_get_display_name,
            "qar_peer_spec_get_display_name",
        )?,
        app_version: read_text(
            api.qar_impl_peer_spec_get_app_version,
            "qar_peer_spec_get_app_version",
        )?,
        custom_info: read_text(
            api.qar_impl_peer_spec_get_app_custom_peer_info,
            "qar_peer_spec_get_app_custom_peer_info",
        )?,
        app_state: AppState::from_raw(app_state),
        version_id: read_text(
            api.qar_impl_peer_spec_get_version_id,
            "qar_peer_spec_get_version_id",
        )?,
        room_tag: read_text(
            api.qar_impl_peer_spec_get_room_tag,
            "qar_peer_spec_get_room_tag",
        )?,
        is_target_app,
        // SAFETY: handle is live (owned or borrowed for the callback).
        identity: unsafe { crate::device::read_peer_identity(api, handle) }?,
    })
}
