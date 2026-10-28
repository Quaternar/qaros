//! Session teardown with render sender creations still pending, as at an application's stop.
//! The header has the session's destroy end every async call still running with
//! `QAR_STATUS_TIMEOUT` and return only after its callback, so the session never leaks and the
//! binding adds no wait of its own. In a process of its own, so the timings are not shared with
//! other tests.

#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

mod support;

use std::time::{Duration, Instant};

#[allow(clippy::wildcard_imports)]
use support::*;

/// How long the stub takes to call back once a creation is cancelled: well past an immediate
/// session drop.
const CANCEL_CALLBACK_DELAY_MS: u64 = 100;
/// Far below the creations' own delay: the teardown does not wait for them to finish.
const TEARDOWN_BOUND: Duration = Duration::from_secs(4);

fn count_of(calls: &[String], entry: &str) -> usize {
    calls.iter().filter(|call| *call == entry).count()
}

/// Two creations for a target that does not answer are cancelled (the sink dropped at a stop)
/// and their session is dropped at once. Both call back 100 ms after the cancel: the session's
/// destroy waits for them, then the session is gone like any other, and everything is released.
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
    assert_eq!(count_of(&calls, "session_handle_destroy"), 1, "{calls:?}");
    assert_eq!(
        count_of(&calls, "render_sender_create_async cancelled"),
        2,
        "{calls:?}"
    );
    assert!(
        last_position_of(&calls, "render_sender_create_async cancelled")
            < position_of(&calls, "session destroyed"),
        "both callbacks came before the session went: {calls:?}"
    );
    assert!(
        teardown_took >= Duration::from_millis(CANCEL_CALLBACK_DELAY_MS),
        "the destroy waited for the callbacks, took {teardown_took:?}"
    );
    assert!(teardown_took < TEARDOWN_BOUND, "took {teardown_took:?}");
    drop(runtime);
    drop(library);
    let calls = fixture.control.calls();
    assert_eq!(count_of(&calls, RUNTIME_RELEASE_CALL), 1, "{calls:?}");
    assert_eq!(count_of(&calls, "library_destroy"), 1, "{calls:?}");
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    assert_eq!(fixture.control.live_handle_count(), 0);
}

/// A creation that ignores its cancel token is still running when its session is dropped: the
/// session's destroy ends it with `QAR_STATUS_TIMEOUT` and returns, without waiting for the
/// creation's own delay. No sender is made, and nothing is left behind.
#[test]
fn a_creation_still_running_at_session_teardown_ends_with_a_timeout() {
    let fixture = Fixture::new();
    fixture.control.set_render_sender_create_delay(10_000);
    fixture
        .control
        .set_render_sender_create_ignores_cancel(true);
    let library = load_stub();
    let (runtime, session) = onboarded_session(&fixture, &library);
    let running = session
        .start_render_sender(&stereo_color_and_depth(VISUALIZER_PEER), None, || {})
        .expect("start the creation");

    let teardown_started = Instant::now();
    drop(running);
    drop(session);
    let teardown_took = teardown_started.elapsed();

    let calls = fixture.control.calls();
    assert!(teardown_took < TEARDOWN_BOUND, "took {teardown_took:?}");
    assert_eq!(
        count_of(
            &calls,
            "render_sender_create_async ended early: ended by its owner's destroy"
        ),
        1,
        "{calls:?}"
    );
    assert!(
        position_of(&calls, "render_sender_create_async called back")
            < position_of(&calls, "session destroyed"),
        "{calls:?}"
    );
    assert_eq!(
        count_of(&calls, "render_sender_create_async delivered"),
        0,
        "{calls:?}"
    );
    drop(runtime);
    drop(library);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    assert_eq!(fixture.control.live_handle_count(), 0);
}
