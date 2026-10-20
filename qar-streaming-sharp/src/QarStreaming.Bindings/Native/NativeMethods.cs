using System;
using System.Runtime.InteropServices;

namespace Quaternar.Qaros.Interop
{
    internal static class NativeMethods
    {
        internal const string LibraryName = "qar-streaming-c";

        // Runs before the first P/Invoke below (explicit static constructor, so the type is
        // not beforefieldinit): binds every import to the runtime InstalledRuntime finds. A
        // failure is left for Library.Init to report as "QAROS is not installed".
        static NativeMethods()
        {
            InstalledRuntime.EnsureLoaded();
        }

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_library_init(ref LibraryInitNative init);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_library_destroy();

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_runtime_info(out RuntimeInfoNative info);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_result_success();

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_result_error(StatusCode code, IntPtr message);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.I1)]
        internal static extern bool qar_impl_result_is_success(ResultNative result);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.I1)]
        internal static extern bool qar_impl_result_is_error(ResultNative result);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.I1)]
        internal static extern bool qar_impl_result_has_code(ResultNative result, StatusCode code);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_error_wrap_result(ResultNative innerResult, StatusCode newCode, IntPtr newMessage);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void qar_impl_result_message(ResultNative result, [Out] byte[] outBuffer, UIntPtr bufferSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void qar_impl_result_log_if_error(ResultNative result);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern PeerIdNative qar_impl_peer_id_unique();

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern SessionIdNative qar_impl_session_id_unique();

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern GuiPanelIdNative qar_impl_gui_panel_id_unique();

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_uuid_to_string(byte[] uuidBytes, [Out] byte[] outBuffer, UIntPtr bufferSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_uuid_from_string(IntPtr text, [Out] byte[] outUuidBytes);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_cancel_token_create(out IntPtr token);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_cancel_token_create_with_timeout(out IntPtr token, uint timeoutMs);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void qar_impl_cancel_token_handle_destroy(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_cancel_token_cancel(IntPtr token);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_cancel_token_cancel_after(IntPtr token, uint timeoutMs);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.I1)]
        internal static extern bool qar_impl_cancel_token_is_cancelled(IntPtr token);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.I1)]
        internal static extern bool qar_impl_cancel_token_is_timeout(IntPtr token);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void qar_impl_runtime_handle_destroy(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_runtime_create(ref RuntimeInitNative init, out IntPtr runtime);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_runtime_rejoin(IntPtr runtime, ref RejoinInitNative init, ProgressCallback? onProgress, IntPtr progressState, IntPtr cancel, out IntPtr session);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_runtime_rejoin_async(IntPtr runtime, ref RejoinInitNative init, RuntimeOnboardResultCallback resultCallback, ProgressCallback? updateCallback, IntPtr userState, IntPtr cancel);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_runtime_onboard(IntPtr runtime, ref OnboardInitNative init, ProgressCallback? onProgress, IntPtr progressState, IntPtr cancel, out OnboardingIdNative onboardingId, out IntPtr session);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_runtime_onboard_async(IntPtr runtime, ref OnboardInitNative init, RuntimeOnboardResultCallback resultCallback, ProgressCallback? updateCallback, IntPtr userState, IntPtr cancel);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_runtime_forget(IntPtr runtime, ref ForgetInitNative init);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void qar_impl_runtime_destroy(IntPtr runtime);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void qar_impl_session_handle_destroy(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_session_get_id(IntPtr session, ref SessionIdNative sessionId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_session_get_graphics_device_id(IntPtr session, ref GraphicsDeviceIdNative outDeviceId, [MarshalAs(UnmanagedType.U1)] out bool outPresent);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_session_request_onboarding_invite(IntPtr session, ref RequestInviteInitNative init, ProgressCallback? onProgress, IntPtr progressState, IntPtr cancel, out IntPtr outInvite);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_session_request_onboarding_invite_async(IntPtr session, ref RequestInviteInitNative init, SessionRequestInviteResultCallback resultCallback, ProgressCallback? updateCallback, IntPtr userState, IntPtr cancel);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_session_request_onboarding_codes(IntPtr session, ref RequestOnboardingCodesInitNative init, ProgressCallback? onProgress, IntPtr progressState, IntPtr cancel, [Out] IntPtr[] outInvites);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_onboarding_invite_get_code(IntPtr invite, [Out] byte[]? outBuffer, UIntPtr bufferSize, out UIntPtr outLength);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_onboarding_invite_deserialize(byte[] wireData, UIntPtr wireDataSize, out IntPtr outInvite);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_onboarding_invite_serialized_size(IntPtr invite, out UIntPtr outSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_onboarding_invite_serialize(IntPtr invite, [Out] byte[] outBuffer, UIntPtr bufferSize, out UIntPtr outBytesWritten);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_onboarding_invite_get_method(IntPtr invite, out OnboardingMethod outMethod);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_onboarding_invite_get_expires_unix(IntPtr invite, out long outExpiresUnix);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.I1)]
        internal static extern bool qar_impl_onboarding_invite_handle_is_valid(IntPtr invite);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void qar_impl_onboarding_invite_handle_destroy(IntPtr invite);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_runtime_create_member_hub_invitation(IntPtr runtime, ref MemberHubInvitationInitNative init, IntPtr cancel, out IntPtr outInvitation);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_runtime_join_hub_as_member(IntPtr runtime, ref JoinHubAsMemberInitNative init, ProgressCallback? onProgress, IntPtr progressState, IntPtr cancel);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_member_hub_invitation_serialized_size(IntPtr invitation, out UIntPtr outSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_member_hub_invitation_serialize(IntPtr invitation, [Out] byte[]? outBuffer, UIntPtr bufferSize, out UIntPtr outBytesWritten);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_member_hub_invitation_deserialize(byte[]? wireData, UIntPtr wireDataSize, out IntPtr outInvitation);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void qar_impl_member_hub_invitation_handle_destroy(IntPtr invitation);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.I1)]
        internal static extern bool qar_impl_peer_spec_handle_is_valid(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void qar_impl_peer_spec_handle_destroy(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_peer_spec_get_id(IntPtr handle, ref PeerIdNative outId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_peer_spec_get_display_name(IntPtr handle, [Out] byte[] outBuffer, UIntPtr bufferSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_peer_spec_get_app_version(IntPtr handle, [Out] byte[] outBuffer, UIntPtr bufferSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_peer_spec_get_app_custom_peer_info(IntPtr handle, [Out] byte[] outBuffer, UIntPtr bufferSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_peer_spec_get_app_state(IntPtr handle, out AppState outState);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_peer_spec_get_version_id(IntPtr handle, [Out] byte[] outBuffer, UIntPtr bufferSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_peer_spec_get_room_tag(IntPtr handle, [Out] byte[] outBuffer, UIntPtr bufferSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_peer_spec_get_role(IntPtr handle, out PeerRole outRole);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_peer_spec_get_category(IntPtr handle, out DeviceCategory outCategory);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_peer_spec_get_device_id(IntPtr handle, [Out] byte[] outBuffer, UIntPtr bufferSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_peer_spec_get_device_family(IntPtr handle, [Out] byte[] outBuffer, UIntPtr bufferSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_peer_spec_get_device_name(IntPtr handle, [Out] byte[] outBuffer, UIntPtr bufferSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_peer_update_device_identity(IntPtr session, IntPtr identity);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_session_get_my_spec(IntPtr session, out IntPtr outHandle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_query_peer_specs_count(IntPtr session, out UIntPtr outCount);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_query_peer_specs(IntPtr session, [Out] IntPtr[] outHandles, UIntPtr handlesBufferSize, out UIntPtr outHandlesWritten);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_peer_update_display_name(IntPtr session, IntPtr name);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_peer_subscribe_updates(IntPtr session, PeerUpdateCallback callback, IntPtr userState, IntPtr token);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void qar_impl_render_stream_handle_destroy(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_render_sender_subscribe_requests(IntPtr session, RenderRequestCallback callback, IntPtr userState, IntPtr token);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void qar_impl_render_request_handle_destroy(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_render_request_get_target_peer_id(IntPtr request, ref PeerIdNative outPeerId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_render_request_get_stream_id(IntPtr request, ref StreamIdNative outStreamId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_render_sender_create(IntPtr session, ref RenderSenderInitNative init, IntPtr cancel, out IntPtr outStream);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_render_sender_create_async(IntPtr session, ref RenderSenderInitNative init, RenderSenderCreateCallback callback, IntPtr userState, IntPtr token);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_render_sender_layout(IntPtr stream, ref VideoFrameLayoutNative outLayout);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_render_sender_stream_id(IntPtr stream, out StreamIdNative outStreamId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_render_sender_change_layout(IntPtr stream, ref VideoFrameLayoutNative layout);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_render_sender_frame_cpu(IntPtr stream, ref VideoFrameCpuNative outFrame);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_render_sender_frame_d3d11(IntPtr stream, ref VideoFrameD3D11Native outFrame);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_render_sender_begin_frame(IntPtr stream, IntPtr token, out IntPtr outFrameInfo);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_render_sender_begin_frame_async(IntPtr stream, RenderSenderBeginFrameCallback callback, IntPtr userState, IntPtr token);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_render_sender_show_frame(IntPtr stream, ref RenderFrameShowNative frameShow);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_render_sender_last_hands(IntPtr stream, ref DeviceHandsWithJointsNative outHands);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void qar_impl_video_sender_handle_destroy(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_video_sender_create(IntPtr session, ref VideoSenderInitNative init, out IntPtr outSender);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_video_sender_stream_id(IntPtr sender, out StreamIdNative outStreamId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_video_sender_try_begin_frame(IntPtr sender, [MarshalAs(UnmanagedType.I1)] out bool outReady);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_video_sender_layout(IntPtr sender, ref VideoFrameLayoutNative outLayout);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_video_sender_change_layout(IntPtr sender, ref VideoFrameLayoutNative layout);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_video_sender_frame_cpu(IntPtr sender, ref VideoFrameCpuNative outFrame);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_video_sender_show_frame(IntPtr sender);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void qar_impl_render_frame_info_handle_destroy(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_render_frame_info_get_view_pose(IntPtr handle, UIntPtr viewIndex, out PoseNative outPose);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_render_frame_info_get_view_fov(IntPtr handle, UIntPtr viewIndex, out FovNative outFov);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panels_get_or_create(IntPtr session, ref GuiPanelInitNative init, ref GuiPanelIdNative outPanelId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panels_update_pose(IntPtr session, ref GuiPanelIdNative id, ref PoseNative pose);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panels_change_size(IntPtr session, ref GuiPanelIdNative id, ref GuiPanelSizeNative size);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panels_set_state(IntPtr session, ref GuiPanelIdNative id, GuiPanelState state);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panels_close_panel(IntPtr session, ref GuiPanelIdNative id);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panels_navigate_to_uri(IntPtr session, ref GuiPanelIdNative id, IntPtr uri);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panels_set_stream(IntPtr session, ref GuiPanelIdNative id, ref StreamIdNative streamId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panels_update_visible_to(IntPtr session, ref GuiPanelIdNative id, IntPtr peerIdsAdditions, UIntPtr additionsCount, IntPtr peerIdsRemovals, UIntPtr removalsCount);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panels_subscribe_updates(IntPtr session, GuiPanelUpdateCallback callback, IntPtr userState, IntPtr token);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panels_subscribe_panel_updates(IntPtr session, ref GuiPanelIdNative id, GuiPanelUpdateCallback callback, IntPtr userState, IntPtr token);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.I1)]
        internal static extern bool qar_impl_gui_panel_handle_is_valid(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void qar_impl_gui_panel_handle_destroy(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panel_get_id(IntPtr handle, ref GuiPanelIdNative outId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panel_get_display_name(IntPtr handle, [Out] byte[] outBuffer, UIntPtr bufferSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panel_get_pose(IntPtr handle, out PoseNative outPose);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panel_get_size(IntPtr handle, out GuiPanelSizeNative outSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panel_get_content_uri(IntPtr handle, [Out] byte[] outUri, UIntPtr bufferSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panel_get_content_type(IntPtr handle, out GuiPanelContentType outType);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panel_get_content_stream_id(IntPtr handle, out StreamIdNative outStreamId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panel_get_state(IntPtr handle, out GuiPanelState outState);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panel_get_visible_to_peers_count(IntPtr handle, out UIntPtr outCount);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panel_get_visible_to_peers(IntPtr handle, [Out] PeerIdNative[] outPeers, UIntPtr peersBufferSize, out UIntPtr outPeersWritten);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_query_gui_panels_count(IntPtr session, out UIntPtr outCount);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_query_gui_panels(IntPtr session, [Out] IntPtr[] outHandles, UIntPtr handlesBufferSize, out UIntPtr outHandlesWritten);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_get_or_create(IntPtr session, ref AppVolumeInitNative init, ref AppVolumeIdNative outVolume);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_change_display_name(IntPtr session, ref AppVolumeIdNative volumeId, IntPtr displayName);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_change_size(IntPtr session, ref AppVolumeIdNative volumeId, ref AppVolumeSizeNative size);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_close_volume(IntPtr session, ref AppVolumeIdNative volumeId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_update_used_by_peers(IntPtr session, ref AppVolumeIdNative volumeId, [In] PeerIdNative[] peerAdditions, UIntPtr additionsCount, [In] PeerIdNative[] peerRemovals, UIntPtr removalsCount);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_start_editing(IntPtr session, ref AppVolumeIdNative volumeId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_stop_editing(IntPtr session, ref AppVolumeIdNative volumeId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_change_app_pose(IntPtr session, ref AppVolumeIdNative volumeId, ref PoseNative appPose);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_change_app_scale(IntPtr session, ref AppVolumeIdNative volumeId, float scale);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_change_app_world_anchor(IntPtr session, ref AppVolumeIdNative volumeId, ref GeoAnchorFrameNative anchor);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_clear_app_world_anchor(IntPtr session, ref AppVolumeIdNative volumeId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_change_pose(IntPtr session, ref AppVolumeIdNative volumeId, ref PoseNative pose);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_subscribe_updates(IntPtr session, AppVolumeUpdateCallback callback, IntPtr userState, IntPtr token);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_subscribe_gesture_events(IntPtr session, ref AppVolumeIdNative volumeId, GestureKind gestureKind, AppVolumeGestureEventCallback callback, IntPtr userState, IntPtr token);

        // App volume gesture accessors. The gesture is an opaque handle valid
        // only for the duration of the callback, so these are the only way to
        // read the event and everything read must be copied out before the
        // callback returns.
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_gesture_get_lifecycle(IntPtr gesture, out PeerIdNative outSourcePeerId, out AppVolumeIdNative outTargetAppVolumeId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_gesture_get_classification(IntPtr gesture, out GestureKind outKind, out GesturePhase outPhase);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_gesture_get_points(IntPtr gesture, out Vector3Native outActionPoint, [MarshalAs(UnmanagedType.I1)] out bool outHasActionPoint, out Vector3Native outStartPoint, [MarshalAs(UnmanagedType.I1)] out bool outHasStartPoint);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_gesture_get_deltas(IntPtr gesture, out Vector3Native outTranslationDelta, out QuaternionNative outRotationDelta, [MarshalAs(UnmanagedType.I1)] out bool outWasMappedToAppTransform);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_gesture_get_timestamp(IntPtr gesture, out TimePointNative outTimestamp);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.I1)]
        internal static extern bool qar_impl_app_volume_handle_is_valid(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void qar_impl_app_volume_handle_destroy(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_id(IntPtr handle, ref AppVolumeIdNative outId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_display_name(IntPtr handle, [Out] byte[] outBuffer, UIntPtr bufferSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_pose(IntPtr handle, out PoseNative outPose);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_size(IntPtr handle, out AppVolumeSizeNative outSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_lifetime_status(IntPtr handle, out AppVolumeLifetimeStatus outStatus);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_editing_status(IntPtr handle, ref AppVolumeEditingStatusNative outStatus);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_used_by_peers_count(IntPtr handle, out UIntPtr outPeerCount);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_used_by_peers(IntPtr handle, [Out] PeerIdNative[] outPeers, UIntPtr peersBufferSize, out UIntPtr outPeersWritten);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_query_app_volumes_count(IntPtr session, out UIntPtr outCount);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_query_app_volumes(IntPtr session, [Out] IntPtr[] outHandles, UIntPtr handlesBufferSize, out UIntPtr outHandlesWritten);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_app_pose(IntPtr handle, out PoseNative outPose);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_app_scale(IntPtr handle, out float outScale);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_app_world_anchor(IntPtr handle, out AppWorldAnchorNative outAnchor);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_latest_app_pose(IntPtr session, ref AppVolumeIdNative volumeId, out PoseNative outPose);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_latest_app_scale(IntPtr session, ref AppVolumeIdNative volumeId, out float outScale);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_latest_pose(IntPtr session, ref AppVolumeIdNative volumeId, out PoseNative outPose);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_latest_size(IntPtr session, ref AppVolumeIdNative volumeId, out AppVolumeSizeNative outSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_gesture_configuration(IntPtr handle, ref AppVolumeGestureConfigurationNative outConfig);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_change_gesture_configuration(IntPtr session, ref AppVolumeIdNative volumeId, ref AppVolumeGestureConfigurationNative config);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_subscribe_all_gesture_events(IntPtr session, ref AppVolumeIdNative volumeId, AppVolumeGestureEventCallback callback, IntPtr userState, IntPtr token);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_gesture_get_handedness(IntPtr gesture, out Handedness outHandedness);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_gesture_get_gesture_id(IntPtr gesture, out ulong outGestureId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_gesture_get_mode(IntPtr gesture, out uint outModeId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_gesture_allowed_peers(IntPtr handle, [Out] PeerIdNative[]? outPeers, UIntPtr peerCapacity, out UIntPtr outPeerCount);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_gesture_holder(IntPtr handle, ref PeerIdNative outPeer, [MarshalAs(UnmanagedType.I1)] out bool outIsHeld);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_change_gesture_mode(IntPtr session, ref AppVolumeIdNative volumeId, uint modeId, ref AppVolumeGestureConfigurationNative config);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_remove_gesture_mode(IntPtr session, ref AppVolumeIdNative volumeId, uint modeId);

        // peer: pointer to a PeerIdNative, or zero for the default every peer without its own mode follows.
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_select_gesture_mode(IntPtr session, ref AppVolumeIdNative volumeId, IntPtr peer, uint modeId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_clear_peer_gesture_mode(IntPtr session, ref AppVolumeIdNative volumeId, ref PeerIdNative peer);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volumes_clear_all_peer_gesture_modes(IntPtr session, ref AppVolumeIdNative volumeId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_gesture_mode_ids(IntPtr handle, [Out] uint[]? outModeIds, UIntPtr modeCapacity, out UIntPtr outModeCount);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_gesture_mode(IntPtr handle, uint modeId, ref AppVolumeGestureConfigurationNative outConfig);

        // peer: pointer to a PeerIdNative, or zero for the default.
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_app_volume_get_active_gesture_mode(IntPtr handle, IntPtr peer, out uint outModeId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panels_subscribe_gesture_events(IntPtr session, ref GuiPanelIdNative id, GuiPanelGestureKind gestureKind, GuiPanelGestureEventCallback callback, IntPtr userState, IntPtr token);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panels_subscribe_all_gesture_events(IntPtr session, ref GuiPanelIdNative id, GuiPanelGestureEventCallback callback, IntPtr userState, IntPtr token);

        // GUI panel gesture accessors. Like the app volume ones, the handle is
        // valid only for the duration of the callback that received it.
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panel_gesture_get_lifecycle(IntPtr gesture, ref PeerIdNative outSourcePeerId, out ulong outGestureId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panel_gesture_get_classification(IntPtr gesture, out GuiPanelGestureKind outKind, out GesturePhase outPhase);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panel_gesture_get_positions(IntPtr gesture, out GuiPanelPointNative outPosition, out GuiPanelPointNative outStartPosition);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panel_gesture_get_panel_id(IntPtr gesture, ref GuiPanelIdNative outPanelId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panel_gesture_get_timestamp(IntPtr gesture, out TimePointNative outTimestamp);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panel_gesture_get_handedness(IntPtr gesture, out Handedness outHandedness);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_gui_panel_gesture_get_tap_count(IntPtr gesture, out byte outTapCount);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_session_find_render_target(IntPtr session, IntPtr connection, ref PeerIdNative outPeer);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_session_invite_target_app(IntPtr session, ref SessionInviteTargetAppInitNative init, ProgressCallback? onProgress, IntPtr progressState, IntPtr cancel, ref PeerIdNative outPeer);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_session_invite_target_app_async(IntPtr session, ref SessionInviteTargetAppInitNative init, TargetAppResultCallback resultCallback, ProgressCallback? updateCallback, IntPtr userState, IntPtr cancel);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_session_kill_target_app(IntPtr session, ref PeerIdNative targetApp, ProgressCallback? onProgress, IntPtr progressState, IntPtr cancel);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_session_kill_target_app_async(IntPtr session, ref PeerIdNative targetApp, TargetAppResultCallback resultCallback, ProgressCallback? updateCallback, IntPtr userState, IntPtr cancel);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_target_app_connection_string_hololens(IntPtr host, [Out] byte[] buffer, UIntPtr bufferSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_target_app_connection_string_quest(IntPtr host, [Out] byte[] buffer, UIntPtr bufferSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_target_app_connection_string_android(IntPtr host, [Out] byte[] buffer, UIntPtr bufferSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_render_sender_try_begin_frame(IntPtr stream, out IntPtr outFrameInfo, [MarshalAs(UnmanagedType.I1)] out bool outReady);

        // Exported only when the library is built with QAR_ENABLE_OPENGL.
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_render_sender_frame_opengl(IntPtr stream, ref VideoFrameOpenGlNative outFrame);

        // Exported only when the library is built with QAR_ENABLE_VULKAN. The
        // frame is sender-owned: the pointer stays valid until the next call or
        // the sender's destruction.
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_render_sender_frame_vulkan(IntPtr stream, out IntPtr outFrame);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_video_sender_frame_d3d11(IntPtr sender, ref VideoFrameD3D11Native outFrame);

        // Exported only when the library is built with QAR_ENABLE_OPENGL.
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_video_sender_frame_opengl(IntPtr sender, ref VideoFrameOpenGlNative outFrame);

        // Exported only when the library is built with QAR_ENABLE_VULKAN.
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_video_sender_frame_vulkan(IntPtr sender, out IntPtr outFrame);
    }
}
