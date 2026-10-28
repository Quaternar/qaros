//! The device an application describes itself as (`QarPeerDeviceIdentity`), and the presence,
//! role and device identity read back from a peer ([`PeerIdentity`]).
//!
//! Only the public header declares these structures and functions; where the bindings lack them
//! (cfg `qaros_device_identity` unset, as with the header of runtime 0.9.0-dev.2916), a set
//! [`crate::PeerPresentation::device_identity`] is refused with [`crate::Error::InvalidArgument`],
//! `Session::update_device_identity` does not exist and [`crate::PeerInfo::identity`] is `None`.

#[cfg(qaros_device_identity)]
use std::ffi::CString;

#[cfg(qaros_device_identity)]
use qaros_sys::{self as sys, defaults};

#[cfg(qaros_device_identity)]
use crate::error::Result;
#[cfg(qaros_device_identity)]
use crate::strings;

/// The device categories an application may declare (`QarDeviceCategory`). An application
/// always joins as a source; the C API refuses every other category with
/// `QAR_STATUS_PEER_DEVICE_CATEGORY_NOT_ALLOWED`, so this enum has none of them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SourceDeviceCategory {
    /// A desktop or laptop PC (`QAR_DEVICE_CATEGORY_DESKTOP_PC`), every source app's default.
    #[default]
    DesktopPc,
    /// A server machine (`QAR_DEVICE_CATEGORY_SERVER`).
    Server,
    /// A camera of any model, its capture process (`QAR_DEVICE_CATEGORY_CAMERA`).
    Camera,
}

impl SourceDeviceCategory {
    #[cfg(qaros_device_identity)]
    pub(crate) fn to_raw(self) -> sys::QarDeviceCategory {
        match self {
            Self::DesktopPc => sys::QarDeviceCategory::QAR_DEVICE_CATEGORY_DESKTOP_PC,
            Self::Server => sys::QarDeviceCategory::QAR_DEVICE_CATEGORY_SERVER,
            Self::Camera => sys::QarDeviceCategory::QAR_DEVICE_CATEGORY_CAMERA,
        }
    }
}

/// The device an application describes itself as (`QarPeerDeviceIdentity`). Without one it
/// joins as a desktop PC with the fixed id of the host OS (`DesktopWindows` / `DesktopLinux`).
///
/// Ids and families are open UTF-8 strings (`"Zed2i"`, `"Zed"`), matched by viewers against the
/// hub's device catalog. An id the catalog does not know is fine: viewers fall back to the family,
/// then to the category, and show `device_name`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DeviceIdentity {
    pub category: SourceDeviceCategory,
    /// Required and non-empty, e.g. `"LuxonisOakD"`; an empty id fails with
    /// [`crate::StatusCode::ARGUMENT_NOT_SUPPORTED`].
    pub device_id: String,
    /// Optional, e.g. `"Luxonis"`.
    pub device_family: Option<String>,
    /// Optional, e.g. `"OAK-D"`.
    pub device_name: Option<String>,
}

/// A [`DeviceIdentity`] as C strings and its stamped structure, which borrows them. The owner
/// keeps it at a stable address (inside a `Box`) while the library may read it.
#[cfg(qaros_device_identity)]
#[derive(Debug)]
pub(crate) struct DeviceIdentityStorage {
    pub(crate) raw: sys::QarPeerDeviceIdentity,
    _device_id: CString,
    _device_family: Option<CString>,
    _device_name: Option<CString>,
}

#[cfg(qaros_device_identity)]
impl DeviceIdentityStorage {
    pub(crate) fn new(identity: &DeviceIdentity) -> Result<Self> {
        let device_id = strings::to_c_string(&identity.device_id, "device id")?;
        let device_family =
            strings::optional_c_string(identity.device_family.as_deref(), "device family")?;
        let device_name =
            strings::optional_c_string(identity.device_name.as_deref(), "device name")?;
        let mut raw = defaults::qar_peer_device_identity_default();
        raw.category = identity.category.to_raw();
        // The CStrings' buffers are on the heap: moving them keeps these pointers valid.
        raw.device_id = device_id.as_ptr();
        raw.device_family = strings::pointer_or_null(device_family.as_ref());
        raw.device_name = strings::pointer_or_null(device_name.as_ref());
        Ok(Self {
            raw,
            _device_id: device_id,
            _device_family: device_family,
            _device_name: device_name,
        })
    }
}

/// Where a peer is (`QarPeerPresence`). Render for [`PeerPresence::Online`] peers and
/// [`PeerPresence::Unknown`] ones; stop at the others. A value the header does not name reads as
/// [`PeerPresence::Unknown`], as the header says to treat it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PeerPresence {
    /// The hub has not reconciled the peer yet; nothing says it is gone: treat it like online.
    #[default]
    Unknown,
    /// The hub sees the peer in the session.
    Online,
    /// The hub no longer sees it (a crash, a kill, a network drop); it reads online once back.
    Offline,
    /// The peer announced its leave; it may come back under the same id.
    Leaving,
    /// The peer left for good and never comes back under its id.
    Retired,
}

/// What a peer is in the streaming graph (`QarPeerRole`). A source renders for the
/// [`PeerRole::Target`] peers. A value the header does not name is [`PeerRole::Other`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PeerRole {
    /// QAROS pipeline infrastructure the hub launched itself.
    Internal,
    /// A source application or a camera capture process (every application using this API).
    Source,
    /// Shows content: a headset's serving app, a Visualizer receiver, a camera's receiver slot.
    Target,
    /// A QAROS runtime (hub) process.
    Runtime,
    /// The operator's Visualizer control session.
    Control,
    /// A value the header does not name.
    Other(i64),
}

