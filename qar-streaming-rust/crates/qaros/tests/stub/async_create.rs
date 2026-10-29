//! `Session::start_render_sender` against the stub's
//! `qar_render_sender_create_async`, which calls back from a detached thread.

use qaros::{PendingRenderSender, PixelFormat, RenderSender};

use super::*;

const CALLBACK_DEADLINE: Duration = Duration::from_secs(5);
/// Dropping a pending creation must not wait for the library; this bound is
/// far below every delay these tests give the stub.
const PROMPT_DROP: Duration = Duration::from_millis(40);

fn poll_until_called_back(pending: &mut PendingRenderSender) -> qaros::Result<RenderSender> {
    let deadline = Instant::now() + CALLBACK_DEADLINE;
    loop {
        if let Some(outcome) = pending.poll() {
            return outcome;
        }
        assert!(
            Instant::now() < deadline,
            "no callback within {CALLBACK_DEADLINE:?}"
        );
        std::thread::sleep(Duration::from_millis(1));
    }
}

pub(super) fn wait_for_log_entry(control: &StubControl, entry: &str) {
    let deadline = Instant::now() + CALLBACK_DEADLINE;
    while !control.calls().iter().any(|call| call == entry) {
        assert!(
            Instant::now() < deadline,
            "{entry} not logged within {CALLBACK_DEADLINE:?}: {:?}",
            control.calls()
        );
        std::thread::sleep(Duration::from_millis(1));
    }
}

/// Waits until `entry` was logged `count` times in all; a plain wait would
/// return at once on an entry an earlier creation already logged.
fn wait_for_log_count(control: &StubControl, entry: &str, count: usize) {
    let deadline = Instant::now() + CALLBACK_DEADLINE;
    while count_of(&control.calls(), entry) < count {
        assert!(
            Instant::now() < deadline,
            "{entry} not logged {count} times within {CALLBACK_DEADLINE:?}: {:?}",
            control.calls()
        );
        std::thread::sleep(Duration::from_millis(1));
    }
}

fn count_of(calls: &[String], entry: &str) -> usize {
    calls.iter().filter(|call| *call == entry).count()
}

fn timed_drop(pending: PendingRenderSender) -> Duration {
    let started = Instant::now();
    drop(pending);
    started.elapsed()
}

fn teardown_is_clean(
    fixture: &Fixture,
    session: qaros::Session,
    runtime: qaros::Runtime,
    library: Library,
) {
    drop(session);
    drop(runtime);
    drop(library);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    assert_eq!(fixture.control.live_handle_count(), 0);
}

#[test]
fn a_later_poll_picks_up_the_sender_after_the_callback_and_wake() {
    let fixture = Fixture::new();
    fixture.control.set_render_sender_create_delay(50);
    let library = load_stub();
    let (runtime, session) = onboarded_session(&fixture, &library);
    let handles_before = fixture.control.live_handle_count();

    let wakes = Arc::new(AtomicUsize::new(0));
    let wake_counter = Arc::clone(&wakes);
    let mut pending = session
        .start_render_sender(&stereo_color_and_depth(VISUALIZER_PEER), None, move || {
            wake_counter.fetch_add(1, Ordering::SeqCst);
        })
        .expect("start");
    assert!(
        pending.poll().is_none(),
        "the stub delays the callback by 50 ms"
    );
    assert_eq!(
        fixture.control.live_handle_count(),
        handles_before + 1,
        "only the creation's cancel token while pending"
    );

    let mut sender = poll_until_called_back(&mut pending).expect("sender");
    assert_eq!(wakes.load(Ordering::SeqCst), 1);
    let layout = sender.layout().expect("layout");
    assert_eq!(layout.views.len(), 4);
    let textures: Vec<(u32, u32, PixelFormat)> = layout
        .textures
        .iter()
        .map(|texture| (texture.width, texture.height, texture.format))
        .collect();
    assert_eq!(
        textures,
        [
            (128, 64, PixelFormat::B8G8R8A8),
            (128, 64, PixelFormat::D32Float)
        ]
    );
    let mut frame = sender
        .begin_owned_frame(&BlockingOptions::default())
        .expect("frame on the async sender");
    let left_eye_x = frame.view(0).expect("left view").pose.position.x;
    assert!((left_eye_x + 0.032).abs() < 1e-6, "{left_eye_x}");
    stamp_frame(&mut frame, 0);
    show(frame);
    assert_eq!(
        fixture.control.entries_starting_with("paired"),
        ["paired peer=56565656 frame=0 stamped=0"]
    );

    let polled_again = pending.poll();
    assert!(
        matches!(polled_again, Some(Err(Error::InvalidArgument(_)))),
        "{polled_again:?}"
    );
    assert_eq!(
        fixture.control.live_handle_count(),
        handles_before + 1,
        "the sender, its token released after the callback"
    );
    let calls = fixture.control.calls();
    assert!(
        position_of(&calls, "render_sender_create_async")
            < position_of(&calls, "render_sender_create_async delivered")
    );
    drop(pending);
    drop(sender);
    teardown_is_clean(&fixture, session, runtime, library);
}

