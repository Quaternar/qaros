using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Quaternar.Qaros;
using Quaternar.Qaros.Mathematics;

namespace Quaternar.Qaros.Interop
{
    internal static class NativeConstants
    {
        internal const int MaxIdLength = 16;
        internal const int MaxStringLength = 512;
        internal const int UuidTextLength = 36;
        internal const int UuidTextBufferSize = 37;
        internal const int MaxFrameViews = 8;
        internal const int MaxFrameTextures = 4;
        internal const int HandJointCount = 26;
        internal const int MaxAppVolumeGestureMappingRules = 16;
        internal const int GpuUuidLength = 16;
        internal const uint AppVolumeGestureModeDefault = 0;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct ResultNative
    {
        public StatusCode Code;
        public uint ErrorHandle;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct StructureHeaderNative
    {
        public StructureType Type;
        public IntPtr Next;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct PeerIdNative
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.MaxIdLength)]
        public byte[] Data;
        public static PeerIdNative CreateDefault() { return new PeerIdNative { Data = new byte[NativeConstants.MaxIdLength] }; }
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct SessionIdNative
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.MaxIdLength)]
        public byte[] Data;
        public static SessionIdNative CreateDefault() { return new SessionIdNative { Data = new byte[NativeConstants.MaxIdLength] }; }
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct GuiPanelIdNative
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.MaxIdLength)]
        public byte[] Data;
        public static GuiPanelIdNative CreateDefault() { return new GuiPanelIdNative { Data = new byte[NativeConstants.MaxIdLength] }; }
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct AppVolumeIdNative
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.MaxIdLength)]
        public byte[] Data;
        public static AppVolumeIdNative CreateDefault() { return new AppVolumeIdNative { Data = new byte[NativeConstants.MaxIdLength] }; }
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct StreamIdNative
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.MaxIdLength)]
        public byte[] Data;
        public static StreamIdNative CreateDefault() { return new StreamIdNative { Data = new byte[NativeConstants.MaxIdLength] }; }
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct OnboardingIdNative
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.MaxIdLength)]
        public byte[] Data;
        public static OnboardingIdNative CreateDefault() { return new OnboardingIdNative { Data = new byte[NativeConstants.MaxIdLength] }; }
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct TimePointNative
    {
        public long UnixNs;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct Vector3Native
    {
        public float X;
        public float Y;
        public float Z;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct Vector3dNative
    {
        public double X;
        public double Y;
        public double Z;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct QuaternionNative
    {
        public float X;
        public float Y;
        public float Z;
        public float W;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct PoseNative
    {
        public QuaternionNative Orientation;
        public Vector3Native Position;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct FovNative
    {
        public float AngleLeft;
        public float AngleRight;
        public float AngleUp;
        public float AngleDown;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct NearFarNative
    {
        public float NearPlane;
        public float FarPlane;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct HandJointLocationNative
    {
        public uint JointId;
        public ulong LocationFlags;
        public PoseNative Pose;
        public float Radius;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct HandJointVelocityNative
    {
        public uint JointId;
        public ulong Flags;
        public Vector3Native LinearVelocity;
        public Vector3Native AngularVelocity;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct HandJointsNative
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool IsTracked;
        [MarshalAs(UnmanagedType.I1)]
        public bool IsActive;
        public PoseNative Pose;
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.HandJointCount)]
        public HandJointLocationNative[] JointLocations;
        [MarshalAs(UnmanagedType.I1)]
        public bool HasVelocity;
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.HandJointCount)]
        public HandJointVelocityNative[] JointVelocities;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct DeviceHandsWithJointsNative
    {
        public PeerIdNative DeviceId;
        public TimePointNative DeviceTimestamp;
        public HandJointsNative LeftHand;
        public HandJointsNative RightHand;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct RenderFrameViewNative
    {
        public VideoFrameViewType DataType;
        public VideoFrameViewEye Eye;
        public uint TextureIndex;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct ImageSizeNative
    {
        public uint Width;
        public uint Height;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct TextureSizeNative
    {
        public PixelFormat Format;
        public uint Width;
        public uint Height;
        public uint ArrayLayers;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct VideoFrameViewNative
    {
        public uint StartX;
        public uint StartY;
        public uint EndX;
        public uint EndY;
        public uint TextureIndex;
        public uint ArrayLayerIndex;
        public PixelFormat TextureFormat;
        public VideoFrameViewType DataType;
        public VideoFrameViewEye Eye;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct VideoFrameLayoutNative
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.MaxFrameViews)]
        public VideoFrameViewNative[] Views;
        public UIntPtr ViewsCount;
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.MaxFrameTextures)]
        public TextureSizeNative[] Textures;
        public UIntPtr TexturesCount;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct VideoSenderInitNative
    {
        public StructureHeaderNative Header;
        public VideoFrameLayoutNative Layout;
        [MarshalAs(UnmanagedType.I1)]
        public bool EnableReconnects;
        public ulong ReconnectTimeoutMs;
        public GraphicsAPI GraphicsApi;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct VideoTextureCpuNative
    {
        public TextureSizeNative Size;
        public uint Pitch;
        public IntPtr TextureData;
        public UIntPtr TextureDataSize;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct VideoFrameCpuNative
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.MaxFrameViews)]
        public VideoFrameViewNative[] TextureViews;
        public UIntPtr TextureViewsCount;
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.MaxFrameTextures)]
        public VideoTextureCpuNative[] Textures;
        public UIntPtr TexturesCount;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct VideoTextureD3D11Native
    {
        public TextureSizeNative Size;
        public IntPtr Texture;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct DXGIKeyedMutexSyncNative
    {
        public IntPtr Mutex;
        public uint Acquire;
        public uint Release;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct SyncFrameD3D11Native
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.MaxFrameTextures)]
        public DXGIKeyedMutexSyncNative[] TextureSync;
        public UIntPtr TextureSyncCount;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct VideoFrameD3D11Native
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.MaxFrameViews)]
        public VideoFrameViewNative[] TextureViews;
        public UIntPtr TextureViewsCount;
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.MaxFrameTextures)]
        public VideoTextureD3D11Native[] Textures;
        public UIntPtr TexturesCount;
        public SyncFrameD3D11Native Synchronization;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct GuiPanelSizeNative
    {
        public float WidthMeters;
        public float HeightMeters;
        public float ContentScale;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct GuiPanelPointNative
    {
        public int XPixels;
        public int YPixels;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct VideoTextureOpenGlNative
    {
        public TextureSizeNative Size;
        public uint Texture;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct VideoFrameOpenGlNative
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.MaxFrameViews)]
        public VideoFrameViewNative[] TextureViews;
        public UIntPtr TextureViewsCount;
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.MaxFrameTextures)]
        public VideoTextureOpenGlNative[] Textures;
        public UIntPtr TexturesCount;
    }

    /// <remarks>
    /// VkImage and VkDeviceMemory are non-dispatchable Vulkan handles: a pointer
    /// on 64-bit targets and a uint64_t on 32-bit ones, 8 bytes either way, so
    /// they are carried as <see cref="ulong"/>. VkImageLayout is an int enum.
    /// </remarks>
    [StructLayout(LayoutKind.Sequential)]
    internal struct VideoTextureVulkanNative
    {
        public TextureSizeNative Size;
        public ulong Image;
        public ulong ImageMemory;
        public uint FormatFeatures;
        public int Layout;
        public uint QueueFamilyIndex;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct SyncFrameVulkanNative
    {
        public ulong Semaphore;
        public ulong Fence;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct VideoFrameVulkanNative
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.MaxFrameViews)]
        public VideoFrameViewNative[] TextureViews;
        public UIntPtr TextureViewsCount;
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.MaxFrameTextures)]
        public VideoTextureVulkanNative[] Textures;
        public UIntPtr TexturesCount;
        public SyncFrameVulkanNative Synchronization;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct GeoAnchorFrameNative
    {
        public WorldReferenceSystem WorldReferenceSystem;
        public Vector3dNative OriginWorld;
        public Vector3dNative AxisXWorld;
        public Vector3dNative AxisYWorld;
        public Vector3dNative AxisZWorld;
        public Handedness Handedness;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct AppWorldAnchorNative
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool HasAnchor;
        public GeoAnchorFrameNative Anchor;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct AppVolumeSizeNative
    {
        public float WidthMeters;
        public float LengthMeters;
        public float HeightMeters;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct AppVolumeEditingStatusNative
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool IsBeingEdited;
        public PeerIdNative EditorPeer;
        public static AppVolumeEditingStatusNative CreateDefault() { return new AppVolumeEditingStatusNative { EditorPeer = PeerIdNative.CreateDefault() }; }
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct AppVolumeControllerRayStyleNative
    {
        public AppVolumeControllerRayLineStyle LineStyle;
        public byte ColorR;
        public byte ColorG;
        public byte ColorB;
        public byte ColorA;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct AppVolumeGestureMappingRuleNative
    {
        public StructureHeaderNative Header;
        public GestureKind GestureKind;
        [MarshalAs(UnmanagedType.I1)]
        public bool Enabled;
        public float Precision;
        public AppScaleSensitivityMode AppScaleSensitivityMode;
        public uint TranslationAxes;
        public uint RotationAxes;
        [MarshalAs(UnmanagedType.I1)]
        public bool EnableGesturesFromOutsideOfVolume;
        public AppVolumeControllerRayStyleNative ControllerRayStyle;
        public GestureAppTransformMapping AppTransformMapping;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct AppVolumeGestureConfigurationNative
    {
        public StructureHeaderNative Header;
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.MaxAppVolumeGestureMappingRules)]
        public AppVolumeGestureMappingRuleNative[] MappingRules;
        public UIntPtr MappingRuleCount;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct AppVolumeGestureAllowedPeersExtNative
    {
        public StructureHeaderNative Header;
        public IntPtr AllowedPeers;
        public UIntPtr AllowedPeerCount;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct LibraryInitNative
    {
        public StructureHeaderNative Header;
        [MarshalAs(UnmanagedType.I1)]
        public bool EnableConsoleLogging;
        public IntPtr LogFolderPath;
        public LogSeverity LogSeverity;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct RuntimeInitNative
    {
        public StructureHeaderNative Header;
        public IntPtr RuntimeBinariesFolderPath;
        public IntPtr StorageFolderPath;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct PeerPresentationNative
    {
        public StructureHeaderNative Header;
        public IntPtr DisplayName;
        public IntPtr AppVersion;
        public IntPtr AppCustomPeerInfo;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct RejoinInitNative
    {
        public StructureHeaderNative Header;
        public OnboardingIdNative OnboardingId;
        public PeerPresentationNative Presentation;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct OnboardInitNative
    {
        public StructureHeaderNative Header;
        public OnboardingIdNative OnboardingId;
        public PeerPresentationNative Presentation;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct OnboardCodeExtNative
    {
        public StructureHeaderNative Header;
        public IntPtr Code;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct PeerDeviceIdentityNative
    {
        public StructureHeaderNative Header;
        public DeviceCategory Category;
        public IntPtr DeviceId;
        public IntPtr DeviceFamily;
        public IntPtr DeviceName;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct OnboardHostExtNative
    {
        public StructureHeaderNative Header;
        public IntPtr Hostname;
        public ushort Port;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct OnboardInviteExtNative
    {
        public StructureHeaderNative Header;
        public IntPtr Invite;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct OnboardPeerIdExtNative
    {
        public StructureHeaderNative Header;
        public PeerIdNative PeerId;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct OnboardLocalAppExtNative
    {
        public StructureHeaderNative Header;
        public uint ApprovalTimeoutMs;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct RuntimeInfoNative
    {
        public IntPtr Version;
        public IntPtr Flavor;
        public IntPtr CApi;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct RequestInviteInitNative
    {
        public StructureHeaderNative Header;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct RequestOnboardingCodesInitNative
    {
        public StructureHeaderNative Header;
        public uint Count;
        public uint LifetimeSeconds;
        public PeerIdNative PlannedPeerId;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct MemberHubInvitationInitNative
    {
        public StructureHeaderNative Header;
        public uint LifetimeSeconds;
        public uint MaxUses;
        public uint Permissions;
        public IntPtr Label;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct JoinHubAsMemberInitNative
    {
        public StructureHeaderNative Header;
        public IntPtr Invitation;
        public uint HubStartTimeoutMs;
        public uint RestartTimeoutMs;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct SessionInviteTargetAppInitNative
    {
        public StructureHeaderNative Header;
        public IntPtr ConnectionString;
    }

    /// <remarks>
    /// Windows LUID (DWORD LowPart, LONG HighPart). Other platforms declare
    /// QarLuid as uint8_t[8]; both are 8 bytes at the same offset inside
    /// QarGraphicsDeviceId.
    /// </remarks>
    [StructLayout(LayoutKind.Sequential)]
    internal struct LuidNative
    {
        public uint LowPart;
        public int HighPart;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct GraphicsDeviceIdNative
    {
        public StructureHeaderNative Header;
        public GpuDeviceIdType IdType;
        public LuidNative Luid;
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.GpuUuidLength)]
        public byte[] Uuid;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct ForgetInitNative
    {
        public StructureHeaderNative Header;
        public OnboardingIdNative OnboardingId;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct GuiPanelInitNative
    {
        public StructureHeaderNative Header;
        public IntPtr CommonName;
        public IntPtr DisplayName;
        public PoseNative Pose;
        public GuiPanelSizeNative Size;
        public IntPtr VisibleToPeers;
        public UIntPtr VisibleToPeerCount;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct GuiPanelStreamIdExtNative
    {
        public StructureHeaderNative Header;
        public StreamIdNative StreamId;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct GuiPanelUrlExtNative
    {
        public StructureHeaderNative Header;
        public IntPtr Url;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct AppVolumeInitNative
    {
        public StructureHeaderNative Header;
        public IntPtr CommonName;
        public IntPtr DisplayName;
        public PoseNative Pose;
        public AppVolumeSizeNative Size;
        public IntPtr InitialPeers;
        public UIntPtr InitialPeerCount;
        public PoseNative AppPose;
        public float AppScale;
        [MarshalAs(UnmanagedType.I1)]
        public bool HasAppWorldAnchor;
        public GeoAnchorFrameNative AppWorldAnchor;
        public IntPtr GestureConfiguration;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct RenderSenderInitNative
    {
        public StructureHeaderNative Header;
        public PeerIdNative PeerId;
        [MarshalAs(UnmanagedType.I1)]
        public bool EnableAutoReconnects;
        public FrameLayout TextureLayout;
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.MaxFrameViews)]
        public RenderFrameViewNative[] FrameViews;
        public UIntPtr FrameViewsCount;
        public IntPtr AppVolumeId;
        public PixelFormat ColorFormat;
        public PixelFormat DepthFormat;
        public GraphicsAPI GraphicsAPI;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct StreamParamsD3D11Native
    {
        public StructureHeaderNative Header;
        public IntPtr D3D11Context;
        public IntPtr D3D11Device;
        public uint ColorBindFlags;
        public uint DepthBindFlags;
        [MarshalAs(UnmanagedType.I1)]
        public bool AcquireKeyedMutexSync;
    }

    /// <remarks>
    /// The four callbacks are raw C function pointers; see
    /// <c>QarOpenGlContextCallbacks</c>. Zero uses the library's own platform
    /// implementation for that step.
    /// </remarks>
    [StructLayout(LayoutKind.Sequential)]
    internal struct OpenGlContextCallbacksNative
    {
        public IntPtr UserState;
        public IntPtr GetProcAddress;
        public IntPtr MakeCurrent;
        public IntPtr CaptureCurrent;
        public IntPtr RestoreCurrent;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct OpenGlContextThreadStateNative
    {
        public IntPtr DisplayOrDeviceContext;
        public UIntPtr DrawSurface;
        public UIntPtr ReadSurface;
        public IntPtr Context;
        [MarshalAs(UnmanagedType.I1)]
        public bool HasContext;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct StreamParamsOpenGlEglNative
    {
        public StructureHeaderNative Header;
        public IntPtr Display;
        public UIntPtr DrawSurface;
        public UIntPtr ReadSurface;
        public IntPtr Context;
        public OpenGlContextCallbacksNative Callbacks;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct StreamParamsOpenGlWglNative
    {
        public StructureHeaderNative Header;
        public IntPtr DeviceContext;
        public IntPtr Context;
        public OpenGlContextCallbacksNative Callbacks;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct StreamParamsOpenGlGlxNative
    {
        public StructureHeaderNative Header;
        public IntPtr Display;
        public UIntPtr Drawable;
        public IntPtr Context;
        public OpenGlContextCallbacksNative Callbacks;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct StreamParamsVulkanNative
    {
        public StructureHeaderNative Header;
        public IntPtr PhysicalDevice;
        public IntPtr Device;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct RenderFrameShowNative
    {
        public StructureHeaderNative Header;
        public NearFarNative RenderedNearFar;
        public float DepthScale;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct RenderFrameViewOverrideNative
    {
        public UIntPtr ViewIndex;
        [MarshalAs(UnmanagedType.I1)]
        public bool OverridePose;
        [MarshalAs(UnmanagedType.I1)]
        public bool OverrideFov;
        public PoseNative Pose;
        public FovNative Fov;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct RenderFrameShowViewOverridesExtNative
    {
        public StructureHeaderNative Header;
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = NativeConstants.MaxFrameViews)]
        public RenderFrameViewOverrideNative[] ViewOverrides;
        public UIntPtr ViewOverridesCount;
    }

    internal sealed class RuntimeSafeHandle : SafeHandle
    {
        private RuntimeSafeHandle() : base(IntPtr.Zero, true) { }
        internal RuntimeSafeHandle(IntPtr handle) : base(IntPtr.Zero, true) { SetHandle(handle); }
        public override bool IsInvalid { get { return handle == IntPtr.Zero; } }
        protected override bool ReleaseHandle() { NativeMethods.qar_impl_runtime_destroy(handle); return true; }
    }

    internal sealed class SessionSafeHandle : SafeHandle
    {
        private SessionSafeHandle() : base(IntPtr.Zero, true) { }
        internal SessionSafeHandle(IntPtr handle) : base(IntPtr.Zero, true) { SetHandle(handle); }
        public override bool IsInvalid { get { return handle == IntPtr.Zero; } }
        protected override bool ReleaseHandle() { NativeMethods.qar_impl_session_handle_destroy(handle); return true; }
    }

    internal sealed class OnboardingInviteSafeHandle : SafeHandle
    {
        private OnboardingInviteSafeHandle() : base(IntPtr.Zero, true) { }
        internal OnboardingInviteSafeHandle(IntPtr handle) : base(IntPtr.Zero, true) { SetHandle(handle); }
        public override bool IsInvalid { get { return handle == IntPtr.Zero; } }
        protected override bool ReleaseHandle() { NativeMethods.qar_impl_onboarding_invite_handle_destroy(handle); return true; }
    }

    internal sealed class MemberHubInvitationSafeHandle : SafeHandle
    {
        private MemberHubInvitationSafeHandle() : base(IntPtr.Zero, true) { }
        internal MemberHubInvitationSafeHandle(IntPtr handle) : base(IntPtr.Zero, true) { SetHandle(handle); }
        public override bool IsInvalid { get { return handle == IntPtr.Zero; } }
        protected override bool ReleaseHandle() { NativeMethods.qar_impl_member_hub_invitation_handle_destroy(handle); return true; }
    }

    internal sealed class CancelTokenSafeHandle : SafeHandle
    {
        private CancelTokenSafeHandle() : base(IntPtr.Zero, true) { }
        internal CancelTokenSafeHandle(IntPtr handle) : base(IntPtr.Zero, true) { SetHandle(handle); }
        public override bool IsInvalid { get { return handle == IntPtr.Zero; } }
        protected override bool ReleaseHandle() { NativeMethods.qar_impl_cancel_token_handle_destroy(handle); return true; }
    }

    internal sealed class RenderSenderSafeHandle : SafeHandle
    {
        private RenderSenderSafeHandle() : base(IntPtr.Zero, true) { }
        internal RenderSenderSafeHandle(IntPtr handle) : base(IntPtr.Zero, true) { SetHandle(handle); }
        public override bool IsInvalid { get { return handle == IntPtr.Zero; } }
        protected override bool ReleaseHandle() { NativeMethods.qar_impl_render_stream_handle_destroy(handle); return true; }
    }

    internal sealed class VideoSenderSafeHandle : SafeHandle
    {
        private VideoSenderSafeHandle() : base(IntPtr.Zero, true) { }
        internal VideoSenderSafeHandle(IntPtr handle) : base(IntPtr.Zero, true) { SetHandle(handle); }
        public override bool IsInvalid { get { return handle == IntPtr.Zero; } }
        protected override bool ReleaseHandle() { NativeMethods.qar_impl_video_sender_handle_destroy(handle); return true; }
    }

    internal sealed class RenderFrameInfoSafeHandle : SafeHandle
    {
        private RenderFrameInfoSafeHandle() : base(IntPtr.Zero, true) { }
        internal RenderFrameInfoSafeHandle(IntPtr handle) : base(IntPtr.Zero, true) { SetHandle(handle); }
        public override bool IsInvalid { get { return handle == IntPtr.Zero; } }
        protected override bool ReleaseHandle() { NativeMethods.qar_impl_render_frame_info_handle_destroy(handle); return true; }
    }
}
