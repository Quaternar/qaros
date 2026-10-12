using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Qar.Native;

namespace Qar
{
    public enum AppState
    {
        Unknown = 0,
        Initializing = 1,
        Running = 10,
        ShuttingDown = 100,
        Retired = 200
    }

    public enum GraphicsAPI
    {
        Cpu = 0,
        D3D11 = 4,
        /// <summary>Requires a library built with QAR_ENABLE_VULKAN and <see cref="StreamParamsVulkan"/>.</summary>
        Vulkan = 6,
        /// <summary>Requires a library built with QAR_ENABLE_OPENGL and exactly one <see cref="StreamParamsOpenGl"/> extension.</summary>
        OpenGL = 7
    }

    /// <summary>Which of a peer's two hands something came from.</summary>
    public enum Hand
    {
        Left = 0,
        Right = 1
    }

    /// <summary>XR_EXT_hand_tracking joint indices; <see cref="HandJointLocation.JointId"/> carries one.</summary>
    public enum HandJoint
    {
        Palm = 0,
        Wrist = 1,
        ThumbMetacarpal = 2,
        ThumbProximal = 3,
        ThumbDistal = 4,
        ThumbTip = 5,
        IndexMetacarpal = 6,
        IndexProximal = 7,
        IndexIntermediate = 8,
        IndexDistal = 9,
        IndexTip = 10,
        MiddleMetacarpal = 11,
        MiddleProximal = 12,
        MiddleIntermediate = 13,
        MiddleDistal = 14,
        MiddleTip = 15,
        RingMetacarpal = 16,
        RingProximal = 17,
        RingIntermediate = 18,
        RingDistal = 19,
        RingTip = 20,
        LittleMetacarpal = 21,
        LittleProximal = 22,
        LittleIntermediate = 23,
        LittleDistal = 24,
        LittleTip = 25
    }

    /// <summary>Bits of <see cref="HandJointLocation.LocationFlags"/> and <see cref="HandJointVelocity.Flags"/>.</summary>
    [Flags]
    public enum HandJointLocationFlags : ulong
    {
        None = 0,
        OrientationValid = 0x1,
        PositionValid = 0x2,
        OrientationTracked = 0x4,
        PositionTracked = 0x8
    }

    /// <summary>What a contact on a GUI panel was recognised as.</summary>
    public enum GuiPanelGestureKind
    {
        /// <summary>Fingertip inside the hover volume. A lifecycle: started on entry, updated as the content position changes, ended on exit.</summary>
        Hover = 0,
        /// <summary>Touched and released with little travel and before the long-press duration. Instant.</summary>
        Tap = 1,
        /// <summary>Touched and held in place for the long-press duration. Instant, once per contact.</summary>
        LongPress = 2,
        /// <summary>A contact that travelled past the tap threshold. A lifecycle: started at the threshold crossing, ended on release, canceled when taken away.</summary>
        Drag = 3
    }

    /// <summary>How <see cref="GraphicsDeviceId"/> names the adapter.</summary>
    public enum GpuDeviceIdType
    {
        Luid = 0,
        Uuid = 1
    }

    public enum PixelFormat
    {
        R32Float = 2,
        R8G8B8A8 = 51,
        B8G8R8A8 = 53,
        B8G8R8X8 = 56,
        R16G16B16A16 = 55,
        D32Float = 101
    }

    public enum FrameLayout
    {
        SideBySide = 0,
        SeparatedTextures = 1,
        Layered = 2
    }

    public enum VideoFrameViewType
    {
        Grayscale = 0,
        Color = 1,
        Depth = 2
    }

    public enum VideoFrameViewEye
    {
        None = 0,
        Left = 10,
        Right = 20
    }

    public enum GuiPanelState
    {
        Visible = 0,
        Minimized = 1,
        Hidden = 2,
        Closed = 3
    }

    public enum WorldReferenceSystem
    {
        EcefWgs84Meters = 0
    }

    public enum Handedness
    {
        Right = 0,
        Left = 1
    }

    public enum AppVolumeLifetimeStatus
    {
        Active = 0,
        Closed = 1
    }

    public enum GestureKind
    {
        Click = 0,
        Hover = 1,
        SinglePointer6Dof = 2,
        DualPointerTranslateDistance = 3,
        DualPointerRotate = 4
    }

    public enum GesturePhase
    {
        Started = 0,
        Updated = 1,
        Ended = 2,
        Instant = 3,
        Canceled = 4
    }

    [Flags]
    public enum AppVolumeAxisFlags
    {
        None = 0,
        X = 1 << 0,
        Y = 1 << 1,
        Z = 1 << 2,
        All = (1 << 3) - 1
    }

    public enum AppScaleSensitivityMode
    {
        /// <summary>Ignore AppScale. With Precision = 1.0, 1 cm hand movement maps to 1 cm of gesture delta.</summary>
        Constant = 0,
        /// <summary>Divide by AppScale. With AppScale = 4.0, 1 cm maps to 0.25 cm, so gestures slow down as the rendered app gets bigger.</summary>
        InverseToAppScale = 1,
        /// <summary>Multiply by AppScale. With AppScale = 4.0, 1 cm maps to 4 cm, so gestures speed up as the rendered app gets bigger.</summary>
        BasedOnAppScale = 2
    }

    public enum GestureAppTransformMapping
    {
        None = 0,
        AppPose = 1,
        AppScale = 2
    }

    public enum AppVolumeControllerRayLineStyle
    {
        None = 0,
        Solid = 1,
        Dotted = 2
    }

    public enum ActionSeverity
    {
        Info = 0,
        Warning = 1,
        Error = 2,
        Done = 3
    }

    public enum OnboardingMethod
    {
        Pake = 0,
        Password = 1
    }

    public enum StructureType
    {
        Unknown = 0,
        LibraryInit = 0x0001,
        RuntimeInit = 0x1000,
        RuntimeRejoinInit = 0x1002,
        RuntimeOnboardInit = 0x1003,
        RuntimeForgetInit = 0x1005,
        RuntimeOnboardHostExt = 0x1006,
        RuntimeOnboardCodeExt = 0x1007,
        RuntimeOnboardInviteExt = 0x1008,
        RuntimeOnboardPeerIdExt = 0x1009,
        SessionGraphicsDeviceId = 0x2004,
        SessionRequestInviteInit = 0x2005,
        PeerPresentation = 0x2006,
        SessionInviteTargetAppInit = 0x2007,
        SessionRequestOnboardingCodesInit = 0x2008,
        RenderingStreamSenderInit = 0x3000,
        RenderingBeginFrame = 0x3001,
        RenderingEndFrame = 0x3002,
        RenderingEndFrameViewOverridesExt = 0x3004,
        VideoSenderInit = 0x3005,
        StreamD3D11ParamsExt = 0x4000,
        StreamVulkanParamsExt = 0x4002,
        StreamOpenGlEglParamsExt = 0x4003,
        StreamOpenGlWglParamsExt = 0x4004,
        StreamOpenGlGlxParamsExt = 0x4005,
        GuiPanelInit = 0x5001,
        GuiPanelStreamIdExt = 0x5002,
        GuiPanelUrlExt = 0x5003,
        AppVolumeInit = 0x5501,
        AppVolumeGestureMappingRule = 0x5502,
        AppVolumeGestureConfiguration = 0x5503,
        AppVolumeGestureAllowedPeersExt = 0x5504
    }

    public enum LogSeverity
    {
        Trace = 0,
        Debug = 1,
        Info = 2,
        Warn = 3,
        Error = 4,
        Fatal = 5
    }

    public enum GuiPanelContentType
    {
        WebsiteUri = 0,
        StreamId = 1
    }

    public readonly struct Vector3
    {
        public Vector3(float x, float y, float z)
        {
            X = x;
            Y = y;
            Z = z;
        }

        public float X { get; }
        public float Y { get; }
        public float Z { get; }

        internal Vector3Native ToNative()
        {
            return new Vector3Native { X = X, Y = Y, Z = Z };
        }

        internal static Vector3 FromNative(Vector3Native value)
        {
            return new Vector3(value.X, value.Y, value.Z);
        }
    }

    public readonly struct Vector3d
    {
        public Vector3d(double x, double y, double z)
        {
            X = x;
            Y = y;
            Z = z;
        }

        public double X { get; }
        public double Y { get; }
        public double Z { get; }

        internal Vector3dNative ToNative()
        {
            return new Vector3dNative { X = X, Y = Y, Z = Z };
        }

        internal static Vector3d FromNative(Vector3dNative value)
        {
            return new Vector3d(value.X, value.Y, value.Z);
        }
    }

    public readonly struct Quaternion
    {
        public Quaternion(float x, float y, float z, float w)
        {
            X = x;
            Y = y;
            Z = z;
            W = w;
        }

        public float X { get; }
        public float Y { get; }
        public float Z { get; }
        public float W { get; }

        public static Quaternion Identity { get { return new Quaternion(0.0f, 0.0f, 0.0f, 1.0f); } }

        internal QuaternionNative ToNative()
        {
            return new QuaternionNative { X = X, Y = Y, Z = Z, W = W };
        }

        internal static Quaternion FromNative(QuaternionNative value)
        {
            return new Quaternion(value.X, value.Y, value.Z, value.W);
        }
    }

    /// <summary>
    /// Position plus orientation (x, y, z, w). A pose carries no space of its
    /// own: every member that holds one names the space it is in. The bindings
    /// convert no coordinates; each value is exactly what the native library
    /// produced.
    /// </summary>
    /// <remarks>
    /// Every 3D space follows OpenXR: right-handed, +Y up, -Z forward, so a view
    /// looks along -Z. The spaces, as in the C header (types.h, "Coordinate
    /// systems") and https://docs.quaternar.com/docs/developer-guide/coordinate-systems/:
    /// <list type="bullet">
    /// <item><description><b>Room space</b>: the frame every peer in a session places things in, in metres. Whatever arrives through data streams and shared objects is in the publishing peer's room space: app volume poses, GUI panel poses, peer data.</description></item>
    /// <item><description><b>App volume space</b>: one app volume's local frame, origin at the cuboid centre, +X along width, +Y up along height, +Z along length, in metres. The app pose and the deltas of a mapped app volume gesture are in it.</description></item>
    /// <item><description><b>App content space</b>: the application's own world inside the volume, the space it renders in: app volume space taken into the app pose, positions divided by the app scale, so in app metres. Everything a render sender hands the source application is in the bound volume's app content space: view poses, the rendered pose of a view override, hand aim poses, joint poses, joint radii and joint velocities.</description></item>
    /// <item><description><b>View space</b>: one render view's camera frame, looking along -Z. <see cref="Fov"/> angles are in it.</description></item>
    /// </list>
    /// </remarks>
    public readonly struct Pose
    {
        public Pose(Quaternion orientation, Vector3 position)
        {
            Orientation = orientation;
            Position = position;
        }

        public Quaternion Orientation { get; }
        public Vector3 Position { get; }

        public static Pose Identity { get { return new Pose(Quaternion.Identity, new Vector3(0, 0, 0)); } }

        internal PoseNative ToNative()
        {
            return new PoseNative { Orientation = Orientation.ToNative(), Position = Position.ToNative() };
        }

        internal static Pose FromNative(PoseNative value)
        {
            return new Pose(Quaternion.FromNative(value.Orientation), Vector3.FromNative(value.Position));
        }
    }

