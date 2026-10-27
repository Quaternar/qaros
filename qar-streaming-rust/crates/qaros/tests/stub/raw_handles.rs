//! Raw calls through `Library::raw_api` on the handles of the safe types (`as_raw`): they reach
//! the same C objects the safe API uses, and the safe types still own and release them.

use std::ptr;

use qaros::qaros_sys::{self as sys, QarStatusCode, defaults};

use super::*;

fn succeeded(result: sys::QarResult) -> bool {
    result.code == QarStatusCode::QAR_STATUS_SUCCESS
}

#[test]
fn raw_peer_and_session_calls_reach_the_session_of_the_safe_api() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    let session = runtime
        .onboard(
            &code("ABCD-EFGH"),
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect("onboard")
        .session;
    let api = library.raw_api();

    let mut count = 0_usize;
    // SAFETY: a live session's handle on this thread; valid out-pointer.
    let result = unsafe { (api.qar_impl_query_peer_specs_count)(session.as_raw(), &raw mut count) };
    assert!(succeeded(result));
    let mut specs: Vec<*mut sys::QarPeerSpec> = vec![ptr::null_mut(); count];
    let mut written = 0_usize;
    // SAFETY: the array is writable for its length; valid out-pointer.
    let result = unsafe {
        (api.qar_impl_query_peer_specs)(
            session.as_raw(),
            specs.as_mut_ptr(),
            specs.len(),
            &raw mut written,
        )
    };
    assert!(succeeded(result));
    let mut raw_peer_ids = Vec::new();
    for spec in &specs[..written] {
        let mut peer_id = defaults::qar_peer_id_default();
        // SAFETY: a spec handle the query handed out, destroyed right after.
        unsafe {
            assert!(succeeded((api.qar_impl_peer_spec_get_id)(
                *spec,
                &raw mut peer_id
            )));
            (api.qar_impl_peer_spec_handle_destroy)(*spec);
        }
        raw_peer_ids.push(PeerId(peer_id.data));
    }
    let mut safe_peer_ids: Vec<PeerId> = session
        .peers()
        .expect("peers")
        .iter()
        .map(|peer| peer.id)
        .collect();
    raw_peer_ids.sort();
    safe_peer_ids.sort();
    assert_eq!(raw_peer_ids, safe_peer_ids);
    assert!(raw_peer_ids.contains(&VISUALIZER_PEER));

    let mut session_id = defaults::qar_session_id_default();
    // SAFETY: a live session's handle; valid out-pointer.
    let result = unsafe { (api.qar_impl_session_get_id)(session.as_raw(), &raw mut session_id) };
    assert!(succeeded(result));
    assert_eq!(session_id.data, session.id().expect("session id").0);

    drop(session);
    drop(runtime);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    drop(library);
    assert_eq!(
        fixture.control.live_handle_count(),
        0,
        "the safe types released every handle their raw pointers named"
    );
}

