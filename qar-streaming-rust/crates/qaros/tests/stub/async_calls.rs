//! The async begin-frame, co-process invite, target-app invite and kill against the stub, which
//! follows the header's rules for them: exactly one callback, `QAR_STATUS_TIMEOUT` on cancel,
//! and an owner's destroy that waits for the callbacks.

use std::sync::mpsc;

use qaros::{DeviceCategory, PeerPresence, PeerRole, TargetDevice};

use super::*;

fn local_app_session(fixture: &Fixture, library: &Library) -> (qaros::Runtime, qaros::Session) {
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    let session = runtime
        .onboard(
            &local_app(),
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect("local app")
        .session;
    (runtime, session)
}

#[test]
fn an_async_begin_frame_delivers_an_owned_frame_and_keeps_the_frame_open_meanwhile() {
    let fixture = Fixture::new();
    let library = load_stub();
    let (_runtime, session) = onboarded_session(&fixture, &library);
    let mut sender = session
        .create_render_sender(
            &stereo_color_and_depth(VISUALIZER_PEER),
            &BlockingOptions::default(),
        )
        .expect("sender");
    fixture.control.set_async_call_delay(20);
    let woken = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&woken);
    let mut pending = sender
        .start_owned_frame(None, move || flag.store(true, Ordering::SeqCst))
        .expect("start");
    assert!(pending.poll().is_none(), "still beginning");
    let refused = sender
        .try_begin_owned_frame()
        .expect_err("a begin-frame is in flight");
    assert!(matches!(refused, Error::InvalidArgument(_)), "{refused}");
    let mut frame = pending.wait().expect("frame");
    assert!(woken.load(Ordering::SeqCst));
    frame.view(0).expect("pose");
    let cpu = frame.cpu_frame().expect("cpu frame");
    assert_eq!(cpu.textures.len(), 2);
    drop(cpu);
    frame
        .show(&ShowParams {
            near: 0.1,
            far: 10.0,
            depth_scale: 1.0,
        })
        .expect("show");
    // The sender is free again.
    let next = sender
        .begin_owned_frame(&BlockingOptions::default())
        .expect("next frame");
    drop(next);
    drop(sender);
    drop(session);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[test]
fn a_cancelled_async_begin_frame_reports_a_timeout_and_frees_the_sender() {
    let fixture = Fixture::new();
    let library = load_stub();
    let (_runtime, session) = onboarded_session(&fixture, &library);
    let mut sender = session
        .create_render_sender(
            &stereo_color_and_depth(VISUALIZER_PEER),
            &BlockingOptions::default(),
        )
        .expect("sender");
    // A paused stream holds the begin-frame, as it holds the blocking one.
    fixture.control.set_peer_paused(VISUALIZER_PEER, true);
    let mut pending = sender.start_owned_frame(None, || {}).expect("start");
    std::thread::sleep(Duration::from_millis(30));
    assert!(
        pending.poll().is_none(),
        "the paused stream holds the frame"
    );
    pending
        .cancel_handle()
        .expect("running")
        .cancel()
        .expect("cancel");
    let cancelled = pending.wait().expect_err("cancelled");
    assert!(cancelled.has_code(StatusCode::TIMEOUT), "{cancelled}");
    fixture.control.set_peer_paused(VISUALIZER_PEER, false);
    let frame = sender
        .begin_owned_frame(&BlockingOptions::default())
        .expect("the sender is free after the cancelled frame");
    drop(frame);

    let timed_out = sender
        .start_owned_frame(Some(Duration::from_millis(1)), || {})
        .and_then(|pending| {
            fixture.control.set_peer_paused(VISUALIZER_PEER, true);
            pending.wait()
        });
    // Either outcome is legal: the frame may begin before the timeout fires.
    if let Err(error) = timed_out {
        assert!(error.has_code(StatusCode::TIMEOUT), "{error}");
    }
    fixture.control.set_peer_paused(VISUALIZER_PEER, false);
    drop(sender);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[test]
fn a_dropped_async_begin_frame_is_settled_by_the_next_begin_and_by_the_sender_destroy() {
    let fixture = Fixture::new();
    let library = load_stub();
    let (_runtime, session) = onboarded_session(&fixture, &library);
    let mut sender = session
        .create_render_sender(
            &stereo_color_and_depth(VISUALIZER_PEER),
            &BlockingOptions::default(),
        )
        .expect("sender");
    fixture.control.set_peer_paused(VISUALIZER_PEER, true);
    drop(sender.start_owned_frame(None, || {}).expect("start"));
    fixture.control.set_peer_paused(VISUALIZER_PEER, false);
    // The next begin waits for the dropped call's callback, then proceeds.
    let frame = sender
        .begin_owned_frame(&BlockingOptions::default())
        .expect("a frame after the dropped one");
    drop(frame);

    fixture.control.set_peer_paused(VISUALIZER_PEER, true);
    drop(sender.start_owned_frame(None, || {}).expect("start"));
    drop(sender);
    let calls = fixture.control.calls();
    assert!(
        last_position_of(&calls, "begin_frame_async called back")
            < last_position_of(&calls, "render sender destroyed"),
        "{calls:?}"
    );
    fixture.control.set_peer_paused(VISUALIZER_PEER, false);
    drop(session);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[test]
fn an_async_co_process_invite_is_minted_for_a_local_app_and_refused_otherwise() {
    let fixture = Fixture::new();
    let library = load_stub();
    let (runtime, session) = local_app_session(&fixture, &library);
    let (progress, reports) = mpsc::channel();
    let invite = session
        .start_onboarding_invite("companion.examples.qaros", Some(progress), None, || {})
        .expect("start")
        .wait()
        .expect("invite");
    assert!(invite.expires_unix().expect("expiry") > 0);
    assert!(reports.try_recv().is_ok(), "a progress report");
    drop(invite);
    let bad_tag = session
        .start_onboarding_invite("member/companion", None, None, || {})
        .expect_err("checked before the call returns");
    assert!(
        bad_tag.has_code(StatusCode::ARGUMENT_NOT_SUPPORTED),
        "{bad_tag}"
    );

    let (code_runtime, code_session) = onboarded_session(&fixture, &library);
    let refused = code_session
        .start_onboarding_invite("companion.examples.qaros", None, None, || {})
        .expect("start")
        .wait()
        .expect_err("refused by the hub");
    assert!(
        refused.has_code(StatusCode::ONBOARDING_INVITE_REFUSED),
        "{refused}"
    );
    drop(code_session);

    // A session destroyed with an abandoned invite request waits for its callback.
    fixture.control.set_async_call_delay(5_000);
    let started = Instant::now();
    drop(
        session
            .start_onboarding_invite("companion.examples.qaros", None, None, || {})
            .expect("start"),
    );
    drop(session);
    assert!(started.elapsed() < Duration::from_secs(4));
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    drop(runtime);
    drop(code_runtime);
    drop(library);
    assert_eq!(fixture.control.live_handle_count(), 0);
}

#[test]
fn a_target_app_is_invited_found_and_killed_on_this_machines_hub() {
    let fixture = Fixture::new();
    let library = load_stub();
    let (_runtime, session) = onboarded_session(&fixture, &library);
    let connection = library
        .target_app_connection_string(TargetDevice::HoloLens, "10.0.0.5")
        .expect("connection string");
    assert_eq!(connection, "hololens:10.0.0.5");
    let peer = session
        .invite_target_app(&connection, &BlockingOptions::default())
        .expect("invited");
    assert_eq!(
        session
            .find_render_target("hololens:10.0.0.5")
            .expect("found"),
        peer
    );
    assert_eq!(
        session
            .find_render_target("visualizer:0")
            .expect("the visualizer"),
        VISUALIZER_PEER
    );
    let target = session
        .peers()
        .expect("peers")
        .into_iter()
        .find(|info| info.id == peer)
        .expect("the target app is listed");
    assert!(target.is_target_app);
    let identity = target.identity;
    assert_eq!(identity.role, PeerRole::Target);
    assert_eq!(identity.presence, PeerPresence::Online);
    assert_eq!(identity.category, DeviceCategory::Headset);
    assert_eq!(identity.device_family, "HoloLens");

    session
        .kill_target_app(peer, &BlockingOptions::default())
        .expect("killed");
    let gone = session
        .find_render_target("hololens:10.0.0.5")
        .expect_err("no longer connected");
    assert!(gone.has_code(StatusCode::ARGUMENT_NOT_SUPPORTED), "{gone}");
    let not_ours = session
        .kill_target_app(peer, &BlockingOptions::default())
        .expect_err("no hub serves it");
    assert!(not_ours.has_code(StatusCode::TIMEOUT), "{not_ours}");

    for (connection, code) in [
        ("visualizer:0", StatusCode::ARGUMENT_NOT_SUPPORTED),
        ("headset:", StatusCode::ARGUMENT_NOT_SUPPORTED),
        ("phone:10.0.0.5", StatusCode::ARGUMENT_NOT_SUPPORTED),
        (
            "headset:uninvitable",
            StatusCode::HUB_TARGET_APP_DEVICE_NOT_INVITABLE,
        ),
    ] {
        let refused = session
            .invite_target_app(connection, &BlockingOptions::default())
            .expect_err(connection);
        assert!(refused.has_code(code), "{connection}: {refused}");
    }
    let empty_host = library
        .target_app_connection_string(TargetDevice::Quest, "")
        .expect_err("an empty host");
    assert!(
        empty_host.has_code(StatusCode::ARGUMENT_NOT_SUPPORTED),
        "{empty_host}"
    );

    fixture.control.set_async_call_delay(10);
    let headset = session
        .start_invite_target_app("quest:headset-1", None, None, || {})
        .expect("start")
        .wait()
        .expect("invited");
    session
        .start_kill_target_app(headset, None, None, || {})
        .expect("start")
        .wait()
        .expect("killed");
    fixture.control.set_async_call_delay(5_000);
    let cancelled = session
        .start_invite_target_app(
            "android:headset-2",
            None,
            Some(Duration::from_millis(10)),
            || {},
        )
        .expect("start")
        .wait()
        .expect_err("timed out");
    assert!(cancelled.has_code(StatusCode::TIMEOUT), "{cancelled}");
    drop(session);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[test]
fn a_joined_devices_declared_identity_is_read_back() {
    use qaros::{DeviceIdentity, SourceDeviceCategory};

    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    let plain = runtime
        .onboard(
            &code("ABCD-EFGH"),
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect("onboard")
        .session;
    let desktop = plain.my_peer().expect("own peer").identity;
    assert_eq!(desktop.role, PeerRole::Source);
    assert_eq!(desktop.category, DeviceCategory::DesktopPc);
    drop(plain);

    let camera = runtime
        .onboard(
            &code("ABCD-EFGH"),
            &PeerPresentation {
                device_identity: Some(DeviceIdentity {
                    category: SourceDeviceCategory::Camera,
                    device_id: "LuxonisOakD".to_string(),
                    device_family: Some("Luxonis".to_string()),
                    device_name: Some("OAK-D".to_string()),
                }),
                ..PeerPresentation::default()
            },
            &BlockingOptions::default(),
        )
        .expect("onboard as a camera")
        .session;
    let declared = camera.my_peer().expect("own peer").identity;
    assert_eq!(declared.category, DeviceCategory::Camera);
    assert_eq!(
        (
            declared.device_id.as_str(),
            declared.device_family.as_str(),
            declared.device_name.as_str()
        ),
        ("LuxonisOakD", "Luxonis", "OAK-D")
    );
    camera
        .update_device_identity(&DeviceIdentity {
            category: SourceDeviceCategory::Server,
            device_id: "RenderNode".to_string(),
            ..DeviceIdentity::default()
        })
        .expect("update");
    let updated = camera.my_peer().expect("own peer").identity;
    assert_eq!(updated.category, DeviceCategory::Server);
    assert_eq!(updated.device_id, "RenderNode");
    drop(camera);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}
