use std::ptr::{self, NonNull};
use std::rc::Rc;

use qaros_sys::{self as sys, defaults};

use crate::callbacks::BlockingOptions;
use crate::error::{Error, Result, StatusCode, check};
use crate::library::{LibraryInner, require_handle};
use crate::session::SessionInner;
use crate::strings;

/// A `QarOnboardingInvite`: a full invite blob, onboarded with [`crate::OnboardMethod::Invite`].
/// A co-process invite ([`crate::Session::request_onboarding_invite`]) or the launch invite
/// QAROS hands an app it starts. A secret: hand it over a private channel (stdin, a pipe), never
/// on a command line. A co-process invite names the requesting app on this PC and works for every
/// start of the co-process while the app's session lives; onboarding with it fetches a fresh hub
/// invite from the app on each attempt and retries until it joins.
#[derive(Debug)]
pub struct OnboardingInvite {
    handle: NonNull<sys::QarOnboardingInvite>,
    library: Rc<LibraryInner>,
}

impl OnboardingInvite {
    pub(crate) fn request(
        session: &Rc<SessionInner>,
        app_tag: &str,
        options: &BlockingOptions<'_>,
    ) -> Result<Self> {
        let api = session.api();
        let app_tag_text = strings::to_c_string(app_tag, "app tag")?;
        let mut init = defaults::qar_request_invite_init_default();
        init.app_tag = app_tag_text.as_ptr();
        let mut handle: *mut sys::QarOnboardingInvite = ptr::null_mut();
        // SAFETY: live session; init is stamped and its tag outlives the call; options borrow
        // the progress sender and token for the call; valid out-pointer.
        let result = unsafe {
            (api.qar_impl_session_request_onboarding_invite)(
                session.handle.as_ptr(),
                &raw const init,
                options.progress_callback(),
                options.progress_state(),
                options.cancel_pointer(),
                &raw mut handle,
            )
        };
        check(api, "qar_session_request_onboarding_invite", result)?;
        Ok(Self {
            handle: require_handle(handle, "qar_session_request_onboarding_invite")?,
            library: Rc::clone(session.library()),
        })
    }

    /// Takes ownership of an invite handle the library handed over.
    pub(crate) fn from_handle(
        handle: NonNull<sys::QarOnboardingInvite>,
        library: &Rc<LibraryInner>,
    ) -> Self {
        Self {
            handle,
            library: Rc::clone(library),
        }
    }

    pub(crate) fn deserialize(library: &Rc<LibraryInner>, wire_bytes: &[u8]) -> Result<Self> {
        let api = &library.api;
        let mut handle: *mut sys::QarOnboardingInvite = ptr::null_mut();
        // SAFETY: wire_bytes is readable for its length; valid out-pointer.
        let result = unsafe {
            (api.qar_impl_onboarding_invite_deserialize)(
                wire_bytes.as_ptr(),
                wire_bytes.len(),
                &raw mut handle,
            )
        };
        check(api, "qar_onboarding_invite_deserialize", result)?;
        Ok(Self {
            handle: require_handle(handle, "qar_onboarding_invite_deserialize")?,
            library: Rc::clone(library),
        })
    }

    /// Wire bytes to hand to the sibling instance.
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let api = &self.library.api;
        let mut size = 0_usize;
        // SAFETY: live invite; valid out-pointer.
        let result = unsafe {
            (api.qar_impl_onboarding_invite_serialized_size)(self.handle.as_ptr(), &raw mut size)
        };
        check(api, "qar_onboarding_invite_serialized_size", result)?;
        let mut bytes = vec![0_u8; size];
        let mut written = 0_usize;
        // SAFETY: the buffer is writable for its length.
        let result = unsafe {
            (api.qar_impl_onboarding_invite_serialize)(
                self.handle.as_ptr(),
                bytes.as_mut_ptr(),
                bytes.len(),
                &raw mut written,
            )
        };
        check(api, "qar_onboarding_invite_serialize", result)?;
        if written > bytes.len() {
            return Err(Error::Api {
                function: "qar_onboarding_invite_serialize",
                code: StatusCode::ARGUMENT_NOT_SUPPORTED,
                message: format!("needs {written} bytes after reporting {size}"),
                causes: Vec::new(),
            });
        }
        bytes.truncate(written);
        Ok(bytes)
    }

    /// The `QarOnboardingInvite` handle, for the functions only the raw table of
    /// [`crate::Library::raw_api`] reaches, such as `qar_onboarding_invite_handle_is_valid`.
    ///
    /// Taking the pointer is safe; using it is `unsafe`, under the rules of
    /// [`crate::Session::as_raw`] for an invite: valid while this `OnboardingInvite` lives, never
    /// destroyed by the caller (it is destroyed on drop).
    pub fn as_raw(&self) -> *mut sys::QarOnboardingInvite {
        self.handle.as_ptr()
    }

    pub(crate) fn as_ptr(&self) -> *const sys::QarOnboardingInvite {
        self.handle.as_ptr()
    }
}

impl Drop for OnboardingInvite {
    fn drop(&mut self) {
        // SAFETY: owned handle, destroyed once; onboarding borrows the invite
        // only for the duration of the call.
        unsafe {
            (self.library.api.qar_impl_onboarding_invite_handle_destroy)(self.handle.as_ptr());
        };
    }
}
