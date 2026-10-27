//! Ports of the header's `static inline` `qar_*_default()` initializers.
//!
//! These functions do not exist in the binary, so every language binding
//! must replicate them. Each port matches the header's initializer field by
//! field, which the `qaros-conformance` tests check by compiling the C
//! originals. Members the C initializers zero-fill (`{ 0 }` or `{}`) are
//! zero here too.

use std::ptr;

#[allow(clippy::wildcard_imports)]
use crate::types::*;

/// Plain C data for which the all-zero bit pattern is a valid value: integers,
/// floats, `bool`, raw pointers, `Option` of function pointers and the
/// newtype enums.
///
/// # Safety
///
/// Implement only for `#[repr(C)]` types whose every field satisfies the
/// above.
unsafe trait ZeroInitializable {}

macro_rules! zero_initializable {
    ($($type_name:ty),* $(,)?) => {
        $(
            // SAFETY: generated from the header; fields are integers, floats,
            // bools, raw pointers or nested types listed here.
            unsafe impl ZeroInitializable for $type_name {}
        )*
    };
}

zero_initializable!(
    QarVideoFrameView,
    QarTextureSize,
    QarVideoTextureCpu,
    QarRenderFrameView,
    QarGeoAnchorFrame,
    QarAppVolumeGestureMappingRule,
    QarHandJointLocation,
    QarHandJointVelocity,
    QarVideoTextureVulkan,
    QarLuid,
);

#[cfg(any(target_os = "windows", target_os = "macos"))]
zero_initializable!(QarVideoTextureD3D11, QarDXGIKeyedMutexSync);

fn zeroed<T: ZeroInitializable>() -> T {
    // SAFETY: ZeroInitializable guarantees the all-zero pattern is valid.
    unsafe { std::mem::zeroed() }
}

/// Header stamp `{ type, NULL }`.
pub fn structure_header(structure_type: QarStructureType) -> QarStructureHeader {
    QarStructureHeader {
        type_: structure_type,
        next: ptr::null_mut(),
    }
}

const ZERO_ID: [u8; 16] = [0; 16];

/// C enums are `int` under MSVC and `unsigned int` under clang outside Windows, so
/// the newtype's inner type differs per target; the mask field is `uint32_t`.
#[allow(clippy::unnecessary_cast, clippy::cast_sign_loss)]
fn axis_mask(flags: QarAppVolumeAxisFlags) -> u32 {
    flags.0 as u32
}

pub fn qar_time_point_default() -> QarTimePoint {
    QarTimePoint { unix_ns: 0 }
}

pub fn qar_vector3_default() -> QarVector3 {
    QarVector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }
}

pub fn qar_quaternion_default() -> QarQuaternion {
    QarQuaternion {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    }
}

pub fn qar_pose_default() -> QarPose {
    QarPose {
        orientation: qar_quaternion_default(),
        position: qar_vector3_default(),
    }
}

/// The header's comment says "all zeros"; the initializer sets ±0.785 rad.
/// This port follows the initializer.
pub fn qar_fov_default() -> QarFov {
    QarFov {
        angle_left: -0.785,
        angle_right: 0.785,
        angle_up: 0.785,
        angle_down: -0.785,
    }
}

/// The header's comment says "zeros"; the initializer sets 0.1 and 100.
/// This port follows the initializer.
pub fn qar_near_far_default() -> QarNearFar {
    QarNearFar {
        near_plane: 0.1,
        far_plane: 100.0,
    }
}

pub fn qar_peer_id_default() -> QarPeerId {
    QarPeerId { data: ZERO_ID }
}

pub fn qar_session_id_default() -> QarSessionId {
    QarSessionId { data: ZERO_ID }
}

pub fn qar_gui_panel_id_default() -> QarGuiPanelId {
    QarGuiPanelId { data: ZERO_ID }
}

pub fn qar_app_volume_id_default() -> QarAppVolumeId {
    QarAppVolumeId { data: ZERO_ID }
}

