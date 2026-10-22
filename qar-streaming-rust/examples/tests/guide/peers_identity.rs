//! Snippets of the peer presence, role, category and device identity functions, which only
//! bindings generated from a header that declares them have; `main.rs` compiles this module
//! there.

use std::ffi::{CStr, c_char};

use qaros::qaros_sys::{
    QarDeviceCategory, QarPeerDeviceIdentity, QarPeerPresence, QarPeerRole, QarPeerSpec,
    QarSession, QarStructureType, defaults,
};
use qaros::{CancelToken, Library, Runtime, Session};

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

fn onboard_as_camera(
    library: &Library,
    runtime: &Runtime,
    cancel: &CancelToken,
) -> *mut QarSession {
    // ANCHOR: onboard_as_camera
    // Declaring your device is raw only: onboard through the raw table with the runtime's
    // handle. The session it hands out is raw too: not a qaros::Session, so destroy it with
    // qar_session_handle_destroy before the runtime drops.
    let api = library.raw_api();
    let identity = QarPeerDeviceIdentity {
        header: defaults::structure_header(
            QarStructureType::QAR_STRUCTURE_TYPE_PEER_DEVICE_IDENTITY,
        ),
        category: QarDeviceCategory::QAR_DEVICE_CATEGORY_CAMERA,
        device_id: c"LuxonisOakD".as_ptr(), // unknown ids are fine
        device_family: c"Luxonis".as_ptr(),
        device_name: c"Luxonis OAK-D".as_ptr(),
    };

    let mut init = defaults::qar_onboard_init_default();
    let mut local = defaults::qar_onboard_local_app_ext_default();
    init.header.next = (&raw mut local.header).cast();
    init.presentation.display_name = c"Lab camera 1".as_ptr();
    init.presentation.header.next = (&raw const identity.header).cast_mut().cast();

    let mut unused = defaults::qar_onboarding_id_default();
    let mut session: *mut QarSession = std::ptr::null_mut();
    // SAFETY: the runtime is alive and used on this thread; init and its chain live on this
    // stack frame for the whole call; the out-pointers are valid.
    let result = unsafe {
        (api.qar_impl_runtime_onboard)(
            runtime.as_raw(),
            &raw const init,
            None,
            std::ptr::null_mut(),
            cancel.as_raw(),
            &raw mut unused,
            &raw mut session,
        )
    };
    // SAFETY: plain value argument.
    unsafe { (api.qar_impl_result_log_if_error)(result) };
    // Joins as Source + Camera + LuxonisOakD.
    // ANCHOR_END: onboard_as_camera
    session
}

fn refine_identity(library: &Library, session: &Session) {
    // ANCHOR: refine_identity
    let api = library.raw_api();
    let refined = QarPeerDeviceIdentity {
        header: defaults::structure_header(
            QarStructureType::QAR_STRUCTURE_TYPE_PEER_DEVICE_IDENTITY,
        ),
        category: QarDeviceCategory::QAR_DEVICE_CATEGORY_CAMERA,
        device_id: c"LuxonisOakDPro".as_ptr(),
        device_family: c"Luxonis".as_ptr(),
        device_name: c"OAK-D Pro W".as_ptr(),
    };
    // SAFETY: the session is alive and used on this thread; `refined` and its strings outlive
    // the call.
    let result =
        unsafe { (api.qar_impl_peer_update_device_identity)(session.as_raw(), &raw const refined) };
    // SAFETY: plain value argument.
    unsafe { (api.qar_impl_result_log_if_error)(result) };
    // ANCHOR_END: refine_identity
}