    public readonly struct Fov
    {
        public Fov(float angleLeft, float angleRight, float angleUp, float angleDown)
        {
            AngleLeft = angleLeft;
            AngleRight = angleRight;
            AngleUp = angleUp;
            AngleDown = angleDown;
        }

        public float AngleLeft { get; }
        public float AngleRight { get; }
        public float AngleUp { get; }
        public float AngleDown { get; }

        internal FovNative ToNative()
        {
            return new FovNative { AngleLeft = AngleLeft, AngleRight = AngleRight, AngleUp = AngleUp, AngleDown = AngleDown };
        }

        internal static Fov FromNative(FovNative value)
        {
            return new Fov(value.AngleLeft, value.AngleRight, value.AngleUp, value.AngleDown);
        }
    }

    public readonly struct NearFar
    {
        public NearFar(float nearPlane, float farPlane)
        {
            NearPlane = nearPlane;
            FarPlane = farPlane;
        }

        public float NearPlane { get; }
        public float FarPlane { get; }

        internal NearFarNative ToNative()
        {
            return new NearFarNative { NearPlane = NearPlane, FarPlane = FarPlane };
        }

        internal static NearFar FromNative(NearFarNative value)
        {
            return new NearFar(value.NearPlane, value.FarPlane);
        }
    }

    public readonly struct TimePoint
    {
        public TimePoint(ulong count, byte precision)
        {
            Count = count;
            Precision = precision;
        }

        public ulong Count { get; }
        public byte Precision { get; }

        internal static TimePoint FromNative(TimePointNative value)
        {
            return new TimePoint(value.Count, value.Precision);
        }
    }

    public readonly struct PeerId : IEquatable<PeerId>
    {
        private readonly byte[]? _data;

        private PeerId(byte[] data)
        {
            _data = IdBytes.Normalize(data);
        }

        public static PeerId Default { get { return new PeerId(new byte[NativeConstants.MaxIdLength]); } }
        public static PeerId Unique() { return FromNative(NativeMethods.qar_impl_peer_id_unique()); }
        public static PeerId Parse(string text) { PeerId id; if (!TryParse(text, out id)) throw new FormatException("QAR IDs must use the native UUID string format."); return id; }
        public static bool TryParse(string text, out PeerId id) { byte[] data; if (!IdBytes.TryParse(text, out data)) { id = default(PeerId); return false; } id = new PeerId(data); return true; }
        public byte[] ToByteArray() { return IdBytes.Copy(_data); }
#if NETSTANDARD2_1_OR_GREATER || NET8_0_OR_GREATER
        public ReadOnlySpan<byte> AsSpan() { return IdBytes.AsSpan(_data); }
        public bool TryCopyTo(Span<byte> destination) { return IdBytes.TryCopyTo(_data, destination); }
#endif
        public override string ToString() { return IdBytes.ToAscii(_data); }
        public bool Equals(PeerId other) { return IdBytes.Equals(_data, other._data); }
        public override bool Equals(object? obj) { return obj is PeerId other && Equals(other); }
        public override int GetHashCode() { return IdBytes.GetHashCode(_data); }
        public static bool operator ==(PeerId left, PeerId right) { return left.Equals(right); }
        public static bool operator !=(PeerId left, PeerId right) { return !left.Equals(right); }
        internal PeerIdNative ToNative() { return new PeerIdNative { Data = ToByteArray() }; }
        internal static PeerId FromNative(PeerIdNative value) { return new PeerId(value.Data); }
        internal static PeerId FromNativePointer(IntPtr ptr) { return FromNative(Marshal.PtrToStructure<PeerIdNative>(ptr)); }
    }

    public readonly struct SessionId : IEquatable<SessionId>
    {
        private readonly byte[]? _data;
        private SessionId(byte[] data) { _data = IdBytes.Normalize(data); }
        public static SessionId Default { get { return new SessionId(new byte[NativeConstants.MaxIdLength]); } }
        public static SessionId Unique() { return FromNative(NativeMethods.qar_impl_session_id_unique()); }
        public static SessionId Parse(string text) { SessionId id; if (!TryParse(text, out id)) throw new FormatException("QAR IDs must use the native UUID string format."); return id; }
        public static bool TryParse(string text, out SessionId id) { byte[] data; if (!IdBytes.TryParse(text, out data)) { id = default(SessionId); return false; } id = new SessionId(data); return true; }
        public byte[] ToByteArray() { return IdBytes.Copy(_data); }
#if NETSTANDARD2_1_OR_GREATER || NET8_0_OR_GREATER
        public ReadOnlySpan<byte> AsSpan() { return IdBytes.AsSpan(_data); }
        public bool TryCopyTo(Span<byte> destination) { return IdBytes.TryCopyTo(_data, destination); }
#endif
        public override string ToString() { return IdBytes.ToAscii(_data); }
        public bool Equals(SessionId other) { return IdBytes.Equals(_data, other._data); }
        public override bool Equals(object? obj) { return obj is SessionId other && Equals(other); }
        public override int GetHashCode() { return IdBytes.GetHashCode(_data); }
        public static bool operator ==(SessionId left, SessionId right) { return left.Equals(right); }
        public static bool operator !=(SessionId left, SessionId right) { return !left.Equals(right); }
        internal SessionIdNative ToNative() { return new SessionIdNative { Data = ToByteArray() }; }
        internal static SessionId FromNative(SessionIdNative value) { return new SessionId(value.Data); }
    }

    public readonly struct GuiPanelId : IEquatable<GuiPanelId>
    {
        private readonly byte[]? _data;
        private GuiPanelId(byte[] data) { _data = IdBytes.Normalize(data); }
        public static GuiPanelId Default { get { return new GuiPanelId(new byte[NativeConstants.MaxIdLength]); } }
        public static GuiPanelId Unique() { return FromNative(NativeMethods.qar_impl_gui_panel_id_unique()); }
        public static GuiPanelId Parse(string text) { GuiPanelId id; if (!TryParse(text, out id)) throw new FormatException("QAR IDs must use the native UUID string format."); return id; }
        public static bool TryParse(string text, out GuiPanelId id) { byte[] data; if (!IdBytes.TryParse(text, out data)) { id = default(GuiPanelId); return false; } id = new GuiPanelId(data); return true; }
        public byte[] ToByteArray() { return IdBytes.Copy(_data); }
#if NETSTANDARD2_1_OR_GREATER || NET8_0_OR_GREATER
        public ReadOnlySpan<byte> AsSpan() { return IdBytes.AsSpan(_data); }
        public bool TryCopyTo(Span<byte> destination) { return IdBytes.TryCopyTo(_data, destination); }
#endif
        public override string ToString() { return IdBytes.ToAscii(_data); }
        public bool Equals(GuiPanelId other) { return IdBytes.Equals(_data, other._data); }
        public override bool Equals(object? obj) { return obj is GuiPanelId other && Equals(other); }
        public override int GetHashCode() { return IdBytes.GetHashCode(_data); }
        public static bool operator ==(GuiPanelId left, GuiPanelId right) { return left.Equals(right); }
        public static bool operator !=(GuiPanelId left, GuiPanelId right) { return !left.Equals(right); }
        internal GuiPanelIdNative ToNative() { return new GuiPanelIdNative { Data = ToByteArray() }; }
        internal static GuiPanelId FromNative(GuiPanelIdNative value) { return new GuiPanelId(value.Data); }
    }

    public readonly struct AppVolumeId : IEquatable<AppVolumeId>
    {
        private readonly byte[]? _data;
        private AppVolumeId(byte[] data) { _data = IdBytes.Normalize(data); }
        public static AppVolumeId Default { get { return new AppVolumeId(new byte[NativeConstants.MaxIdLength]); } }
        public static AppVolumeId Parse(string text) { AppVolumeId id; if (!TryParse(text, out id)) throw new FormatException("QAR IDs must use the native UUID string format."); return id; }
        public static bool TryParse(string text, out AppVolumeId id) { byte[] data; if (!IdBytes.TryParse(text, out data)) { id = default(AppVolumeId); return false; } id = new AppVolumeId(data); return true; }
        public byte[] ToByteArray() { return IdBytes.Copy(_data); }
#if NETSTANDARD2_1_OR_GREATER || NET8_0_OR_GREATER
        public ReadOnlySpan<byte> AsSpan() { return IdBytes.AsSpan(_data); }
        public bool TryCopyTo(Span<byte> destination) { return IdBytes.TryCopyTo(_data, destination); }
#endif
        public override string ToString() { return IdBytes.ToAscii(_data); }
        public bool Equals(AppVolumeId other) { return IdBytes.Equals(_data, other._data); }
        public override bool Equals(object? obj) { return obj is AppVolumeId other && Equals(other); }
        public override int GetHashCode() { return IdBytes.GetHashCode(_data); }
        public static bool operator ==(AppVolumeId left, AppVolumeId right) { return left.Equals(right); }
        public static bool operator !=(AppVolumeId left, AppVolumeId right) { return !left.Equals(right); }
        internal AppVolumeIdNative ToNative() { return new AppVolumeIdNative { Data = ToByteArray() }; }
        internal static AppVolumeId FromNative(AppVolumeIdNative value) { return new AppVolumeId(value.Data); }
    }