pub fn qar_stream_id_default() -> QarStreamId {
    QarStreamId { data: ZERO_ID }
}

pub fn qar_onboarding_id_default() -> QarOnboardingId {
    QarOnboardingId { data: ZERO_ID }
}

pub fn qar_hand_joint_location_default() -> QarHandJointLocation {
    QarHandJointLocation {
        joint_id: 0,
        location_flags: 0,
        pose: qar_pose_default(),
        radius: 0.0,
    }
}

pub fn qar_hand_joint_velocity_default() -> QarHandJointVelocity {
    QarHandJointVelocity {
        joint_id: 0,
        flags: 0,
        linear_velocity: qar_vector3_default(),
        angular_velocity: qar_vector3_default(),
    }
}

pub fn qar_hand_joints_default() -> QarHandJoints {
    QarHandJoints {
        is_tracked: false,
        is_active: false,
        pose: qar_pose_default(),
        joint_locations: [qar_hand_joint_location_default(); QAR_HAND_JOINT_COUNT as usize],
        has_velocity: false,
        joint_velocities: [qar_hand_joint_velocity_default(); QAR_HAND_JOINT_COUNT as usize],
    }
}

pub fn qar_device_hand_with_joints_default() -> QarDeviceHandsWithJoints {
    QarDeviceHandsWithJoints {
        device_id: qar_peer_id_default(),
        device_timestamp: qar_time_point_default(),
        left_hand: qar_hand_joints_default(),
        right_hand: qar_hand_joints_default(),
    }
}

pub fn qar_render_frame_view_default() -> QarRenderFrameView {
    QarRenderFrameView {
        data_type: QarVideoFrameViewType::QAR_VIDEO_FRAME_VIEW_TYPE_GRAYSCALE,
        eye: QarVideoFrameViewEye::QAR_VIDEO_FRAME_VIEW_EYE_NONE,
        texture_index: 0,
    }
}

pub fn qar_image_size_default() -> QarImageSize {
    QarImageSize {
        width: 0,
        height: 0,
    }
}

pub fn qar_texture_size_default() -> QarTextureSize {
    QarTextureSize {
        format: QarPixelFormat::QAR_PIXEL_FORMAT_B8G8R8A8,
        width: 0,
        height: 0,
        array_layers: 1,
    }
}

pub fn qar_video_frame_view_default() -> QarVideoFrameView {
    QarVideoFrameView {
        start_x: 0,
        start_y: 0,
        end_x: 0,
        end_y: 0,
        texture_index: 0,
        array_layer_index: 0,
        texture_format: QarPixelFormat::QAR_PIXEL_FORMAT_B8G8R8A8,
        data_type: QarVideoFrameViewType::QAR_VIDEO_FRAME_VIEW_TYPE_COLOR,
        eye: QarVideoFrameViewEye::QAR_VIDEO_FRAME_VIEW_EYE_NONE,
    }
}

/// Views and textures are zero-filled, as in the header (not set to their
/// own defaults).
pub fn qar_video_frame_layout_default() -> QarVideoFrameLayout {
    QarVideoFrameLayout {
        views: [zeroed(); QAR_MAX_FRAME_VIEWS as usize],
        views_count: 0,
        textures: [zeroed(); QAR_MAX_FRAME_TEXTURES as usize],
        textures_count: 0,
    }
}

pub fn qar_video_texture_cpu_default() -> QarVideoTextureCpu {
    QarVideoTextureCpu {
        size: qar_texture_size_default(),
        pitch: 0,
        texture_data: ptr::null_mut(),
        texture_data_size: 0,
    }
}

