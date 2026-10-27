use std::ffi::c_char;
use std::fmt;

use qaros_sys::{QarApi, QarResult, QarStatusCode};

/// A `QarStatusCode` as returned by the C API.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct StatusCode(QarStatusCode);

impl StatusCode {
    pub const SUCCESS: Self = Self(QarStatusCode::QAR_STATUS_SUCCESS);
    pub const UNCLASSIFIED: Self = Self(QarStatusCode::QAR_STATUS_UNCLASSIFIED);
    pub const NOT_IMPLEMENTED: Self = Self(QarStatusCode::QAR_STATUS_NOT_IMPLEMENTED);
    pub const ARGUMENT_NOT_SUPPORTED: Self = Self(QarStatusCode::QAR_STATUS_ARGUMENT_NOT_SUPPORTED);
    pub const TIMEOUT: Self = Self(QarStatusCode::QAR_STATUS_TIMEOUT);
    pub const LOGIC_ERROR: Self = Self(QarStatusCode::QAR_STATUS_LOGIC_ERROR);
    pub const GUI_PANEL_INVALID_ID: Self = Self(QarStatusCode::QAR_STATUS_GUI_PANEL_INVALID_ID);
    pub const APP_VOLUME_INVALID_ID: Self = Self(QarStatusCode::QAR_STATUS_APP_VOLUME_INVALID_ID);
    pub const RENDERING_UNABLE_TO_BEGIN_FRAME: Self =
        Self(QarStatusCode::QAR_STATUS_RENDERING_PRODUCER_UNABLE_TO_DO_BEGIN_FRAME);
    pub const RENDERING_STREAM_IS_CLOSED: Self =
        Self(QarStatusCode::QAR_STATUS_RENDERING_PRODUCER_STREAM_IS_CLOSED);
    /// The receiver paused the stream. Not a failure; a render sender's
    /// begin-frame waits through a pause instead of reporting it.
    pub const MEDIA_STREAMING_STREAM_IS_PAUSED: Self =
        Self(QarStatusCode::QAR_STATUS_MEDIA_STREAMING_STREAM_IS_PAUSED);
    pub const MEDIA_STREAMING_STREAM_IS_RECONNECTING: Self =
        Self(QarStatusCode::QAR_STATUS_MEDIA_STREAMING_STREAM_IS_RECONNECTING);
    pub const MEDIA_STREAMING_NO_ACTIVE_TRANSFERS: Self =
        Self(QarStatusCode::QAR_STATUS_MEDIA_STREAMING_NO_ACTIVE_TRANSFERS);
    /// The sender's app volume is not known (yet) where the receiver runs. Recoverable: keep
    /// calling begin-frame; a volume that was removed never resolves.
    pub const RENDERING_APP_VOLUME_UNRESOLVED: Self =
        Self(QarStatusCode::QAR_STATUS_RENDERING_PRODUCER_APP_VOLUME_UNRESOLVED);
    /// The render sender's depth format is one its graphics API cannot take; returned by the
    /// creation before any receiver is waited for, with the accepted formats in the message.
    /// Named by value, since only the public header declares it.
    pub const RENDERING_DEPTH_FORMAT_NOT_SUPPORTED: Self = Self(QarStatusCode(806));
    pub const PAKE_ERROR: Self = Self(QarStatusCode::QAR_STATUS_PAKE_ERROR);
    pub const ONBOARDING_SESSION_NOT_FOUND: Self =
        Self(QarStatusCode::QAR_STATUS_ONBOARDING_SESSION_NOT_FOUND);
    pub const ONBOARDING_FAILED: Self = Self(QarStatusCode::QAR_STATUS_ONBOARDING_FAILED);
    pub const ONBOARDING_HUB_UNREACHABLE: Self =
        Self(QarStatusCode::QAR_STATUS_ONBOARDING_HUB_UNREACHABLE);
    pub const ONBOARDING_SESSION_STILL_ACTIVE: Self =
        Self(QarStatusCode::QAR_STATUS_ONBOARDING_SESSION_STILL_ACTIVE);
    pub const ONBOARDING_APPROVAL_DENIED: Self =
        Self(QarStatusCode::QAR_STATUS_ONBOARDING_APPROVAL_DENIED);
    pub const ONBOARDING_APPROVAL_TIMEOUT: Self =
        Self(QarStatusCode::QAR_STATUS_ONBOARDING_APPROVAL_TIMEOUT);
    pub const ONBOARDING_HUB_NOT_AUTHENTIC: Self =
        Self(QarStatusCode::QAR_STATUS_ONBOARDING_HUB_NOT_AUTHENTIC);
    pub const ONBOARDING_APP_IDENTITY_MISMATCH: Self =
        Self(QarStatusCode::QAR_STATUS_ONBOARDING_APP_IDENTITY_MISMATCH);
    /// The device's certificate expired or is no longer renewed, and the pairing credential given
    /// did not replace it: pair the device again with a new code. Named by value, since only the
    /// public header declares it.
    pub const ONBOARDING_RE_PAIR_REQUIRED: Self = Self(QarStatusCode(1809));
    /// The hub refused to renew the certificate because the renewal did not prove it holds the
    /// certificate's key: update to the current SDK, else pair again. Named by value, since only
    /// the public header declares it.
    pub const ONBOARDING_RENEWAL_POSSESSION_UNPROVEN: Self = Self(QarStatusCode(1810));
    /// The hub refused to mint a co-process invite: this session did not join as a local app
    /// (or as an app the QAROS launcher started). Final; nothing was minted.
    pub const ONBOARDING_INVITE_REFUSED: Self =
        Self(QarStatusCode::QAR_STATUS_ONBOARDING_INVITE_REFUSED);