    public readonly struct StreamId : IEquatable<StreamId>
    {
        private readonly byte[]? _data;
        private StreamId(byte[] data) { _data = IdBytes.Normalize(data); }
        public static StreamId Default { get { return new StreamId(new byte[NativeConstants.MaxIdLength]); } }
        public static StreamId Parse(string text) { StreamId id; if (!TryParse(text, out id)) throw new FormatException("QAR IDs must use the native UUID string format."); return id; }
        public static bool TryParse(string text, out StreamId id) { byte[] data; if (!IdBytes.TryParse(text, out data)) { id = default(StreamId); return false; } id = new StreamId(data); return true; }
        public byte[] ToByteArray() { return IdBytes.Copy(_data); }
#if NETSTANDARD2_1_OR_GREATER || NET8_0_OR_GREATER
        public ReadOnlySpan<byte> AsSpan() { return IdBytes.AsSpan(_data); }
        public bool TryCopyTo(Span<byte> destination) { return IdBytes.TryCopyTo(_data, destination); }
#endif
        public override string ToString() { return IdBytes.ToAscii(_data); }
        public bool Equals(StreamId other) { return IdBytes.Equals(_data, other._data); }
        public override bool Equals(object? obj) { return obj is StreamId other && Equals(other); }
        public override int GetHashCode() { return IdBytes.GetHashCode(_data); }
        public static bool operator ==(StreamId left, StreamId right) { return left.Equals(right); }
        public static bool operator !=(StreamId left, StreamId right) { return !left.Equals(right); }
        internal StreamIdNative ToNative() { return new StreamIdNative { Data = ToByteArray() }; }
        internal static StreamId FromNative(StreamIdNative value) { return new StreamId(value.Data); }
    }

    public abstract class Structure
    {
        protected Structure(StructureType type)
        {
            Type = type;
        }

        public StructureType Type { get; }
        internal Structure? Next { get; set; }
    }

    internal static class StructureChainBuilder
    {
        internal static Structure? Append(Structure? head, Structure extension)
        {
            if (extension == null)
                throw new ArgumentNullException(nameof(extension));
            extension.Next = null;
            if (head == null)
                return extension;
            var current = head;
            while (current.Next != null)
                current = current.Next;
            current.Next = extension;
            return head;
        }
    }

    /// <summary>
    /// Extension: join the session under a caller-chosen peer id instead of a
    /// freshly minted one. Chain on <see cref="OnboardOptions"/> with
    /// <see cref="OnboardOptions.WithPeerId(PeerId)"/>. The id is persisted with
    /// the slot, so a later rejoin keeps it. A zero id is rejected with
    /// <see cref="StatusCode.ArgumentNotSupported"/>.
    /// </summary>
    public sealed class OnboardPeerId : Structure
    {
        public OnboardPeerId(PeerId peerId)
            : base(StructureType.RuntimeOnboardPeerIdExt)
        {
            PeerId = peerId;
        }

        public PeerId PeerId { get; set; }
    }

    /// <summary>
    /// Extension: selects the graphics adapter the session's Vulkan and D3D11
    /// devices, and with them every render sender's frame textures, are created
    /// on. Chain on <see cref="OnboardOptions"/> or <see cref="RejoinOptions"/>;
    /// absent, the library picks one. Windows accepts a LUID, other platforms a
    /// UUID. The C API currently supports NVIDIA adapters only.
    /// </summary>
    public sealed class GraphicsDeviceId : Structure
    {
        public GraphicsDeviceId()
            : base(StructureType.SessionGraphicsDeviceId)
        {
            IdType = GpuDeviceIdType.Luid;
            Uuid = new byte[16];
        }

        public GpuDeviceIdType IdType { get; set; }

        /// <summary>The adapter LUID as one 64-bit value: HighPart in the upper 32 bits, LowPart in the lower.</summary>
        public long Luid { get; set; }

        /// <summary>The adapter UUID, 16 bytes; used when <see cref="IdType"/> is <see cref="GpuDeviceIdType.Uuid"/>.</summary>
        public byte[] Uuid { get; set; }

        public static GraphicsDeviceId FromLuid(long luid)
        {
            return new GraphicsDeviceId { IdType = GpuDeviceIdType.Luid, Luid = luid };
        }

        public static GraphicsDeviceId FromUuid(byte[] uuid)
        {
            if (uuid == null)
                throw new ArgumentNullException(nameof(uuid));
            if (uuid.Length != 16)
                throw new ArgumentException("A GPU UUID is 16 bytes.", nameof(uuid));
            return new GraphicsDeviceId { IdType = GpuDeviceIdType.Uuid, Uuid = (byte[])uuid.Clone() };
        }

        internal static GraphicsDeviceId FromNative(GraphicsDeviceIdNative native)
        {
            if (native.IdType == GpuDeviceIdType.Uuid)
                return FromUuid(native.Uuid);
            return FromLuid(((long)native.Luid.HighPart << 32) | native.Luid.LowPart);
        }

        internal GraphicsDeviceIdNative ToNative()
        {
            var uuid = new byte[NativeConstants.GpuUuidLength];
            if (Uuid != null)
                Array.Copy(Uuid, uuid, Math.Min(Uuid.Length, uuid.Length));
            return new GraphicsDeviceIdNative
            {
                Header = NativeDefaults.Header(StructureType.SessionGraphicsDeviceId),
                IdType = IdType,
                Luid = new LuidNative { LowPart = unchecked((uint)Luid), HighPart = (int)(Luid >> 32) },
                Uuid = uuid
            };
        }
    }

    /// <summary>
    /// Raw C function pointers overriding how the library binds an OpenGL
    /// context; see <c>QarOpenGlContextCallbacks</c> in types.h. Every one is
    /// optional and called on the library's internal graphics thread; zero uses
    /// the library's own platform implementation for that step.
    /// </summary>
    public readonly struct OpenGlContextCallbacks
    {
        public OpenGlContextCallbacks(IntPtr userState, IntPtr getProcAddress, IntPtr makeCurrent, IntPtr captureCurrent, IntPtr restoreCurrent)
        {
            UserState = userState;
            GetProcAddress = getProcAddress;
            MakeCurrent = makeCurrent;
            CaptureCurrent = captureCurrent;
            RestoreCurrent = restoreCurrent;
        }

        public IntPtr UserState { get; }
        public IntPtr GetProcAddress { get; }
        public IntPtr MakeCurrent { get; }
        public IntPtr CaptureCurrent { get; }
        public IntPtr RestoreCurrent { get; }

        internal OpenGlContextCallbacksNative ToNative()
        {
            return new OpenGlContextCallbacksNative { UserState = UserState, GetProcAddress = GetProcAddress, MakeCurrent = MakeCurrent, CaptureCurrent = CaptureCurrent, RestoreCurrent = RestoreCurrent };
        }
    }

    /// <summary>
    /// Base of the three OpenGL stream-parameter extensions. Pick the one matching
    /// the context the application owns; requires a library built with
    /// QAR_ENABLE_OPENGL.
    /// </summary>
    public abstract class StreamParamsOpenGl : Structure
    {
        private protected StreamParamsOpenGl(StructureType type)
            : base(type)
        {
        }

        /// <summary>Required. EGLContext, HGLRC or GLXContext; a null context is rejected.</summary>
        public IntPtr Context { get; set; }

        public OpenGlContextCallbacks Callbacks { get; set; }
    }

    /// <summary>Extension: adopt an existing EGL context.</summary>
    public sealed class StreamParamsOpenGlEgl : StreamParamsOpenGl
    {
        public StreamParamsOpenGlEgl()
            : base(StructureType.StreamOpenGlEglParamsExt)
        {
        }

        /// <summary>EGLDisplay.</summary>
        public IntPtr Display { get; set; }

        /// <summary>EGLSurface used for drawing.</summary>
        public UIntPtr DrawSurface { get; set; }

        /// <summary>EGLSurface used for reading.</summary>
        public UIntPtr ReadSurface { get; set; }
    }

    /// <summary>Extension: adopt an existing WGL context.</summary>
    public sealed class StreamParamsOpenGlWgl : StreamParamsOpenGl
    {
        public StreamParamsOpenGlWgl()
            : base(StructureType.StreamOpenGlWglParamsExt)
        {
        }

        /// <summary>HDC the context was created against.</summary>
        public IntPtr DeviceContext { get; set; }
    }

    /// <summary>Extension: adopt an existing GLX context.</summary>
    public sealed class StreamParamsOpenGlGlx : StreamParamsOpenGl
    {
        public StreamParamsOpenGlGlx()
            : base(StructureType.StreamOpenGlGlxParamsExt)
        {
        }

        /// <summary>X11 Display*.</summary>
        public IntPtr Display { get; set; }

        /// <summary>GLXDrawable.</summary>
        public UIntPtr Drawable { get; set; }
    }

    /// <summary>Extension: adopt an existing Vulkan device. Requires a library built with QAR_ENABLE_VULKAN.</summary>
    public sealed class StreamParamsVulkan : Structure
    {
        public StreamParamsVulkan()
            : base(StructureType.StreamVulkanParamsExt)
        {
        }

        /// <summary>VkPhysicalDevice.</summary>
        public IntPtr PhysicalDevice { get; set; }

        /// <summary>VkDevice.</summary>
        public IntPtr Device { get; set; }
    }

    public sealed class StreamParamsD3D11 : Structure
    {
        public StreamParamsD3D11()
            : base(StructureType.StreamD3D11ParamsExt)
        {
            AcquireKeyedMutexSync = true;
        }

        public IntPtr D3D11Context { get; set; }
        public IntPtr D3D11Device { get; set; }
        public uint ColorBindFlags { get; set; }
        public uint DepthBindFlags { get; set; }
        public bool AcquireKeyedMutexSync { get; set; }
    }

    public sealed class RenderFrameShowViewOverrides : Structure
    {
        public RenderFrameShowViewOverrides()
            : base(StructureType.RenderingEndFrameViewOverridesExt)
        {
            ViewOverrides = Array.Empty<RenderFrameViewOverride>();
        }

        public RenderFrameViewOverride[] ViewOverrides { get; set; }
    }

    public sealed class LibraryOptions
    {
        public LibraryOptions()
        {
            EnableConsoleLogging = true;
            LogSeverity = LogSeverity.Info;
        }

        public bool EnableConsoleLogging { get; set; }
        public string? LogFolderPath { get; set; }
        public LogSeverity LogSeverity { get; set; }
        internal Structure? Extension { get; private set; }

        public LibraryOptions WithExtension(Structure extension)
        {
            Extension = StructureChainBuilder.Append(Extension, extension);
            return this;
        }
    }

    public sealed class RuntimeOptions
    {
        public string? RuntimeBinariesFolderPath { get; set; }
        public string? StorageFolderPath { get; set; }
        internal Structure? Extension { get; private set; }

        public RuntimeOptions WithExtension(Structure extension)
        {
            Extension = StructureChainBuilder.Append(Extension, extension);
            return this;
        }
    }

