//! (d) Two users against the real runtime: two extension viewer peers, one
//! CPU render receiver each, one render sender per target on the same
//! application session, frames interleaved, each user's picture distinct
//! (its frame channel offset). Then the second user leaves: its receiver and
//! peer are destroyed and the first user keeps streaming. Records what the
//! session's peer list and the hub show for the departed peer, what its
//! sender reports, and whether the receivers' requests reach
//! `qar_render_sender_subscribe_requests`.

#![cfg(feature = "e2e")]
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    clippy::too_many_lines
)]

mod common;

use std::time::{Duration, Instant};

use common::pattern;
use common::scenario::{self, Scenario};
use qar_streaming_rust_ext::RenderReceiver;
use qaros::{AppState, BlockingOptions, RenderSender, ShowParams};

/// The second user's picture counts frames from here, so the two differ.
const SECOND_USER_FRAME_OFFSET: u64 = 100;

struct User {
    name: &'static str,
    receiver: RenderReceiver,
    sender: RenderSender,
    frame_offset: u64,
}

#[test]
fn two_users_get_their_own_streams_and_one_can_leave() {
    let Some(bin) = common::package_bin() else {
        return;
    };
    let (eye_width, eye_height) = common::eye_size_from_environment();
    let frame_total = common::frame_count_from_environment();
    let world = Scenario::start(&bin, "multi-user");
    let (forward, requests) = qaros::channel();
    let _requests_subscription = world
        .session
        .subscribe_render_requests(forward)
        .expect("subscribe to render requests");
    let (second_viewer, second_id) =
        scenario::add_viewer(&world.hub, &world.session, "e2e-viewer-b");
    let layout = world.layout(eye_width, eye_height);

    let cancel = world.cancel_token(Duration::from_secs(60));
    let options = BlockingOptions {
        progress: None,
        cancel: Some(&cancel),
    };
    let mut users = Vec::new();
    for (name, viewer, target, frame_offset) in [
        ("first", &world.viewer, world.viewer_id, 0),
        (
            "second",
            &second_viewer,
            second_id,
            SECOND_USER_FRAME_OFFSET,
        ),
    ] {
        let receiver = viewer
            .create_render_receiver(&layout, None)
            .expect("create a render receiver");
        let config = world.sender_config(target);
        let sender = scenario::connect(&receiver, || {
            world
                .session
                .create_render_sender(&config, &options)
                .expect("create a render sender")
        });
        users.push(User {
            name,
            receiver,
            sender,
            frame_offset,
        });
    }
    let streams = world.hub.media_streams().expect("hub media streams");
    eprintln!("hub media-stream records with two users: {}", streams.len());
    for record in &streams {
        eprintln!(
            "  stream kind {} lifetime {} video {} receivers {}",
            record.kind.0, record.lifetime.0, record.has_video, record.receiver_count
        );
    }

    let started = Instant::now();
    for frame_number in 0..frame_total {
        render_round(&mut users, &layout, frame_number);
    }
    let elapsed = started.elapsed();
    #[allow(clippy::cast_precision_loss)]
    let rate = frame_total as f64 / elapsed.as_secs_f64();
    eprintln!(
        "RATE two users, CPU path, {eye_width}x{eye_height} per eye: {frame_total} rounds of two frames in {:.2} s = {rate:.1} rounds/s",
        elapsed.as_secs_f64()
    );

    let delivered: Vec<_> = requests.try_iter().collect();
    eprintln!(
        "render requests delivered to subscribe_render_requests: {}",
        delivered.len()
    );
    for request in &delivered {
        eprintln!(
            "  request from {} stream {}",
            request.target_peer, request.stream_id
        );
    }

    let second = users.pop().expect("the second user");
    let left_at = Instant::now();
    drop(second.receiver);
    drop(second_viewer);
    let gone = common::wait_for(common::scaled(Duration::from_secs(10)), || {
        let peers = world.session.peers().expect("peer list");
        (!peers.iter().any(|peer| peer.id == second_id)).then_some(())
    });
    let listed_after_leaving = world
        .session
        .peers()
        .expect("peer list")
        .into_iter()
        .find(|peer| peer.id == second_id);
    let hub_lists_it = world
        .hub
        .find_peer_display_name(&common::qaros_sys_peer_id(second_id.0))
        .expect("hub lookup");
    eprintln!(
        "second user left {:?} ago: gone from the session's peers: {}; session entry: {listed_after_leaving:?}; hub lists it as: {hub_lists_it:?}",
        left_at.elapsed(),
        gone.is_some()
    );
    assert!(
        listed_after_leaving
            .as_ref()
            .is_none_or(|peer| peer.app_state == AppState::ShuttingDown),
        "a departed viewer is gone or shutting down"
    );
    let mut second_sender = second.sender;
    match second_sender.try_begin_owned_frame() {
        Ok(Some(_)) => eprintln!("the departed user's sender still begins frames"),
        Ok(None) => eprintln!("the departed user's sender has no frame ready (Ok(None))"),
        Err(error) => eprintln!("the departed user's sender reports: {error}"),
    }
    drop(second_sender);

    for frame_number in frame_total..frame_total + 10 {
        render_round(&mut users, &layout, frame_number);
    }
    eprintln!("the first user kept streaming after the second left");
    drop(users);
}

/// One frame for every user: lend and request for all, then begin, fill and
/// show per sender, then receive and check per receiver.
fn render_round(
    users: &mut [User],
    layout: &qaros::qaros_sys::QarVideoFrameLayout,
    frame_number: u64,
) {
    for user in users.iter_mut() {
        let accepted = common::wait_for(common::scaled(Duration::from_secs(5)), || {
            user.receiver
                .provide_cpu_frame()
                .expect("lend CPU buffers")
                .then_some(())
        });
        assert!(accepted.is_some(), "{} user: buffers accepted", user.name);
        user.receiver
            .request_render(common::head_pose(frame_number), Some(frame_number))
            .expect("request a render");
    }
    for user in users.iter_mut() {
        let mut frame = user
            .sender
            .begin_frame(&BlockingOptions::default())
            .expect("begin a frame");
        {
            let mut cpu_frame = frame.cpu_frame().expect("CPU frame");
            pattern::fill_cpu_frame(&mut cpu_frame, frame_number + user.frame_offset);
        }
        frame.show(&ShowParams::default()).expect("show");
    }
    for user in users.iter() {
        let info = user
            .receiver
            .receive_frame(common::scaled(Duration::from_secs(5)))
            .expect("receive")
            .unwrap_or_else(|| panic!("{} user: frame {frame_number} arrives", user.name));
        assert!(info.valid_frame);
        assert_eq!(
            info.sequence_number, frame_number,
            "{} user's sequence",
            user.name
        );
        let buffers = user.receiver.cpu_buffers().expect("CPU buffers");
        let textures: Vec<(&[u8], usize)> = (0..layout.textures_count)
            .map(|index| {
                (
                    buffers.texture(index).unwrap_or_default(),
                    buffers.pitch(index).unwrap_or_default() as usize,
                )
            })
            .collect();
        let verdict = pattern::check_frame(layout, &textures, frame_number + user.frame_offset);
        assert!(
            verdict.mismatches.is_empty(),
            "{} user, frame {frame_number}: {:?}",
            user.name,
            verdict.mismatches
        );
    }
}