/// What kind of device a peer is (`QarDeviceCategory`): the fallback when a viewer does not know
/// the peer's device id or family. A value the header does not name reads as
/// [`DeviceCategory::Unknown`], as the header says to treat it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DeviceCategory {
    /// Infrastructure, runtime, control, test targets and unrecognised devices.
    #[default]
    Unknown,
    DesktopPc,
    Server,
    /// A headset or glasses of any model.
    Headset,
    /// A Visualizer receiver showing streams on the PC.
    Visualizer,
    /// A camera of any model, its capture process.
    Camera,
    /// A target app that sees the scene through a camera.
    Observer,
}

/// A peer's presence, role and device identity (`qar_peer_spec_get_presence`, `_get_role`,
/// `_get_category`, `_get_device_id`, `_get_device_family`, `_get_device_name`), read with the
/// rest of [`crate::PeerInfo`]. The device id and name can change while the peer runs (a serving
/// app refines them once the device reports its model): re-read them in a peer update.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeerIdentity {
    pub presence: PeerPresence,
    pub role: PeerRole,
    pub category: DeviceCategory,
    /// An open UTF-8 id of the exact device or app (`"MetaQuest3"`, `"DesktopWindows"`).
    pub device_id: String,
    /// The device family (`"MetaQuest"`), the fallback for an unknown id; empty for none.
    pub device_family: String,
    /// The device's human name (`"Meta Quest 3"`); empty when nothing better than the id is
    /// known.
    pub device_name: String,
}

#[cfg(qaros_device_identity)]
impl PeerPresence {
    pub(crate) fn from_raw(raw: sys::QarPeerPresence) -> Self {
        match raw {
            sys::QarPeerPresence::QAR_PEER_PRESENCE_ONLINE => Self::Online,
            sys::QarPeerPresence::QAR_PEER_PRESENCE_OFFLINE => Self::Offline,
            sys::QarPeerPresence::QAR_PEER_PRESENCE_LEAVING => Self::Leaving,
            sys::QarPeerPresence::QAR_PEER_PRESENCE_RETIRED => Self::Retired,
            _ => Self::Unknown,
        }
    }
}

#[cfg(qaros_device_identity)]
impl PeerRole {
    pub(crate) fn from_raw(raw: sys::QarPeerRole) -> Self {
        match raw {
            sys::QarPeerRole::QAR_PEER_ROLE_INTERNAL => Self::Internal,
            sys::QarPeerRole::QAR_PEER_ROLE_SOURCE => Self::Source,
            sys::QarPeerRole::QAR_PEER_ROLE_TARGET => Self::Target,
            sys::QarPeerRole::QAR_PEER_ROLE_RUNTIME => Self::Runtime,
            sys::QarPeerRole::QAR_PEER_ROLE_CONTROL => Self::Control,
            other => Self::Other(i64::from(other.0)),
        }
    }
}

#[cfg(qaros_device_identity)]
impl DeviceCategory {
    pub(crate) fn from_raw(raw: sys::QarDeviceCategory) -> Self {
        match raw {
            sys::QarDeviceCategory::QAR_DEVICE_CATEGORY_DESKTOP_PC => Self::DesktopPc,
            sys::QarDeviceCategory::QAR_DEVICE_CATEGORY_SERVER => Self::Server,
            sys::QarDeviceCategory::QAR_DEVICE_CATEGORY_HEADSET => Self::Headset,
            sys::QarDeviceCategory::QAR_DEVICE_CATEGORY_VISUALIZER => Self::Visualizer,
            sys::QarDeviceCategory::QAR_DEVICE_CATEGORY_CAMERA => Self::Camera,
            sys::QarDeviceCategory::QAR_DEVICE_CATEGORY_OBSERVER => Self::Observer,
            _ => Self::Unknown,
        }
    }
}

/// Reads a peer spec's identity.
///
/// # Safety
///
/// `handle` is a live peer spec handle (owned, or borrowed for a callback).
#[cfg(qaros_device_identity)]
pub(crate) unsafe fn read_peer_identity(
    api: &sys::QarApi,
    handle: *mut sys::QarPeerSpec,
) -> Result<PeerIdentity> {
    use crate::error::check;

    let mut presence = sys::QarPeerPresence::QAR_PEER_PRESENCE_UNKNOWN;
    // SAFETY: forwarded to the caller; valid out-pointer.
    let result = unsafe { (api.qar_impl_peer_spec_get_presence)(handle, &raw mut presence) };
    check(api, "qar_peer_spec_get_presence", result)?;
    let mut role = sys::QarPeerRole::QAR_PEER_ROLE_INTERNAL;
    // SAFETY: as above.
    let result = unsafe { (api.qar_impl_peer_spec_get_role)(handle, &raw mut role) };
    check(api, "qar_peer_spec_get_role", result)?;
    let mut category = sys::QarDeviceCategory::QAR_DEVICE_CATEGORY_UNKNOWN;
    // SAFETY: as above.
    let result = unsafe { (api.qar_impl_peer_spec_get_category)(handle, &raw mut category) };
    check(api, "qar_peer_spec_get_category", result)?;
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
    Ok(PeerIdentity {
        presence: PeerPresence::from_raw(presence),
        role: PeerRole::from_raw(role),
        category: DeviceCategory::from_raw(category),
        device_id: read_text(
            api.qar_impl_peer_spec_get_device_id,
            "qar_peer_spec_get_device_id",
        )?,
        device_family: read_text(
            api.qar_impl_peer_spec_get_device_family,
            "qar_peer_spec_get_device_family",
        )?,
        device_name: read_text(
            api.qar_impl_peer_spec_get_device_name,
            "qar_peer_spec_get_device_name",
        )?,
    })
}
