//! Session teardown right after render sender creations were cancelled, as at an application's
//! stop: creations whose callbacks come shortly after the cancel no longer make the session
//! leak. In a process of its own, since a leaked session keeps the library loaded and no later
//! test could load it again.

#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

mod support;

use std::time::{Duration, Instant};

#[allow(clippy::wildcard_imports)]
use support::*;

/// How long the stub takes to call back once a creation is cancelled: well past an immediate
/// session drop, well inside the teardown's wait.
const CANCEL_CALLBACK_DELAY_MS: u64 = 100;

fn count_of(calls: &[String], entry: &str) -> usize {
    calls.iter().filter(|call| *call == entry).count()
}

/// Two creations for a target that does not answer are cancelled (the sink dropped at a stop)
/// and their session is dropped at once. Both call back 100 ms after the cancel: the session
/// waits for them, then is destroyed like any other, and everything is released.
#[test]
fn a_session_dropped_right_after_cancelling_creations_waits_for_their_callbacks() {
    let fixture = Fixture::new();
    fixture.control.set_render_sender_create_delay(10_000);
    fixture
        .control
        .set_render_sender_create_cancel_delay(CANCEL_CALLBACK_DELAY_MS);
    let library = load_stub();
    let (runtime, session) = onboarded_session(&fixture, &library);
    let first = session
        .start_render_sender(&stereo_color_and_depth(VISUALIZER_PEER), None, || {})
        .expect("start the first creation");
    let second = session
        .start_render_sender(&stereo_color_and_depth(VISUALIZER_PEER), None, || {})
        .expect("start the second creation");

    let teardown_started = Instant::now();
    drop(first);
    drop(second);
    drop(session);
    let teardown_took = teardown_started.elapsed();

    let calls = fixture.control.calls();
    assert_eq!(
        count_of(&calls, "session_handle_destroy"),
        1,
        "the session is destroyed, not leaked: {calls:?}"
    );
    assert_eq!(
        count_of(&calls, "render_sender_create_async cancelled"),
        2,
        "{calls:?}"
    );
    assert!(
        last_position_of(&calls, "render_sender_create_async cancelled")
            < position_of(&calls, "session_handle_destroy"),
        "both callbacks came before the session went: {calls:?}"
    );
    assert!(
        teardown_took >= Duration::from_millis(CANCEL_CALLBACK_DELAY_MS),
        "the teardown waited for the callbacks, took {teardown_took:?}"
    );
    assert!(
        teardown_took < qaros::CANCELLED_CREATION_WAIT,
        "the teardown stopped waiting once both had called back, took {teardown_took:?}"
    );
    drop(runtime);
    drop(library);
    let calls = fixture.control.calls();
    assert_eq!(count_of(&calls, runtime_release_call()), 1, "{calls:?}");
    assert_eq!(runtime_releases(&calls), 1, "{calls:?}");
    assert_eq!(count_of(&calls, "library_destroy"), 1, "{calls:?}");
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    assert_eq!(fixture.control.live_handle_count(), 0);
}

/// A creation that completes although it was cancelled, while the session waits at teardown:
/// its sender is destroyed before the session.
#[test]
fn a_sender_delivered_during_the_teardown_wait_is_destroyed_before_the_session() {
    let fixture = Fixture::new();
    fixture
        .control
        .set_render_sender_create_delay(CANCEL_CALLBACK_DELAY_MS);
    fixture
        .control
        .set_render_sender_create_ignores_cancel(true);
    let library = load_stub();
    let (runtime, session) = onboarded_session(&fixture, &library);
    let completing = session
        .start_render_sender(&stereo_color_and_depth(VISUALIZER_PEER), None, || {})
        .expect("start the creation");

    drop(completing);
    drop(session);

    let calls = fixture.control.calls();
    assert_eq!(
        count_of(&calls, "render_sender_create_async delivered"),
        1,
        "{calls:?}"
    );
    assert_eq!(
        count_of(&calls, "render_stream_handle_destroy"),
        1,
        "{calls:?}"
    );
    assert!(
        position_of(&calls, "render_stream_handle_destroy")
            < position_of(&calls, "session_handle_destroy"),
        "the unwanted sender goes before its session: {calls:?}"
    );
    drop(runtime);
    drop(library);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    assert_eq!(fixture.control.live_handle_count(), 0);
}
