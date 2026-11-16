//! `Session::open_qaros_view` against the stub: each view reaches the library as the
//! `QarOpenQarosViewInfo` of its link, a local-app session opens it, and a session that joined by
//! invite is refused, as the header states.

use super::*;
use qaros::{QarosDeviceKind, QarosView};

#[test]
fn every_view_reaches_the_library_as_its_link() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    let session = runtime
        .onboard(
            &local_app(),
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect("onboard as a local app")
        .session;

    let views = [
        (
            QarosView::PairHeadsetQr,
            "qaros://visualizer/v1/devices/pair-qr",
        ),
        (
            QarosView::RoomOriginQr,
            "qaros://visualizer/v1/scene-origin/qr",
        ),
        (
            QarosView::DeviceSetup(QarosDeviceKind::MetaQuest),
            "qaros://visualizer/v1/devices/setup?kind=quest",
        ),
        (
            QarosView::DeviceSetup(QarosDeviceKind::Metalens),
            "qaros://visualizer/v1/devices/setup?kind=metalens",
        ),
        (
            QarosView::DeviceSetup(QarosDeviceKind::AndroidXr),
            "qaros://visualizer/v1/devices/setup?kind=android-xr",
        ),
        (
            QarosView::DeviceSetup(QarosDeviceKind::HoloLens),
            "qaros://visualizer/v1/devices/setup?kind=hololens",
        ),
    ];
    for (view, _) in views {
        session.open_qaros_view(view).expect("open the view");
    }
    let expected: Vec<String> = views
        .iter()
        .map(|(_, link)| format!("open qaros view {link}"))
        .collect();
    assert_eq!(
        fixture.control.entries_starting_with("open qaros view"),
        expected
    );
    drop(session);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[test]
fn a_session_that_joined_by_invite_cannot_open_a_view() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    let local = runtime
        .onboard(
            &local_app(),
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect("onboard as a local app")
        .session;
    let invite = local
        .request_onboarding_invite("companion.examples.qaros", &BlockingOptions::default())
        .expect("invite");
    let wire_bytes = invite.serialize().expect("serialize");
    drop(invite);
    let companion_runtime = library
        .create_runtime(&RuntimeConfig {
            runtime_binaries_directory: None,
            storage_directory: Some(fixture.storage.join("companion")),
            ephemeral_storage: true,
        })
        .expect("companion runtime");
    let received = library
        .deserialize_invite(&wire_bytes)
        .expect("deserialize");
    let companion = companion_runtime
        .onboard(
            &OnboardMethod::Invite(&received),
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect("companion onboard")
        .session;

    let refused = companion
        .open_qaros_view(QarosView::PairHeadsetQr)
        .expect_err("not a local-app session");
    assert!(
        refused.has_code(StatusCode::ARGUMENT_NOT_SUPPORTED),
        "{refused}"
    );
    let Error::Api { function, .. } = &refused else {
        panic!("expected an API error, got {refused}");
    };
    assert_eq!(*function, "qar_session_open_qaros_view");
    assert_eq!(
        fixture.control.entries_starting_with("open qaros view"),
        Vec::<String>::new()
    );
    drop(companion);
    drop(received);
    drop(local);
}
