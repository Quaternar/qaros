//! Member hubs: join the QAROS hub of one PC to the session of the hub on another PC, when an
//! application already connects the two PCs.
//!
//! 1. On the primary's PC, [`Runtime::create_member_hub_invitation`] returns an invitation the
//!    primary hub signed. [`MemberHubInvitation::serialize`] it and send the bytes to the
//!    application on the joining PC over its own trusted channel.
//! 2. On the joining PC, [`crate::Library::deserialize_member_hub_invitation`] reads it and
//!    [`Runtime::join_hub_as_member`] hands it to the hub there, which joins the primary and
//!    restarts into its session. Then onboard the application itself as a local app
//!    ([`crate::OnboardMethod::LocalApp`]).
//!
//! An invitation is a secret bearer credential until it expires or is used up: never log it.
//! Windows only; elsewhere the calls fail with [`crate::StatusCode::NOT_IMPLEMENTED`].
//!
//! Only the public header declares these functions and types: where the bindings lack them (cfg
//! `qaros_member_hubs` unset), [`MemberHubInvitation`] is an empty enum, so no value of it exists,
//! and the functions are absent.

use std::num::NonZeroU32;
use std::ops::{BitOr, BitOrAssign};
use std::time::Duration;

#[cfg(qaros_member_hubs)]
use std::ffi::CString;
#[cfg(qaros_member_hubs)]
use std::ptr::{self, NonNull};
#[cfg(qaros_member_hubs)]
use std::rc::Rc;

#[cfg(qaros_member_hubs)]
use qaros_sys::{self as sys, defaults};

#[cfg(qaros_member_hubs)]
use crate::callbacks::BlockingOptions;
#[cfg(qaros_member_hubs)]
use crate::cancel::CancelToken;
#[cfg(qaros_member_hubs)]
use crate::error::{Error, Result, StatusCode, check};
#[cfg(qaros_member_hubs)]
use crate::library::{Library, LibraryInner, require_handle};
#[cfg(qaros_member_hubs)]
use crate::runtime::{Runtime, optional_milliseconds};
#[cfg(qaros_member_hubs)]
use crate::strings;

/// The permissions a member-hub invitation grants at the primary hub
/// (`QarMemberHubInvitationPermissionFlagBits`). Combine them with `|`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct MemberHubPermissions(u32);

impl MemberHubPermissions {
    /// A hub the primary does not know joins as a new member.
    pub const JOIN: Self = Self(0x1);
    /// A member the primary already admitted (and has not removed) is answered again; lets a
    /// member that lost its membership rejoin.
    pub const REJOIN: Self = Self(0x2);
    /// A member the primary's operator removed is admitted again.
    pub const READMIT: Self = Self(0x4);
    /// A hub that was a primary of its own moves its paired devices, launcher children and
    /// approved local apps into the primary's session.
    pub const BRING_DEVICES: Self = Self(0x8);