    public sealed class RenderSenderOptions
    {
        public RenderSenderOptions(PeerId peerId, GraphicsAPI graphicsAPI)
        {
            PeerId = peerId;
            GraphicsAPI = graphicsAPI;
            EnableAutoReconnects = true;
            TextureLayout = FrameLayout.SideBySide;
            FrameViews = Array.Empty<RenderFrameView>();
            ColorFormat = PixelFormat.B8G8R8A8;
            DepthFormat = PixelFormat.D32Float;
        }

        /// <summary>
        /// Required. The peer this stream renders for - the headset or viewer that
        /// receives the frames, never this process' own peer id. Take it from a
        /// render stream request (<see cref="RenderSenderControl.SubscribeRequests"/>)
        /// or from <see cref="Session.InviteTargetApp"/>. <see cref="PeerId.Unique"/>
        /// belongs to no peer, so a stream created with one never produces a frame.
        /// </summary>
        public PeerId PeerId { get; set; }
        public GraphicsAPI GraphicsAPI { get; set; }
        public bool EnableAutoReconnects { get; set; }
        public FrameLayout TextureLayout { get; set; }
        /// <summary>
        /// The views of one video frame. <b>The order is guaranteed</b>: the
        /// layout (<see cref="RenderSender"/> layout) and every frame the sender
        /// hands back list their views in exactly this order, each with the eye
        /// and data type declared at its position. Textures come in the order the
        /// views first use them (one per view for separated textures, one per data
        /// type for side by side and layered), and each view's texture index names
        /// its texture. A view the frame layout provides but was not declared
        /// follows the declared ones.
        /// </summary>
        public RenderFrameView[] FrameViews { get; set; }
        /// <summary>
        /// Required. The app volume this stream is displayed in, which places it in
        /// the shared scene and defines the app content space every pose the sender
        /// hands out is in. Create it first with <see cref="AppVolumeControl.GetOrCreate"/>.
        /// Null is rejected with <see cref="StatusCode.AppVolumeInvalidId"/>.
        /// </summary>
        public AppVolumeId? AppVolumeId { get; set; }
        public PixelFormat ColorFormat { get; set; }
        public PixelFormat DepthFormat { get; set; }
        /// <summary>Required when <see cref="GraphicsAPI"/> is <see cref="GraphicsAPI.D3D11"/>; must be null otherwise.</summary>
        public StreamParamsD3D11? StreamParamsD3D11 { get; set; }
        /// <summary>Required when <see cref="GraphicsAPI"/> is <see cref="GraphicsAPI.Vulkan"/>; must be null otherwise.</summary>
        public StreamParamsVulkan? StreamParamsVulkan { get; set; }
        /// <summary>Required when <see cref="GraphicsAPI"/> is <see cref="GraphicsAPI.OpenGL"/>; must be null otherwise.</summary>
        public StreamParamsOpenGl? StreamParamsOpenGl { get; set; }
    }

    public sealed class PrimarylessVideoSenderOptions
    {
        public PrimarylessVideoSenderOptions(VideoFrameLayout layout)
        {
            Layout = layout ?? throw new ArgumentNullException(nameof(layout));
            EnableReconnects = true;
            ReconnectTimeout = TimeSpan.FromMinutes(5);
            GraphicsAPI = GraphicsAPI.Cpu;
        }

        public VideoFrameLayout Layout { get; set; }
        public bool EnableReconnects { get; set; }
        public TimeSpan ReconnectTimeout { get; set; }
        public GraphicsAPI GraphicsAPI { get; set; }
        /// <summary>Required when <see cref="GraphicsAPI"/> is <see cref="GraphicsAPI.D3D11"/>; must be null otherwise.</summary>
        public StreamParamsD3D11? StreamParamsD3D11 { get; set; }
        /// <summary>Required when <see cref="GraphicsAPI"/> is <see cref="GraphicsAPI.Vulkan"/>; must be null otherwise.</summary>
        public StreamParamsVulkan? StreamParamsVulkan { get; set; }
        /// <summary>Required when <see cref="GraphicsAPI"/> is <see cref="GraphicsAPI.OpenGL"/>; must be null otherwise.</summary>
        public StreamParamsOpenGl? StreamParamsOpenGl { get; set; }
    }

    public sealed class GuiPanelOptions
    {
        /// <param name="commonName">
        /// Required stable identity key the panel's id is derived from:
        /// <see cref="GuiPanelControl.GetOrCreate"/> returns the existing panel with
        /// this name, or creates one.
        /// </param>
        public GuiPanelOptions(string commonName)
        {
            CommonName = commonName ?? throw new ArgumentNullException(nameof(commonName));
            Pose = Pose.Identity;
            Size = GuiPanelSize.Default;
            VisibleToPeers = Array.Empty<PeerId>();
        }

        /// <summary>Required stable identity key the panel's id is derived from.</summary>
        public string CommonName { get; set; }
        public string? DisplayName { get; set; }
        /// <summary>
        /// The panel's pose in room space: the midpoint of its top edge, front
        /// normal along -Z of the pose, the surface hanging below it.
        /// </summary>
        public Pose Pose { get; set; }
        public GuiPanelSize Size { get; set; }
        public string? ContentUri { get; set; }
        public StreamId? ContentStreamId { get; set; }
        public PeerId[] VisibleToPeers { get; set; }
        internal Structure? Extension { get; private set; }

        public GuiPanelOptions WithExtension(Structure extension)
        {
            Extension = StructureChainBuilder.Append(Extension, extension);
            return this;
        }
    }

    public sealed class AppVolumeOptions
    {
        /// <param name="commonName">
        /// Required stable identity key the volume's id is derived from:
        /// <see cref="AppVolumeControl.GetOrCreate"/> returns the existing volume
        /// with this name, or creates one.
        /// </param>
        public AppVolumeOptions(string commonName)
        {
            CommonName = commonName ?? throw new ArgumentNullException(nameof(commonName));
            Pose = Pose.Identity;
            Size = AppVolumeSize.Default;
            InitialPeers = Array.Empty<PeerId>();
            AppPose = Pose.Identity;
            AppScale = 1.0f;
        }

        /// <summary>Required stable identity key the volume's id is derived from.</summary>
        public string CommonName { get; set; }
        public string? DisplayName { get; set; }
        /// <summary>
        /// Placement of the cuboid's centre in room space, and the rotation taking
        /// app volume space into room space.
        /// </summary>
        public Pose Pose { get; set; }
        /// <summary>Cuboid extents in metres along app-volume-space X (width), Z (length) and Y (height).</summary>
        public AppVolumeSize Size { get; set; }
        public PeerId[] InitialPeers { get; set; }
        /// <summary>
        /// Origin of the rendering application inside app volume space, in volume
        /// metres, relative to the cuboid centre. Not a room-space pose, and
        /// <see cref="AppScale"/> does not scale it.
        /// </summary>
        public Pose AppPose { get; set; }
        /// <summary>
        /// Room-metres per app-metre: the factor between app content space and
        /// app volume space. 1.0 means 1 m in app coordinates renders as 1 m in
        /// the app volume; 0.5 makes the app appear smaller, 10.0 bigger.
        /// <see cref="AppPose"/> remains in app-volume metres.
        /// </summary>
        public float AppScale { get; set; }
        public GeoAnchorFrame? AppWorldAnchor { get; set; }
        public AppVolumeGestureConfiguration? GestureConfiguration { get; set; }
        internal Structure? Extension { get; private set; }

        public AppVolumeOptions WithExtension(Structure extension)
        {
            Extension = StructureChainBuilder.Append(Extension, extension);
            return this;
        }
    }

    public sealed class RenderFrameShow
    {
        public RenderFrameShow()
        {
            RenderedNearFar = new NearFar(0.1f, 100.0f);
            DepthScale = 1.0f;
        }

        /// <summary>The near and far the frame was rendered with, in app units (app content space).</summary>
        public NearFar RenderedNearFar { get; set; }
        /// <summary>Factor from the depth buffer's units to the view poses' units. The sender folds in the app scale itself; do not include it here.</summary>
        public float DepthScale { get; set; }
        internal Structure? Extension { get; private set; }

        public RenderFrameShow WithExtension(Structure extension)
        {
            Extension = StructureChainBuilder.Append(Extension, extension);
            return this;
        }

        public RenderFrameShow WithViewOverrides(params RenderFrameViewOverride[] viewOverrides)
        {
            return WithExtension(new RenderFrameShowViewOverrides
            {
                ViewOverrides = viewOverrides ?? Array.Empty<RenderFrameViewOverride>()
            });
        }
    }

    public readonly struct RenderFrameView
    {
        public RenderFrameView(VideoFrameViewType dataType, VideoFrameViewEye eye, uint textureIndex)
        {
            DataType = dataType;
            Eye = eye;
            TextureIndex = textureIndex;
        }

        public VideoFrameViewType DataType { get; }
        public VideoFrameViewEye Eye { get; }
        public uint TextureIndex { get; }

        internal RenderFrameViewNative ToNative()
        {
            return new RenderFrameViewNative { DataType = DataType, Eye = Eye, TextureIndex = TextureIndex };
        }

        internal static RenderFrameView FromNative(RenderFrameViewNative value)
        {
            return new RenderFrameView(value.DataType, value.Eye, value.TextureIndex);
        }
    }

    public readonly struct ImageSize
    {
        public ImageSize(uint width, uint height)
        {
            Width = width;
            Height = height;
        }

        public uint Width { get; }
        public uint Height { get; }
    }

    public readonly struct TextureSize
    {
        public TextureSize(PixelFormat format, uint width, uint height, uint arrayLayers)
        {
            Format = format;
            Width = width;
            Height = height;
            ArrayLayers = arrayLayers;
        }

        public PixelFormat Format { get; }
        public uint Width { get; }
        public uint Height { get; }
        public uint ArrayLayers { get; }

        internal TextureSizeNative ToNative()
        {
            return new TextureSizeNative { Format = Format, Width = Width, Height = Height, ArrayLayers = ArrayLayers };
        }

        internal static TextureSize FromNative(TextureSizeNative value)
        {
            return new TextureSize(value.Format, value.Width, value.Height, value.ArrayLayers);
        }
    }

