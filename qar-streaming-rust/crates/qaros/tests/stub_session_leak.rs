//! A session dropped while cancelled render sender creations have not called back within
//! `CANCELLED_CREATION_WAIT` is leaked together with its runtime and the library, so this runs
//! in a process of its own: the library cannot be loaded again after it.

#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

mod support;

use std::time::{Duration, Instant};

#[allow(clippy::wildcard_imports)]
use support::*;

const CALLBACK_DEADLINE: Duration = Duration::from_secs(5);
/// What the teardown may take beyond its wait for the callbacks.
const TEARDOWN_SLACK: Duration = Duration::from_millis(100);

fn count_of(calls: &[String], entry: &str) -> usize {
    calls.iter().filter(|call| *call == entry).count()
}

fn wait_for_log_entry(control: &StubControl, entry: &str) {
    let deadline = Instant::now() + CALLBACK_DEADLINE;
    while count_of(&control.calls(), entry) == 0 {
        assert!(
            Instant::now() < deadline,
            "{entry} not logged within {CALLBACK_DEADLINE:?}: {:?}",
            control.calls()
        );
        std::thread::sleep(Duration::from_millis(1));
    }
}

/// One creation never calls back, as a cancelled one may on a runtime older than the header's
/// exactly-once rule;
/// another delivers its sender only after the session's wait for them ran out. Dropping the
/// creations and the session blocks no longer than that wait, destroys nothing the library may
/// still call into, and leaves the late sender alone.
#[test]
fn a_session_with_unanswered_creations_is_leaked_after_a_bounded_wait() {
    let fixture = Fixture::new();
    let library = load_stub();
    let (runtime, session) = onboarded_session(&fixture, &library);

    fixture.control.set_render_sender_create_delay(1_500);
    fixture
        .control
        .set_render_sender_create_ignores_cancel(true);
    let late = session
        .start_render_sender(&stereo_color_and_depth(VISUALIZER_PEER), None, || {})
        .expect("start the late creation");
    fixture
        .control
        .set_render_sender_create_ignores_cancel(false);
    fixture.control.set_render_sender_create_delay(0);
    fixture
        .control
        .set_render_sender_create_never_calls_back(true);
    let unanswered = session
        .start_render_sender(&stereo_color_and_depth(VISUALIZER_PEER), None, || {})
        .expect("start the unanswered creation");
    wait_for_log_entry(
        &fixture.control,
        "render_sender_create_async never calls back",
    );

    let drops_started = Instant::now();
    drop(late);
    drop(unanswered);
    drop(session);
    drop(runtime);
    drop(library);
    let drops_took = drops_started.elapsed();
    assert!(
        drops_took >= qaros::CANCELLED_CREATION_WAIT,
        "the teardown gave the creations their wait, took {drops_took:?}"
    );
    assert!(
        drops_took < qaros::CANCELLED_CREATION_WAIT + TEARDOWN_SLACK,
        "the teardown blocked past its wait, took {drops_took:?}"
    );

    wait_for_log_entry(&fixture.control, "render_sender_create_async delivered");
    let calls = fixture.control.calls();
    for teardown in [
        "session_handle_destroy",
        "library_destroy",
        "render_stream_handle_destroy",
    ] {
        assert_eq!(count_of(&calls, teardown), 0, "{teardown} ran: {calls:?}");
    }
    assert_eq!(
        runtime_releases(&calls),
        0,
        "the runtime was released: {calls:?}"
    );
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}
