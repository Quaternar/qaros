/* Exposes the header's static inline default initializers as real symbols so
 * the Rust ports can be compared against them. */
#define QAR_ENABLE_VULKAN
#include "qar_streaming.h"

#define EXPORT_DEFAULT(TYPE, NAME)                                             \
    void conformance_##NAME(TYPE* out)                                         \
    {                                                                          \
        *out = NAME();                                                         \
    }

EXPORT_DEFAULT(QarTimePoint, qar_time_point_default)
EXPORT_DEFAULT(QarVector3, qar_vector3_default)
EXPORT_DEFAULT(QarQuaternion, qar_quaternion_default)
EXPORT_DEFAULT(QarPose, qar_pose_default)
EXPORT_DEFAULT(QarFov, qar_fov_default)
EXPORT_DEFAULT(QarNearFar, qar_near_far_default)
EXPORT_DEFAULT(QarPeerId, qar_peer_id_default)
EXPORT_DEFAULT(QarSessionId, qar_session_id_default)
EXPORT_DEFAULT(QarGuiPanelId, qar_gui_panel_id_default)
EXPORT_DEFAULT(QarAppVolumeId, qar_app_volume_id_default)
EXPORT_DEFAULT(QarStreamId, qar_stream_id_default)
EXPORT_DEFAULT(QarOnboardingId, qar_onboarding_id_default)
EXPORT_DEFAULT(QarHandJointLocation, qar_hand_joint_location_default)
EXPORT_DEFAULT(QarHandJointVelocity, qar_hand_joint_velocity_default)
EXPORT_DEFAULT(QarHandJoints, qar_hand_joints_default)
EXPORT_DEFAULT(QarDeviceHandsWithJoints, qar_device_hand_with_joints_default)
EXPORT_DEFAULT(QarRenderFrameView, qar_render_frame_view_default)
EXPORT_DEFAULT(QarImageSize, qar_image_size_default)
EXPORT_DEFAULT(QarTextureSize, qar_texture_size_default)
EXPORT_DEFAULT(QarVideoFrameView, qar_video_frame_view_default)
EXPORT_DEFAULT(QarVideoFrameLayout, qar_video_frame_layout_default)
EXPORT_DEFAULT(QarVideoTextureCpu, qar_video_texture_cpu_default)
EXPORT_DEFAULT(QarVideoFrameCpu, qar_video_frame_cpu_default)
EXPORT_DEFAULT(QarGuiPanelSize, qar_gui_panel_size_default)
EXPORT_DEFAULT(QarAppVolumeSize, qar_app_volume_size_default)
EXPORT_DEFAULT(QarAppVolumeEditingStatus, qar_app_volume_editing_status_default)
EXPORT_DEFAULT(QarAppVolumeGestureMappingRule, qar_app_volume_gesture_mapping_rule_default)
EXPORT_DEFAULT(QarAppVolumeGestureConfiguration, qar_app_volume_gesture_configuration_default)
EXPORT_DEFAULT(QarLibraryInit, qar_library_init_default)
EXPORT_DEFAULT(QarRuntimeInit, qar_runtime_init_default)
EXPORT_DEFAULT(QarPeerPresentation, qar_peer_presentation_default)
EXPORT_DEFAULT(QarRejoinInit, qar_rejoin_init_default)
EXPORT_DEFAULT(QarOnboardInit, qar_onboard_init_default)
EXPORT_DEFAULT(QarOnboardCodeExt, qar_onboard_code_ext_default)
EXPORT_DEFAULT(QarOnboardHostExt, qar_onboard_host_ext_default)
EXPORT_DEFAULT(QarOnboardInviteExt, qar_onboard_invite_ext_default)
EXPORT_DEFAULT(QarOnboardLocalAppExt, qar_onboard_local_app_ext_default)
EXPORT_DEFAULT(QarRuntimeEphemeralStorageExt,
               qar_runtime_ephemeral_storage_ext_default)
EXPORT_DEFAULT(QarOnboardLocalAppTagExt, qar_onboard_local_app_tag_ext_default)
EXPORT_DEFAULT(QarRequestInviteInit, qar_request_invite_init_default)
EXPORT_DEFAULT(QarForgetInit, qar_forget_init_default)
EXPORT_DEFAULT(QarRenderSenderInit, qar_render_sender_init_default)
EXPORT_DEFAULT(QarRenderSenderAlphaModeExt,
               qar_render_sender_alpha_mode_ext_default)
EXPORT_DEFAULT(QarRenderSenderColorEncodingExt,
               qar_render_sender_color_encoding_ext_default)
EXPORT_DEFAULT(QarGuiPanelInit, qar_gui_panel_init_default)
EXPORT_DEFAULT(QarAppVolumeInit, qar_app_volume_init_default)
EXPORT_DEFAULT(QarRenderFrameShow, qar_render_frame_show_default)
EXPORT_DEFAULT(QarGraphicsDeviceId, qar_graphics_device_id_default)
EXPORT_DEFAULT(QarSessionLifecycleInfo, qar_session_lifecycle_info_default)
EXPORT_DEFAULT(QarStreamParamsVulkan, qar_stream_params_vulkan_default)
EXPORT_DEFAULT(QarSyncFrameVulkan, qar_sync_frame_vulkan_default)
EXPORT_DEFAULT(QarVideoFrameVulkan, qar_video_frame_vulkan_default)

/* The header enables its D3D11 types on Windows; build.rs enables them on
 * macOS too, where the bindings carry them for qaros-stub. */
#ifdef QAR_ENABLE_D3D11
EXPORT_DEFAULT(QarVideoTextureD3D11, qar_video_texture_d3d11_default)
EXPORT_DEFAULT(QarDXGIKeyedMutexSync, qar_dxgi_keyed_mutex_sync_default)
EXPORT_DEFAULT(QarSyncFrameD3D11, qar_sync_frame_d3d11_default)
EXPORT_DEFAULT(QarVideoFrameD3D11, qar_video_frame_d3d11_default)
EXPORT_DEFAULT(QarStreamParamsD3D11, qar_stream_params_d3d11_default)
#endif