#[test]
fn errors_from_the_callback_and_from_the_call_are_reported() {
    let fixture = Fixture::new();
    let library = load_stub();
    let (runtime, session) = onboarded_session(&fixture, &library);
    let handles_before = fixture.control.live_handle_count();

    fixture.control.fail_next(
        "render_sender_create_async_callback",
        StatusCode::APP_VOLUME_INVALID_ID.value(),
    );
    let mut pending = session
        .start_render_sender(&stereo_color_and_depth(VISUALIZER_PEER), None, || {})
        .expect("the call itself succeeds");
    let delivered = poll_until_called_back(&mut pending).expect_err("delivered failure");
    let Error::Api {
        function,
        code,
        message,
        ..
    } = &delivered
    else {
        panic!("not an API error: {delivered:?}");
    };
    assert_eq!(*function, "qar_render_sender_create_async");
    assert_eq!(*code, StatusCode::APP_VOLUME_INVALID_ID);
    assert_eq!(message, "injected failure");
    drop(pending);
    assert_eq!(
        fixture.control.live_handle_count(),
        handles_before,
        "no sender, and the token is released after the callback"
    );

    fixture.control.fail_next(
        "render_sender_create_async",
        StatusCode::LOGIC_ERROR.value(),
    );
    let refused = session
        .start_render_sender(&stereo_color_and_depth(VISUALIZER_PEER), None, || {})
        .expect_err("synchronous failure");
    assert!(refused.has_code(StatusCode::LOGIC_ERROR), "{refused}");
    assert_eq!(
        fixture.control.live_handle_count(),
        handles_before + 1,
        "after a synchronous error the token stays until the library goes"
    );

    let mut too_many_views = stereo_color_and_depth(VISUALIZER_PEER);
    too_many_views.views = [too_many_views.views.clone(), too_many_views.views].concat();
    too_many_views.views.push(too_many_views.views[0]);
    let invalid = session
        .start_render_sender(&too_many_views, None, || {})
        .expect_err("nine views");
    assert!(matches!(invalid, Error::InvalidArgument(_)), "{invalid:?}");
    assert_eq!(
        count_of(&fixture.control.calls(), "render_sender_create_async"),
        2,
        "the invalid config never reaches the library"
    );
    teardown_is_clean(&fixture, session, runtime, library);
}

#[test]
fn a_timeout_cancels_the_creation_through_its_token() {
    let fixture = Fixture::new();
    fixture.control.set_render_sender_create_delay(5_000);
    let library = load_stub();
    let (runtime, session) = onboarded_session(&fixture, &library);
    let handles_before = fixture.control.live_handle_count();

    let timeout = Duration::from_millis(30);
    let started = Instant::now();
    let mut pending = session
        .start_render_sender(
            &stereo_color_and_depth(VISUALIZER_PEER),
            Some(timeout),
            || {},
        )
        .expect("start");
    let timed_out = poll_until_called_back(&mut pending).expect_err("timed out");
    let elapsed = started.elapsed();
    assert!(timed_out.has_code(StatusCode::TIMEOUT), "{timed_out}");
    assert!(
        elapsed >= timeout && elapsed < Duration::from_secs(4),
        "{elapsed:?}"
    );
    assert_eq!(
        fixture
            .control
            .entries_starting_with("render_sender_create_async timed out")
            .len(),
        1
    );
    assert_eq!(fixture.control.live_handle_count(), handles_before);
    drop(pending);
    teardown_is_clean(&fixture, session, runtime, library);
}