/// Views and textures are zero-filled, as in the header.
pub fn qar_video_frame_cpu_default() -> QarVideoFrameCpu {
    QarVideoFrameCpu {
        texture_views: [zeroed(); QAR_MAX_FRAME_VIEWS as usize],
        texture_views_count: 0,
        textures: [zeroed(); QAR_MAX_FRAME_TEXTURES as usize],
        textures_count: 0,
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn qar_video_texture_d3d11_default() -> QarVideoTextureD3D11 {
    QarVideoTextureD3D11 {
        size: qar_texture_size_default(),
        texture: ptr::null_mut(),
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn qar_dxgi_keyed_mutex_sync_default() -> QarDXGIKeyedMutexSync {
    QarDXGIKeyedMutexSync {
        mutex: ptr::null_mut(),
        acquire: 0,
        release: 0,
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn qar_sync_frame_d3d11_default() -> QarSyncFrameD3D11 {
    QarSyncFrameD3D11 {
        texture_sync: [zeroed(); QAR_MAX_FRAME_TEXTURES as usize],
        texture_sync_count: 0,
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn qar_video_frame_d3d11_default() -> QarVideoFrameD3D11 {
    QarVideoFrameD3D11 {
        texture_views: [zeroed(); QAR_MAX_FRAME_VIEWS as usize],
        texture_views_count: 0,
        textures: [zeroed(); QAR_MAX_FRAME_TEXTURES as usize],
        textures_count: 0,
        synchronization: qar_sync_frame_d3d11_default(),
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn qar_stream_params_d3d11_default() -> QarStreamParamsD3D11 {
    QarStreamParamsD3D11 {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_STREAM_D3D11_PARAMS_EXT),
        d3d11_context: ptr::null_mut(),
        d3d11_device: ptr::null_mut(),
        color_bind_flags: 0,
        depth_bind_flags: 0,
        acquire_keyed_mutex_sync: true,
    }
}

pub fn qar_gui_panel_size_default() -> QarGuiPanelSize {
    QarGuiPanelSize {
        width_meters: 1.0,
        height_meters: 1.0,
        content_scale: 1.0,
    }
}

pub fn qar_app_volume_size_default() -> QarAppVolumeSize {
    QarAppVolumeSize {
        width_meters: 1.0,
        length_meters: 1.0,
        height_meters: 1.0,
    }
}

pub fn qar_app_volume_editing_status_default() -> QarAppVolumeEditingStatus {
    QarAppVolumeEditingStatus {
        is_being_edited: false,
        editor_peer: qar_peer_id_default(),
    }
}

pub fn qar_app_volume_gesture_mapping_rule_default() -> QarAppVolumeGestureMappingRule {
    QarAppVolumeGestureMappingRule {
        header: structure_header(
            QarStructureType::QAR_STRUCTURE_TYPE_APP_VOLUME_GESTURE_MAPPING_RULE,
        ),
        gesture_kind: QarGestureKind::QAR_GESTURE_CLICK,
        enabled: true,
        precision: 1.0,
        app_scale_sensitivity_mode: QarAppScaleSensitivityMode::QAR_APP_SCALE_SENSITIVITY_CONSTANT,
        translation_axes: axis_mask(QarAppVolumeAxisFlags::QAR_APP_VOLUME_AXIS_ALL),
        rotation_axes: axis_mask(QarAppVolumeAxisFlags::QAR_APP_VOLUME_AXIS_ALL),
        enable_gestures_from_outside_of_volume: true,
        controller_ray_style: QarAppVolumeControllerRayStyle {
            line_style:
                QarAppVolumeControllerRayLineStyle::QAR_APP_VOLUME_CONTROLLER_RAY_LINE_STYLE_SOLID,
            color: QarColor {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
        },
        app_transform_mapping:
            QarGestureAppTransformMapping::QAR_GESTURE_APP_TRANSFORM_MAPPING_NONE,
    }
}

pub fn qar_app_volume_gesture_configuration_default() -> QarAppVolumeGestureConfiguration {
    const MANIPULATION_GREEN: QarColor = QarColor {
        r: 0,
        g: 200,
        b: 0,
        a: 255,
    };
    const CLICK_LIGHT_GREEN: QarColor = QarColor {
        r: 144,
        g: 238,
        b: 144,
        a: 255,
    };

    let mut configuration = QarAppVolumeGestureConfiguration {
        header: structure_header(
            QarStructureType::QAR_STRUCTURE_TYPE_APP_VOLUME_GESTURE_CONFIGURATION,
        ),
        mapping_rules: [zeroed(); QAR_MAX_APP_VOLUME_GESTURE_MAPPING_RULES as usize],
        mapping_rule_count: 4,
    };

    let mut scale_rule = qar_app_volume_gesture_mapping_rule_default();
    scale_rule.gesture_kind = QarGestureKind::QAR_GESTURE_DUAL_POINTER_TRANSLATE_DISTANCE;
    scale_rule.precision = 1.0;
    scale_rule.app_scale_sensitivity_mode =
        QarAppScaleSensitivityMode::QAR_APP_SCALE_SENSITIVITY_BASED_ON_APP_SCALE;
    scale_rule.rotation_axes = axis_mask(QarAppVolumeAxisFlags::QAR_APP_VOLUME_AXIS_NONE);
    scale_rule.app_transform_mapping =
        QarGestureAppTransformMapping::QAR_GESTURE_APP_TRANSFORM_MAPPING_APP_SCALE;
    scale_rule.controller_ray_style.color = MANIPULATION_GREEN;
    configuration.mapping_rules[0] = scale_rule;

    let mut pose_rule = qar_app_volume_gesture_mapping_rule_default();
    pose_rule.gesture_kind = QarGestureKind::QAR_GESTURE_SINGLE_POINTER_6DOF;
    pose_rule.precision = 1.0;
    pose_rule.rotation_axes = axis_mask(QarAppVolumeAxisFlags::QAR_APP_VOLUME_AXIS_NONE);
    pose_rule.app_transform_mapping =
        QarGestureAppTransformMapping::QAR_GESTURE_APP_TRANSFORM_MAPPING_APP_POSE;
    pose_rule.controller_ray_style.color = MANIPULATION_GREEN;
    configuration.mapping_rules[1] = pose_rule;

    let mut hover_rule = qar_app_volume_gesture_mapping_rule_default();
    hover_rule.gesture_kind = QarGestureKind::QAR_GESTURE_HOVER;
    hover_rule.controller_ray_style.line_style =
        QarAppVolumeControllerRayLineStyle::QAR_APP_VOLUME_CONTROLLER_RAY_LINE_STYLE_DOTTED;
    configuration.mapping_rules[2] = hover_rule;

    let mut click_rule = qar_app_volume_gesture_mapping_rule_default();
    click_rule.gesture_kind = QarGestureKind::QAR_GESTURE_CLICK;
    click_rule.controller_ray_style.color = CLICK_LIGHT_GREEN;
    configuration.mapping_rules[3] = click_rule;

    configuration
}

pub fn qar_library_init_default() -> QarLibraryInit {
    QarLibraryInit {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_LIBRARY_INIT),
        enable_console_logging: true,
        log_folder_path: ptr::null(),
        log_severity: QarLogSeverity::QAR_LOG_SEVERITY_INFO,
    }
}

pub fn qar_runtime_init_default() -> QarRuntimeInit {
    QarRuntimeInit {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_INIT),
        runtime_binaries_folder_path: ptr::null(),
        storage_folder_path: ptr::null(),
    }
}

pub fn qar_peer_presentation_default() -> QarPeerPresentation {
    QarPeerPresentation {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_PEER_PRESENTATION),
        display_name: ptr::null(),
        app_version: ptr::null(),
        app_custom_peer_info: ptr::null(),
    }
}

pub fn qar_rejoin_init_default() -> QarRejoinInit {
    QarRejoinInit {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_REJOIN_INIT),
        onboarding_id: qar_onboarding_id_default(),
        presentation: qar_peer_presentation_default(),
    }
}

pub fn qar_onboard_init_default() -> QarOnboardInit {
    QarOnboardInit {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_ONBOARD_INIT),
        onboarding_id: qar_onboarding_id_default(),
        presentation: qar_peer_presentation_default(),
    }
}

pub fn qar_onboard_code_ext_default() -> QarOnboardCodeExt {
    QarOnboardCodeExt {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_ONBOARD_CODE_EXT),
        code: ptr::null(),
    }
}

pub fn qar_onboard_host_ext_default() -> QarOnboardHostExt {
    QarOnboardHostExt {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_ONBOARD_HOST_EXT),
        hostname: ptr::null(),
        port: 0,
    }
}

pub fn qar_onboard_invite_ext_default() -> QarOnboardInviteExt {
    QarOnboardInviteExt {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_ONBOARD_INVITE_EXT),
        invite: ptr::null(),
    }
}

pub fn qar_onboard_local_app_ext_default() -> QarOnboardLocalAppExt {
    QarOnboardLocalAppExt {
        header: structure_header(
            QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_ONBOARD_LOCAL_APP_EXT,
        ),
        approval_timeout_ms: 0,
    }
}

pub fn qar_runtime_ephemeral_storage_ext_default() -> QarRuntimeEphemeralStorageExt {
    QarRuntimeEphemeralStorageExt {
        header: structure_header(
            QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_EPHEMERAL_STORAGE_EXT,
        ),
    }
}

/// `app_tag` is required; the caller sets it.
pub fn qar_onboard_local_app_tag_ext_default() -> QarOnboardLocalAppTagExt {
    QarOnboardLocalAppTagExt {
        header: structure_header(
            QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_ONBOARD_LOCAL_APP_TAG_EXT,
        ),
        app_tag: ptr::null(),
    }
}

/// `app_tag` is required; the caller sets it.
pub fn qar_request_invite_init_default() -> QarRequestInviteInit {
    QarRequestInviteInit {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_SESSION_REQUEST_INVITE_INIT),
        app_tag: ptr::null(),
    }
}

pub fn qar_forget_init_default() -> QarForgetInit {
    QarForgetInit {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_FORGET_INIT),
        onboarding_id: qar_onboarding_id_default(),
    }
}

