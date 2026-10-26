//! Runtime-loading path, error mapping and teardown order, exercised against
//! the `qaros-stub` cdylib. Threading and cancellation semantics of the real
//! library are out of scope here.

#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use qaros::{
    AppState, AppVolumeConfig, BlockingOptions, ColorAlphaMode, ColorEncoding, Error, Eye,
    GraphicsAdapter, GuiPanelConfig, GuiPanelState, HubAddress, Library, OnboardMethod, OwnedFrame,
    PeerId, PeerPresentation, PixelFormat, Pose, RuntimeConfig, ShowParams, StatusCode, Vec3,
    ViewKind,
};
use qaros_stub::frame_stamp;

mod support;
#[allow(clippy::wildcard_imports)]
use support::*;

#[path = "stub/async_create.rs"]
mod async_create;
#[cfg(qaros_d3d11)]
#[path = "stub/d3d11_path.rs"]
mod d3d11_path;
#[path = "stub/raw_handles.rs"]
mod raw_handles;
#[cfg(feature = "vulkan")]
#[path = "stub/vulkan_path.rs"]
mod vulkan_path;

/// A system library by its absolute path: an explicit path is only that file, never a search.
#[cfg(target_os = "macos")]
fn library_without_qaros_symbols() -> std::path::PathBuf {
    std::path::PathBuf::from("/usr/lib/libSystem.B.dylib")
}

#[cfg(target_os = "linux")]
fn library_without_qaros_symbols() -> std::path::PathBuf {
    [
        "/lib/x86_64-linux-gnu/libc.so.6",
        "/usr/lib64/libc.so.6",
        "/lib64/libc.so.6",
    ]
    .into_iter()
    .map(std::path::PathBuf::from)
    .find(|candidate| candidate.exists())
    .expect("the C library at one of the usual paths")
}

#[cfg(target_os = "windows")]
fn library_without_qaros_symbols() -> std::path::PathBuf {
    let system_root = std::env::var_os("SystemRoot").expect("SystemRoot is set on Windows");
    Path::new(&system_root)
        .join("System32")
        .join("kernel32.dll")
}

#[test]
fn every_missing_symbol_is_reported_and_nothing_loads() {
    let _fixture = Fixture::new();
    // SAFETY: the system library is only opened and searched for symbols.
    let error = unsafe { Library::load(library_without_qaros_symbols(), &quiet()) }
        .expect_err("a system library lacks the QAROS symbols");
    let Error::Load(qaros::qaros_sys::LoadError::MissingSymbols { symbols, .. }) = &error else {
        panic!("expected MissingSymbols, got {error}");
    };
    assert_eq!(symbols.len(), qaros::qaros_sys::SYMBOL_NAMES.len());
    assert!(symbols.contains(&"qar_impl_library_init"));
    assert!(symbols.contains(&"qar_impl_render_sender_frame_cpu"));
    assert!(
        error
            .to_string()
            .contains("qar_impl_session_handle_destroy")
    );

    let reloaded = unsafe { Library::load(stub_path(), &quiet()) };
    assert!(
        reloaded.is_ok(),
        "a failed load must release the process slot"
    );
}

#[test]
fn a_cancel_handle_cancels_from_another_thread_until_the_token_is_gone() {
    let _fixture = Fixture::new();
    let library = load_stub();
    let token = library.create_cancel_token(None).expect("token");
    let handle = token.cancel_handle();
    let cancelled = std::thread::spawn(move || handle.cancel())
        .join()
        .expect("the cancelling thread");
    assert!(matches!(cancelled, Ok(true)), "{cancelled:?}");
    assert!(token.is_cancelled());

    let late_handle = token.cancel_handle();
    drop(token);
    assert!(matches!(late_handle.cancel(), Ok(false)));
}

#[test]
fn a_missing_file_is_an_open_error() {
    let _fixture = Fixture::new();
    // SAFETY: the path does not exist, so nothing is opened.
    let error = unsafe { Library::load("/nonexistent/qar-streaming-c.dll", &quiet()) }
        .expect_err("missing file");
    assert!(
        matches!(error, Error::Load(qaros::qaros_sys::LoadError::Open { .. })),
        "{error}"
    );
}

#[test]
fn only_one_library_is_initialized_per_process() {
    let fixture = Fixture::new();
    let first = load_stub();
    // SAFETY: as Fixture::load.
    let second = unsafe { Library::load(stub_path(), &quiet()) };
    assert!(matches!(second, Err(Error::LibraryAlreadyLoaded)));
    drop(first);
    let third = load_stub();
    drop(third);
    let calls = fixture.control.calls();
    assert_eq!(
        calls.iter().filter(|call| *call == "library_init").count(),
        2
    );
    assert_eq!(
        calls
            .iter()
            .filter(|call| *call == "library_destroy")
            .count(),
        2
    );
}