#[test]
fn a_creation_for_an_unanswering_peer_times_out_while_a_live_peer_answers() {
    const DEAD_PEER: PeerId = PeerId([0xde; 16]);
    let fixture = Fixture::new();
    let library = load_stub();
    let (runtime, session) = onboarded_session(&fixture, &library);
    fixture.control.peer_join(DEAD_PEER, "Gone Headset");
    fixture.control.set_peer_unanswering(DEAD_PEER, 60);

    let started = Instant::now();
    let mut dead_creation = session
        .start_render_sender(
            &stereo_color_and_depth(DEAD_PEER),
            Some(Duration::from_secs(10)),
            || {},
        )
        .expect("start for the dead peer");
    let mut live_creation = session
        .start_render_sender(
            &stereo_color_and_depth(VISUALIZER_PEER),
            Some(Duration::from_secs(10)),
            || {},
        )
        .expect("start for the live peer");

    let live_sender = poll_until_called_back(&mut live_creation).expect("the live peer answers");
    assert!(
        dead_creation.poll().is_none(),
        "the dead peer's creation still runs"
    );
    let timed_out = poll_until_called_back(&mut dead_creation).expect_err("no answer");
    let elapsed = started.elapsed();
    assert!(timed_out.has_code(StatusCode::TIMEOUT), "{timed_out}");
    assert!(
        elapsed >= Duration::from_millis(60) && elapsed < Duration::from_secs(4),
        "{elapsed:?}"
    );
    assert_eq!(
        fixture
            .control
            .entries_starting_with("render_sender_create_async timed out")
            .len(),
        1
    );
    drop((live_sender, live_creation, dead_creation));
    teardown_is_clean(&fixture, session, runtime, library);
}

#[test]
fn dropping_before_the_callback_cancels_without_waiting() {
    let fixture = Fixture::new();
    fixture.control.set_render_sender_create_delay(200);
    let library = load_stub();
    let (runtime, session) = onboarded_session(&fixture, &library);

    let pending = session
        .start_render_sender(&stereo_color_and_depth(VISUALIZER_PEER), None, || {})
        .expect("start");
    let drop_took = timed_drop(pending);
    assert!(
        drop_took < PROMPT_DROP,
        "the drop blocked for {drop_took:?}"
    );
    wait_for_log_entry(&fixture.control, "render_sender_create_async cancelled");
    let calls = fixture.control.calls();
    assert_eq!(count_of(&calls, "render_sender_create_async delivered"), 0);
    assert_eq!(count_of(&calls, "render_stream_handle_destroy"), 0);
    teardown_is_clean(&fixture, session, runtime, library);
}