    // The codes below are named by value, so they exist under both binding sets: the header of
    // 0.9.0-dev.2916 may not declare them. `name()` gives the header's name.
    pub const GUI_PANEL_CHANGE_NOT_PERMITTED: Self = Self(QarStatusCode(304));
    pub const APP_VOLUME_GESTURE_CONFIGURATION_INVALID: Self = Self(QarStatusCode(326));
    pub const APP_VOLUME_GESTURE_MODE_UNKNOWN: Self = Self(QarStatusCode(327));
    pub const MEDIA_STREAMING_DESTINATION_RESERVATION_FAILED: Self = Self(QarStatusCode(605));
    pub const MEDIA_STREAMING_SEND_PIPELINE_FAILED: Self = Self(QarStatusCode(606));
    pub const MEDIA_STREAMING_LAYOUT_CHANGE_FAILED: Self = Self(QarStatusCode(607));
    pub const HUB_TARGET_APP_DEVICE_NOT_INVITABLE: Self = Self(QarStatusCode(3100));
    /// A declared device category other than desktop PC, server or camera.
    pub const PEER_DEVICE_CATEGORY_NOT_ALLOWED: Self = Self(QarStatusCode(3101));
    /// The hub is in the middle of another session-wide operation; retry.
    pub const HUB_LIFECYCLE_OPERATION_RUNNING: Self = Self(QarStatusCode(3300));
    pub const MEMBER_JOIN_PENDING: Self = Self(QarStatusCode(3400));
    pub const MEMBER_JOIN_DENIED: Self = Self(QarStatusCode(3401));
    /// The hub on this PC hosts members of its own, is a member of another primary, or the
    /// invitation does not permit the join; the message names the reason.
    pub const MEMBER_JOIN_REFUSED: Self = Self(QarStatusCode(3402));
    pub const MEMBER_JOIN_REDIRECTED: Self = Self(QarStatusCode(3403));
    /// The primary removed this hub and the invitation does not permit readmitting it.
    pub const MEMBER_REMOVED: Self = Self(QarStatusCode(3404));
    /// None of the invitation's addresses of the primary hub answers.
    pub const PRIMARY_UNREACHABLE: Self = Self(QarStatusCode(3405));
    /// The hub on this PC is a member: create the member-hub invitation on its primary's PC.
    pub const SESSION_WIDE_ACTION_ON_MEMBER: Self = Self(QarStatusCode(3406));
    /// The app's approval does not allow it to bring in member hubs.
    pub const MEMBER_SPONSOR_NOT_ALLOWED: Self = Self(QarStatusCode(3407));
    /// The app's approval has used its quota of member hubs.
    pub const MEMBER_SPONSOR_QUOTA_EXCEEDED: Self = Self(QarStatusCode(3408));
    pub const AUDIT_LOG_CORRUPT: Self = Self(QarStatusCode(3409));
    pub const SECURE_STORAGE_ERROR: Self = Self(QarStatusCode(3500));
    pub const SECURE_STORAGE_NOT_FOUND: Self = Self(QarStatusCode(3501));
    pub const SECURE_STORAGE_TAMPERED: Self = Self(QarStatusCode(3502));
    pub const SECURE_STORAGE_PROTECTOR_MISMATCH: Self = Self(QarStatusCode(3503));
    pub const SECURE_STORAGE_PROTECTOR_UNAVAILABLE: Self = Self(QarStatusCode(3504));
    pub const SECURE_STORAGE_INVALID_KEY: Self = Self(QarStatusCode(3505));
    pub const SECURE_STORAGE_LOCKED: Self = Self(QarStatusCode(3506));
    /// The member-hub invitation's signature is wrong, its bytes are no invitation, or the
    /// primary does not know it.
    pub const MEMBER_INVITATION_INVALID: Self = Self(QarStatusCode(3600));
    /// Ask the primary's app for a new member-hub invitation.
    pub const MEMBER_INVITATION_EXPIRED: Self = Self(QarStatusCode(3601));
    /// Ask the primary's app for a new member-hub invitation.
    pub const MEMBER_INVITATION_USED_UP: Self = Self(QarStatusCode(3602));
    /// Ask the primary's app for a new member-hub invitation.
    pub const MEMBER_INVITATION_REVOKED: Self = Self(QarStatusCode(3603));
    /// This PC does not trust the invitation's primary hub.
    pub const MEMBER_INVITATION_PRIMARY_NOT_ALLOWED: Self = Self(QarStatusCode(3604));
    /// This PC's member-invitation policy can be written by a non-administrator.
    pub const MEMBER_INVITATION_POLICY_NOT_PROTECTED: Self = Self(QarStatusCode(3605));