/// `peer_id` is required (the zero id is refused); the caller sets it.
pub fn qar_onboard_peer_id_ext_default() -> QarOnboardPeerIdExt {
    QarOnboardPeerIdExt {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_ONBOARD_PEER_ID_EXT),
        peer_id: qar_peer_id_default(),
    }
}

/// Category `QAR_DEVICE_CATEGORY_DESKTOP_PC`, NULL strings; `device_id` is required.
#[cfg(qaros_device_identity)]
pub fn qar_peer_device_identity_default() -> QarPeerDeviceIdentity {
    QarPeerDeviceIdentity {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_PEER_DEVICE_IDENTITY),
        category: QarDeviceCategory::QAR_DEVICE_CATEGORY_DESKTOP_PC,
        device_id: ptr::null(),
        device_family: ptr::null(),
        device_name: ptr::null(),
    }
}

/// `connection_string` is required; the caller sets it.
pub fn qar_session_invite_target_app_init_default() -> QarSessionInviteTargetAppInit {
    QarSessionInviteTargetAppInit {
        header: structure_header(
            QarStructureType::QAR_STRUCTURE_TYPE_SESSION_INVITE_TARGET_APP_INIT,
        ),
        connection_string: ptr::null(),
    }
}

/// Zero fields select the defaults: 24 h, one use, join, rejoin and bring devices.
#[cfg(qaros_member_hubs)]
pub fn qar_member_hub_invitation_init_default() -> QarMemberHubInvitationInit {
    QarMemberHubInvitationInit {
        header: structure_header(
            QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_CREATE_MEMBER_HUB_INVITATION_INIT,
        ),
        lifetime_seconds: 0,
        max_uses: 0,
        permissions: 0,
        label: ptr::null(),
    }
}

