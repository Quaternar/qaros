using System;
using System.Runtime.InteropServices;
using Quaternar.Qaros;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros.Tests.Ext
{
    // P/Invoke layer of qar-streaming-c-ext, the never-shipped DLL of the C
    // API's counterparts (hub, peers, receivers). It lives in the tests only:
    // the API under test is the binding of the unchanged qar-streaming-c.
    // Struct layouts mirror qar_streaming_c_ext/v0/ext_types.h.

    internal static class ExtStructureType
    {
        internal const StructureType HubInit = (StructureType)0xE001;
        internal const StructureType PeerInit = (StructureType)0xE002;
        internal const StructureType RenderReceiverInit = (StructureType)0xE003;
        internal const StructureType TargetAppResponderInit = (StructureType)0xE004;
        internal const StructureType VideoReceiverInit = (StructureType)0xE005;
        internal const StructureType StreamVulkanParams = (StructureType)0x4002;
    }

    internal static class ExtConstants
    {
        internal const int MaxTextLength = 256;
    }

    public enum ExtInviteMethod
    {
        Pake = 0,
        Password = 1,
    }

    public enum ExtVideoReceiverRole
    {
        Auto = 0,
        Primary = 1,
        Secondary = 2,
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct ExtHubInitNative
    {
        public StructureHeaderNative Header;
        [MarshalAs(UnmanagedType.I1)]
        public bool EnablePasswordOnboarding;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct ExtPeerInitNative
    {
        public StructureHeaderNative Header;
        public IntPtr DisplayName;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct ExtVideoReceiverInitNative
    {
        public StructureHeaderNative Header;
        public VideoFrameLayoutNative OptimalLayout;
        [MarshalAs(UnmanagedType.I1)]
        public bool HasStreamId;
        public StreamIdNative StreamId;
        public ExtVideoReceiverRole Role;
        [MarshalAs(UnmanagedType.I1)]
        public bool StartPaused;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct ExtRenderReceiverInitNative
    {
        public StructureHeaderNative Header;
        public VideoFrameLayoutNative OptimalLayout;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct ExtRenderRequestNative
    {
        public PoseNative HeadPose;
        public ulong FrameCount;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct ExtVideoFrameInfoNative
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool ValidFrame;
        public ulong SequenceNumber;
        public long CreationTimeNs;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct ExtTargetAppResponderInitNative
    {
        public StructureHeaderNative Header;
        public IntPtr MachineId;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct ExtTargetAppInviteRecordNative
    {
        public PeerIdNative Producer;
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = ExtConstants.MaxTextLength)]
        public byte[] Hostname;
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = ExtConstants.MaxTextLength)]
        public byte[] Device;
        [MarshalAs(UnmanagedType.I1)]
        public bool Answered;
        public PeerIdNative Target;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct ExtTargetAppShutdownRecordNative
    {
        public PeerIdNative PeerToShutdown;
        [MarshalAs(UnmanagedType.I1)]
        public bool Answered;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct ExtMediaStreamRecordNative
    {
        public StreamIdNative Id;
        public int Kind;
        public int Lifetime;
        [MarshalAs(UnmanagedType.I1)]
        public bool HasVideo;
        public PeerIdNative SenderPeer;
        public UIntPtr ReceiverCount;
        [MarshalAs(UnmanagedType.I1)]
        public bool HasSourceLayout;
        public VideoFrameLayoutNative SourceLayout;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct ExtGuiPanelGestureNative
    {
        public GuiPanelIdNative Panel;
        public int Kind;
        public int Phase;
        public ulong GestureId;
        public int Handedness;
        public int X;
        public int Y;
        public int StartX;
        public int StartY;
        public byte TapCount;
    }

    internal static class ExtNativeMethods
    {
        private const string LibraryName = "qar-streaming-c-ext";

        // core
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern double qar_impl_ext_time_scale();

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ulong qar_impl_ext_scale_ms(ulong milliseconds);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_layout_side_by_side(uint eyeWidth, uint eyeHeight, out VideoFrameLayoutNative outLayout);

        // hub
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_hub_create(ref ExtHubInitNative init, out IntPtr outHub);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void qar_impl_ext_hub_handle_destroy(IntPtr hub);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_hub_get_peer_id(IntPtr hub, out PeerIdNative outPeerId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_hub_mint_invite(IntPtr hub, ExtInviteMethod method, [Out] byte[]? outBuffer, UIntPtr bufferSize, out UIntPtr outSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_hub_find_peer_display_name(IntPtr hub, ref PeerIdNative peerId, [Out] byte[] outBuffer, UIntPtr bufferSize, out UIntPtr outSize, [MarshalAs(UnmanagedType.I1)] out bool outFound);

        // peer
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_peer_create(IntPtr hub, ref ExtPeerInitNative init, out IntPtr outPeer);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void qar_impl_ext_peer_handle_destroy(IntPtr peer);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_peer_get_id(IntPtr peer, out PeerIdNative outPeerId);

        // target apps
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_target_app_responder_create(IntPtr hub, ref ExtTargetAppResponderInitNative init, out IntPtr outResponder);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void qar_impl_ext_target_app_responder_handle_destroy(IntPtr responder);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_target_app_responder_invite_count(IntPtr responder, out UIntPtr outCount);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_target_app_responder_get_invite(IntPtr responder, UIntPtr index, out ExtTargetAppInviteRecordNative outRecord);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_target_app_responder_shutdown_count(IntPtr responder, out UIntPtr outCount);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_target_app_responder_get_shutdown(IntPtr responder, UIntPtr index, out ExtTargetAppShutdownRecordNative outRecord);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_target_app_responder_running_count(IntPtr responder, out UIntPtr outCount);

        // media streams
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_hub_query_media_streams(IntPtr hub, [Out] ExtMediaStreamRecordNative[]? outRecords, UIntPtr capacity, out UIntPtr outCount);

        // video receiver
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_video_receiver_create(IntPtr peer, ref ExtVideoReceiverInitNative init, out IntPtr outReceiver);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void qar_impl_ext_video_receiver_handle_destroy(IntPtr receiver);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_video_receiver_wait_connected(IntPtr receiver, uint timeoutMs, [MarshalAs(UnmanagedType.I1)] out bool outConnected);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_video_receiver_stream_id(IntPtr receiver, out StreamIdNative outStreamId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_video_receiver_try_provide_frame_cpu(IntPtr receiver, ref VideoFrameCpuNative frame, [MarshalAs(UnmanagedType.I1)] out bool outAccepted);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_video_receiver_receive_frame(IntPtr receiver, uint timeoutMs, out ExtVideoFrameInfoNative outInfo, [MarshalAs(UnmanagedType.I1)] out bool outReceived);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_video_receiver_pause(IntPtr receiver);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_video_receiver_resume(IntPtr receiver);

        // render receiver
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_render_receiver_create(IntPtr peer, ref ExtRenderReceiverInitNative init, out IntPtr outReceiver);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void qar_impl_ext_render_receiver_handle_destroy(IntPtr receiver);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_render_receiver_video(IntPtr receiver, out IntPtr outVideo);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_render_receiver_request_render(IntPtr receiver, ref ExtRenderRequestNative request);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_render_receiver_send_pose(IntPtr receiver, ref PoseNative headPose);

        // input
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_peer_send_device_pose(IntPtr peer, ref PoseNative headPose);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_peer_send_hand_joints(IntPtr peer, ref DeviceHandsWithJointsNative hands);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_peer_gui_panel_gesture(IntPtr peer, ref ExtGuiPanelGestureNative gesture);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_peer_app_volume_add_peer(IntPtr peer, ref AppVolumeIdNative volume, ref PeerIdNative added);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_peer_app_volume_remove_peer(IntPtr peer, ref AppVolumeIdNative volume, ref PeerIdNative removed);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_peer_app_volume_start_gesturing(IntPtr peer, ref AppVolumeIdNative volume);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_peer_app_volume_stop_gesturing(IntPtr peer, ref AppVolumeIdNative volume);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern ResultNative qar_impl_ext_peer_app_volume_set_focused(IntPtr peer, ref AppVolumeIdNative volume, [MarshalAs(UnmanagedType.I1)] bool focused);
    }
}