#[test]
fn dropping_after_the_callback_destroys_the_unwanted_sender() {
    let fixture = Fixture::new();
    let library = load_stub();
    let (runtime, session) = onboarded_session(&fixture, &library);
    let handles_before = fixture.control.live_handle_count();

    let woken = Arc::new(AtomicBool::new(false));
    let woken_by_callback = Arc::clone(&woken);
    let pending = session
        .start_render_sender(&stereo_color_and_depth(VISUALIZER_PEER), None, move || {
            woken_by_callback.store(true, Ordering::SeqCst);
        })
        .expect("start");
    let deadline = Instant::now() + CALLBACK_DEADLINE;
    while !woken.load(Ordering::SeqCst) {
        assert!(Instant::now() < deadline, "no callback");
        std::thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(
        fixture.control.live_handle_count(),
        handles_before + 2,
        "the delivered sender and the token"
    );
    drop(pending);
    assert_eq!(fixture.control.live_handle_count(), handles_before);
    assert_eq!(
        count_of(&fixture.control.calls(), "render_stream_handle_destroy"),
        1
    );
    teardown_is_clean(&fixture, session, runtime, library);
}

/// A creation that completes although it was cancelled: its sender is
/// destroyed by the session's next start or poll, or by its teardown.
#[test]
fn a_sender_delivered_after_the_drop_is_destroyed_by_the_session() {
    let fixture = Fixture::new();
    fixture.control.set_render_sender_create_delay(30);
    fixture
        .control
        .set_render_sender_create_ignores_cancel(true);
    let library = load_stub();
    let (runtime, session) = onboarded_session(&fixture, &library);
    let handles_before = fixture.control.live_handle_count();

    let abandoned = session
        .start_render_sender(&stereo_color_and_depth(VISUALIZER_PEER), None, || {})
        .expect("start");
    let drop_took = timed_drop(abandoned);
    assert!(
        drop_took < PROMPT_DROP,
        "the drop blocked for {drop_took:?}"
    );
    wait_for_log_entry(&fixture.control, "render_sender_create_async delivered");
    assert_eq!(
        fixture.control.live_handle_count(),
        handles_before + 2,
        "the unwanted sender and the abandoned token, until the session looks again"
    );

    session.release_abandoned_creations();
    assert_eq!(
        count_of(&fixture.control.calls(), "render_stream_handle_destroy"),
        1,
        "the per-frame release destroyed the unwanted sender"
    );
    assert_eq!(
        fixture.control.live_handle_count(),
        handles_before,
        "the unwanted sender and its creation's token are gone"
    );

    fixture
        .control
        .set_render_sender_create_ignores_cancel(false);
    fixture.control.set_render_sender_create_delay(0);
    let mut next = session
        .start_render_sender(&stereo_color_and_depth(VISUALIZER_PEER), None, || {})
        .expect("next start");
    assert_eq!(
        count_of(&fixture.control.calls(), "render_stream_handle_destroy"),
        1,
        "nothing more to destroy at the next start"
    );
    let sender = poll_until_called_back(&mut next).expect("next sender");
    assert_eq!(
        fixture.control.live_handle_count(),
        handles_before + 1,
        "only the wanted sender"
    );
    drop(sender);
    drop(next);

    fixture
        .control
        .set_render_sender_create_ignores_cancel(true);
    fixture.control.set_render_sender_create_delay(30);
    let abandoned_at_teardown = session
        .start_render_sender(&stereo_color_and_depth(VISUALIZER_PEER), None, || {})
        .expect("start");
    let delivered_before = count_of(
        &fixture.control.calls(),
        "render_sender_create_async delivered",
    );
    drop(abandoned_at_teardown);
    wait_for_log_count(
        &fixture.control,
        "render_sender_create_async delivered",
        delivered_before + 1,
    );
    drop(session);
    let calls = fixture.control.calls();
    assert!(
        last_position_of(&calls, "render_stream_handle_destroy")
            < position_of(&calls, "session_handle_destroy"),
        "{calls:?}"
    );
    drop(runtime);
    drop(library);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    assert_eq!(fixture.control.live_handle_count(), 0);
}

#[test]
fn a_pending_creation_keeps_its_session_alive() {
    let fixture = Fixture::new();
    fixture.control.set_render_sender_create_delay(50);
    let library = load_stub();
    let (runtime, session) = onboarded_session(&fixture, &library);

    let mut pending = session
        .start_render_sender(&stereo_color_and_depth(VISUALIZER_PEER), None, || {})
        .expect("start");
    drop(session);
    assert_eq!(
        count_of(&fixture.control.calls(), "session_handle_destroy"),
        0
    );
    let sender = poll_until_called_back(&mut pending).expect("sender");
    assert_eq!(sender.layout().expect("layout").views.len(), 4);
    drop(pending);
    drop(sender);
    let calls = fixture.control.calls();
    assert!(
        position_of(&calls, "render_stream_handle_destroy")
            < position_of(&calls, "session_handle_destroy")
    );
    drop(runtime);
    drop(library);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    assert_eq!(fixture.control.live_handle_count(), 0);
}