#[test]
fn teardown_follows_sender_session_runtime_library_whatever_the_drop_order() {
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

    let (forward, requests) = qaros::channel();
    let subscription = session
        .subscribe_render_requests(forward)
        .expect("subscribe");
    let request = requests
        .try_recv()
        .expect("the stub's visualizer requests a stream");
    assert_eq!(request.target_peer, VISUALIZER_PEER);

    let mut sender = session
        .create_render_sender(
            &stereo_color_and_depth(request.target_peer),
            &BlockingOptions::default(),
        )
        .expect("sender");
    {
        let mut frame = sender
            .begin_frame(&BlockingOptions::default())
            .expect("begin");
        let left = frame.view(0).expect("left view");
        let right = frame.view(1).expect("right view");
        assert!(left.pose.position.x < right.pose.position.x);
        {
            let cpu = frame.cpu_frame().expect("cpu frame");
            assert_eq!(cpu.views.len(), 4);
            assert_eq!(cpu.textures.len(), 2);
            for texture in cpu.textures {
                assert_eq!(
                    texture.data.len(),
                    (texture.pitch * texture.size.height) as usize
                );
                texture.data.fill(0x7f);
            }
        }
        frame
            .show(&ShowParams {
                near: 0.05,
                far: 25.0,
                depth_scale: 1.0,
            })
            .expect("show");
    }

    subscription.detach();
    drop(library);
    drop(runtime);
    drop(session);
    let calls_before_sender_drop = fixture.control.calls();
    assert!(
        !calls_before_sender_drop
            .iter()
            .any(|call| call == "session_handle_destroy"),
        "the sender keeps the session alive"
    );
    drop(sender);

    let calls = fixture.control.calls();
    assert!(
        calls
            .iter()
            .any(|call| call == "shown near=0.05 far=25 depth_scale=1")
    );
    let sender_destroyed = position_of(&calls, "render_stream_handle_destroy");
    let session_destroyed = position_of(&calls, "session_handle_destroy");
    let runtime_destroyed = position_of(&calls, runtime_release_call());
    let library_destroyed = position_of(&calls, "library_destroy");
    assert!(sender_destroyed < session_destroyed);
    assert!(session_destroyed < runtime_destroyed);
    assert!(runtime_destroyed < library_destroyed);
    assert!(
        position_of(&calls, "render_frame_info_handle_destroy")
            > position_of(&calls, "render_sender_show_frame")
    );
    assert!(last_position_of(&calls, "cancel_token_handle_destroy") > session_destroyed);
    assert!(
        calls
            .iter()
            .any(|call| call == "render_request_handle_destroy"),
        "the delivered render request is released, as the header asks"
    );
    assert_eq!(
        runtime_releases(&calls),
        1,
        "the runtime is released once: {calls:?}"
    );
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    assert_eq!(fixture.control.live_handle_count(), 0);
}

#[test]
fn an_owned_frame_outlives_its_sender_value_and_blocks_a_second_frame() {
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
    let mut sender = session
        .create_render_sender(
            &stereo_color_and_depth(VISUALIZER_PEER),
            &BlockingOptions::default(),
        )
        .expect("sender");

    let abandoned = sender
        .begin_owned_frame(&BlockingOptions::default())
        .expect("first frame");
    let second = sender.begin_owned_frame(&BlockingOptions::default());
    assert!(
        matches!(second, Err(Error::InvalidArgument(_))),
        "{second:?}"
    );
    drop(abandoned);

    let mut frame = sender
        .begin_owned_frame(&BlockingOptions::default())
        .expect("a frame after the abandoned one");
    drop(sender);
    assert!(
        !fixture
            .control
            .calls()
            .iter()
            .any(|call| call == "render_stream_handle_destroy"),
        "the open frame keeps the sender alive"
    );
    let right_eye_x = frame.view(1).expect("right view").pose.position.x;
    assert!((right_eye_x - 0.032).abs() < 1e-6, "{right_eye_x}");
    for texture in frame.cpu_frame().expect("cpu frame").textures {
        texture.data.fill(0x40);
    }
    frame
        .show(&ShowParams {
            near: 0.1,
            far: 10.0,
            depth_scale: 1.0,
        })
        .expect("show");

    let calls = fixture.control.calls();
    assert!(
        position_of(&calls, "render_stream_handle_destroy")
            > last_position_of(&calls, "render_frame_info_handle_destroy")
    );
    drop(session);
    drop(runtime);
    drop(library);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    assert_eq!(fixture.control.live_handle_count(), 0);
}

