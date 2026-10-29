use std::sync::mpsc::Sender;

use qaros::qaros_sys::{QarDeviceHandsWithJoints, defaults};
use qaros::{
    AppState, AppVolumeId, BlockingOptions, Eye, FrameLayout, FrameViewSpec, Library, PeerId,
    PeerInfo, PixelFormat, RenderSender, RenderSenderConfig, Session, ShowParams, Subscription,
    ViewKind,
};

fn find_targets(session: &Session, render_thread: Sender<PeerId>) -> qaros::Result<Subscription> {
    // ANCHOR: find_targets
    // Runs on a library thread: hand the target to your render thread, which creates its
    // sender (see "Creating a sender" below).
    let on_peer = move |peer: PeerInfo| {
        // The header derives the app state from the presence: Running is ONLINE, or UNKNOWN
        // on a hub that has not reconciled the peer. ShuttingDown is OFFLINE or LEAVING: the
        // peer is gone for now and may come back as Running. Retired never comes back. Serve
        // Running targets, and stop a target's sender when it leaves Running.
        if peer.is_target_app && peer.app_state == AppState::Running {
            let _ = render_thread.send(peer.id);
        }
    };
    let subscription = session.subscribe_peer_updates(on_peer)?;
    // ANCHOR_END: find_targets
    Ok(subscription)
}

fn create_sender(
    session: &Session,
    target_peer_id: PeerId,
    volume_id: AppVolumeId,
) -> qaros::Result<RenderSender> {
    // ANCHOR: create_sender
    // create_render_sender is the CPU path; the Vulkan one is behind the `vulkan` feature.
    let config = RenderSenderConfig {
        auto_reconnect: true,        // survive network hiccups
        app_volume: Some(volume_id), // composite into our app volume
        color_format: PixelFormat::R8G8B8A8,
        depth_format: PixelFormat::R32Float,

        // Describe the views inside one video frame: stereo color + depth.
        layout: FrameLayout::SideBySide,
        views: vec![
            FrameViewSpec {
                kind: ViewKind::Color,
                eye: Eye::Left,
                texture_index: 0,
            },
            // ... one entry per view (right eye, depth views, ...) ...
        ],
        ..RenderSenderConfig::new(target_peer_id) // who receives this stream
    };

    // The sender releases the native sender when it drops.
    let sender = session.create_render_sender(&config, &BlockingOptions::default())?;
    // ANCHOR_END: create_sender
    Ok(sender)
}

fn render_my_scene(_textures: &mut qaros::CpuFrame<'_>, _view: &qaros::ViewPose) {}

fn frame_loop(sender: &mut RenderSender) -> qaros::Result<()> {
    // ANCHOR: frame_loop
    loop {
        // 1. Begin: get the poses and FOVs you must render with.
        let mut frame = match sender.begin_frame(&BlockingOptions::default()) {
            Ok(frame) => frame,
            Err(_reconnecting_or_volume_unresolved) => continue,
        };
        let view = frame.view(0)?; // view.pose, view.fov
        // ... same for the other views ...

        // 2. Get the frame's textures and render into them.
        let mut textures = frame.cpu_frame()?;
        // textures.textures[i].data / .pitch / .size: write your pixels.
        // textures.views[i] is the view you declared at views[i];
        // its texture_index names the texture it lives in.
        render_my_scene(&mut textures, &view);

        // 3. Show: submit with the projection metadata you used.
        frame.show(&ShowParams {
            near: 0.1,
            far: 100.0,
            depth_scale: 1.0,
        })?;
    }
    // ANCHOR_END: frame_loop
}

#[cfg(windows)]
fn d3d11_params(
    device: &windows::Win32::Graphics::Direct3D11::ID3D11Device1,
    context: &windows::Win32::Graphics::Direct3D11::ID3D11DeviceContext1,
) -> qaros::d3d11::D3D11StreamParams {
    use windows::Win32::Graphics::Direct3D11::{
        D3D11_BIND_DEPTH_STENCIL, D3D11_BIND_RENDER_TARGET, D3D11_BIND_SHADER_RESOURCE,
    };
    // ANCHOR: d3d11_params
    // Feature `d3d11`, on Windows. The parameters hold a reference on the device and the
    // context, which every sender created with them keeps.
    let mut d3d = qaros::d3d11::D3D11StreamParams::new(device, context);
    d3d.color_bind_flags = (D3D11_BIND_RENDER_TARGET | D3D11_BIND_SHADER_RESOURCE)
        .0
        .cast_unsigned();
    d3d.depth_bind_flags = D3D11_BIND_DEPTH_STENCIL.0.cast_unsigned();
    d3d.acquire_keyed_mutex_sync = true; // let QAROS manage keyed-mutex sync
    // Then: unsafe { session.create_d3d11_render_sender(&config, d3d, &options) }
    // ANCHOR_END: d3d11_params
    d3d
}

fn last_hands(library: &Library, sender: &RenderSender) {
    // ANCHOR: last_hands
    // Hands are raw only: call the raw table with the sender's handle.
    let api = library.raw_api();
    let mut hands: QarDeviceHandsWithJoints = defaults::qar_device_hand_with_joints_default();
    // SAFETY: the sender is alive and driven on this thread; `hands` is a valid out-pointer.
    let result =
        unsafe { (api.qar_impl_render_sender_last_hands)(sender.as_raw(), &raw mut hands) };
    // SAFETY: plain value argument.
    if unsafe { (api.qar_impl_result_is_success)(result) } {
        // 26 joints per hand, with locations and velocities, in app content space:
        // the same space your view poses are in
    }
    // ANCHOR_END: last_hands
}
