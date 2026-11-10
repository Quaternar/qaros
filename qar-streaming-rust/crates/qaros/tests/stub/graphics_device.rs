//! A session without a graphics device (`OnboardExtensions::without_graphics_device`) against the
//! stub, which refuses its media streaming calls with `QAR_STATUS_GRAPHICS_DEVICE_NOT_INITIALIZED`
//! as the header states.

use super::*;

fn without_graphics_device() -> OnboardExtensions<'static> {
    OnboardExtensions {
        without_graphics_device: true,
        ..OnboardExtensions::default()
    }
}

fn assert_not_initialized<T: std::fmt::Debug>(result: qaros::Result<T>, function: &str) {
    let error = result.expect_err("refused on a session without a graphics device");
    assert!(
        error.has_code(StatusCode::GRAPHICS_DEVICE_NOT_INITIALIZED),
        "{function}: {error}"
    );
}

#[test]
fn a_session_without_a_graphics_device_joins_and_refuses_media_streaming() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    let session = runtime
        .onboard_with(
            &local_app(),
            &PeerPresentation::default(),
            &without_graphics_device(),
            &BlockingOptions::default(),
        )
        .expect("onboard without a graphics device")
        .session;
    let calls = fixture.control.calls();
    position_of(&calls, "no graphics device");

    assert!(
        session
            .peers()
            .expect("peers")
            .iter()
            .any(|peer| peer.id == VISUALIZER_PEER)
    );
    assert_not_initialized(session.graphics_adapter(), "graphics_adapter");
    assert_not_initialized(
        session.subscribe_render_requests(|_| {}),
        "subscribe_render_requests",
    );
    assert_not_initialized(
        session.create_render_sender(
            &stereo_color_and_depth(VISUALIZER_PEER),
            &BlockingOptions::default(),
        ),
        "create_render_sender",
    );
    assert_not_initialized(
        session.start_render_sender(&stereo_color_and_depth(VISUALIZER_PEER), None, || {}),
        "start_render_sender",
    );
    assert_eq!(
        StatusCode::GRAPHICS_DEVICE_NOT_INITIALIZED.name(),
        Some("GRAPHICS_DEVICE_NOT_INITIALIZED")
    );

    drop(session);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    drop(runtime);
    drop(library);
    assert_eq!(fixture.control.live_handle_count(), 0);
}

#[test]
fn an_async_onboard_without_a_graphics_device_delivers_such_a_session() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    let session = runtime
        .start_onboard(
            &local_app(),
            &PeerPresentation::default(),
            &without_graphics_device(),
            None,
            None,
            || {},
        )
        .expect("start")
        .wait()
        .expect("onboarded")
        .session;
    assert_not_initialized(session.graphics_adapter(), "graphics_adapter");
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[test]
fn no_graphics_device_together_with_an_adapter_is_refused_before_the_call() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    let extensions = OnboardExtensions {
        adapter: Some(GraphicsAdapter::Uuid([3; 16])),
        without_graphics_device: true,
        ..OnboardExtensions::default()
    };
    let refused = runtime
        .onboard_with(
            &local_app(),
            &PeerPresentation::default(),
            &extensions,
            &BlockingOptions::default(),
        )
        .expect_err("refused before the call");
    assert!(matches!(refused, Error::InvalidArgument(_)), "{refused}");
    let calls = fixture.control.calls();
    assert!(
        !calls.iter().any(|call| call.contains("onboard")),
        "{calls:?}"
    );

    // A session with an adapter still streams.
    let session = runtime
        .onboard_with(
            &local_app(),
            &PeerPresentation::default(),
            &OnboardExtensions {
                adapter: Some(GraphicsAdapter::Uuid([3; 16])),
                ..OnboardExtensions::default()
            },
            &BlockingOptions::default(),
        )
        .expect("onboard on an adapter")
        .session;
    session
        .create_render_sender(
            &stereo_color_and_depth(VISUALIZER_PEER),
            &BlockingOptions::default(),
        )
        .expect("sender");
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}
