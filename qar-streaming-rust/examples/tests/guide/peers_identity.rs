//! Snippets of the peer presence, role, category and device identity functions, which only
//! bindings generated from a header that declares them have; `main.rs` compiles this module
//! there. Declaring the device is safe (`PeerPresentation::device_identity`,
//! `Session::update_device_identity`); the getters are raw only.

use std::ffi::{CStr, c_char};

use qaros::qaros_sys::{QarDeviceCategory, QarPeerPresence, QarPeerRole, QarPeerSpec};
use qaros::{
    BlockingOptions, CancelToken, DeviceIdentity, Library, OnboardMethod, PeerPresentation,
    Runtime, Session, SourceDeviceCategory,
};

fn show_target(_name: &str, _family: &str, _category: QarDeviceCategory) {}

fn render_for(_peer: *mut QarPeerSpec, _is_here: bool) {}

fn read_presence(library: &Library, session: &Session) {
    // ANCHOR: read_presence
    // The presence is raw only: query the peer specs through the raw table with the session's
    // handle. Every peer spec the query hands out is yours to destroy.
    let api = library.raw_api();
    let mut peers: [*mut QarPeerSpec; 64] = [std::ptr::null_mut(); 64];
    let mut written = 0_usize;
    // SAFETY: the session is alive and used on this thread; each spec handle is used before
    // it is destroyed, and every buffer is writable for the length passed with it.
    unsafe {
        let result = (api.qar_impl_query_peer_specs)(
            session.as_raw(),
            peers.as_mut_ptr(),
            peers.len(),
            &raw mut written,
        );
        (api.qar_impl_result_log_if_error)(result);
        for &peer in &peers[..written] {
            let mut presence = QarPeerPresence::QAR_PEER_PRESENCE_UNKNOWN;
            (api.qar_impl_peer_spec_get_presence)(peer, &raw mut presence);
            // Render for ONLINE peers and UNKNOWN ones (not reconciled yet); stop at OFFLINE,
            // LEAVING or RETIRED. OFFLINE and LEAVING peers may come back as ONLINE.
            let is_here = presence == QarPeerPresence::QAR_PEER_PRESENCE_ONLINE
                || presence == QarPeerPresence::QAR_PEER_PRESENCE_UNKNOWN;
            render_for(peer, is_here);
            (api.qar_impl_peer_spec_handle_destroy)(peer);
        }
    }
    // ANCHOR_END: read_presence
}

fn show_targets(library: &Library, session: &Session) {
    // ANCHOR: show_targets
    // The role, category and device getters are raw only: call the raw table with the
    // session's handle. Every peer spec the query hands out is yours to destroy.
    let api = library.raw_api();
    let mut peers: [*mut QarPeerSpec; 64] = [std::ptr::null_mut(); 64];
    let mut written = 0_usize;
    // SAFETY: the session is alive and used on this thread; each spec handle is used before
    // it is destroyed, and every buffer is writable for the length passed with it.
    unsafe {
        let result = (api.qar_impl_query_peer_specs)(
            session.as_raw(),
            peers.as_mut_ptr(),
            peers.len(),
            &raw mut written,
        );
        (api.qar_impl_result_log_if_error)(result);
        for &peer in &peers[..written] {
            let mut role = QarPeerRole::QAR_PEER_ROLE_INTERNAL;
            let mut category = QarDeviceCategory::QAR_DEVICE_CATEGORY_UNKNOWN;
            let (mut name, mut family) = ([0 as c_char; 128], [0 as c_char; 64]);
            let role_read = (api.qar_impl_peer_spec_get_role)(peer, &raw mut role);
            if (api.qar_impl_result_is_success)(role_read)
                && role == QarPeerRole::QAR_PEER_ROLE_TARGET
            {
                (api.qar_impl_peer_spec_get_category)(peer, &raw mut category);
                (api.qar_impl_peer_spec_get_device_name)(peer, name.as_mut_ptr(), name.len());
                (api.qar_impl_peer_spec_get_device_family)(peer, family.as_mut_ptr(), family.len());
                let name = CStr::from_ptr(name.as_ptr()).to_string_lossy(); // "Meta Quest 3"
                let family = CStr::from_ptr(family.as_ptr()).to_string_lossy(); // "MetaQuest"
                show_target(&name, &family, category); // your icon table, category as fallback
            }
            (api.qar_impl_peer_spec_handle_destroy)(peer);
        }
    }
    // ANCHOR_END: show_targets
}

fn onboard_as_camera(runtime: &Runtime, cancel: &CancelToken) -> qaros::Result<Session> {
    // ANCHOR: onboard_as_camera
    let presentation = PeerPresentation {
        display_name: Some("Lab camera 1".to_string()),
        device_identity: Some(DeviceIdentity {
            category: SourceDeviceCategory::Camera,
            device_id: "LuxonisOakD".to_string(), // unknown ids are fine
            device_family: Some("Luxonis".to_string()),
            device_name: Some("Luxonis OAK-D".to_string()),
        }),
        ..PeerPresentation::default()
    };
    let onboarded = runtime.onboard(
        &OnboardMethod::LocalApp {
            approval_timeout: None,
            app_tag: None,
        },
        &presentation,
        &BlockingOptions {
            progress: None,
            cancel: Some(cancel),
        },
    )?;
    // Joins as Source + Camera + LuxonisOakD.
    // ANCHOR_END: onboard_as_camera
    Ok(onboarded.session)
}

fn refine_identity(session: &Session) -> qaros::Result<()> {
    // ANCHOR: refine_identity
    session.update_device_identity(&DeviceIdentity {
        category: SourceDeviceCategory::Camera,
        device_id: "LuxonisOakDPro".to_string(),
        device_family: Some("Luxonis".to_string()),
        device_name: Some("OAK-D Pro W".to_string()),
    })?;
    // ANCHOR_END: refine_identity
    Ok(())
}