    public readonly struct VideoFrameView
    {
        public VideoFrameView(
            uint startX,
            uint startY,
            uint endX,
            uint endY,
            uint textureIndex,
            uint arrayLayerIndex,
            PixelFormat textureFormat,
            VideoFrameViewType dataType,
            VideoFrameViewEye eye)
        {
            StartX = startX;
            StartY = startY;
            EndX = endX;
            EndY = endY;
            TextureIndex = textureIndex;
            ArrayLayerIndex = arrayLayerIndex;
            TextureFormat = textureFormat;
            DataType = dataType;
            Eye = eye;
        }

        public uint StartX { get; }
        public uint StartY { get; }
        public uint EndX { get; }
        public uint EndY { get; }
        public uint TextureIndex { get; }
        public uint ArrayLayerIndex { get; }
        public PixelFormat TextureFormat { get; }
        public VideoFrameViewType DataType { get; }
        public VideoFrameViewEye Eye { get; }

        internal VideoFrameViewNative ToNative()
        {
            return new VideoFrameViewNative
            {
                StartX = StartX,
                StartY = StartY,
                EndX = EndX,
                EndY = EndY,
                TextureIndex = TextureIndex,
                ArrayLayerIndex = ArrayLayerIndex,
                TextureFormat = TextureFormat,
                DataType = DataType,
                Eye = Eye
            };
        }

        internal static VideoFrameView FromNative(VideoFrameViewNative value)
        {
            return new VideoFrameView(value.StartX, value.StartY, value.EndX, value.EndY, value.TextureIndex, value.ArrayLayerIndex, value.TextureFormat, value.DataType, value.Eye);
        }
    }

    public sealed class VideoFrameLayout
    {
        public VideoFrameLayout()
        {
            Views = Array.Empty<VideoFrameView>();
            Textures = Array.Empty<TextureSize>();
        }

        public VideoFrameView[] Views { get; set; }
        public TextureSize[] Textures { get; set; }

        internal VideoFrameLayoutNative ToNative()
        {
            var native = NativeDefaults.VideoFrameLayout();
            NativeCopy.CopyToFixed(Views ?? Array.Empty<VideoFrameView>(), native.Views, NativeConstants.MaxFrameViews, v => v.ToNative());
            native.ViewsCount = (UIntPtr)Math.Min((Views ?? Array.Empty<VideoFrameView>()).Length, NativeConstants.MaxFrameViews);
            NativeCopy.CopyToFixed(Textures ?? Array.Empty<TextureSize>(), native.Textures, NativeConstants.MaxFrameTextures, t => t.ToNative());
            native.TexturesCount = (UIntPtr)Math.Min((Textures ?? Array.Empty<TextureSize>()).Length, NativeConstants.MaxFrameTextures);
            return native;
        }

        internal static VideoFrameLayout FromNative(VideoFrameLayoutNative native)
        {
            return new VideoFrameLayout
            {
                Views = NativeCopy.CopyFromFixed(native.Views, native.ViewsCount, NativeConstants.MaxFrameViews, VideoFrameView.FromNative),
                Textures = NativeCopy.CopyFromFixed(native.Textures, native.TexturesCount, NativeConstants.MaxFrameTextures, TextureSize.FromNative)
            };
        }
    }

    public readonly struct GuiPanelSize
    {
        public GuiPanelSize(float widthMeters, float heightMeters, float contentScale)
        {
            WidthMeters = widthMeters;
            HeightMeters = heightMeters;
            ContentScale = contentScale;
        }

        public float WidthMeters { get; }
        public float HeightMeters { get; }
        public float ContentScale { get; }
        public static GuiPanelSize Default { get { return new GuiPanelSize(1.0f, 1.0f, 1.0f); } }
        internal GuiPanelSizeNative ToNative() { return new GuiPanelSizeNative { WidthMeters = WidthMeters, HeightMeters = HeightMeters, ContentScale = ContentScale }; }
        internal static GuiPanelSize FromNative(GuiPanelSizeNative value) { return new GuiPanelSize(value.WidthMeters, value.HeightMeters, value.ContentScale); }
    }

    public readonly struct GeoAnchorFrame
    {
        public GeoAnchorFrame(WorldReferenceSystem worldReferenceSystem, Vector3d originWorld, Vector3d axisXWorld, Vector3d axisYWorld, Vector3d axisZWorld, Handedness handedness)
        {
            WorldReferenceSystem = worldReferenceSystem;
            OriginWorld = originWorld;
            AxisXWorld = axisXWorld;
            AxisYWorld = axisYWorld;
            AxisZWorld = axisZWorld;
            Handedness = handedness;
        }

        public WorldReferenceSystem WorldReferenceSystem { get; }
        public Vector3d OriginWorld { get; }
        public Vector3d AxisXWorld { get; }
        public Vector3d AxisYWorld { get; }
        public Vector3d AxisZWorld { get; }
        public Handedness Handedness { get; }
        internal GeoAnchorFrameNative ToNative()
        {
            return new GeoAnchorFrameNative
            {
                WorldReferenceSystem = WorldReferenceSystem,
                OriginWorld = OriginWorld.ToNative(),
                AxisXWorld = AxisXWorld.ToNative(),
                AxisYWorld = AxisYWorld.ToNative(),
                AxisZWorld = AxisZWorld.ToNative(),
                Handedness = Handedness
            };
        }
        internal static GeoAnchorFrame FromNative(GeoAnchorFrameNative value)
        {
            return new GeoAnchorFrame(value.WorldReferenceSystem, Vector3d.FromNative(value.OriginWorld), Vector3d.FromNative(value.AxisXWorld), Vector3d.FromNative(value.AxisYWorld), Vector3d.FromNative(value.AxisZWorld), value.Handedness);
        }
    }

    public readonly struct AppWorldAnchor
    {
        public AppWorldAnchor(bool hasAnchor, GeoAnchorFrame anchor)
        {
            HasAnchor = hasAnchor;
            Anchor = anchor;
        }

        public bool HasAnchor { get; }
        public GeoAnchorFrame Anchor { get; }
        internal static AppWorldAnchor FromNative(AppWorldAnchorNative value) { return new AppWorldAnchor(value.HasAnchor, GeoAnchorFrame.FromNative(value.Anchor)); }
    }

    public readonly struct AppVolumeSize
    {
        public AppVolumeSize(float widthMeters, float lengthMeters, float heightMeters)
        {
            WidthMeters = widthMeters;
            LengthMeters = lengthMeters;
            HeightMeters = heightMeters;
        }

        public float WidthMeters { get; }
        public float LengthMeters { get; }
        public float HeightMeters { get; }
        public static AppVolumeSize Default { get { return new AppVolumeSize(1.0f, 1.0f, 1.0f); } }
        internal AppVolumeSizeNative ToNative() { return new AppVolumeSizeNative { WidthMeters = WidthMeters, LengthMeters = LengthMeters, HeightMeters = HeightMeters }; }
        internal static AppVolumeSize FromNative(AppVolumeSizeNative value) { return new AppVolumeSize(value.WidthMeters, value.LengthMeters, value.HeightMeters); }
    }

    public readonly struct AppVolumeEditingStatus
    {
        public AppVolumeEditingStatus(bool isBeingEdited, PeerId editorPeer)
        {
            IsBeingEdited = isBeingEdited;
            EditorPeer = editorPeer;
        }

        public bool IsBeingEdited { get; }
        public PeerId EditorPeer { get; }
        internal static AppVolumeEditingStatus FromNative(AppVolumeEditingStatusNative value) { return new AppVolumeEditingStatus(value.IsBeingEdited, PeerId.FromNative(value.EditorPeer)); }
    }

    public readonly struct AppVolumeControllerRayStyle
    {
        public AppVolumeControllerRayStyle(AppVolumeControllerRayLineStyle lineStyle, byte colorR, byte colorG, byte colorB, byte colorA)
        {
            LineStyle = lineStyle;
            ColorR = colorR;
            ColorG = colorG;
            ColorB = colorB;
            ColorA = colorA;
        }

        public AppVolumeControllerRayLineStyle LineStyle { get; }
        public byte ColorR { get; }
        public byte ColorG { get; }
        public byte ColorB { get; }
        public byte ColorA { get; }
        public static AppVolumeControllerRayStyle Default { get { return new AppVolumeControllerRayStyle(AppVolumeControllerRayLineStyle.Solid, byte.MaxValue, byte.MaxValue, byte.MaxValue, byte.MaxValue); } }
        internal AppVolumeControllerRayStyleNative ToNative()
        {
            return new AppVolumeControllerRayStyleNative
            {
                LineStyle = LineStyle,
                ColorR = ColorR,
                ColorG = ColorG,
                ColorB = ColorB,
                ColorA = ColorA
            };
        }
        internal static AppVolumeControllerRayStyle FromNative(AppVolumeControllerRayStyleNative value)
        {
            return new AppVolumeControllerRayStyle(
                value.LineStyle,
                value.ColorR,
                value.ColorG,
                value.ColorB,
                value.ColorA);
        }
    }

    public sealed class AppVolumeGestureMappingRule
    {
        public AppVolumeGestureMappingRule()
        {
            GestureKind = GestureKind.Click;
            Enabled = true;
            Precision = 1.0f;
            AppScaleSensitivityMode = AppScaleSensitivityMode.Constant;
            TranslationAxes = AppVolumeAxisFlags.All;
            RotationAxes = AppVolumeAxisFlags.All;
            EnableGesturesFromOutsideOfVolume = true;
            ControllerRayStyle = AppVolumeControllerRayStyle.Default;
            AppTransformMapping = GestureAppTransformMapping.None;
        }

        public GestureKind GestureKind { get; set; }
        public bool Enabled { get; set; }
        public float Precision { get; set; }
        public AppScaleSensitivityMode AppScaleSensitivityMode { get; set; }
        public AppVolumeAxisFlags TranslationAxes { get; set; }
        public AppVolumeAxisFlags RotationAxes { get; set; }
        public bool EnableGesturesFromOutsideOfVolume { get; set; }
        public AppVolumeControllerRayStyle ControllerRayStyle { get; set; }
        public GestureAppTransformMapping AppTransformMapping { get; set; }

        internal AppVolumeGestureMappingRuleNative ToNative()
        {
            return new AppVolumeGestureMappingRuleNative
            {
                Header = NativeDefaults.Header(StructureType.AppVolumeGestureMappingRule),
                GestureKind = GestureKind,
                Enabled = Enabled,
                Precision = Precision,
                AppScaleSensitivityMode = AppScaleSensitivityMode,
                TranslationAxes = (uint)TranslationAxes,
                RotationAxes = (uint)RotationAxes,
                EnableGesturesFromOutsideOfVolume = EnableGesturesFromOutsideOfVolume,
                ControllerRayStyle = ControllerRayStyle.ToNative(),
                AppTransformMapping = AppTransformMapping
            };
        }