    pub(crate) fn from_raw(raw: QarStatusCode) -> Self {
        Self(raw)
    }

    /// The raw `qaros_sys` code, for calls through [`crate::Library::raw_api`].
    pub fn raw(self) -> QarStatusCode {
        self.0
    }

    /// Numeric value of the code.
    pub fn value(self) -> i64 {
        i64::from(self.0.0)
    }

    /// The header's name without the `QAR_STATUS_` prefix, or `None` for a
    /// code this crate does not name.
    pub fn name(self) -> Option<&'static str> {
        let name = match self {
            Self::SUCCESS => "SUCCESS",
            Self::UNCLASSIFIED => "UNCLASSIFIED",
            Self::NOT_IMPLEMENTED => "NOT_IMPLEMENTED",
            Self::ARGUMENT_NOT_SUPPORTED => "ARGUMENT_NOT_SUPPORTED",
            Self::TIMEOUT => "TIMEOUT",
            Self::LOGIC_ERROR => "LOGIC_ERROR",
            Self::GUI_PANEL_INVALID_ID => "GUI_PANEL_INVALID_ID",
            Self::APP_VOLUME_INVALID_ID => "APP_VOLUME_INVALID_ID",
            Self::RENDERING_UNABLE_TO_BEGIN_FRAME => "RENDERING_PRODUCER_UNABLE_TO_DO_BEGIN_FRAME",
            Self::RENDERING_STREAM_IS_CLOSED => "RENDERING_PRODUCER_STREAM_IS_CLOSED",
            Self::MEDIA_STREAMING_STREAM_IS_PAUSED => "MEDIA_STREAMING_STREAM_IS_PAUSED",
            Self::MEDIA_STREAMING_STREAM_IS_RECONNECTING => {
                "MEDIA_STREAMING_STREAM_IS_RECONNECTING"
            }
            Self::MEDIA_STREAMING_NO_ACTIVE_TRANSFERS => "MEDIA_STREAMING_NO_ACTIVE_TRANSFERS",
            Self::RENDERING_APP_VOLUME_UNRESOLVED => "RENDERING_PRODUCER_APP_VOLUME_UNRESOLVED",
            Self::RENDERING_DEPTH_FORMAT_NOT_SUPPORTED => {
                "RENDERING_PRODUCER_DEPTH_FORMAT_NOT_SUPPORTED"
            }
            Self::PAKE_ERROR => "PAKE_ERROR",
            Self::ONBOARDING_SESSION_NOT_FOUND => "ONBOARDING_SESSION_NOT_FOUND",
            Self::ONBOARDING_FAILED => "ONBOARDING_FAILED",
            Self::ONBOARDING_HUB_UNREACHABLE => "ONBOARDING_HUB_UNREACHABLE",
            Self::ONBOARDING_SESSION_STILL_ACTIVE => "ONBOARDING_SESSION_STILL_ACTIVE",
            Self::ONBOARDING_APPROVAL_DENIED => "ONBOARDING_APPROVAL_DENIED",
            Self::ONBOARDING_APPROVAL_TIMEOUT => "ONBOARDING_APPROVAL_TIMEOUT",
            Self::ONBOARDING_HUB_NOT_AUTHENTIC => "ONBOARDING_HUB_NOT_AUTHENTIC",
            Self::ONBOARDING_APP_IDENTITY_MISMATCH => "ONBOARDING_APP_IDENTITY_MISMATCH",
            Self::ONBOARDING_RE_PAIR_REQUIRED => "ONBOARDING_RE_PAIR_REQUIRED",
            Self::ONBOARDING_RENEWAL_POSSESSION_UNPROVEN => {
                "ONBOARDING_RENEWAL_POSSESSION_UNPROVEN"
            }
            Self::ONBOARDING_INVITE_REFUSED => "ONBOARDING_INVITE_REFUSED",
            Self::GUI_PANEL_CHANGE_NOT_PERMITTED => "GUI_PANEL_CHANGE_NOT_PERMITTED",
            Self::APP_VOLUME_GESTURE_CONFIGURATION_INVALID => {
                "APP_VOLUME_GESTURE_CONFIGURATION_INVALID"
            }
            Self::APP_VOLUME_GESTURE_MODE_UNKNOWN => "APP_VOLUME_GESTURE_MODE_UNKNOWN",
            Self::MEDIA_STREAMING_DESTINATION_RESERVATION_FAILED => {
                "MEDIA_STREAMING_DESTINATION_RESERVATION_FAILED"
            }
            Self::MEDIA_STREAMING_SEND_PIPELINE_FAILED => "MEDIA_STREAMING_SEND_PIPELINE_FAILED",
            Self::MEDIA_STREAMING_LAYOUT_CHANGE_FAILED => "MEDIA_STREAMING_LAYOUT_CHANGE_FAILED",
            Self::HUB_TARGET_APP_DEVICE_NOT_INVITABLE => "HUB_TARGET_APP_DEVICE_NOT_INVITABLE",
            Self::PEER_DEVICE_CATEGORY_NOT_ALLOWED => "PEER_DEVICE_CATEGORY_NOT_ALLOWED",
            Self::HUB_LIFECYCLE_OPERATION_RUNNING => "HUB_LIFECYCLE_OPERATION_RUNNING",
            Self::MEMBER_JOIN_PENDING => "MEMBER_JOIN_PENDING",
            Self::MEMBER_JOIN_DENIED => "MEMBER_JOIN_DENIED",
            Self::MEMBER_JOIN_REFUSED => "MEMBER_JOIN_REFUSED",
            Self::MEMBER_JOIN_REDIRECTED => "MEMBER_JOIN_REDIRECTED",
            Self::MEMBER_REMOVED => "MEMBER_REMOVED",
            Self::PRIMARY_UNREACHABLE => "PRIMARY_UNREACHABLE",
            Self::SESSION_WIDE_ACTION_ON_MEMBER => "SESSION_WIDE_ACTION_ON_MEMBER",
            Self::MEMBER_SPONSOR_NOT_ALLOWED => "MEMBER_SPONSOR_NOT_ALLOWED",
            Self::MEMBER_SPONSOR_QUOTA_EXCEEDED => "MEMBER_SPONSOR_QUOTA_EXCEEDED",
            Self::AUDIT_LOG_CORRUPT => "AUDIT_LOG_CORRUPT",
            Self::SECURE_STORAGE_ERROR => "SECURE_STORAGE_ERROR",
            Self::SECURE_STORAGE_NOT_FOUND => "SECURE_STORAGE_NOT_FOUND",
            Self::SECURE_STORAGE_TAMPERED => "SECURE_STORAGE_TAMPERED",
            Self::SECURE_STORAGE_PROTECTOR_MISMATCH => "SECURE_STORAGE_PROTECTOR_MISMATCH",
            Self::SECURE_STORAGE_PROTECTOR_UNAVAILABLE => "SECURE_STORAGE_PROTECTOR_UNAVAILABLE",
            Self::SECURE_STORAGE_INVALID_KEY => "SECURE_STORAGE_INVALID_KEY",
            Self::SECURE_STORAGE_LOCKED => "SECURE_STORAGE_LOCKED",
            Self::MEMBER_INVITATION_INVALID => "MEMBER_INVITATION_INVALID",
            Self::MEMBER_INVITATION_EXPIRED => "MEMBER_INVITATION_EXPIRED",
            Self::MEMBER_INVITATION_USED_UP => "MEMBER_INVITATION_USED_UP",
            Self::MEMBER_INVITATION_REVOKED => "MEMBER_INVITATION_REVOKED",
            Self::MEMBER_INVITATION_PRIMARY_NOT_ALLOWED => "MEMBER_INVITATION_PRIMARY_NOT_ALLOWED",
            Self::MEMBER_INVITATION_POLICY_NOT_PROTECTED => {
                "MEMBER_INVITATION_POLICY_NOT_PROTECTED"
            }
            _ => return None,
        };
        Some(name)
    }

    /// What the user has to do about a local-app enrollment refusal
    /// ([`crate::OnboardMethod::LocalApp`]); `None` for every other code.
    pub fn user_action(self) -> Option<&'static str> {
        let action = match self {
            Self::ONBOARDING_APPROVAL_DENIED => {
                "the app was denied in the QAROS hub, or its approval was revoked; allow it in the hub, then start the app again"
            }
            Self::ONBOARDING_APPROVAL_TIMEOUT => {
                "nobody answered the approval prompt in the QAROS hub in time; answer the prompt on the hub's PC and retry"
            }
            Self::ONBOARDING_HUB_NOT_AUTHENTIC => {
                "the process serving the enrollment pipe is not the installed QAROS hub; check which hub runs on this PC, do not retry blindly"
            }
            Self::ONBOARDING_APP_IDENTITY_MISMATCH => {
                "the app no longer matches what was approved (moved, rebuilt or re-signed); revoke it in the QAROS hub and approve it again"
            }
            _ => return None,
        };
        Some(action)
    }
}

