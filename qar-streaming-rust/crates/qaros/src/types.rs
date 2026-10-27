//! Value types: identifiers, math, and the C enums the safe API uses.

use std::ffi::CStr;
use std::fmt;

use qaros_sys as sys;

macro_rules! define_id {
    ($(#[$documentation:meta])* $name:ident, $raw:ty) => {
        $(#[$documentation])*
        ///
        /// Formats as colon-separated hex bytes, like the C examples'
        /// `print_hex_id`. Persist onboarding ids with
        /// [`crate::Library::uuid_to_string`] instead, which uses the
        /// library's own UUID text form.
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
        pub struct $name(pub [u8; 16]);

        #[allow(dead_code)]
        impl $name {
            /// The all-zero id, which names nothing.
            pub const ZERO: Self = Self([0; 16]);

            /// Whether this is [`Self::ZERO`].
            pub fn is_zero(&self) -> bool {
                self.0 == [0; 16]
            }

            pub(crate) fn to_raw(self) -> $raw {
                <$raw>::from_bytes(self.0)
            }

            pub(crate) fn from_raw(raw: $raw) -> Self {
                Self(raw.data)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write_hex_id(formatter, &self.0)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "{}(", stringify!($name))?;
                write_hex_id(formatter, &self.0)?;
                formatter.write_str(")")
            }
        }
    };
}

fn write_hex_id(formatter: &mut fmt::Formatter<'_>, bytes: &[u8; 16]) -> fmt::Result {
    for (index, byte) in bytes.iter().enumerate() {
        if index > 0 {
            formatter.write_str(":")?;
        }
        write!(formatter, "{byte:02X}")?;
    }
    Ok(())
}

trait FromIdBytes {
    fn from_bytes(bytes: [u8; 16]) -> Self;
}

macro_rules! impl_from_id_bytes {
    ($($raw:ty),*) => {
        $(impl FromIdBytes for $raw {
            fn from_bytes(bytes: [u8; 16]) -> Self {
                Self { data: bytes }
            }
        })*
    };
}

impl_from_id_bytes!(
    sys::QarPeerId,
    sys::QarSessionId,
    sys::QarGuiPanelId,
    sys::QarAppVolumeId,
    sys::QarStreamId,
    sys::QarOnboardingId
);

define_id!(
    /// A participant in the session: a device or an application.
    PeerId,
    sys::QarPeerId
);
define_id!(SessionId, sys::QarSessionId);
define_id!(GuiPanelId, sys::QarGuiPanelId);
define_id!(AppVolumeId, sys::QarAppVolumeId);
define_id!(
    /// Correlates render stream requests.
    StreamId,
    sys::QarStreamId
);
define_id!(
    /// Key of a persisted identity slot. Persist it after onboarding; it is
    /// the ticket for every later rejoin or forget.
    OnboardingId,
    sys::QarOnboardingId
);

/// Position in meters. Room space is right-handed, Y up, minus Z forward
/// (OpenXR conventions).
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    /// A vector from its components.
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// Rotation quaternion, scalar last.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quat {
    /// No rotation.
    pub const IDENTITY: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };
}

impl Default for Quat {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// Orientation and position; the default is the identity pose, as
/// `qar_pose_default()`.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Pose {
    pub orientation: Quat,
    pub position: Vec3,
}

impl Pose {
    /// A pose at `position` with no rotation.
    pub const fn from_position(position: Vec3) -> Self {
        Self {
            orientation: Quat::IDENTITY,
            position,
        }
    }

    pub(crate) fn to_raw(self) -> sys::QarPose {
        sys::QarPose {
            orientation: sys::QarQuaternion {
                x: self.orientation.x,
                y: self.orientation.y,
                z: self.orientation.z,
                w: self.orientation.w,
            },
            position: sys::QarVector3 {
                x: self.position.x,
                y: self.position.y,
                z: self.position.z,
            },
        }
    }

