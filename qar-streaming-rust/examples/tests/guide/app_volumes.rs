use std::ffi::c_void;

use qaros::qaros_sys::{
    QarAppVolumeGesture, QarAppVolumeId, QarGeoAnchorFrame, QarGestureKind, QarGesturePhase,
    QarHandedness, QarVector3d, QarWorldReferenceSystem, defaults,
};
use qaros::{
    AppVolumeConfig, AppVolumeId, AppVolumeSize, CancelToken, Library, Pose, Session, Vec3,
};

fn get_or_create(session: &Session) -> qaros::Result<AppVolumeId> {
    // ANCHOR: get_or_create
    // The common name is required: get_or_create returns the existing volume of that name.
    let config = AppVolumeConfig {
        display_name: Some("Assembly View".to_string()),
        // Pose is the cuboid CENTER, room space.
        pose: Pose::from_position(Vec3::new(0.0, 1.2, 0.0)),
        size: AppVolumeSize {
            width: 1.0,
            length: 1.0,
            height: 0.8,
        },
        app_pose: Pose::default(), // app origin inside the box
        app_scale: 0.01,           // 1 app-meter -> 1 room-cm: a 100 m model fits the box
        ..AppVolumeConfig::new("assembly-view.myapp.example.com")
    };

    let volume_id = session.app_volumes().get_or_create(&config)?;
    // ANCHOR_END: get_or_create
    Ok(volume_id)
}

fn world_anchor(
    library: &Library,
    session: &Session,
    volume_id: AppVolumeId,
    origin: QarVector3d,
    [east, up, south]: [QarVector3d; 3],
) {
    // ANCHOR: world_anchor
    // World anchors are raw only: call the raw table with the session's handle.
    let api = library.raw_api();
    let volume = QarAppVolumeId { data: volume_id.0 };
    let frame = QarGeoAnchorFrame {
        world_ref_system: QarWorldReferenceSystem::QAR_WORLD_REF_SYSTEM_ECEF_WGS84_METERS,
        handedness: QarHandedness::QAR_HANDEDNESS_RIGHT, // required: UNSPECIFIED is rejected
        origin_world: origin,                            // meters, ECEF
        axis_x_world: east,                              // unit vectors
        axis_y_world: up,
        axis_z_world: south,
    };
    // SAFETY: the session is alive and used on this thread; the id and frame outlive the call.
    let result = unsafe {
        (api.qar_impl_app_volumes_change_app_world_anchor)(
            session.as_raw(),
            &raw const volume,
            &raw const frame,
        )
    };
    // SAFETY: plain value argument.
    unsafe { (api.qar_impl_result_log_if_error)(result) };
    // ...
    // SAFETY: as above.
    let result = unsafe {
        (api.qar_impl_app_volumes_clear_app_world_anchor)(session.as_raw(), &raw const volume)
    };
    // SAFETY: plain value argument.
    unsafe { (api.qar_impl_result_log_if_error)(result) };
    // ANCHOR_END: world_anchor
}

// ANCHOR: on_gesture
/// Runs on a library thread. `gesture` is valid only during this call: copy what you keep.
/// The library table travels as the user state.
unsafe extern "C" fn on_gesture(gesture: *mut QarAppVolumeGesture, user_state: *mut c_void) {
    // SAFETY: the user state is the table subscribe_gestures passed, alive while the library is.
    let api = unsafe { &*user_state.cast::<qaros::qaros_sys::QarApi>() };
    let mut peer = defaults::qar_peer_id_default();
    let mut volume = defaults::qar_app_volume_id_default();
    let mut kind = QarGestureKind::QAR_GESTURE_CLICK;
    let mut phase = QarGesturePhase::QAR_GESTURE_PHASE_STARTED;
    let (mut action, mut start) = (
        defaults::qar_vector3_default(),
        defaults::qar_vector3_default(),
    );
    let (mut has_action, mut has_start) = (false, false);
    // SAFETY: the gesture is valid during the callback; every out-pointer is valid.
    unsafe {
        (api.qar_impl_app_volume_gesture_get_lifecycle)(gesture, &raw mut peer, &raw mut volume);
        (api.qar_impl_app_volume_gesture_get_classification)(
            gesture,
            &raw mut kind,
            &raw mut phase,
        );
        (api.qar_impl_app_volume_gesture_get_points)(
            gesture,
            &raw mut action,
            &raw mut has_action,
            &raw mut start,
            &raw mut has_start,
        );
    }
    // qar_app_volume_gesture_get_deltas: accumulated deltas in app volume space, plus whether
    // the library already applied them to app_pose/scale
}
// ANCHOR_END: on_gesture

fn subscribe_gestures(
    library: &Library,
    session: &Session,
    volume_id: AppVolumeId,
) -> qaros::Result<CancelToken> {
    // ANCHOR: subscribe_gestures
    // Gestures are raw only: subscribe through the raw table with the session's handle. The
    // returned token owns the subscription.
    let api = library.raw_api();
    let volume = QarAppVolumeId { data: volume_id.0 };
    let subscription = library.create_cancel_token(None)?;
    // SAFETY: the session is alive and used on this thread; the user state (the table) lives
    // as long as the library, which outlives the session.
    let result = unsafe {
        (api.qar_impl_app_volumes_subscribe_all_gesture_events)(
            session.as_raw(),
            &raw const volume,
            Some(on_gesture),
            std::ptr::from_ref(api).cast_mut().cast(),
            subscription.as_raw(),
        )
    };
    // SAFETY: plain value argument.
    unsafe { (api.qar_impl_result_log_if_error)(result) };
    // ANCHOR_END: subscribe_gestures
    Ok(subscription)
}

fn end_gestures(subscription: &CancelToken) -> qaros::Result<()> {
    // ANCHOR: end_gestures
    // Ends the subscription: no gesture callback starts after this returns, and one running on
    // another thread has returned. Dropping the token's last clone ends it as well.
    subscription.cancel()?;
    // ANCHOR_END: end_gestures
    Ok(())
}