impl fmt::Debug for StatusCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name() {
            Some(name) => write!(formatter, "QAR_STATUS_{name} ({})", self.value()),
            None => write!(formatter, "status {}", self.value()),
        }
    }
}

/// Every failure the safe API reports.
#[derive(Debug)]
pub enum Error {
    /// The C API library could not be opened or lacks symbols.
    Load(qaros_sys::LoadError),
    /// A C API call returned an error result.
    Api {
        /// The public C name of the failed call, such as `qar_runtime_create`.
        function: &'static str,
        /// The status code of the result.
        code: StatusCode,
        /// The text of `qar_result_message`, empty when the library has none.
        message: String,
        /// The codes of [`StatusCode::WRAPPED_CAUSES`] that the result wraps under `code`,
        /// as `qar_result_has_code` reports them.
        causes: Vec<StatusCode>,
    },
    /// `qar_library_init` is process-wide; a second `Library` cannot coexist
    /// with a live one.
    LibraryAlreadyLoaded,
    /// The argument cannot be passed to the C API (interior NUL, too many
    /// views, ...).
    InvalidArgument(String),
}

impl Error {
    /// The status code of an [`Error::Api`].
    pub fn code(&self) -> Option<StatusCode> {
        match self {
            Self::Api { code, .. } => Some(*code),
            _ => None,
        }
    }