    /// The bits, as `QarMemberHubInvitationPermissionFlags`.
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Whether every permission of `other` is set.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl BitOr for MemberHubPermissions {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl BitOrAssign for MemberHubPermissions {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

/// Options for [`Runtime::create_member_hub_invitation`] (`QarMemberHubInvitationInit`). `None`
/// selects the C API's default.
#[derive(Clone, Debug, Default)]
pub struct MemberHubInvitationConfig {
    /// How long the invitation can be used, in whole seconds (rounded down). `None`: 24 h. At
    /// most 7 days, else [`crate::StatusCode::ARGUMENT_NOT_SUPPORTED`]. Under one second, or over
    /// `u32::MAX` seconds, is [`crate::Error::InvalidArgument`] before the call.
    pub lifetime: Option<Duration>,
    /// How many hubs may join with it. `None`: 1. At most 64, and never more than the calling
    /// app's remaining quota of member hubs.
    pub max_uses: Option<NonZeroU32>,
    /// `None`: [`MemberHubPermissions::JOIN`] | [`MemberHubPermissions::REJOIN`] |
    /// [`MemberHubPermissions::BRING_DEVICES`]. An empty set is [`crate::Error::InvalidArgument`]
    /// before the call, since the C API reads zero as the default.
    pub permissions: Option<MemberHubPermissions>,
    /// A UTF-8 label of at most 64 bytes, shown in the primary hub's visualizer next to the
    /// invitation.
    pub label: Option<String>,
    /// Also grant one remote app on the joining PC (`QarMemberHubInvitationAppGrantExt`), under
    /// this app tag, with the app tag rules of [`crate::OnboardMethod::LocalApp`]. The grant is
    /// bound to the calling app's approval and the tag. On a member hub of this primary, the first
    /// executable that onboards as a local app with the invitation
    /// ([`crate::OnboardExtensions::remote_app_grant`]) is approved there without a prompt;
    /// later starts of that executable onboard with no invitation and no prompt, under the same
    /// peer id while no other live process holds it. A different executable is refused the grant
    /// once it is redeemed. One grant admits one app on one member PC, however many hubs
    /// `max_uses` lets join: issue one tag per remote app. With a grant, the calling app's own
    /// approval is not extended to the member hub (without one it is). Revoking the calling app's
    /// approval, or the granted one, on the primary revokes the granted app.
    pub app_grant_tag: Option<String>,
}

/// Options for [`Runtime::join_hub_as_member`] (`QarJoinHubAsMemberInit`). `None` selects the
/// C API's default; under 1 ms, or over `u32::MAX` milliseconds, is
/// [`crate::Error::InvalidArgument`] before the call, since the C API reads zero as the default.
#[derive(Clone, Copy, Debug, Default)]
pub struct JoinHubAsMemberConfig {
    /// How long to wait for a hub this call started to accept connections. `None`: 60 s.
    pub hub_start_timeout: Option<Duration>,
    /// How long to wait for the hub to come back as a member after its restart. `None`: 120 s.
    pub restart_timeout: Option<Duration>,
}

/// A member-hub invitation (`QarMemberHubInvitation`), destroyed on drop
/// (`qar_member_hub_invitation_handle_destroy`). A secret bearer credential: send its bytes only
/// over a trusted channel and never log them. The primary hub's operator sees it in the
/// visualizer and can revoke it.
#[cfg(qaros_member_hubs)]
#[derive(Debug)]
pub struct MemberHubInvitation {
    handle: NonNull<sys::QarMemberHubInvitation>,
    library: Rc<LibraryInner>,
}

/// A member-hub invitation. These bindings come from a header without member hubs, so no value
/// of this type exists.
#[cfg(not(qaros_member_hubs))]
#[derive(Debug)]
pub enum MemberHubInvitation {}

#[cfg(qaros_member_hubs)]
impl MemberHubInvitation {
    fn from_handle(
        library: &Rc<LibraryInner>,
        handle: *mut sys::QarMemberHubInvitation,
        function: &'static str,
    ) -> Result<Self> {
        Ok(Self {
            handle: require_handle(handle, function)?,
            library: Rc::clone(library),
        })
    }

    /// The invitation's bytes for the application's own channel
    /// (`qar_member_hub_invitation_serialized_size`, `qar_member_hub_invitation_serialize`):
    /// UTF-8 JSON of at most 16 KiB that carries the invitation's secret.
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let api = &self.library.api;
        let mut size = 0_usize;
        // SAFETY: live invitation; valid out-pointer.
        let result = unsafe {
            (api.qar_impl_member_hub_invitation_serialized_size)(
                self.handle.as_ptr(),
                &raw mut size,
            )
        };
        check(api, "qar_member_hub_invitation_serialized_size", result)?;
        let mut bytes = vec![0_u8; size];
        let mut written = 0_usize;
        // SAFETY: live invitation; the buffer is writable for its length; valid out-pointer.
        let result = unsafe {
            (api.qar_impl_member_hub_invitation_serialize)(
                self.handle.as_ptr(),
                bytes.as_mut_ptr(),
                bytes.len(),
                &raw mut written,
            )
        };
        check(api, "qar_member_hub_invitation_serialize", result)?;
        if written > bytes.len() {
            return Err(Error::Api {
                function: "qar_member_hub_invitation_serialize",
                code: StatusCode::ARGUMENT_NOT_SUPPORTED,
                message: format!("needs {written} bytes after reporting {size}"),
                causes: Vec::new(),
            });
        }
        bytes.truncate(written);
        Ok(bytes)
    }

    /// The `QarMemberHubInvitation` handle, for raw calls through [`Library::raw_api`]. Taking
    /// the pointer is safe; using it is `unsafe`, under the rules of [`crate::Session::as_raw`]
    /// for an invitation: valid while this value lives, never destroyed by the caller.
    pub fn as_raw(&self) -> *mut sys::QarMemberHubInvitation {
        self.handle.as_ptr()
    }

    pub(crate) fn as_ptr(&self) -> *const sys::QarMemberHubInvitation {
        self.handle.as_ptr()
    }
}

#[cfg(qaros_member_hubs)]
impl Drop for MemberHubInvitation {
    fn drop(&mut self) {
        // SAFETY: owned handle, destroyed once; the calls that take it borrow it only for the
        // call.
        unsafe {
            (self
                .library
                .api
                .qar_impl_member_hub_invitation_handle_destroy)(self.handle.as_ptr());
        };
    }
}

#[cfg(qaros_member_hubs)]
impl Library {
    /// Reads an invitation from the bytes [`MemberHubInvitation::serialize`] wrote
    /// (`qar_member_hub_invitation_deserialize`). Strict: an invitation with an unknown or missing
    /// field, of another kind or version, out of its bounds, or over 16 KiB fails with
    /// [`StatusCode::MEMBER_INVITATION_INVALID`]. Its signature is checked by
    /// [`Runtime::join_hub_as_member`], not here.
    pub fn deserialize_member_hub_invitation(
        &self,
        wire_bytes: &[u8],
    ) -> Result<MemberHubInvitation> {
        let api = &self.inner.api;
        let mut handle: *mut sys::QarMemberHubInvitation = ptr::null_mut();
        // SAFETY: wire_bytes is readable for its length; valid out-pointer.
        let result = unsafe {
            (api.qar_impl_member_hub_invitation_deserialize)(
                wire_bytes.as_ptr(),
                wire_bytes.len(),
                &raw mut handle,
            )
        };
        check(api, "qar_member_hub_invitation_deserialize", result)?;
        MemberHubInvitation::from_handle(
            &self.inner,
            handle,
            "qar_member_hub_invitation_deserialize",
        )
    }
}

#[cfg(qaros_member_hubs)]
impl Runtime {
    /// Asks the QAROS hub on this PC, the session's primary hub, for a member-hub invitation
    /// (`qar_runtime_create_member_hub_invitation`). Needs no session: the hub identifies this
    /// process as for a local app, and answers only an application it approved with the right to
    /// bring in member hubs and with quota left (spent per hub the invitation admits, not per
    /// invitation). It never prompts. The C call takes a cancel token but no progress callback.
    ///
    /// Fails with [`StatusCode::ARGUMENT_NOT_SUPPORTED`] for a field out of range,
    /// [`StatusCode::ONBOARDING_HUB_UNREACHABLE`], [`StatusCode::ONBOARDING_HUB_NOT_AUTHENTIC`],
    /// [`StatusCode::ONBOARDING_APPROVAL_DENIED`] (the app is not approved),
    /// [`StatusCode::MEMBER_SPONSOR_NOT_ALLOWED`], [`StatusCode::MEMBER_SPONSOR_QUOTA_EXCEEDED`],
    /// [`StatusCode::SESSION_WIDE_ACTION_ON_MEMBER`] (create it on the primary's PC) or
    /// [`StatusCode::NOT_IMPLEMENTED`] (not on Windows).
    pub fn create_member_hub_invitation(
        &self,
        config: &MemberHubInvitationConfig,
        cancel: Option<&CancelToken>,
    ) -> Result<MemberHubInvitation> {
        let mut init = defaults::qar_member_hub_invitation_init_default();
        init.lifetime_seconds = lifetime_seconds(config.lifetime)?;
        init.max_uses = config.max_uses.map_or(0, NonZeroU32::get);
        init.permissions = match config.permissions {
            None => 0,
            Some(permissions) if permissions.bits() == 0 => {
                return Err(Error::InvalidArgument(
                    "an empty permission set; pass None for the default".to_string(),
                ));
            }
            Some(permissions) => permissions.bits(),
        };
        let label = strings::optional_c_string(config.label.as_deref(), "invitation label")?;
        init.label = strings::pointer_or_null(label.as_ref());
        let app_grant_tag: Option<CString> =
            strings::optional_c_string(config.app_grant_tag.as_deref(), "app grant tag")?;
        let mut app_grant = defaults::qar_member_hub_invitation_app_grant_ext_default();
        if let Some(tag) = &app_grant_tag {
            app_grant.app_tag = tag.as_ptr();
            init.header.next = (&raw mut app_grant).cast();
        }

        let api = &self.inner.library.api;
        let cancel_pointer = cancel.map_or(ptr::null_mut(), CancelToken::as_ptr);
        let mut handle: *mut sys::QarMemberHubInvitation = ptr::null_mut();
        // SAFETY: live runtime; init, its extension and strings live for the call; the token is
        // borrowed for the call; valid out-pointer.
        let result = unsafe {
            (api.qar_impl_runtime_create_member_hub_invitation)(
                self.as_raw(),
                &raw const init,
                cancel_pointer,
                &raw mut handle,
            )
        };
        check(api, "qar_runtime_create_member_hub_invitation", result)?;
        MemberHubInvitation::from_handle(
            &self.inner.library,
            handle,
            "qar_runtime_create_member_hub_invitation",
        )
    }

    /// Joins the QAROS hub on this PC to the invitation's primary hub as a member
    /// (`qar_runtime_join_hub_as_member`). This joins the PC's hub, not the calling application:
    /// it does not onboard. Once it returns, onboard as a local app, which then succeeds without a
    /// prompt for the application that created the invitation.
    ///
    /// Starts the installed hub when none runs for this user; checks the invitation's signature,
    /// expiry and whether this PC trusts its primary; joins, rejoins or readmits the hub and blocks
    /// through the hub's restart into the primary's session. Called again with an invitation of
    /// the primary the hub is already a member of, it succeeds at once. The invitation is borrowed
    /// for the call.
    ///
    /// Fails with [`StatusCode::MEMBER_INVITATION_INVALID`],
    /// [`StatusCode::MEMBER_INVITATION_EXPIRED`], [`StatusCode::MEMBER_INVITATION_USED_UP`],
    /// [`StatusCode::MEMBER_INVITATION_REVOKED`] (ask for a new invitation),
    /// [`StatusCode::MEMBER_INVITATION_PRIMARY_NOT_ALLOWED`],
    /// [`StatusCode::MEMBER_INVITATION_POLICY_NOT_PROTECTED`], [`StatusCode::MEMBER_JOIN_REFUSED`],
    /// [`StatusCode::MEMBER_REMOVED`], [`StatusCode::PRIMARY_UNREACHABLE`],
    /// [`StatusCode::HUB_LIFECYCLE_OPERATION_RUNNING`] (retry),
    /// [`StatusCode::ONBOARDING_HUB_UNREACHABLE`], [`StatusCode::ONBOARDING_HUB_NOT_AUTHENTIC`],
    /// [`StatusCode::TIMEOUT`] (the join is committed: calling again succeeds once the hub is
    /// back) or [`StatusCode::NOT_IMPLEMENTED`] (not on Windows).
    pub fn join_hub_as_member(
        &self,
        invitation: &MemberHubInvitation,
        config: &JoinHubAsMemberConfig,
        options: &BlockingOptions<'_>,
    ) -> Result<()> {
        let mut init = defaults::qar_join_hub_as_member_init_default();
        init.invitation = invitation.as_ptr();
        init.hub_start_timeout_ms =
            optional_milliseconds(config.hub_start_timeout, "hub start timeout")?;
        init.restart_timeout_ms = optional_milliseconds(config.restart_timeout, "restart timeout")?;
        let api = &self.inner.library.api;
        // SAFETY: live runtime; init and the invitation it borrows live for the call; the
        // progress state and cancel token are borrowed from options for the call.
        let result = unsafe {
            (api.qar_impl_runtime_join_hub_as_member)(
                self.as_raw(),
                &raw const init,
                options.progress_callback(),
                options.progress_state(),
                options.cancel_pointer(),
            )
        };
        check(api, "qar_runtime_join_hub_as_member", result)
    }
}

#[cfg(qaros_member_hubs)]
fn lifetime_seconds(lifetime: Option<Duration>) -> Result<u32> {
    let Some(lifetime) = lifetime else {
        return Ok(0);
    };
    match u32::try_from(lifetime.as_secs()) {
        Ok(0) => Err(Error::InvalidArgument(
            "an invitation lifetime under one second; pass None for the default".to_string(),
        )),
        Ok(seconds) => Ok(seconds),
        Err(_) => Err(Error::InvalidArgument(format!(
            "invitation lifetime {lifetime:?} exceeds u32 seconds"
        ))),
    }
}