        internal static AppVolumeGestureMappingRule FromNative(AppVolumeGestureMappingRuleNative value)
        {
            return new AppVolumeGestureMappingRule
            {
                GestureKind = value.GestureKind,
                Enabled = value.Enabled,
                Precision = value.Precision,
                AppScaleSensitivityMode = value.AppScaleSensitivityMode,
                TranslationAxes = (AppVolumeAxisFlags)value.TranslationAxes,
                RotationAxes = (AppVolumeAxisFlags)value.RotationAxes,
                EnableGesturesFromOutsideOfVolume = value.EnableGesturesFromOutsideOfVolume,
                ControllerRayStyle = AppVolumeControllerRayStyle.FromNative(value.ControllerRayStyle),
                AppTransformMapping = value.AppTransformMapping
            };
        }
    }

    /// <summary>
    /// A gesture mode's mapping rules, in descending priority order, plus the
    /// peers whose gestures may move or scale the app.
    /// </summary>
    /// <remarks>
    /// The parameterless constructor mirrors qar_app_volume_gesture_configuration_default(),
    /// which is what a volume created without a configuration gets: two-hand
    /// distance scales the app, single-pointer 6DoF moves it (both with rotation
    /// off), plus hover and click rules.
    /// </remarks>
    public sealed class AppVolumeGestureConfiguration
    {
        public AppVolumeGestureConfiguration()
        {
            MappingRules = new[]
            {
                new AppVolumeGestureMappingRule
                {
                    GestureKind = GestureKind.DualPointerTranslateDistance,
                    Precision = 1.0f,
                    AppScaleSensitivityMode = AppScaleSensitivityMode.BasedOnAppScale,
                    RotationAxes = AppVolumeAxisFlags.None,
                    AppTransformMapping = GestureAppTransformMapping.AppScale,
                    ControllerRayStyle = new AppVolumeControllerRayStyle(AppVolumeControllerRayLineStyle.Solid, 0, 200, 0, byte.MaxValue)
                },
                new AppVolumeGestureMappingRule
                {
                    GestureKind = GestureKind.SinglePointer6Dof,
                    Precision = 1.0f,
                    RotationAxes = AppVolumeAxisFlags.None,
                    AppTransformMapping = GestureAppTransformMapping.AppPose,
                    ControllerRayStyle = new AppVolumeControllerRayStyle(AppVolumeControllerRayLineStyle.Solid, 0, 200, 0, byte.MaxValue)
                },
                new AppVolumeGestureMappingRule
                {
                    GestureKind = GestureKind.Hover,
                    ControllerRayStyle = new AppVolumeControllerRayStyle(AppVolumeControllerRayLineStyle.Dotted, byte.MaxValue, byte.MaxValue, byte.MaxValue, byte.MaxValue)
                },
                new AppVolumeGestureMappingRule
                {
                    GestureKind = GestureKind.Click,
                    ControllerRayStyle = new AppVolumeControllerRayStyle(AppVolumeControllerRayLineStyle.Solid, 144, 238, 144, byte.MaxValue)
                }
            };
            AllowedPeers = Array.Empty<PeerId>();
        }

        /// <summary>Rules in descending priority order; element 0 has the highest priority. At most 16 are used.</summary>
        public AppVolumeGestureMappingRule[] MappingRules { get; set; }

        /// <summary>
        /// The peers whose gestures may move or scale the app; empty lets every
        /// peer's. Sent as QarAppVolumeGestureAllowedPeersExt when non-empty. A
        /// configuration is replaced whole, so set it every time. Belongs to the
        /// volume, not to a mode: <see cref="AppVolumeControl.ChangeGestureMode"/>
        /// rejects a non-empty list with <see cref="StatusCode.ArgumentNotSupported"/>.
        /// </summary>
        public PeerId[] AllowedPeers { get; set; }

        internal AppVolumeGestureConfigurationNative ToNative(NativeMemoryScope scope)
        {
            var native = NativeDefaults.AppVolumeGestureConfiguration();
            var rules = MappingRules ?? Array.Empty<AppVolumeGestureMappingRule>();
            NativeCopy.CopyToFixed(rules, native.MappingRules, NativeConstants.MaxAppVolumeGestureMappingRules, r => r.ToNative());
            native.MappingRuleCount = (UIntPtr)Math.Min(rules.Length, NativeConstants.MaxAppVolumeGestureMappingRules);
            var allowedPeers = AllowedPeers ?? Array.Empty<PeerId>();
            if (allowedPeers.Length == 0)
                return native;
            native.Header.Next = scope.AllocStruct(new AppVolumeGestureAllowedPeersExtNative
            {
                Header = NativeDefaults.Header(StructureType.AppVolumeGestureAllowedPeersExt),
                AllowedPeers = scope.AllocArray(NativeIdArrays.ToNative(allowedPeers)),
                AllowedPeerCount = (UIntPtr)allowedPeers.Length
            });
            return native;
        }

        internal static AppVolumeGestureConfiguration FromNative(AppVolumeGestureConfigurationNative value)
        {
            return new AppVolumeGestureConfiguration
            {
                MappingRules = NativeCopy.CopyFromFixed(value.MappingRules, value.MappingRuleCount, NativeConstants.MaxAppVolumeGestureMappingRules, AppVolumeGestureMappingRule.FromNative)
            };
        }
    }

    /// <summary>One mapped gesture on an app volume, copied out of the native callback.</summary>
    public readonly struct AppVolumeGestureEvent
    {
        public AppVolumeGestureEvent(PeerId sourcePeerId, AppVolumeId targetAppVolumeId, GestureKind gestureKind, GesturePhase state, TimePoint timestamp, bool hasStartPoint, Vector3 startPoint, bool hasActionPoint, Vector3 actionPoint, Vector3 translationDelta, Quaternion rotationDelta, bool wasMappedToAppTransform, bool hasHand, Hand hand, ulong gestureId, uint modeId)
        {
            SourcePeerId = sourcePeerId;
            TargetAppVolumeId = targetAppVolumeId;
            GestureKind = gestureKind;
            State = state;
            Timestamp = timestamp;
            HasStartPoint = hasStartPoint;
            StartPoint = startPoint;
            HasActionPoint = hasActionPoint;
            ActionPoint = actionPoint;
            TranslationDelta = translationDelta;
            RotationDelta = rotationDelta;
            WasMappedToAppTransform = wasMappedToAppTransform;
            HasHand = hasHand;
            Hand = hand;
            GestureId = gestureId;
            ModeId = modeId;
        }

        /// <summary>The peer whose hand or controller produced the gesture, not the peer that recognised it.</summary>
        public PeerId SourcePeerId { get; }
        public AppVolumeId TargetAppVolumeId { get; }
        public GestureKind GestureKind { get; }
        /// <summary>Where in its lifecycle the gesture sits.</summary>
        public GesturePhase State { get; }
        /// <summary>When the recogniser emitted it, on its own clock; comparable only within one source peer.</summary>
        public TimePoint Timestamp { get; }
        public bool HasStartPoint { get; }
        /// <summary>Where the hand was when the gesture began, in app content space, in app units. Zero when <see cref="HasStartPoint"/> is false.</summary>
        public Vector3 StartPoint { get; }
        public bool HasActionPoint { get; }
        /// <summary>The intersection the gesture acts on (or the start point), in app content space, in app units. Zero when <see cref="HasActionPoint"/> is false.</summary>
        public Vector3 ActionPoint { get; }
        /// <summary>Accumulated translation since the gesture started, in the target volume's app volume space (not app content space), shaped by the mapping rule.</summary>
        public Vector3 TranslationDelta { get; }
        /// <summary>Accumulated rotation since the gesture started, in the target volume's app volume space.</summary>
        public Quaternion RotationDelta { get; }
        /// <summary>True when the library already folded the delta into the volume's app pose or app scale; applying it again moves the content twice.</summary>
        public bool WasMappedToAppTransform { get; }
        /// <summary>False for two-hand gestures, which have no single hand.</summary>
        public bool HasHand { get; }
        public Hand Hand { get; }
        /// <summary>Lifecycle id, unique within the source peer; 0 for an instant gesture.</summary>
        public ulong GestureId { get; }
        /// <summary>The gesture mode whose rule mapped the gesture, fixed for the gesture's whole lifecycle.</summary>
        public uint ModeId { get; }
        /// <summary>
        /// Reads every accessor off a native gesture handle into a managed copy.
        /// </summary>
        /// <remarks>
        /// The handle is library-owned and valid only while the callback that
        /// received it is running, so this must be called from inside that
        /// callback and the result is the only thing that may outlive it. A
        /// failing accessor leaves its group at its default rather than
        /// throwing, because a callback that throws across the native boundary
        /// would tear the process down.
        /// </remarks>
        internal static AppVolumeGestureEvent FromHandle(IntPtr gesture)
        {
            NativeMethods.qar_impl_app_volume_gesture_get_lifecycle(gesture, out var sourcePeerId, out var targetAppVolumeId);
            NativeMethods.qar_impl_app_volume_gesture_get_classification(gesture, out var gestureKind, out var state);
            NativeMethods.qar_impl_app_volume_gesture_get_points(gesture, out var actionPoint, out var hasActionPoint, out var startPoint, out var hasStartPoint);
            NativeMethods.qar_impl_app_volume_gesture_get_deltas(gesture, out var translationDelta, out var rotationDelta, out var wasMappedToAppTransform);
            NativeMethods.qar_impl_app_volume_gesture_get_timestamp(gesture, out var timestamp);
            NativeMethods.qar_impl_app_volume_gesture_get_hand(gesture, out var hand, out var hasHand);
            NativeMethods.qar_impl_app_volume_gesture_get_gesture_id(gesture, out var gestureId);
            NativeMethods.qar_impl_app_volume_gesture_get_mode(gesture, out var modeId);
            return new AppVolumeGestureEvent(
                PeerId.FromNative(sourcePeerId),
                AppVolumeId.FromNative(targetAppVolumeId),
                gestureKind,
                state,
                TimePoint.FromNative(timestamp),
                hasStartPoint,
                Vector3.FromNative(startPoint),
                hasActionPoint,
                Vector3.FromNative(actionPoint),
                Vector3.FromNative(translationDelta),
                Quaternion.FromNative(rotationDelta),
                wasMappedToAppTransform,
                hasHand,
                hand,
                gestureId,
                modeId);
        }
    }