    /// Whether this is an [`Error::Api`] with the `expected` code.
    pub fn has_code(&self, expected: StatusCode) -> bool {
        self.code() == Some(expected)
    }

    /// Whether the error is `expected` itself or wraps it as a cause, the C API's
    /// `qar_result_has_code`. Wrapped causes are known for [`StatusCode::WRAPPED_CAUSES`] only:
    /// begin-frame reports a paused, reconnecting or transfer-less stream inside
    /// [`StatusCode::RENDERING_UNABLE_TO_BEGIN_FRAME`].
    pub fn has_cause(&self, expected: StatusCode) -> bool {
        match self {
            Self::Api { code, causes, .. } => *code == expected || causes.contains(&expected),
            _ => false,
        }
    }
}

impl StatusCode {
    /// The causes an [`Error::Api`] records when the result wraps them, read at the failing
    /// call because the C result's cause chain is not kept.
    pub const WRAPPED_CAUSES: [Self; 5] = [
        Self::MEDIA_STREAMING_STREAM_IS_PAUSED,
        Self::MEDIA_STREAMING_STREAM_IS_RECONNECTING,
        Self::MEDIA_STREAMING_NO_ACTIVE_TRANSFERS,
        Self::RENDERING_APP_VOLUME_UNRESOLVED,
        Self::RENDERING_STREAM_IS_CLOSED,
    ];
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Load(load_error) => write!(formatter, "loading the QAROS library: {load_error}"),
            Self::Api {
                function,
                code,
                message,
                ..
            } => {
                if message.is_empty() {
                    write!(formatter, "{function} failed with {code}")?;
                } else {
                    write!(formatter, "{function} failed with {code}: {message}")?;
                }
                match code.user_action() {
                    Some(action) => write!(formatter, " ({action})"),
                    None => Ok(()),
                }
            }
            Self::LibraryAlreadyLoaded => {
                formatter.write_str("a QAROS library is already initialized in this process")
            }
            Self::InvalidArgument(reason) => write!(formatter, "invalid argument: {reason}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Load(load_error) => Some(load_error),
            _ => None,
        }
    }
}

