//! Snippets of the peer presence, role, category and device identity functions, which only
//! bindings generated from a header that declares them have; `main.rs` compiles this module
//! there. All of it is in the safe API: `PeerInfo::identity`, `PeerPresentation::device_identity`
//! and `Session::update_device_identity`.

use qaros::{
    BlockingOptions, CancelToken, DeviceCategory, DeviceIdentity, OnboardMethod, PeerInfo,
    PeerPresence, PeerPresentation, PeerRole, Runtime, Session, SourceDeviceCategory,
};

fn show_target(_name: &str, _family: &str, _category: DeviceCategory) {}

fn render_for(_peer: &PeerInfo, _is_here: bool) {}

fn read_presence(session: &Session) -> qaros::Result<()> {
    // ANCHOR: read_presence
    for peer in session.peers()? {
        // `identity` is None only with bindings from a header without these getters.
        let presence = peer
            .identity
            .as_ref()
            .map_or(PeerPresence::Unknown, |identity| identity.presence);
        // Render for ONLINE peers and UNKNOWN ones (not reconciled yet); stop at OFFLINE,
        // LEAVING or RETIRED. OFFLINE and LEAVING peers may come back as ONLINE.
        let is_here = matches!(presence, PeerPresence::Online | PeerPresence::Unknown);
        render_for(&peer, is_here);
    }
    // ANCHOR_END: read_presence
    Ok(())
}

fn show_targets(session: &Session) -> qaros::Result<()> {
    // ANCHOR: show_targets
    for peer in session.peers()? {
        let Some(identity) = peer.identity else {
            continue;
        };
        if identity.role == PeerRole::Target {
            // "Meta Quest 3", "MetaQuest": your icon table, the category as the fallback
            show_target(&identity.device_name, &identity.device_family, identity.category);
        }
    }
    // ANCHOR_END: show_targets
    Ok(())
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