    /// <summary>A position on a GUI panel in panel-content-space pixels: origin top-left as the viewer sees it, +X right, +Y down.</summary>
    public readonly struct GuiPanelPoint
    {
        public GuiPanelPoint(int xPixels, int yPixels)
        {
            XPixels = xPixels;
            YPixels = yPixels;
        }

        public int XPixels { get; }
        public int YPixels { get; }
        internal static GuiPanelPoint FromNative(GuiPanelPointNative value) { return new GuiPanelPoint(value.XPixels, value.YPixels); }
    }

    /// <summary>One recognised gesture on a GUI panel, copied out of the native callback.</summary>
    /// <remarks>
    /// Several producers publish on one panel, one per user. Key lifecycles on
    /// (<see cref="SourcePeerId"/>, <see cref="GestureId"/>): the id is unique only
    /// within a source peer.
    /// </remarks>
    public readonly struct GuiPanelGestureEvent
    {
        public GuiPanelGestureEvent(PeerId sourcePeerId, ulong gestureId, GuiPanelId panelId, GuiPanelGestureKind kind, GesturePhase phase, GuiPanelPoint position, GuiPanelPoint startPosition, TimePoint timestamp, Hand hand, byte tapCount)
        {
            SourcePeerId = sourcePeerId;
            GestureId = gestureId;
            PanelId = panelId;
            Kind = kind;
            Phase = phase;
            Position = position;
            StartPosition = startPosition;
            Timestamp = timestamp;
            Hand = hand;
            TapCount = tapCount;
        }

        /// <summary>The peer whose hand made the contact, not the peer that recognised it.</summary>
        public PeerId SourcePeerId { get; }
        /// <summary>Non-zero and stable for one hover or drag lifecycle; 0 for tap and long press.</summary>
        public ulong GestureId { get; }
        public GuiPanelId PanelId { get; }
        public GuiPanelGestureKind Kind { get; }
        public GesturePhase Phase { get; }
        /// <summary>Where the contact is now, in panel-content-space pixels.</summary>
        public GuiPanelPoint Position { get; }
        /// <summary>Where this lifecycle began, in panel-content-space pixels; equals <see cref="Position"/> for instant kinds.</summary>
        public GuiPanelPoint StartPosition { get; }
        /// <summary>When the recogniser emitted it, on its own clock; comparable only within one source peer.</summary>
        public TimePoint Timestamp { get; }
        public Hand Hand { get; }
        /// <summary>Taps in this run for <see cref="GuiPanelGestureKind.Tap"/>; 0 for every other kind.</summary>
        public byte TapCount { get; }

        /// <summary>Reads every accessor off a native gesture handle; call only inside the callback that received it.</summary>
        internal static GuiPanelGestureEvent FromHandle(IntPtr gesture)
        {
            var sourcePeerId = PeerIdNative.CreateDefault();
            var panelId = GuiPanelIdNative.CreateDefault();
            NativeMethods.qar_impl_gui_panel_gesture_get_lifecycle(gesture, ref sourcePeerId, out var gestureId);
            NativeMethods.qar_impl_gui_panel_gesture_get_classification(gesture, out var kind, out var phase);
            NativeMethods.qar_impl_gui_panel_gesture_get_positions(gesture, out var position, out var startPosition);
            NativeMethods.qar_impl_gui_panel_gesture_get_panel_id(gesture, ref panelId);
            NativeMethods.qar_impl_gui_panel_gesture_get_timestamp(gesture, out var timestamp);
            NativeMethods.qar_impl_gui_panel_gesture_get_hand(gesture, out var hand);
            NativeMethods.qar_impl_gui_panel_gesture_get_tap_count(gesture, out var tapCount);
            return new GuiPanelGestureEvent(
                PeerId.FromNative(sourcePeerId),
                gestureId,
                GuiPanelId.FromNative(panelId),
                kind,
                phase,
                GuiPanelPoint.FromNative(position),
                GuiPanelPoint.FromNative(startPosition),
                TimePoint.FromNative(timestamp),
                hand,
                tapCount);
        }
    }

    public readonly struct RenderFrameViewOverride
    {
        public RenderFrameViewOverride(UIntPtr viewIndex, bool overridePose, bool overrideFov, Pose pose, Fov fov)
        {
            ViewIndex = viewIndex;
            OverridePose = overridePose;
            OverrideFov = overrideFov;
            Pose = pose;
            Fov = fov;
        }

        public UIntPtr ViewIndex { get; }
        public bool OverridePose { get; }
        public bool OverrideFov { get; }
        /// <summary>
        /// The pose the view was really rendered with, in the bound app volume's
        /// app content space - the space <see cref="RenderFrameInfo.GetViewPose"/>
        /// hands it out in.
        /// </summary>
        public Pose Pose { get; }
        /// <summary>In view space, radians.</summary>
        public Fov Fov { get; }
        internal RenderFrameViewOverrideNative ToNative()
        {
            return new RenderFrameViewOverrideNative { ViewIndex = ViewIndex, OverridePose = OverridePose, OverrideFov = OverrideFov, Pose = Pose.ToNative(), Fov = Fov.ToNative() };
        }
    }

    public readonly struct VideoTextureCpu
    {
        public VideoTextureCpu(TextureSize size, uint pitch, IntPtr textureData, UIntPtr textureDataSize)
        {
            Size = size;
            Pitch = pitch;
            TextureData = textureData;
            TextureDataSize = textureDataSize;
        }

        public TextureSize Size { get; }
        public uint Pitch { get; }
        public IntPtr TextureData { get; }
        public UIntPtr TextureDataSize { get; }
#if NETSTANDARD2_1_OR_GREATER || NET8_0_OR_GREATER
        public unsafe ReadOnlySpan<byte> TextureDataSpan
        {
            get
            {
                if (TextureData == IntPtr.Zero || TextureDataSize == UIntPtr.Zero)
                    return ReadOnlySpan<byte>.Empty;
                return new ReadOnlySpan<byte>(TextureData.ToPointer(), NativeConvert.ToInt32(TextureDataSize));
            }
        }
#endif
        internal static VideoTextureCpu FromNative(VideoTextureCpuNative value) { return new VideoTextureCpu(TextureSize.FromNative(value.Size), value.Pitch, value.TextureData, value.TextureDataSize); }
    }

    public readonly struct VideoFrameCpu
    {
        public VideoFrameCpu(VideoFrameView[] textureViews, VideoTextureCpu[] textures)
        {
            TextureViews = textureViews;
            Textures = textures;
        }

        public VideoFrameView[] TextureViews { get; }
        public VideoTextureCpu[] Textures { get; }
        internal static VideoFrameCpu FromNative(VideoFrameCpuNative value)
        {
            return new VideoFrameCpu(
                NativeCopy.CopyFromFixed(value.TextureViews, value.TextureViewsCount, NativeConstants.MaxFrameViews, VideoFrameView.FromNative),
                NativeCopy.CopyFromFixed(value.Textures, value.TexturesCount, NativeConstants.MaxFrameTextures, VideoTextureCpu.FromNative));
        }
    }

    public readonly struct VideoTextureD3D11
    {
        public VideoTextureD3D11(TextureSize size, IntPtr texture)
        {
            Size = size;
            Texture = texture;
        }

        public TextureSize Size { get; }
        public IntPtr Texture { get; }
        internal static VideoTextureD3D11 FromNative(VideoTextureD3D11Native value) { return new VideoTextureD3D11(TextureSize.FromNative(value.Size), value.Texture); }
    }

    public readonly struct DXGIKeyedMutexSync
    {
        public DXGIKeyedMutexSync(IntPtr mutex, uint acquire, uint release)
        {
            Mutex = mutex;
            Acquire = acquire;
            Release = release;
        }

        public IntPtr Mutex { get; }
        public uint Acquire { get; }
        public uint Release { get; }
        internal static DXGIKeyedMutexSync FromNative(DXGIKeyedMutexSyncNative value) { return new DXGIKeyedMutexSync(value.Mutex, value.Acquire, value.Release); }
    }

    public readonly struct SyncFrameD3D11
    {
        public SyncFrameD3D11(DXGIKeyedMutexSync[] textureSync)
        {
            TextureSync = textureSync;
        }

        public DXGIKeyedMutexSync[] TextureSync { get; }
        internal static SyncFrameD3D11 FromNative(SyncFrameD3D11Native value)
        {
            return new SyncFrameD3D11(NativeCopy.CopyFromFixed(value.TextureSync, value.TextureSyncCount, NativeConstants.MaxFrameTextures, DXGIKeyedMutexSync.FromNative));
        }
    }

    public readonly struct VideoFrameD3D11
    {
        public VideoFrameD3D11(VideoFrameView[] textureViews, VideoTextureD3D11[] textures, SyncFrameD3D11 synchronization)
        {
            TextureViews = textureViews;
            Textures = textures;
            Synchronization = synchronization;
        }

        public VideoFrameView[] TextureViews { get; }
        public VideoTextureD3D11[] Textures { get; }
        public SyncFrameD3D11 Synchronization { get; }
        internal static VideoFrameD3D11 FromNative(VideoFrameD3D11Native value)
        {
            return new VideoFrameD3D11(
                NativeCopy.CopyFromFixed(value.TextureViews, value.TextureViewsCount, NativeConstants.MaxFrameViews, VideoFrameView.FromNative),
                NativeCopy.CopyFromFixed(value.Textures, value.TexturesCount, NativeConstants.MaxFrameTextures, VideoTextureD3D11.FromNative),
                SyncFrameD3D11.FromNative(value.Synchronization));
        }
    }

    public readonly struct VideoTextureOpenGl
    {
        public VideoTextureOpenGl(TextureSize size, uint texture)
        {
            Size = size;
            Texture = texture;
        }

        public TextureSize Size { get; }
        /// <summary>The GL texture name.</summary>
        public uint Texture { get; }
        internal static VideoTextureOpenGl FromNative(VideoTextureOpenGlNative value) { return new VideoTextureOpenGl(TextureSize.FromNative(value.Size), value.Texture); }
    }

    public readonly struct VideoFrameOpenGl
    {
        public VideoFrameOpenGl(VideoFrameView[] textureViews, VideoTextureOpenGl[] textures)
        {
            TextureViews = textureViews;
            Textures = textures;
        }

        public VideoFrameView[] TextureViews { get; }
        public VideoTextureOpenGl[] Textures { get; }
        internal static VideoFrameOpenGl FromNative(VideoFrameOpenGlNative value)
        {
            return new VideoFrameOpenGl(
                NativeCopy.CopyFromFixed(value.TextureViews, value.TextureViewsCount, NativeConstants.MaxFrameViews, VideoFrameView.FromNative),
                NativeCopy.CopyFromFixed(value.Textures, value.TexturesCount, NativeConstants.MaxFrameTextures, VideoTextureOpenGl.FromNative));
        }
    }