/// `app_tag` is required; the caller sets it.
#[cfg(qaros_member_hubs)]
pub fn qar_member_hub_invitation_app_grant_ext_default() -> QarMemberHubInvitationAppGrantExt {
    QarMemberHubInvitationAppGrantExt {
        header: structure_header(
            QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_MEMBER_HUB_INVITATION_APP_GRANT_EXT,
        ),
        app_tag: ptr::null(),
    }
}

/// `invitation` is required; the caller sets it.
#[cfg(qaros_member_hubs)]
pub fn qar_onboard_remote_app_grant_ext_default() -> QarOnboardRemoteAppGrantExt {
    QarOnboardRemoteAppGrantExt {
        header: structure_header(
            QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_ONBOARD_REMOTE_APP_GRANT_EXT,
        ),
        invitation: ptr::null(),
    }
}

/// `invitation` is required; the zero timeouts select 60 s and 120 s.
#[cfg(qaros_member_hubs)]
pub fn qar_join_hub_as_member_init_default() -> QarJoinHubAsMemberInit {
    QarJoinHubAsMemberInit {
        header: structure_header(
            QarStructureType::QAR_STRUCTURE_TYPE_RUNTIME_JOIN_HUB_AS_MEMBER_INIT,
        ),
        invitation: ptr::null(),
        hub_start_timeout_ms: 0,
        restart_timeout_ms: 0,
    }
}