impl From<qaros_sys::LoadError> for Error {
    fn from(load_error: qaros_sys::LoadError) -> Self {
        Self::Load(load_error)
    }
}

/// The result type of every fallible call in this crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;

const MESSAGE_BUFFER_SIZE: usize = 1024;

/// Maps a `QarResult` to `Ok(())` or an [`Error::Api`] carrying the message
/// from `qar_result_message` and the [`StatusCode::WRAPPED_CAUSES`] it wraps.
pub(crate) fn check(api: &QarApi, function: &'static str, result: QarResult) -> Result<()> {
    if result.code == QarStatusCode::QAR_STATUS_SUCCESS {
        return Ok(());
    }
    let mut buffer = [0 as c_char; MESSAGE_BUFFER_SIZE];
    // SAFETY: the buffer is writable for its full length, which is passed.
    unsafe { (api.qar_impl_result_message)(result, buffer.as_mut_ptr(), buffer.len()) };
    let code = StatusCode::from_raw(result.code);
    let causes = StatusCode::WRAPPED_CAUSES
        .into_iter()
        .filter(|&cause| cause != code)
        // SAFETY: plain value arguments.
        .filter(|cause| unsafe { (api.qar_impl_result_has_code)(result, cause.raw()) })
        .collect();
    Err(Error::Api {
        function,
        code,
        message: crate::strings::from_buffer(&buffer),
        causes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_wrapped_cause_is_found_but_not_taken_for_the_code() {
        let error = Error::Api {
            function: "qar_render_sender_try_begin_frame",
            code: StatusCode::RENDERING_UNABLE_TO_BEGIN_FRAME,
            message: String::new(),
            causes: vec![StatusCode::MEDIA_STREAMING_STREAM_IS_RECONNECTING],
        };
        assert!(error.has_cause(StatusCode::MEDIA_STREAMING_STREAM_IS_RECONNECTING));
        assert!(error.has_cause(StatusCode::RENDERING_UNABLE_TO_BEGIN_FRAME));
        assert!(!error.has_code(StatusCode::MEDIA_STREAMING_STREAM_IS_RECONNECTING));
        assert!(!error.has_cause(StatusCode::MEDIA_STREAMING_STREAM_IS_PAUSED));
        assert!(!Error::LibraryAlreadyLoaded.has_cause(StatusCode::LOGIC_ERROR));
    }

    #[test]
    fn the_codes_named_by_value_carry_the_public_headers_values_and_names() {
        assert_eq!(
            StatusCode::ONBOARDING_RE_PAIR_REQUIRED.to_string(),
            "QAR_STATUS_ONBOARDING_RE_PAIR_REQUIRED (1809)"
        );
        assert_eq!(
            StatusCode::ONBOARDING_RENEWAL_POSSESSION_UNPROVEN.to_string(),
            "QAR_STATUS_ONBOARDING_RENEWAL_POSSESSION_UNPROVEN (1810)"
        );
        assert_eq!(
            StatusCode::RENDERING_DEPTH_FORMAT_NOT_SUPPORTED.to_string(),
            "QAR_STATUS_RENDERING_PRODUCER_DEPTH_FORMAT_NOT_SUPPORTED (806)"
        );
        assert_eq!(
            StatusCode::MEMBER_INVITATION_EXPIRED.to_string(),
            "QAR_STATUS_MEMBER_INVITATION_EXPIRED (3601)"
        );
        assert_eq!(
            StatusCode::PEER_DEVICE_CATEGORY_NOT_ALLOWED.to_string(),
            "QAR_STATUS_PEER_DEVICE_CATEGORY_NOT_ALLOWED (3101)"
        );
    }
}