    /// <summary>One sender-owned Vulkan image and the state it is in.</summary>
    public readonly struct VideoTextureVulkan
    {
        public VideoTextureVulkan(TextureSize size, ulong image, ulong imageMemory, uint formatFeatures, int layout, uint queueFamilyIndex)
        {
            Size = size;
            Image = image;
            ImageMemory = imageMemory;
            FormatFeatures = formatFeatures;
            Layout = layout;
            QueueFamilyIndex = queueFamilyIndex;
        }

        public TextureSize Size { get; }
        /// <summary>VkImage handle.</summary>
        public ulong Image { get; }
        /// <summary>VkDeviceMemory handle.</summary>
        public ulong ImageMemory { get; }
        /// <summary>VkFormatFeatureFlags.</summary>
        public uint FormatFeatures { get; }
        /// <summary>VkImageLayout the image is in.</summary>
        public int Layout { get; }
        public uint QueueFamilyIndex { get; }
        internal static VideoTextureVulkan FromNative(VideoTextureVulkanNative value) { return new VideoTextureVulkan(TextureSize.FromNative(value.Size), value.Image, value.ImageMemory, value.FormatFeatures, value.Layout, value.QueueFamilyIndex); }
    }

    /// <summary>
    /// Access to a sender-owned Vulkan frame. The library owns the native frame;
    /// this object is valid until the next GetVulkanFrame call on the same sender
    /// or the sender's disposal. Before showing a frame, write the layouts and
    /// queue families the images were left in and the synchronization objects
    /// with <see cref="SetTextureState"/> and <see cref="SetSynchronization"/>.
    /// </summary>
    public sealed class VideoFrameVulkan
    {
        private readonly IntPtr _frame;

        private VideoFrameVulkan(IntPtr frame, VideoFrameView[] textureViews, VideoTextureVulkan[] textures, ulong semaphore, ulong fence)
        {
            _frame = frame;
            TextureViews = textureViews;
            Textures = textures;
            Semaphore = semaphore;
            Fence = fence;
        }

        /// <summary>Snapshot of the views when the frame was fetched.</summary>
        public VideoFrameView[] TextureViews { get; }
        /// <summary>Snapshot of the textures when the frame was fetched.</summary>
        public VideoTextureVulkan[] Textures { get; }
        /// <summary>VkSemaphore when the frame was fetched.</summary>
        public ulong Semaphore { get; }
        /// <summary>VkFence when the frame was fetched.</summary>
        public ulong Fence { get; }

        /// <summary>Records the VkImageLayout and queue family texture <paramref name="textureIndex"/> was left in.</summary>
        public void SetTextureState(int textureIndex, int layout, uint queueFamilyIndex)
        {
            if (textureIndex < 0 || textureIndex >= Textures.Length)
                throw new ArgumentOutOfRangeException(nameof(textureIndex));
            var textures = Marshal.OffsetOf(typeof(VideoFrameVulkanNative), nameof(VideoFrameVulkanNative.Textures)).ToInt32();
            var texture = textures + (textureIndex * Marshal.SizeOf(typeof(VideoTextureVulkanNative)));
            Marshal.WriteInt32(_frame, texture + Marshal.OffsetOf(typeof(VideoTextureVulkanNative), nameof(VideoTextureVulkanNative.Layout)).ToInt32(), layout);
            Marshal.WriteInt32(_frame, texture + Marshal.OffsetOf(typeof(VideoTextureVulkanNative), nameof(VideoTextureVulkanNative.QueueFamilyIndex)).ToInt32(), unchecked((int)queueFamilyIndex));
        }

        /// <summary>Records the VkSemaphore and VkFence the frame's rendering signals.</summary>
        public void SetSynchronization(ulong semaphore, ulong fence)
        {
            var sync = Marshal.OffsetOf(typeof(VideoFrameVulkanNative), nameof(VideoFrameVulkanNative.Synchronization)).ToInt32();
            Marshal.WriteInt64(_frame, sync + Marshal.OffsetOf(typeof(SyncFrameVulkanNative), nameof(SyncFrameVulkanNative.Semaphore)).ToInt32(), unchecked((long)semaphore));
            Marshal.WriteInt64(_frame, sync + Marshal.OffsetOf(typeof(SyncFrameVulkanNative), nameof(SyncFrameVulkanNative.Fence)).ToInt32(), unchecked((long)fence));
        }

        internal static VideoFrameVulkan FromNativePointer(IntPtr frame)
        {
            var native = Marshal.PtrToStructure<VideoFrameVulkanNative>(frame);
            return new VideoFrameVulkan(
                frame,
                NativeCopy.CopyFromFixed(native.TextureViews, native.TextureViewsCount, NativeConstants.MaxFrameViews, VideoFrameView.FromNative),
                NativeCopy.CopyFromFixed(native.Textures, native.TexturesCount, NativeConstants.MaxFrameTextures, VideoTextureVulkan.FromNative),
                native.Synchronization.Semaphore,
                native.Synchronization.Fence);
        }
    }

    /// <summary>One XR_EXT_hand_tracking joint.</summary>
    public readonly struct HandJointLocation
    {
        public HandJointLocation(uint jointId, ulong locationFlags, Pose pose, float radius)
        {
            JointId = jointId;
            LocationFlags = locationFlags;
            Pose = pose;
            Radius = radius;
        }
        /// <summary>The joint index; see <see cref="Joint"/>.</summary>
        public uint JointId { get; }
        public HandJoint Joint { get { return (HandJoint)JointId; } }
        /// <summary>Bits of <see cref="HandJointLocationFlags"/>.</summary>
        public ulong LocationFlags { get; }
        /// <summary>
        /// In the space of the <see cref="HandJoints"/> holding it: the bound app
        /// volume's app content space when it comes from
        /// <see cref="RenderSender.GetLastHands"/>.
        /// </summary>
        public Pose Pose { get; }
        /// <summary>
        /// Joint radius in the same space and units as <see cref="Pose"/>: app
        /// content units when it comes from <see cref="RenderSender.GetLastHands"/>.
        /// </summary>
        public float Radius { get; }
        internal static HandJointLocation FromNative(HandJointLocationNative value) { return new HandJointLocation(value.JointId, value.LocationFlags, Pose.FromNative(value.Pose), value.Radius); }
    }

    /// <summary>Velocity of one joint.</summary>
    public readonly struct HandJointVelocity
    {
        public HandJointVelocity(uint jointId, ulong flags, Vector3 linearVelocity, Vector3 angularVelocity)
        {
            JointId = jointId;
            Flags = flags;
            LinearVelocity = linearVelocity;
            AngularVelocity = angularVelocity;
        }
        public uint JointId { get; }
        public HandJoint Joint { get { return (HandJoint)JointId; } }
        public ulong Flags { get; }
        /// <summary>
        /// Units per second, in the axes and units of the joint poses: the bound
        /// app volume's app content space when it comes from
        /// <see cref="RenderSender.GetLastHands"/>.
        /// </summary>
        public Vector3 LinearVelocity { get; }
        /// <summary>
        /// Radians per second, about the axes of the joint poses: app content
        /// space axes when it comes from <see cref="RenderSender.GetLastHands"/>.
        /// </summary>
        public Vector3 AngularVelocity { get; }
        internal static HandJointVelocity FromNative(HandJointVelocityNative value) { return new HandJointVelocity(value.JointId, value.Flags, Vector3.FromNative(value.LinearVelocity), Vector3.FromNative(value.AngularVelocity)); }
    }

    /// <summary>One hand's tracking state: 26 joint locations and velocities indexed by <see cref="HandJoint"/>.</summary>
    public readonly struct HandJoints
    {
        public HandJoints(bool isTracked, bool isActive, Pose pose, HandJointLocation[] jointLocations, bool hasVelocity, HandJointVelocity[] jointVelocities)
        {
            IsTracked = isTracked;
            IsActive = isActive;
            Pose = pose;
            JointLocations = jointLocations;
            HasVelocity = hasVelocity;
            JointVelocities = jointVelocities;
        }
        /// <summary>False when the hand is not tracked; also false for both hands once no sample arrived for a second, or while the render sender's app volume does not resolve.</summary>
        public bool IsTracked { get; }
        public bool IsActive { get; }
        /// <summary>
        /// The hand's aim pose, the OpenXR /input/aim/pose: its -Z is the hand ray.
        /// The bound app volume's app content space when it comes from
        /// <see cref="RenderSender.GetLastHands"/>.
        /// </summary>
        public Pose Pose { get; }
        public HandJointLocation[] JointLocations { get; }
        public bool HasVelocity { get; }
        public HandJointVelocity[] JointVelocities { get; }
        internal static HandJoints FromNative(HandJointsNative value)
        {
            return new HandJoints(
                value.IsTracked,
                value.IsActive,
                Pose.FromNative(value.Pose),
                NativeCopy.CopyFromFixed(value.JointLocations, (UIntPtr)NativeConstants.HandJointCount, NativeConstants.HandJointCount, HandJointLocation.FromNative),
                value.HasVelocity,
                NativeCopy.CopyFromFixed(value.JointVelocities, (UIntPtr)NativeConstants.HandJointCount, NativeConstants.HandJointCount, HandJointVelocity.FromNative));
        }
    }

    /// <summary>
    /// The newest hands sample a device sent. From <see cref="RenderSender.GetLastHands"/>
    /// every pose, radius and velocity in it is in the bound app volume's app
    /// content space (right-handed, +Y up, -Z forward, app units), like the view
    /// poses.
    /// </summary>
    public readonly struct DeviceHandsWithJoints
    {
        public DeviceHandsWithJoints(PeerId deviceId, TimePoint deviceTimestamp, HandJoints leftHand, HandJoints rightHand)
        {
            DeviceId = deviceId;
            DeviceTimestamp = deviceTimestamp;
            LeftHand = leftHand;
            RightHand = rightHand;
        }
        public PeerId DeviceId { get; }
        public TimePoint DeviceTimestamp { get; }
        public HandJoints LeftHand { get; }
        public HandJoints RightHand { get; }
        internal static DeviceHandsWithJoints FromNative(DeviceHandsWithJointsNative value)
        {
            return new DeviceHandsWithJoints(PeerId.FromNative(value.DeviceId), TimePoint.FromNative(value.DeviceTimestamp), HandJoints.FromNative(value.LeftHand), HandJoints.FromNative(value.RightHand));
        }
    }
}