#[test]
fn errors_carry_function_status_code_and_library_message() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");

    let rejected = runtime
        .onboard(
            &code("WRONG"),
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect_err("wrong code");
    assert!(rejected.has_code(StatusCode::PAKE_ERROR));
    let Error::Api {
        function, message, ..
    } = &rejected
    else {
        panic!("expected an API error, got {rejected}");
    };
    assert_eq!(*function, "qar_runtime_onboard");
    assert_eq!(message, "pairing code rejected");

    let unreachable = runtime
        .onboard(
            &OnboardMethod::Code {
                code: "UNREACHABLE",
                hub: Some(qaros::HubAddress {
                    hostname: "192.0.2.10",
                    port: 0,
                }),
            },
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect_err("hub unreachable");
    assert!(unreachable.has_code(StatusCode::ONBOARDING_HUB_UNREACHABLE));

    let unknown = runtime
        .rejoin(
            qaros::OnboardingId([9; 16]),
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect_err("unknown slot");
    assert!(unknown.has_code(StatusCode::ONBOARDING_SESSION_NOT_FOUND));

    fixture.control.fail_next("runtime_create", 7);
    let injected = library
        .create_runtime(&fixture.runtime_config())
        .expect_err("injected");
    assert!(injected.has_code(StatusCode::LOGIC_ERROR));
    assert_eq!(
        injected.to_string(),
        "qar_runtime_create failed with QAR_STATUS_LOGIC_ERROR (7): injected failure"
    );

    let interior_nul = runtime.onboard(
        &code("AB\0CD"),
        &PeerPresentation::default(),
        &BlockingOptions::default(),
    );
    assert!(matches!(interior_nul, Err(Error::InvalidArgument(_))));
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[test]
fn onboarding_id_persists_for_rejoin_until_forget() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    let (progress_sender, progress) = std::sync::mpsc::channel();
    let onboarded = runtime
        .onboard(
            &code("ABCD-EFGH"),
            &PeerPresentation {
                display_name: Some("Rust test".to_string()),
                ..PeerPresentation::default()
            },
            &BlockingOptions {
                progress: Some(progress_sender),
                cancel: None,
            },
        )
        .expect("onboard");
    let report = progress.try_recv().expect("progress report");
    assert_eq!(report.severity, qaros::ActionSeverity::Done);
    assert_eq!(report.message, "stub onboarding complete");

    let onboarding_id = onboarded.onboarding_id;
    assert!(!onboarding_id.is_zero());
    let text = library.uuid_to_string(&onboarding_id.0).expect("uuid text");
    assert_eq!(text.len(), 36);
    assert_eq!(
        library.uuid_from_string(&text).expect("parse"),
        onboarding_id.0
    );

    assert_eq!(
        onboarded.session.my_peer().expect("my peer").display_name,
        "Rust test"
    );
    let still_active = runtime.forget(onboarding_id).expect_err("session alive");
    assert!(still_active.has_code(StatusCode::ONBOARDING_SESSION_STILL_ACTIVE));
    drop(onboarded.session);

    let rejoined = runtime
        .rejoin(
            onboarding_id,
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect("rejoin");
    drop(rejoined);
    runtime.forget(onboarding_id).expect("forget");
    let gone = runtime
        .rejoin(
            onboarding_id,
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect_err("forgotten");
    assert!(gone.has_code(StatusCode::ONBOARDING_SESSION_NOT_FOUND));
    assert!(matches!(
        runtime.forget(qaros::OnboardingId::ZERO),
        Err(Error::InvalidArgument(_))
    ));
}

#[test]
fn a_dropped_guard_cancels_but_callback_state_lives_until_session_destroy() {
    let fixture = Fixture::new();
    fixture.control.set_auto_render_request(false);
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

    let (forward, requests) = qaros::channel();
    let subscription = session
        .subscribe_render_requests(forward)
        .expect("subscribe");
    let peer = PeerId([3; 16]);
    assert_eq!(fixture.control.emit_render_request(peer), 1);
    assert_eq!(requests.try_recv().expect("delivered").target_peer, peer);

    drop(subscription);
    assert_eq!(fixture.control.emit_render_request(peer), 0);
    let calls = fixture.control.calls();
    let cancelled = last_position_of(&calls, "cancel_token_cancel");
    assert!(
        !calls[cancelled..]
            .iter()
            .any(|call| call == "cancel_token_handle_destroy"),
        "the guard must not free the token or state while the session lives"
    );

    drop(session);
    let calls = fixture.control.calls();
    assert!(
        last_position_of(&calls, "cancel_token_handle_destroy")
            > position_of(&calls, "session_handle_destroy")
    );
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[test]
fn gui_panels_and_app_volumes_round_trip() {
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

    let panels = session.gui_panels();
    let mut panel_config = GuiPanelConfig::new("status.tests.qaros");
    panel_config.display_name = Some("Status".to_string());
    let panel = panels.get_or_create(&panel_config).expect("panel");
    assert_eq!(panels.get_or_create(&panel_config).expect("again"), panel);

    let (forward, updates) = qaros::channel();
    let subscription = panels.subscribe_panel(panel, forward).expect("subscribe");
    let moved = Pose::from_position(Vec3::new(0.2, 1.6, -1.0));
    panels.set_pose(panel, moved).expect("pose");
    assert_eq!(updates.try_recv().expect("update").pose, moved);
    panels
        .navigate_to_uri(panel, "http://127.0.0.1:9090/?view=timeseries")
        .expect("navigate");
    panels
        .set_state(panel, GuiPanelState::Minimized)
        .expect("state");
    let listed = panels.list().expect("list");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].display_name, "Status");
    assert_eq!(
        listed[0].content_uri,
        "http://127.0.0.1:9090/?view=timeseries"
    );
    assert_eq!(listed[0].state, GuiPanelState::Minimized);
    subscription.cancel().expect("cancel");
    panels.close(panel).expect("close");
    let missing = panels.close(panel).expect_err("closed");
    assert!(missing.has_code(StatusCode::GUI_PANEL_INVALID_ID));

    let volumes = session.app_volumes();
    let mut volume_config = AppVolumeConfig::new("scene.tests.qaros");
    volume_config.pose = Pose::from_position(Vec3::new(0.0, 1.0, -1.5));
    let volume = volumes.get_or_create(&volume_config).expect("volume");
    volumes.set_app_scale(volume, 0.25).expect("scale");
    assert!((volumes.latest_app_scale(volume).expect("latest scale") - 0.25).abs() < f32::EPSILON);
    assert_eq!(
        volumes.latest_pose(volume).expect("latest pose"),
        volume_config.pose
    );
    let listed = volumes.list().expect("list");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, volume);

    let peers = session.peers().expect("peers");
    assert_eq!(peers.len(), 2);
    assert!(peers.iter().any(|peer| peer.id == VISUALIZER_PEER));
    let target_apps: Vec<PeerId> = peers
        .iter()
        .filter(|peer| peer.is_target_app)
        .map(|peer| peer.id)
        .collect();
    assert_eq!(target_apps, vec![VISUALIZER_PEER]);
    assert!(!session.my_peer().expect("my peer").is_target_app);
    assert_eq!(session.graphics_adapter().expect("adapter"), None);
    for reported in [
        GraphicsAdapter::Luid([0xb1, 0x5f, 0x0d, 0x04, 0, 0, 0, 0]),
        GraphicsAdapter::Uuid([7; 16]),
    ] {
        fixture.control.set_reported_adapter(Some(reported));
        assert_eq!(
            session.graphics_adapter().expect("adapter"),
            Some(reported),
            "the session reports what the launch invite carried"
        );
    }
    fixture.control.set_reported_adapter(None);
    assert_eq!(session.graphics_adapter().expect("adapter"), None);

    drop(session);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    drop(runtime);
    drop(library);
    assert_eq!(fixture.control.live_handle_count(), 0);
}

#[test]
fn a_sibling_runtime_onboards_from_a_serialized_invite() {
    let fixture = Fixture::new();
    let library = load_stub();
    let primary = library
        .create_runtime(&fixture.runtime_config())
        .expect("primary runtime");
    let session = primary
        .onboard(
            &code("ABCD-EFGH"),
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect("onboard")
        .session;
    let invite = session
        .request_onboarding_invite(&BlockingOptions::default())
        .expect("invite");
    assert!(invite.expires_unix().expect("expiry") > 0);
    let wire_bytes = invite.serialize().expect("serialize");
    drop(invite);

    let sibling = library
        .create_runtime(&RuntimeConfig {
            runtime_binaries_directory: None,
            storage_directory: Some(fixture.storage.join("sibling")),
        })
        .expect("sibling runtime");
    let received = library
        .deserialize_invite(&wire_bytes)
        .expect("deserialize");
    let sibling_onboarded = sibling
        .onboard(
            &OnboardMethod::Invite(&received),
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect("sibling onboard");
    assert!(!sibling_onboarded.onboarding_id.is_zero());

    let garbage = library
        .deserialize_invite(b"not an invite")
        .expect_err("garbage");
    assert!(garbage.has_code(StatusCode::PAKE_ERROR));
    drop(sibling_onboarded);
    drop(received);
    drop(session);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[test]
fn stub_path_points_at_a_built_library() {
    assert!(Path::new(&stub_path()).exists());
}

/// Draws the frame stamp into every color view, as a renderer that checks frame pairing does.
fn stamp_frame(frame: &mut OwnedFrame, frame_id: u32) {
    let mut cpu_frame = frame.cpu_frame().expect("cpu frame");
    let views = cpu_frame.views.clone();
    for view in views.iter().filter(|view| view.kind == ViewKind::Color) {
        let texture = &mut cpu_frame.textures[view.texture_index as usize];
        let area = frame_stamp::ViewArea {
            start_x: view.start_x,
            start_y: view.start_y,
            width: view.end_x - view.start_x,
            height: view.end_y - view.start_y,
            pitch: texture.pitch as usize,
            bytes_per_pixel: 4,
        };
        frame_stamp::write(
            texture.data,
            area,
            frame_stamp::Stamp {
                frame_id,
                right_eye: view.eye == Eye::Right,
            },
        );
    }
}

fn show(frame: OwnedFrame) {
    frame
        .show(&ShowParams {
            near: 0.1,
            far: 10.0,
            depth_scale: 1.0,
        })
        .expect("show");
}

#[test]
fn joined_peers_get_their_own_poses_and_frame_pairing() {
    let fixture = Fixture::new();
    fixture.control.set_auto_render_request(false);
    let library = load_stub();
    let (runtime, session) = onboarded_session(&fixture, &library);
    let (forward_requests, requests) = qaros::channel();
    session
        .subscribe_render_requests(forward_requests)
        .expect("render requests")
        .detach();

    let first_user = PeerId([0xa1; 16]);
    let second_user = PeerId([0xb2; 16]);
    assert_eq!(fixture.control.peer_join(first_user, "Headset A"), 1);
    assert_eq!(
        requests.try_recv().expect("first request").target_peer,
        first_user
    );

    let mut first_sender = session
        .create_render_sender(
            &stereo_color_and_depth(first_user),
            &BlockingOptions::default(),
        )
        .expect("first sender");
    for frame_id in 0..3 {
        let mut frame = first_sender
            .begin_owned_frame(&BlockingOptions::default())
            .expect("first user frame");
        let head_z = frame.view(0).expect("left view").pose.position.z;
        #[allow(clippy::cast_precision_loss)]
        let expected_z = 0.001 * frame_id as f32;
        assert!((head_z - expected_z).abs() < 1e-6, "{head_z}");
        stamp_frame(&mut frame, frame_id);
        show(frame);
    }

    assert_eq!(fixture.control.peer_join(second_user, "Headset B"), 1);
    assert_eq!(
        requests.try_recv().expect("second request").target_peer,
        second_user
    );
    assert_eq!(
        session.peers().expect("peers").len(),
        4,
        "self, visualizer, A, B"
    );
    let mut second_sender = session
        .create_render_sender(
            &stereo_color_and_depth(second_user),
            &BlockingOptions::default(),
        )
        .expect("second sender");
    let mut first_frame = first_sender
        .begin_owned_frame(&BlockingOptions::default())
        .expect("first user frame 3");
    let mut second_frame = second_sender
        .begin_owned_frame(&BlockingOptions::default())
        .expect("second user frame 0");
    let first_x = first_frame.view(0).expect("A left").pose.position.x;
    let second_x = second_frame.view(0).expect("B left").pose.position.x;
    assert!((first_x - (0.5 - 0.032)).abs() < 1e-6, "{first_x}");
    assert!((second_x - (1.0 - 0.032)).abs() < 1e-6, "{second_x}");
    stamp_frame(&mut first_frame, 3);
    // The second user's image claims the first user's frame number: a swapped pairing.
    stamp_frame(&mut second_frame, 3);
    show(first_frame);
    show(second_frame);

    assert_eq!(
        fixture.control.entries_starting_with("paired"),
        [
            "paired peer=a1a1a1a1 frame=0 stamped=0",
            "paired peer=a1a1a1a1 frame=1 stamped=1",
            "paired peer=a1a1a1a1 frame=2 stamped=2",
            "paired peer=a1a1a1a1 frame=3 stamped=3",
        ]
    );
    let mismatches = fixture.control.entries_starting_with("ERROR pairing");
    assert_eq!(mismatches.len(), 1, "{mismatches:?}");
    assert!(mismatches[0].starts_with("ERROR pairing peer=b2b2b2b2 issued frame 0"));
    assert_eq!(fixture.control.frames_shown(first_user), 4);
    assert_eq!(fixture.control.frames_shown(second_user), 1);

    drop(first_sender);
    drop(second_sender);
    drop(session);
    drop(runtime);
    drop(library);
    let unexpected: Vec<String> = fixture
        .errors_in_log()
        .into_iter()
        .filter(|entry| !entry.starts_with("ERROR pairing"))
        .collect();
    assert_eq!(unexpected, Vec::<String>::new());
    assert_eq!(fixture.control.live_handle_count(), 0);
}

#[test]
fn a_retired_peer_is_reported_as_retired() {
    let fixture = Fixture::new();
    fixture.control.set_auto_render_request(false);
    let library = load_stub();
    let (_runtime, session) = onboarded_session(&fixture, &library);
    let (forward_updates, updates) = qaros::channel();
    session
        .subscribe_peer_updates(forward_updates)
        .expect("peer updates")
        .detach();
    let retiring_user = PeerId([0xa1; 16]);
    fixture.control.peer_join(retiring_user, "Headset A");

    fixture.control.peer_retire(retiring_user);
    let retired = updates.try_iter().last().expect("retire update");
    assert_eq!(
        (retired.id, retired.app_state),
        (retiring_user, AppState::Retired)
    );
}

#[test]
fn a_peer_that_leaves_closes_only_its_own_stream() {
    let fixture = Fixture::new();
    fixture.control.set_auto_render_request(false);
    let library = load_stub();
    let (runtime, session) = onboarded_session(&fixture, &library);
    let (forward_updates, updates) = qaros::channel();
    session
        .subscribe_peer_updates(forward_updates)
        .expect("peer updates")
        .detach();
    let leaving_user = PeerId([0xa1; 16]);
    let staying_user = PeerId([0xb2; 16]);
    fixture.control.peer_join(leaving_user, "Headset A");
    let joined = updates.try_recv().expect("join update");
    assert_eq!(
        (joined.id, joined.display_name.as_str(), joined.app_state),
        (leaving_user, "Headset A", AppState::Running)
    );
    fixture.control.peer_join(staying_user, "Headset B");
    let mut leaving_sender = session
        .create_render_sender(
            &stereo_color_and_depth(leaving_user),
            &BlockingOptions::default(),
        )
        .expect("leaving user's sender");
    let mut staying_sender = session
        .create_render_sender(
            &stereo_color_and_depth(staying_user),
            &BlockingOptions::default(),
        )
        .expect("staying user's sender");

    fixture.control.peer_leave(leaving_user);
    let left = updates.try_iter().last().expect("leave update");
    assert_eq!(
        (left.id, left.app_state),
        (leaving_user, AppState::ShuttingDown)
    );
    assert!(
        !session
            .peers()
            .expect("peers")
            .iter()
            .any(|peer| peer.id == leaving_user)
    );
    let closed = leaving_sender
        .begin_owned_frame(&BlockingOptions::default())
        .expect_err("the stream closed");
    assert!(
        closed.has_code(StatusCode::RENDERING_STREAM_IS_CLOSED),
        "{closed}"
    );
    show(
        staying_sender
            .begin_owned_frame(&BlockingOptions::default())
            .expect("the other user keeps streaming"),
    );

    drop(leaving_sender);
    drop(staying_sender);
    drop(session);
    drop(runtime);
    drop(library);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    assert_eq!(fixture.control.live_handle_count(), 0);
}

#[test]
fn the_adapter_is_chained_into_onboard_and_rejoin() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    let luid = GraphicsAdapter::parse("0102030405060708").expect("luid");
    let onboarded = runtime
        .onboard_on_adapter(
            &OnboardMethod::Code {
                code: "ABCD-EFGH",
                hub: Some(HubAddress {
                    hostname: "hub.example.com",
                    port: 0,
                }),
            },
            &PeerPresentation::default(),
            Some(luid),
            &BlockingOptions::default(),
        )
        .expect("onboard on a LUID");
    let onboarding_id = onboarded.onboarding_id;
    drop(onboarded.session);

    let uuid = GraphicsAdapter::parse("00112233445566778899aabbccddeeff").expect("uuid");
    let rejoined = runtime
        .rejoin_on_adapter(
            onboarding_id,
            &PeerPresentation::default(),
            Some(uuid),
            &BlockingOptions::default(),
        )
        .expect("rejoin on a UUID");
    drop(rejoined);

    assert_eq!(
        fixture.control.entries_starting_with("adapter "),
        vec![
            "adapter luid=0102030405060708".to_string(),
            "adapter uuid=00112233445566778899aabbccddeeff".to_string(),
        ]
    );
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[test]
fn runtime_info_reports_the_loaded_library() {
    let _fixture = Fixture::new();
    let library = load_stub();
    assert_eq!(
        library.runtime_info().expect("runtime info"),
        qaros::RuntimeInfo {
            version: "0.9.0-stub".to_string(),
            flavor: "stub".to_string(),
            c_api: "v0".to_string(),
        }
    );
}

#[test]
fn a_local_app_onboards_without_a_code_and_gets_the_zero_id() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    let onboarded = runtime
        .onboard(
            &OnboardMethod::LocalApp {
                approval_timeout: Some(Duration::from_secs(30)),
            },
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect("local app onboard");
    assert!(onboarded.onboarding_id.is_zero());
    drop(onboarded.session);

    let hub_default = runtime
        .onboard(
            &OnboardMethod::LocalApp {
                approval_timeout: None,
            },
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect("local app onboard with the hub's default timeout");
    drop(hub_default.session);

    assert_eq!(
        fixture
            .control
            .entries_starting_with("local app approval_timeout_ms="),
        vec![
            "local app approval_timeout_ms=30000".to_string(),
            "local app approval_timeout_ms=0".to_string(),
        ]
    );
    let zero_timeout = runtime
        .onboard(
            &OnboardMethod::LocalApp {
                approval_timeout: Some(Duration::ZERO),
            },
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect_err("a zero timeout would silently mean the hub default");
    assert!(matches!(zero_timeout, Error::InvalidArgument(_)));
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[test]
fn the_zero_id_of_a_local_app_is_refused_before_any_rejoin_reaches_the_library() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    let onboarded = runtime
        .onboard(
            &OnboardMethod::LocalApp {
                approval_timeout: None,
            },
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect("local app onboard");
    assert!(onboarded.onboarding_id.is_zero());
    drop(onboarded.session);

    let rejoin = runtime.rejoin(
        onboarded.onboarding_id,
        &PeerPresentation::default(),
        &BlockingOptions::default(),
    );
    assert!(matches!(rejoin, Err(Error::InvalidArgument(_))));
    let rejoin_on_adapter = runtime.rejoin_on_adapter(
        onboarded.onboarding_id,
        &PeerPresentation::default(),
        Some(GraphicsAdapter::Luid([1, 2, 3, 4, 5, 6, 7, 8])),
        &BlockingOptions::default(),
    );
    assert!(matches!(rejoin_on_adapter, Err(Error::InvalidArgument(_))));
    assert_eq!(
        fixture.control.entries_starting_with("runtime_rejoin"),
        Vec::<String>::new()
    );
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[test]
fn local_app_refusals_name_what_the_user_has_to_do() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    let refusals = [
        (
            StatusCode::ONBOARDING_APPROVAL_DENIED,
            1805,
            "QAR_STATUS_ONBOARDING_APPROVAL_DENIED (1805)",
            "allow it in the hub",
        ),
        (
            StatusCode::ONBOARDING_APPROVAL_TIMEOUT,
            1806,
            "QAR_STATUS_ONBOARDING_APPROVAL_TIMEOUT (1806)",
            "answer the prompt on the hub's PC and retry",
        ),
        (
            StatusCode::ONBOARDING_HUB_NOT_AUTHENTIC,
            1807,
            "QAR_STATUS_ONBOARDING_HUB_NOT_AUTHENTIC (1807)",
            "is not the installed QAROS hub",
        ),
        (
            StatusCode::ONBOARDING_APP_IDENTITY_MISMATCH,
            1808,
            "QAR_STATUS_ONBOARDING_APP_IDENTITY_MISMATCH (1808)",
            "revoke it in the QAROS hub and approve it again",
        ),
    ];
    for (status, value, status_text, action_text) in refusals {
        assert_eq!(status.value(), value);
        fixture.control.fail_next("runtime_onboard", value);
        let refused = runtime
            .onboard(
                &OnboardMethod::LocalApp {
                    approval_timeout: None,
                },
                &PeerPresentation::default(),
                &BlockingOptions::default(),
            )
            .expect_err("injected refusal");
        assert!(refused.has_code(status));
        let text = refused.to_string();
        assert!(
            text.starts_with(&format!(
                "qar_runtime_onboard failed with {status_text}: injected failure ("
            )),
            "{text}"
        );
        assert!(text.contains(action_text), "{text}");
    }
    assert_eq!(StatusCode::ONBOARDING_FAILED.user_action(), None);
}

#[test]
fn a_paused_stream_has_no_frame_ready_and_resumes_on_the_same_sender() {
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
    for _ in 0..3 {
        assert!(
            sender
                .try_begin_owned_frame()
                .expect("paused, not failed")
                .is_none(),
            "no frame while paused"
        );
    }
    fixture.control.set_peer_paused(VISUALIZER_PEER, false);
    let mut frame = sender
        .try_begin_owned_frame()
        .expect("try begin")
        .expect("ready again after the pause");
    stamp_frame(&mut frame, 0);
    show(frame);
    assert!(
        fixture
            .control
            .calls()
            .contains(&"paired peer=56565656 frame=0 stamped=0".to_string()),
        "{:?}",
        fixture.control.calls()
    );
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[test]
fn a_sender_destroyed_during_its_begin_wait_is_noted_and_one_whose_wait_completed_is_not() {
    const PENDING_NOTE: &str = "render sender destroyed with a begin-frame wait pending";
    let fixture = Fixture::new();
    fixture.control.set_try_begin_frame_delay(30);
    let library = load_stub();
    let (_runtime, session) = onboarded_session(&fixture, &library);
    let create = || {
        session
            .create_render_sender(
                &stereo_color_and_depth(VISUALIZER_PEER),
                &BlockingOptions::default(),
            )
            .expect("sender")
    };

    let mut waiting_sender = create();
    assert!(
        waiting_sender
            .try_begin_owned_frame()
            .expect("try begin")
            .is_none(),
        "the wait takes 30 ms"
    );
    drop(waiting_sender);
    assert_eq!(fixture.control.entries_starting_with(PENDING_NOTE).len(), 1);

    let mut drained_sender = create();
    assert!(drained_sender.try_begin_owned_frame().unwrap().is_none());
    let started = Instant::now();
    let frame = loop {
        if let Some(frame) = drained_sender.try_begin_owned_frame().expect("try begin") {
            break frame;
        }
        assert!(started.elapsed() < Duration::from_secs(5), "never ready");
        std::thread::sleep(Duration::from_millis(1));
    };
    drop(frame);
    drop(drained_sender);
    assert_eq!(
        fixture.control.entries_starting_with(PENDING_NOTE).len(),
        1,
        "the completed wait leaves nothing pending"
    );
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[test]
fn try_begin_polls_a_cpu_frame_and_vulkan_access_is_refused_on_it() {
    let fixture = Fixture::new();
    let library = load_stub();
    let (_runtime, session) = onboarded_session(&fixture, &library);
    let mut sender = session
        .create_render_sender(
            &stereo_color_and_depth(VISUALIZER_PEER),
            &BlockingOptions::default(),
        )
        .expect("sender");
    let mut frame = sender
        .try_begin_owned_frame()
        .expect("try begin")
        .expect("the stub's frame is ready at once");
    assert!(matches!(
        sender.try_begin_owned_frame(),
        Err(Error::InvalidArgument(_))
    ));
    stamp_frame(&mut frame, 0);
    show(frame);
    #[cfg(feature = "vulkan")]
    {
        let refused = sender.vulkan_frame_state();
        assert!(
            matches!(&refused, Err(Error::InvalidArgument(message)) if message.contains("not created for Vulkan")),
            "{refused:?}"
        );
    }
    assert!(
        fixture
            .control
            .calls()
            .contains(&"paired peer=56565656 frame=0 stamped=0".to_string()),
        "{:?}",
        fixture.control.calls()
    );
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[cfg(qaros_color_extensions)]
#[test]
fn the_colour_declarations_are_chained_only_when_set() {
    let fixture = Fixture::new();
    let library = load_stub();
    let (_runtime, session) = onboarded_session(&fixture, &library);
    let mut config = stereo_color_and_depth(VISUALIZER_PEER);
    let plain = session
        .create_render_sender(&config, &BlockingOptions::default())
        .expect("sender without colour declarations");
    drop(plain);
    config.alpha_mode = ColorAlphaMode::Premultiplied;
    config.color_encoding = ColorEncoding::Linear;
    let both = session
        .create_render_sender(&config, &BlockingOptions::default())
        .expect("sender with an alpha mode and a colour encoding");
    drop(both);
    config.alpha_mode = ColorAlphaMode::Straight;
    config.color_encoding = ColorEncoding::Srgb;
    let alpha_only = session
        .create_render_sender(&config, &BlockingOptions::default())
        .expect("sender with an alpha mode only");
    drop(alpha_only);

    assert_eq!(
        fixture
            .control
            .entries_starting_with("render sender colour "),
        vec![
            "render sender colour alpha_mode=2 color_encoding=1".to_string(),
            "render sender colour alpha_mode=3 color_encoding=none".to_string(),
        ]
    );
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[cfg(not(qaros_color_extensions))]
#[test]
fn a_colour_declaration_is_refused_where_the_bindings_lack_the_extensions() {
    let fixture = Fixture::new();
    let library = load_stub();
    let (_runtime, session) = onboarded_session(&fixture, &library);
    let mut config = stereo_color_and_depth(VISUALIZER_PEER);
    config.color_encoding = ColorEncoding::Linear;
    let refused = session.create_render_sender(&config, &BlockingOptions::default());
    assert!(
        matches!(&refused, Err(Error::InvalidArgument(message)) if message.contains("colour extensions")),
        "{:?}",
        refused.map(|_| ())
    );
    config.color_encoding = ColorEncoding::Srgb;
    assert_eq!(config.alpha_mode, ColorAlphaMode::Unspecified);
    let sender = session
        .create_render_sender(&config, &BlockingOptions::default())
        .expect("the defaults chain nothing");
    drop(sender);
}

#[test]
fn a_depth_format_with_stencil_is_refused_on_the_cpu_path() {
    let fixture = Fixture::new();
    let library = load_stub();
    let (_runtime, session) = onboarded_session(&fixture, &library);
    let mut config = stereo_color_and_depth(VISUALIZER_PEER);
    config.depth_format = PixelFormat::D24UnormS8Uint;
    let refused = session.create_render_sender(&config, &BlockingOptions::default());
    let Err(error) = refused else {
        panic!("a CPU sender took a depth-stencil format");
    };
    assert!(
        error.has_code(StatusCode::RENDERING_DEPTH_FORMAT_NOT_SUPPORTED),
        "{error:?}"
    );

    config.depth_format = PixelFormat::D16Unorm;
    let sender = session
        .create_render_sender(&config, &BlockingOptions::default())
        .expect("a CPU sender takes 16-bit depth");
    drop(sender);
}
