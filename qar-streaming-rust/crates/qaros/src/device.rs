//! The device an application describes itself as (`QarPeerDeviceIdentity`).
//!
//! Only the public header declares the structure; where the bindings lack it (cfg
//! `qaros_device_identity` unset, as with the header of runtime 0.9.0-dev.2916), a set
//! [`crate::PeerPresentation::device_identity`] is refused with [`crate::Error::InvalidArgument`]
//! and `Session::update_device_identity` does not exist.

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
