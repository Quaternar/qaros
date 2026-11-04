use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use qaros::{
    AppState, BlockingOptions, OwnedFrame, PeerId, PeerInfo, RenderLoop, RenderSender,
    RenderSenderConfig, Session, ShowParams, StatusCode, Subscription, ViewPose,
};

const VIEW_COUNT: usize = 2;

fn render_views(_textures: &mut qaros::CpuFrame<'_>, _views: &[ViewPose]) {}
fn render_into_own_target(_target: &mut Vec<u8>, _views: &[ViewPose]) {}
fn copy_into_frame(_textures: &mut qaros::CpuFrame<'_>, _target: &[u8]) {}

fn create_senders(session: &Session, viewer: PeerId) -> qaros::Result<()> {
    // ANCHOR: create_modes
    // Request-driven: the default config chains no extension.
    let request_driven = RenderSenderConfig {
        // ... app volume, formats, views: see Rendering Streams ...
        ..RenderSenderConfig::new(viewer)
    };

    // Free-running: chains QarRenderSenderRenderLoopExt.
    let free_running = RenderSenderConfig {
        render_loop: RenderLoop::FreeRunning,
        ..RenderSenderConfig::new(viewer)
    };

    let sender = session.create_render_sender(&free_running, &BlockingOptions::default())?;
    // ANCHOR_END: create_modes
    drop((request_driven, sender));
    Ok(())
}

// ANCHOR: viewer
/// One viewer: its sender and a frame begun but not rendered yet.
struct Viewer {
    sender: RenderSender,
    ready: Option<OwnedFrame>,
    target: Vec<u8>,
}
// ANCHOR_END: viewer

fn subscribe_viewers(session: &Session) -> qaros::Result<(Subscription, Receiver<PeerInfo>)> {
    // ANCHOR: viewers
    // Library thread -> render thread.
    let (forward, peer_updates) = qaros::channel();
    let subscription = session.subscribe_peer_updates(forward)?;
    // ANCHOR_END: viewers
    Ok((subscription, peer_updates))
}

fn config_for(viewer: PeerId) -> RenderSenderConfig {
    RenderSenderConfig::new(viewer)
}

// ANCHOR: update_viewers
/// Render thread, once per pass: add joined viewers, drop the ones that left.
fn update_viewers(
    session: &Session,
    viewers: &mut HashMap<PeerId, Viewer>,
    peer_updates: &Receiver<PeerInfo>,
) {
    for peer in peer_updates.try_iter().filter(|peer| peer.is_target_app) {
        let online = peer.app_state == AppState::Running;
        if online && !viewers.contains_key(&peer.id) {
            if let Ok(sender) =
                session.create_render_sender(&config_for(peer.id), &BlockingOptions::default())
            {
                let viewer = Viewer {
                    sender,
                    ready: None,
                    target: Vec::new(),
                };
                viewers.insert(peer.id, viewer);
            }
        } else if !online {
            // Dropping releases its frame and sender; its next online update serves it again.
            viewers.remove(&peer.id);
        }
    }
}
// ANCHOR_END: update_viewers

fn views_of(frame: &OwnedFrame) -> qaros::Result<Vec<ViewPose>> {
    (0..VIEW_COUNT).map(|view| frame.view(view)).collect()
}

fn blocking_viewer(sender: &mut RenderSender, leaving: &dyn Fn() -> bool) -> qaros::Result<()> {
    // ANCHOR: blocking
    // One thread per viewer, started when the viewer joins.
    while !leaving() {
        let mut frame = match sender.begin_owned_frame(&BlockingOptions::default()) {
            Ok(frame) => frame,
            // Gone: the thread's owner drops the sender.
            Err(error) if error.has_code(StatusCode::RENDERING_STREAM_IS_CLOSED) => break,
            Err(_reconnecting_or_not_set_up_yet) => continue,
        };
        let views = views_of(&frame)?;
        render_views(&mut frame.cpu_frame()?, &views); // your renderer
        frame.show(&ShowParams::default())?; // also when rendering failed
    }
    // ANCHOR_END: blocking
    Ok(())
}

// ANCHOR: non_blocking
enum Poll {
    Frame(OwnedFrame),
    NotNow,
    Closed,
}

/// Never blocks.
fn poll_frame(viewer: &mut Viewer) -> Poll {
    if let Some(frame) = viewer.ready.take() {
        return Poll::Frame(frame); // came back when the last show armed it
    }
    match viewer.sender.try_begin_owned_frame() {
        Ok(Some(frame)) => Poll::Frame(frame),
        Ok(None) => Poll::NotNow, // busy, paused
        Err(error) if error.has_code(StatusCode::RENDERING_STREAM_IS_CLOSED) => Poll::Closed,
        Err(_reconnecting_or_not_set_up_yet) => Poll::NotNow,
    }
}

fn request_driven_loop(
    session: &Session,
    peer_updates: &Receiver<PeerInfo>,
    running: &dyn Fn() -> bool,
) {
    let mut viewers: HashMap<PeerId, Viewer> = HashMap::new();
    while running() {
        // Your loop, at its own rate.
        update_viewers(session, &mut viewers, peer_updates);
        let mut rendered = false;
        viewers.retain(|_, viewer| {
            let mut frame = match poll_frame(viewer) {
                Poll::Frame(frame) => frame,
                Poll::NotNow => return true,
                Poll::Closed => return false, // drops the viewer's sender
            };
            // Only for a viewer that asked; shown also when rendering failed.
            let views = views_of(&frame);
            if let (Ok(views), Ok(mut textures)) = (views, frame.cpu_frame()) {
                render_views(&mut textures, &views);
            }
            let _ = frame.show(&ShowParams::default());
            // Arm the next wait now, so it runs while the loop is idle.
            if let Poll::Frame(next) = poll_frame(viewer) {
                viewer.ready = Some(next);
            }
            rendered = true;
            true
        });
        if !rendered {
            // Nobody needed a frame: do not spin (or wait for your frame tick).
            std::thread::sleep(Duration::from_millis(1));
        }
    }
}
// ANCHOR_END: non_blocking

fn free_running_loop(
    session: &Session,
    peer_updates: &Receiver<PeerInfo>,
    running: &dyn Fn() -> bool,
) {
    let mut viewers: HashMap<PeerId, Viewer> = HashMap::new();
    // ANCHOR: free_running
    while running() {
        // Your loop, at its own rate, or uncapped.
        update_viewers(session, &mut viewers, peer_updates);
        let mut rendered = false;
        viewers.retain(|_, viewer| {
            let ready = match viewer.sender.is_ready_for_next_frame() {
                Ok(ready) => ready,
                Err(error) if error.has_code(StatusCode::RENDERING_STREAM_IS_CLOSED) => {
                    return false; // drops the viewer's sender
                }
                Err(_) => false,
            };
            if !ready {
                return true; // busy, paused, reconnecting: no pose, no render, no copy
            }
            let Ok(views) = viewer.sender.next_render_pose(VIEW_COUNT) else {
                return true; // no head pose yet, app volume unresolved: next pass
            };
            render_into_own_target(&mut viewer.target, &views);

            // Returns at once with the reserved destination and exactly these poses.
            let Ok(mut frame) = viewer.sender.begin_owned_frame(&BlockingOptions::default()) else {
                return true; // the stream changed state since the check: next pass
            };
            if let Ok(mut textures) = frame.cpu_frame() {
                copy_into_frame(&mut textures, &viewer.target);
            }
            let _ = frame.show(&ShowParams::default());
            rendered = true;
            true
        });
        if !rendered {
            // Nobody was ready: do not spin (or wait for your frame tick).
            std::thread::sleep(Duration::from_millis(1));
        }
    }
    // ANCHOR_END: free_running
}