#[test]
fn raw_runtime_sender_token_and_invite_calls_reach_the_safe_handles() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    let onboarded = runtime
        .onboard(
            &code("ABCD-EFGH"),
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect("onboard");
    let session = onboarded.session;
    let api = library.raw_api();

    let mut forget = defaults::qar_forget_init_default();
    forget.onboarding_id = sys::QarOnboardingId {
        data: onboarded.onboarding_id.0,
    };
    // SAFETY: a live runtime's handle; the init is stamped and lives for the call.
    let result = unsafe { (api.qar_impl_runtime_forget)(runtime.as_raw(), &raw const forget) };
    assert_eq!(
        result.code,
        StatusCode::ONBOARDING_SESSION_STILL_ACTIVE.raw(),
        "the raw call sees the session the safe API opened on this runtime"
    );

    let sender = session
        .create_render_sender(
            &stereo_color_and_depth(VISUALIZER_PEER),
            &BlockingOptions::default(),
        )
        .expect("sender");
    let mut layout = defaults::qar_video_frame_layout_default();
    // SAFETY: a live sender's handle, no frame open; valid out-pointer.
    let result = unsafe { (api.qar_impl_render_sender_layout)(sender.as_raw(), &raw mut layout) };
    assert!(succeeded(result));
    let safe_layout = sender.layout().expect("layout");
    assert_eq!(layout.views_count, safe_layout.views.len());
    assert_eq!(layout.textures_count, safe_layout.textures.len());

    let token = library.create_cancel_token(None).expect("token");
    assert!(!token.is_cancelled());
    // SAFETY: a live token's handle on its owner's thread.
    let result = unsafe { (api.qar_impl_cancel_token_cancel)(token.as_raw()) };
    assert!(succeeded(result));
    assert!(token.is_cancelled(), "the safe token sees the raw cancel");

    // Only a local app gets a co-process invite.
    let local_session = runtime
        .onboard(
            &local_app(),
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect("local app onboard")
        .session;
    let invite = local_session
        .request_onboarding_invite("companion.examples.qaros", &BlockingOptions::default())
        .expect("invite");
    let mut expires = 0_i64;
    // SAFETY: a live invite's handle; valid out-pointer.
    let result = unsafe {
        (api.qar_impl_onboarding_invite_get_expires_unix)(invite.as_raw(), &raw mut expires)
    };
    assert!(succeeded(result));
    assert_eq!(expires, invite.expires_unix().expect("expiry"));
    // SAFETY: a live invite's handle.
    assert!(unsafe { (api.qar_impl_onboarding_invite_handle_is_valid)(invite.as_raw()) });

    drop(invite);
    drop(local_session);
    drop(token);
    drop(sender);
    drop(session);
    drop(runtime);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    drop(library);
    assert_eq!(fixture.control.live_handle_count(), 0);
}

#[test]
fn a_raw_onboard_chain_is_refused_where_the_header_refuses_it() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    let api = library.raw_api();
    // Onboards with a pairing code and `after_code` chained after the code extension; returns
    // the status and the session, if any.
    let onboard_with_code = |after_code: *mut std::ffi::c_void| {
        let code_text = c"ABCD-EFGH";
        let mut code_extension = defaults::qar_onboard_code_ext_default();
        code_extension.code = code_text.as_ptr();
        code_extension.header.next = after_code;
        let mut init = defaults::qar_onboard_init_default();
        init.header.next = (&raw mut code_extension).cast();
        let mut onboarding_id = defaults::qar_onboarding_id_default();
        let mut session: *mut sys::QarSession = ptr::null_mut();
        // SAFETY: a live runtime's handle; the init chain and its strings live for the call;
        // valid out-pointers.
        let result = unsafe {
            (api.qar_impl_runtime_onboard)(
                runtime.as_raw(),
                &raw const init,
                None,
                ptr::null_mut(),
                ptr::null_mut(),
                &raw mut onboarding_id,
                &raw mut session,
            )
        };
        (result, session)
    };

    // An app tag applies only to a local app.
    let tag_text = c"left.examples.qaros";
    let mut tag_extension = defaults::qar_onboard_local_app_tag_ext_default();
    tag_extension.app_tag = tag_text.as_ptr();
    let (result, session) = onboard_with_code((&raw mut tag_extension).cast());
    assert_eq!(result.code, StatusCode::ARGUMENT_NOT_SUPPORTED.raw());
    assert!(session.is_null());

    // A planned peer id must not be zero.
    let mut zero_peer_id = defaults::qar_onboard_peer_id_ext_default();
    let (result, session) = onboard_with_code((&raw mut zero_peer_id).cast());
    assert_eq!(result.code, StatusCode::ARGUMENT_NOT_SUPPORTED.raw());
    assert!(session.is_null());

    let mut planned_peer_id = defaults::qar_onboard_peer_id_ext_default();
    planned_peer_id.peer_id.data = [1; 16];
    let (result, session) = onboard_with_code((&raw mut planned_peer_id).cast());
    assert!(succeeded(result));
    // SAFETY: the raw onboard handed out this session; destroyed once, before the runtime.
    unsafe { (api.qar_impl_session_handle_destroy)(session) };
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[cfg(qaros_member_hubs)]
#[test]
fn a_raw_member_hub_call_reaches_the_invitation_of_the_safe_api() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    let invitation = runtime
        .create_member_hub_invitation(&qaros::MemberHubInvitationConfig::default(), None)
        .expect("invitation");
    let api = library.raw_api();
    let mut size = 0_usize;
    // SAFETY: a live invitation's handle; valid out-pointer.
    let result = unsafe {
        (api.qar_impl_member_hub_invitation_serialized_size)(invitation.as_raw(), &raw mut size)
    };
    assert!(succeeded(result));
    assert_eq!(size, invitation.serialize().expect("serialize").len());
    drop(invitation);
    drop(runtime);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    drop(library);
    assert_eq!(fixture.control.live_handle_count(), 0);
}
