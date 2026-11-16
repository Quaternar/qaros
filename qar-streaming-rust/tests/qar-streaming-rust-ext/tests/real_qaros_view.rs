//! `Session::open_qaros_view` against the real library: a session that joined an extension hub
//! by invite has no QAROS of this PC behind it, so every view is refused with
//! `ARGUMENT_NOT_SUPPORTED` before any `qaros.exe` is looked for. Nothing here starts a QAROS.

#![cfg(feature = "e2e")]
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

mod common;

use qaros::{Error, QarosDeviceKind, QarosView, StatusCode};

#[test]
fn a_session_that_joined_by_invite_cannot_open_a_qaros_view() {
    let Some(bin) = common::native_bin() else {
        return;
    };
    let loaded = common::load(&bin, "qaros-view");
    let runtime = common::create_runtime(&loaded.library, &bin, &loaded.scratch, "qaros-view");
    let hub = loaded.extension.create_hub(false).expect("create a hub");
    let session = common::onboard_into(&hub, &loaded.library, &runtime, "QAROS view e2e");

    for view in [
        QarosView::PairHeadsetQr,
        QarosView::RoomOriginQr,
        QarosView::DeviceSetup(QarosDeviceKind::HoloLens),
    ] {
        let refused = session
            .open_qaros_view(view)
            .expect_err("an invite session has no QAROS of this PC");
        assert!(
            refused.has_code(StatusCode::ARGUMENT_NOT_SUPPORTED),
            "{view:?}: {refused}"
        );
        let Error::Api { function, .. } = &refused else {
            panic!("expected an API error, got {refused}");
        };
        assert_eq!(*function, "qar_session_open_qaros_view");
    }
    drop(session);
    drop(hub);
}