    pub(crate) fn from_raw(raw: sys::QarPose) -> Self {
        Self {
            orientation: Quat {
                x: raw.orientation.x,
                y: raw.orientation.y,
                z: raw.orientation.z,
                w: raw.orientation.w,
            },
            position: Vec3 {
                x: raw.position.x,
                y: raw.position.y,
                z: raw.position.z,
            },
        }
    }
}

/// Field of view as four angles in radians, OpenXR style: `left` and `down`
/// are normally negative.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fov {
    pub angle_left: f32,
    pub angle_right: f32,
    pub angle_up: f32,
    pub angle_down: f32,
}

impl Fov {
    pub(crate) fn from_raw(raw: sys::QarFov) -> Self {
        Self {
            angle_left: raw.angle_left,
            angle_right: raw.angle_right,
            angle_up: raw.angle_up,
            angle_down: raw.angle_down,
        }
    }
}

/// Maps a newtype C enum to a Rust enum with an `Other` variant for values
/// this binding does not know, so values coming from the library never cause
/// undefined behaviour. Values only the public header declares follow
/// `by value:` with their number, so the same sources build against the
/// bindings of the header of runtime 0.9.0-dev.2916, which lacks them.
macro_rules! c_enum {
    (
        $(#[$documentation:meta])*
        $name:ident: $raw:ident {
            $($(#[$variant_documentation:meta])* $variant:ident = $constant:ident),* $(,)?
            $(; by value: $(
                $(#[$by_value_documentation:meta])* $by_value_variant:ident = $value:literal
            ),* $(,)?)?
        }
    ) => {
        $(#[$documentation])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum $name {
            $($(#[$variant_documentation])* $variant,)*
            $($($(#[$by_value_documentation])* $by_value_variant,)*)?
            /// A value not in the C header.
            Other(sys::$raw),
        }

        #[allow(dead_code)]
        impl $name {
            pub(crate) fn from_raw(raw: sys::$raw) -> Self {
                $(if raw == sys::$raw::$constant {
                    return Self::$variant;
                })*
                $($(if raw == sys::$raw($value) {
                    return Self::$by_value_variant;
                })*)?
                Self::Other(raw)
            }

            pub(crate) fn to_raw(self) -> sys::$raw {
                match self {
                    $(Self::$variant => sys::$raw::$constant,)*
                    $($(Self::$by_value_variant => sys::$raw($value),)*)?
                    Self::Other(raw) => raw,
                }
            }
        }
    };
}

c_enum!(
    /// Pixel format of a frame texture (`QarPixelFormat`).
    PixelFormat: QarPixelFormat {
        R32Float = QAR_PIXEL_FORMAT_R32_FLOAT,
        R8G8B8A8 = QAR_PIXEL_FORMAT_R8G8B8A8,
        B8G8R8A8 = QAR_PIXEL_FORMAT_B8G8R8A8,
        R16G16B16A16 = QAR_PIXEL_FORMAT_R16G16B16A16,
        B8G8R8X8 = QAR_PIXEL_FORMAT_B8G8R8X8,
        D32Float = QAR_PIXEL_FORMAT_D32_FLOAT;
        by value:
        /// 16-bit normalized depth, accepted only as a render sender's depth format; the sender
        /// widens it to 32-bit float before the frame leaves the application.
        D16Unorm = 100,
        /// Depth with stencil, accepted only as an OpenGL or Vulkan sender's depth format (the
        /// CPU and D3D11 senders refuse it); only the depth is streamed, as 32-bit float.
        D24UnormS8Uint = 102,
        /// 32-bit float depth with stencil, accepted as [`Self::D24UnormS8Uint`] is.
        D32FloatS8X24Uint = 103,
    }
);

c_enum!(
    /// How views pack into the frame's textures.
    FrameLayout: QarFrameLayout {
        SideBySide = QAR_FRAME_LAYOUT_SIDE_BY_SIDE,
        SeparatedTextures = QAR_FRAME_LAYOUT_SEPARATED_TEXTURES,
        Layered = QAR_FRAME_LAYOUT_LAYERED,
    }
);

c_enum!(
    /// What a frame view holds: color, grayscale or depth (`QarVideoFrameViewType`).
    ViewKind: QarVideoFrameViewType {
        Grayscale = QAR_VIDEO_FRAME_VIEW_TYPE_GRAYSCALE,
        Color = QAR_VIDEO_FRAME_VIEW_TYPE_COLOR,
        Depth = QAR_VIDEO_FRAME_VIEW_TYPE_DEPTH,
    }
);

c_enum!(
    /// The eye a frame view is for (`QarVideoFrameViewEye`).
    Eye: QarVideoFrameViewEye {
        None = QAR_VIDEO_FRAME_VIEW_EYE_NONE,
        Left = QAR_VIDEO_FRAME_VIEW_EYE_LEFT,
        Right = QAR_VIDEO_FRAME_VIEW_EYE_RIGHT,
    }
);

c_enum!(
    /// Lifecycle state of a peer's application (`QarAppState`), deprecated in the public header
    /// for the peer presence and computed from it there; see [`crate::PeerInfo::app_state`].
    AppState: QarAppState {
        Unknown = QAR_APP_STATE_UNKNOWN,
        Initializing = QAR_APP_STATE_INITIALIZING,
        Running = QAR_APP_STATE_RUNNING,
        ShuttingDown = QAR_APP_STATE_SHUTTING_DOWN,
        /// Left for good: never comes back under its id.
        Retired = QAR_APP_STATE_RETIRED,
    }
);

c_enum!(
    /// Visibility of a GUI panel (`QarGuiPanelState`).
    GuiPanelState: QarGuiPanelState {
        Visible = QAR_GUI_PANEL_STATE_VISIBLE,
        Minimized = QAR_GUI_PANEL_STATE_MINIMIZED,
        Hidden = QAR_GUI_PANEL_STATE_HIDDEN,
        Closed = QAR_GUI_PANEL_STATE_CLOSED,
    }
);

c_enum!(
    /// Whether an app volume is active or closed (`QarAppVolumeLifetimeStatus`).
    AppVolumeLifetime: QarAppVolumeLifetimeStatus {
        Active = QAR_APP_VOLUME_ACTIVE,
        Closed = QAR_APP_VOLUME_CLOSED,
    }
);

c_enum!(
    /// Severity of a library log message (`QarLogSeverity`).
    LogSeverity: QarLogSeverity {
        Trace = QAR_LOG_SEVERITY_TRACE,
        Debug = QAR_LOG_SEVERITY_DEBUG,
        Info = QAR_LOG_SEVERITY_INFO,
        Warn = QAR_LOG_SEVERITY_WARN,
        Error = QAR_LOG_SEVERITY_ERROR,
        Fatal = QAR_LOG_SEVERITY_FATAL,
    }
);

c_enum!(
    /// Severity of a progress callback message (`QarActionSeverity`).
    ActionSeverity: QarActionSeverity {
        Info = QAR_ACTION_SEVERITY_INFO,
        Warning = QAR_ACTION_SEVERITY_WARNING,
        Error = QAR_ACTION_SEVERITY_ERROR,
        Done = QAR_ACTION_SEVERITY_DONE,
    }
);

c_enum!(
    /// How an onboarding invite authenticates (`QarOnboardingMethod`).
    OnboardingMethod: QarOnboardingMethod {
        Pake = QAR_ONBOARDING_METHOD_PAKE,
        Password = QAR_ONBOARDING_METHOD_PASSWORD,
    }
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_depth_formats_named_by_value_carry_the_public_headers_values() {
        for (format, value) in [
            (PixelFormat::D16Unorm, 100),
            (PixelFormat::D32Float, 101),
            (PixelFormat::D24UnormS8Uint, 102),
            (PixelFormat::D32FloatS8X24Uint, 103),
        ] {
            assert_eq!(format.to_raw(), sys::QarPixelFormat(value));
            assert_eq!(PixelFormat::from_raw(sys::QarPixelFormat(value)), format);
        }
        assert_eq!(
            PixelFormat::from_raw(sys::QarPixelFormat(104)),
            PixelFormat::Other(sys::QarPixelFormat(104))
        );
    }

    #[test]
    fn a_retired_peer_reads_as_retired_and_back() {
        let raw = sys::QarAppState::QAR_APP_STATE_RETIRED;
        assert_eq!(AppState::from_raw(raw), AppState::Retired);
        assert_eq!(AppState::Retired.to_raw(), raw);
        assert_eq!(format!("{:?}", AppState::from_raw(raw)), "Retired");
    }
}

c_enum!(
    /// Where a session stands in its hub's session lifecycle (`QarSessionLifecycleState`).
    /// Treat [`Self::Other`] as [`Self::HubUnreachable`]: the session keeps trying.
    SessionLifecycleState: QarSessionLifecycleState {
        /// In the hub's current session; traffic flows.
        Connected = QAR_SESSION_LIFECYCLE_STATE_CONNECTED,
        /// Moving into another session in place; traffic resumes once connected.
        Migrating = QAR_SESSION_LIFECYCLE_STATE_MIGRATING,
        /// The hub has not answered for a while; the session keeps trying by itself.
        HubUnreachable = QAR_SESSION_LIFECYCLE_STATE_HUB_UNREACHABLE,
        /// The hub no longer admits this peer (its approval was revoked, it missed too many
        /// rotations, or the hub was reset). Terminal: drop the session and onboard again.
        RePairRequired = QAR_SESSION_LIFECYCLE_STATE_RE_PAIR_REQUIRED,
    }
);

c_enum!(
    /// Why the session moved into another session id (`QarSessionChangeCause`). Informational
    /// only; treat [`Self::Other`] as [`Self::Rotated`].
    SessionChangeCause: QarSessionChangeCause {
        /// No move: a state change within the same session.
        None = QAR_SESSION_CHANGE_CAUSE_NONE,
        /// The hub serves a new session of its own.
        Rotated = QAR_SESSION_CHANGE_CAUSE_ROTATED,
        /// The hub this app paired with joined another hub as its member.
        JoinedPrimary = QAR_SESSION_CHANGE_CAUSE_JOINED_PRIMARY,
    }
);

/// One state of a session's lifecycle (`QarSessionLifecycleInfo`), copied out of the library.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionLifecycle {
    pub state: SessionLifecycleState,
    /// Why the latest move happened; [`SessionChangeCause::None`] outside a move.
    pub cause: SessionChangeCause,
    /// The session the app is in now: during [`SessionLifecycleState::Migrating`] still the old
    /// one. Compare it with the id the app last saw to tell a new move from an old one.
    pub session_id: SessionId,
    /// The session it moved from, on the state that ends a move.
    pub previous_session_id: Option<SessionId>,
    /// Text for logs and the operator, never a secret. Empty from
    /// [`crate::Session::lifecycle`].
    pub reason: String,
}

impl SessionLifecycle {
    pub(crate) fn from_raw(info: &sys::QarSessionLifecycleInfo) -> Self {
        let reason = if info.reason.is_null() {
            String::new()
        } else {
            // SAFETY: non-null and NUL-terminated; valid during the callback that delivered it.
            unsafe { CStr::from_ptr(info.reason) }
                .to_string_lossy()
                .into_owned()
        };
        Self {
            state: SessionLifecycleState::from_raw(info.state),
            cause: SessionChangeCause::from_raw(info.cause),
            session_id: SessionId::from_raw(info.session_id),
            previous_session_id: info
                .has_previous_session
                .then(|| SessionId::from_raw(info.previous_session_id)),
            reason,
        }
    }
}
