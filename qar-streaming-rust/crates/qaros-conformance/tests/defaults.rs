//! Every Rust port in `qaros_sys::defaults` equals the header's C original,
//! field by field (bindgen's derived `PartialEq`). The D3D11 ports exist where
//! the bindings carry the D3D11 types, Windows and macOS, and are checked there.

use std::mem::MaybeUninit;

// Links the crate that carries the compiled C originals.
use qaros_conformance as _;

#[allow(clippy::wildcard_imports)]
use qaros_sys::*;

macro_rules! conformance {
    ($($type_name:ident => $function:ident),* $(,)?) => {
        mod c_originals {
            #[allow(clippy::wildcard_imports)]
            use qaros_sys::*;

            unsafe extern "C" {
                $(
                    #[link_name = concat!("conformance_", stringify!($function))]
                    pub fn $function(out: *mut $type_name);
                )*
            }
        }

        $(
            #[test]
            fn $function() {
                let mut c_value = MaybeUninit::<$type_name>::zeroed();
                // SAFETY: the C function writes a whole $type_name.
                let c_value = unsafe {
                    c_originals::$function(c_value.as_mut_ptr());
                    c_value.assume_init()
                };
                assert_eq!(defaults::$function(), c_value);
            }
        )*
    };
}

conformance!(
    QarTimePoint => qar_time_point_default,
    QarVector3 => qar_vector3_default,
    QarQuaternion => qar_quaternion_default,
    QarPose => qar_pose_default,
    QarFov => qar_fov_default,
    QarNearFar => qar_near_far_default,
    QarPeerId => qar_peer_id_default,
    QarSessionId => qar_session_id_default,
    QarGuiPanelId => qar_gui_panel_id_default,
    QarAppVolumeId => qar_app_volume_id_default,
    QarStreamId => qar_stream_id_default,
    QarOnboardingId => qar_onboarding_id_default,
    QarHandJointLocation => qar_hand_joint_location_default,
    QarHandJointVelocity => qar_hand_joint_velocity_default,
    QarHandJoints => qar_hand_joints_default,
    QarDeviceHandsWithJoints => qar_device_hand_with_joints_default,
    QarRenderFrameView => qar_render_frame_view_default,
    QarImageSize => qar_image_size_default,
    QarTextureSize => qar_texture_size_default,
    QarVideoFrameView => qar_video_frame_view_default,
    QarVideoFrameLayout => qar_video_frame_layout_default,
    QarVideoTextureCpu => qar_video_texture_cpu_default,
    QarVideoFrameCpu => qar_video_frame_cpu_default,
    QarGuiPanelSize => qar_gui_panel_size_default,
    QarAppVolumeSize => qar_app_volume_size_default,
    QarAppVolumeEditingStatus => qar_app_volume_editing_status_default,
    QarAppVolumeGestureMappingRule => qar_app_volume_gesture_mapping_rule_default,
    QarAppVolumeGestureConfiguration => qar_app_volume_gesture_configuration_default,
    QarLibraryInit => qar_library_init_default,
    QarRuntimeInit => qar_runtime_init_default,
    QarPeerPresentation => qar_peer_presentation_default,
    QarRejoinInit => qar_rejoin_init_default,
    QarOnboardInit => qar_onboard_init_default,
    QarOnboardCodeExt => qar_onboard_code_ext_default,
    QarOnboardHostExt => qar_onboard_host_ext_default,
    QarOnboardInviteExt => qar_onboard_invite_ext_default,
    QarOnboardLocalAppExt => qar_onboard_local_app_ext_default,
    QarRuntimeEphemeralStorageExt => qar_runtime_ephemeral_storage_ext_default,
    QarOnboardLocalAppTagExt => qar_onboard_local_app_tag_ext_default,
    QarRequestInviteInit => qar_request_invite_init_default,
    QarForgetInit => qar_forget_init_default,
    QarOnboardPeerIdExt => qar_onboard_peer_id_ext_default,
    QarSessionInviteTargetAppInit => qar_session_invite_target_app_init_default,
    QarRenderSenderInit => qar_render_sender_init_default,
    QarGuiPanelStreamIdExt => qar_gui_panel_stream_id_ext_default,
    QarGuiPanelUrlExt => qar_gui_panel_url_ext_default,
    QarAppVolumeGestureAllowedPeersExt => qar_app_volume_gesture_allowed_peers_ext_default,
    QarVideoSenderInit => qar_video_sender_init_default,
    QarGuiPanelInit => qar_gui_panel_init_default,
    QarAppVolumeInit => qar_app_volume_init_default,
    QarRenderFrameShow => qar_render_frame_show_default,
    QarGraphicsDeviceId => qar_graphics_device_id_default,
    QarSessionLifecycleInfo => qar_session_lifecycle_info_default,
    QarStreamParamsVulkan => qar_stream_params_vulkan_default,
    QarSyncFrameVulkan => qar_sync_frame_vulkan_default,
    QarVideoFrameVulkan => qar_video_frame_vulkan_default,
    QarRenderSenderAlphaModeExt => qar_render_sender_alpha_mode_ext_default,
    QarRenderSenderColorEncodingExt => qar_render_sender_color_encoding_ext_default,
    QarMemberHubInvitationInit => qar_member_hub_invitation_init_default,
    QarMemberHubInvitationAppGrantExt => qar_member_hub_invitation_app_grant_ext_default,
    QarOnboardRemoteAppGrantExt => qar_onboard_remote_app_grant_ext_default,
    QarJoinHubAsMemberInit => qar_join_hub_as_member_init_default,
    QarPeerDeviceIdentity => qar_peer_device_identity_default,
);

#[cfg(any(target_os = "windows", target_os = "macos"))]
mod d3d11 {
    use std::mem::MaybeUninit;

    #[allow(clippy::wildcard_imports)]
    use qaros_sys::*;

    conformance!(
        QarVideoTextureD3D11 => qar_video_texture_d3d11_default,
        QarDXGIKeyedMutexSync => qar_dxgi_keyed_mutex_sync_default,
        QarSyncFrameD3D11 => qar_sync_frame_d3d11_default,
        QarVideoFrameD3D11 => qar_video_frame_d3d11_default,
        QarStreamParamsD3D11 => qar_stream_params_d3d11_default,
    );
}