/// Frame views are zero-filled, as in the header.
pub fn qar_render_sender_init_default() -> QarRenderSenderInit {
    QarRenderSenderInit {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_RENDERING_STREAM_SENDER_INIT),
        peer_id: qar_peer_id_default(),
        enable_auto_reconnects: true,
        texture_layout: QarFrameLayout::QAR_FRAME_LAYOUT_SIDE_BY_SIDE,
        frame_views: [zeroed(); QAR_MAX_FRAME_VIEWS as usize],
        frame_views_count: 0,
        app_volume_id: ptr::null(),
        color_format: QarPixelFormat::QAR_PIXEL_FORMAT_B8G8R8A8,
        depth_format: QarPixelFormat::QAR_PIXEL_FORMAT_D32_FLOAT,
        graphics_api: QarGraphicsAPI::QAR_GRAPHICS_API_CPU,
    }
}

#[cfg(qaros_color_extensions)]
pub fn qar_render_sender_alpha_mode_ext_default() -> QarRenderSenderAlphaModeExt {
    QarRenderSenderAlphaModeExt {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_RENDER_SENDER_ALPHA_MODE_EXT),
        alpha_mode: QarColorAlphaMode::QAR_COLOR_ALPHA_MODE_UNSPECIFIED,
    }
}

#[cfg(qaros_color_extensions)]
pub fn qar_render_sender_color_encoding_ext_default() -> QarRenderSenderColorEncodingExt {
    QarRenderSenderColorEncodingExt {
        header: structure_header(
            QarStructureType::QAR_STRUCTURE_TYPE_RENDER_SENDER_COLOR_ENCODING_EXT,
        ),
        color_encoding: QarColorEncoding::QAR_COLOR_ENCODING_SRGB,
    }
}

/// The zero stream id; the caller sets it.
pub fn qar_gui_panel_stream_id_ext_default() -> QarGuiPanelStreamIdExt {
    QarGuiPanelStreamIdExt {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_GUI_PANEL_STREAM_ID_EXT),
        stream_id: qar_stream_id_default(),
    }
}

pub fn qar_gui_panel_url_ext_default() -> QarGuiPanelUrlExt {
    QarGuiPanelUrlExt {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_GUI_PANEL_URL_EXT),
        url: ptr::null(),
    }
}

/// No peers: the caller sets `allowed_peers` and `allowed_peer_count`.
pub fn qar_app_volume_gesture_allowed_peers_ext_default() -> QarAppVolumeGestureAllowedPeersExt {
    QarAppVolumeGestureAllowedPeersExt {
        header: structure_header(
            QarStructureType::QAR_STRUCTURE_TYPE_APP_VOLUME_GESTURE_ALLOWED_PEERS_EXT,
        ),
        allowed_peers: ptr::null(),
        allowed_peer_count: 0,
    }
}

/// Reconnects on, 300 s reconnect timeout, CPU frames.
pub fn qar_video_sender_init_default() -> QarVideoSenderInit {
    QarVideoSenderInit {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_VIDEO_SENDER_INIT),
        layout: qar_video_frame_layout_default(),
        enable_reconnects: true,
        reconnect_timeout_ms: 300_000,
        graphics_api: QarGraphicsAPI::QAR_GRAPHICS_API_CPU,
    }
}

pub fn qar_gui_panel_init_default() -> QarGuiPanelInit {
    QarGuiPanelInit {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_GUI_PANEL_INIT),
        common_name: ptr::null(),
        display_name: ptr::null(),
        pose: qar_pose_default(),
        size: qar_gui_panel_size_default(),
        visible_to_peers: ptr::null(),
        visible_to_peer_count: 0,
    }
}

/// The world anchor is zero-filled, as in the header.
pub fn qar_app_volume_init_default() -> QarAppVolumeInit {
    QarAppVolumeInit {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_APP_VOLUME_INIT),
        common_name: ptr::null(),
        display_name: ptr::null(),
        pose: qar_pose_default(),
        size: qar_app_volume_size_default(),
        initial_peers: ptr::null_mut(),
        initial_peer_count: 0,
        app_pose: qar_pose_default(),
        app_scale: 1.0,
        has_app_world_anchor: false,
        app_world_anchor: zeroed(),
        gesture_configuration: ptr::null(),
    }
}

/// The header stamps this structure with `QAR_STRUCTURE_TYPE_RENDERING_END_FRAME`.
pub fn qar_render_frame_show_default() -> QarRenderFrameShow {
    QarRenderFrameShow {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_RENDERING_END_FRAME),
        rendered_near_far: qar_near_far_default(),
        depth_scale: 1.0,
    }
}

pub fn qar_session_lifecycle_info_default() -> QarSessionLifecycleInfo {
    QarSessionLifecycleInfo {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_SESSION_LIFECYCLE_INFO),
        state: QarSessionLifecycleState::QAR_SESSION_LIFECYCLE_STATE_CONNECTED,
        cause: QarSessionChangeCause::QAR_SESSION_CHANGE_CAUSE_NONE,
        session_id: qar_session_id_default(),
        previous_session_id: qar_session_id_default(),
        has_previous_session: false,
        reason: ptr::null(),
    }
}

/// The header leaves the id empty and picks the id type by platform: LUID on
/// Windows, UUID elsewhere.
pub fn qar_graphics_device_id_default() -> QarGraphicsDeviceId {
    QarGraphicsDeviceId {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_SESSION_GRAPHICS_DEVICE_ID),
        id_type: if cfg!(target_os = "windows") {
            QarGpuDeviceIdType::QAR_GPU_DEVICE_ID_TYPE_LUID
        } else {
            QarGpuDeviceIdType::QAR_GPU_DEVICE_ID_TYPE_UUID
        },
        luid: zeroed(),
        uuid: ZERO_ID,
    }
}

/// Null `VkPhysicalDevice` and `VkDevice`.
pub fn qar_stream_params_vulkan_default() -> QarStreamParamsVulkan {
    QarStreamParamsVulkan {
        header: structure_header(QarStructureType::QAR_STRUCTURE_TYPE_STREAM_VULKAN_PARAMS_EXT),
        physical_device: ptr::null_mut(),
        device: ptr::null_mut(),
    }
}

/// `VK_NULL_HANDLE` semaphore and fence.
pub fn qar_sync_frame_vulkan_default() -> QarSyncFrameVulkan {
    QarSyncFrameVulkan {
        semaphore: 0,
        fence: 0,
    }
}

pub fn qar_video_frame_vulkan_default() -> QarVideoFrameVulkan {
    QarVideoFrameVulkan {
        texture_views: [zeroed(); QAR_MAX_FRAME_VIEWS as usize],
        texture_views_count: 0,
        textures: [zeroed(); QAR_MAX_FRAME_TEXTURES as usize],
        textures_count: 0,
        synchronization: qar_sync_frame_vulkan_default(),
    }
}
