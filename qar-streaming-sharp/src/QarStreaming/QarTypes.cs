using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Quaternar.Qaros.Interop;
using Quaternar.Qaros.Mathematics;

namespace Quaternar.Qaros
{
    /// <summary>
    /// The state a peer last claimed about itself. Deprecated: read <see cref="PeerSpec.Presence"/>.
    /// Every peer claims <see cref="Running"/> as soon as it joins, so the value says nothing beyond
    /// "joined", never "finished configuring".
    /// </summary>
    [Obsolete("Read PeerSpec.Presence (PeerPresence) instead. AppState.Running only means the peer joined.")]
    public enum AppState
    {
        Unknown = 0,
        Initializing = 1,
        Running = 10,
        ShuttingDown = 100,
        /// <summary>The peer left for good and never comes back under its id.</summary>
        Retired = 200
    }

    /// <summary>
    /// Whether a peer is here, as an application should act on it (C <c>QarPeerPresence</c>).
    /// Render for, wait for and count <see cref="Online"/> and <see cref="Unknown"/> peers
    /// (<see cref="PeerSpec.IsReachable"/>). The hub's own view of the session decides
    /// <see cref="Online"/> and <see cref="Offline"/>; a leave the peer announced decides
    /// <see cref="Leaving"/> and <see cref="Retired"/>.
    /// </summary>
    public enum PeerPresence
    {
        /// <summary>
        /// The hub has not reconciled the peer yet (an older hub that never does, or one that just
        /// started). Nothing says it is gone: treat it like <see cref="Online"/>.
        /// </summary>
        Unknown = 0,
        /// <summary>The hub sees the peer in the session.</summary>
        Online = 1,
        /// <summary>
        /// The hub no longer sees the peer: a crash, a kill, a network loss or a short drop. It reads
        /// <see cref="Online"/> again once it is back, without doing anything.
        /// </summary>
        Offline = 2,
        /// <summary>The peer announced it is leaving (a clean exit); it may come back under its id.</summary>
        Leaving = 3,
        /// <summary>The peer left for good and never comes back under its id.</summary>
        Retired = 4
    }

    /// <summary>
    /// Where a session stands in its hub's session lifecycle (C <c>QarSessionLifecycleState</c>);
    /// see <see cref="Session.SubscribeLifecycle(EventHandler{SessionLifecycleChangedEventArgs})"/>.
    /// Treat a value not named here as <see cref="HubUnreachable"/>.
    /// </summary>
    public enum SessionLifecycleState
    {
        /// <summary>In the hub's current session; traffic flows.</summary>
        Connected = 0,
        /// <summary>Moving into another session in place; traffic resumes once <see cref="Connected"/>.</summary>
        Migrating = 1,
        /// <summary>The hub has not answered for a while; the session keeps trying by itself.</summary>
        HubUnreachable = 2,
        /// <summary>
        /// The hub no longer admits this app (its approval was revoked, it missed too many
        /// rotations, or the hub was reset to a new install). Terminal: dispose the session,
        /// onboard again and recreate what the app owns.
        /// </summary>
        RePairRequired = 3
    }

    /// <summary>
    /// Why a session moved into another session id (C <c>QarSessionChangeCause</c>).
    /// Informational: the move is the same. Treat a value not named here as <see cref="Rotated"/>.
    /// </summary>
    public enum SessionChangeCause
    {
        /// <summary>No move: a state change within the same session.</summary>
        None = 0,
        /// <summary>
        /// The hub serves a new session of its own: its operator started a new session, a member hub
        /// left its primary, or this app missed the announcement of a join.
        /// </summary>
        Rotated = 1,
        /// <summary>
        /// The hub this app paired with joined another hub as its member; the session moved into
        /// that primary hub's session.
        /// </summary>
        JoinedPrimary = 2
    }

    /// <summary>
    /// What a peer is in the streaming graph (C <c>QarPeerRole</c>). An application using this
    /// API is always <see cref="Source"/>; it renders for peers whose role is <see cref="Target"/>.
    /// </summary>
    public enum PeerRole
    {
        /// <summary>QAROS pipeline infrastructure the hub launched itself.</summary>
        Internal = 0,
        /// <summary>Provides content: a source application or a camera capture process.</summary>
        Source = 1,
        /// <summary>
        /// Shows content: a headset's serving app, a Visualizer receiver, a camera's receiver slot.
        /// </summary>
        Target = 2,
        /// <summary>A QAROS runtime (hub) process.</summary>
        Runtime = 3,
        /// <summary>The operator's Visualizer control session.</summary>
        Control = 4
    }

    /// <summary>
    /// What kind of device a peer is (C <c>QarDeviceCategory</c>): the rendering fallback when the
    /// peer's <see cref="PeerSpec.DeviceId"/> and <see cref="PeerSpec.DeviceFamily"/> are unknown
    /// to the application. A new device model never adds a value; treat an unnamed value as
    /// <see cref="Unknown"/>.
    /// </summary>
    public enum DeviceCategory
    {
        /// <summary>Infrastructure, runtime, control, test targets and unrecognised devices.</summary>
        Unknown = 0,
        /// <summary>A desktop or laptop PC (every source application by default).</summary>
        DesktopPC = 1,
        /// <summary>A server machine.</summary>
        Server = 2,
        /// <summary>A headset or glasses of any model.</summary>
        Headset = 3,
        /// <summary>A Visualizer receiver showing streams on the PC.</summary>
        Visualizer = 4,
        /// <summary>A camera of any model: its capture process. (A build before
        /// <see cref="Observer"/> also used it for a camera's receiver.)</summary>
        Camera = 5,
        /// <summary>A target app that sees the scene through a camera: it blends a camera's
        /// RGB-D frames and pose with the mixer's stream. Its device id and name are the
        /// camera's.</summary>
        Observer = 6
    }

    /// <summary>
    /// The graphics backend a sender's frame textures live on. Every GPU value requires exactly
    /// the matching stream-parameter extension on the options; <see cref="Cpu"/> takes none.
    /// </summary>
    public enum GraphicsAPI
    {
        /// <summary>Library-owned CPU memory; portable and always available.</summary>
        Cpu = 0,
        /// <summary>Windows only. Requires <see cref="StreamParamsD3D11"/>.</summary>
        D3D11 = 4,
        /// <summary>Requires a library built with QAR_ENABLE_VULKAN and <see cref="StreamParamsVulkan"/>.</summary>
        Vulkan = 6,
        /// <summary>Requires a library built with QAR_ENABLE_OPENGL and exactly one <see cref="StreamParamsOpenGl"/> extension.</summary>
        OpenGL = 7
    }

    /// <summary>
    /// Left or right: which hand made a gesture, and the handedness of the axes of a
    /// <see cref="GeoAnchorFrame"/>.
    /// </summary>
    public enum Handedness
    {
        /// <summary>
        /// None given: a gesture no single hand made (two-hand gestures), or a default-initialized
        /// value. A <see cref="GeoAnchorFrame"/> with it is rejected with
        /// <see cref="StatusCode.ArgumentNotSupported"/>.
        /// </summary>
        Unspecified = 0,
        Left = 1,
        Right = 2
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
        /// <summary>Four-byte BGR colour with an unused X byte. X is not alpha.</summary>
        B8G8R8X8 = 56,
        R16G16B16A16 = 55,
        D32Float = 101
    }

    /// <summary>How the views of one video frame are packed into textures.</summary>
    public enum FrameLayout
    {
        /// <summary>All views of one data type share one texture, side by side (both eyes in one texture).</summary>
        SideBySide = 0,
        /// <summary>One texture per view.</summary>
        SeparatedTextures = 1,
        /// <summary>One texture array per data type, the views in its layers.</summary>
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
        /// <summary>
        /// Not bound to an eye. A render sender gives such a view the head pose and the left
        /// eye's field of view.
        /// </summary>
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
        /// <summary>Earth-centred, Earth-fixed WGS84, in metres.</summary>
        EcefWgs84Meters = 0
    }

    public enum AppVolumeLifetimeStatus
    {
        Active = 0,
        /// <summary>
        /// Retired with <see cref="AppVolumeControl.Close"/>. A render sender bound to it hands
        /// out no more frames; recreate the volume to stream into it again.
        /// </summary>
        Closed = 1
    }

    /// <summary>What a hand interaction with an app volume was recognised as.</summary>
    public enum GestureKind
    {
        /// <summary>Short tap or click, emitted as an <see cref="GesturePhase.Instant"/> event.</summary>
        Click = 0,
        /// <summary>Single-pointer hover over a targetable surface or volume.</summary>
        Hover = 1,
        /// <summary>
        /// Single-pointer 6DoF manipulation. The translation delta is the accumulated
        /// hand-position offset and the rotation delta the accumulated hand-orientation offset,
        /// both in the target volume's app volume space and shaped by its mapping rule.
        /// </summary>
        SinglePointer6Dof = 2,
        /// <summary>
        /// Two-hand distance change, recognised in the user's head-yaw frame. The translation
        /// delta is the accumulated change of the distance between the two hands as a vector
        /// along their separation: its length is the distance change and all components share
        /// one sign. Measured between the hands, not between the points their rays strike.
        /// </summary>
        DualPointerTranslateDistance = 3,
        /// <summary>
        /// Two-hand rotation of the inter-hand vector, recognised about the user's head-yaw axes
        /// and reported in the target volume's app volume space.
        /// </summary>
        DualPointerRotate = 4
    }

    public enum GesturePhase
    {
        Started = 0,
        Updated = 1,
        Ended = 2,
        /// <summary>A single event with no lifecycle: no started, updated or ended phase follows.</summary>
        Instant = 3,
        Canceled = 4
    }

    /// <summary>
    /// Bitmask of the axes a mapping rule lets a gesture translate along or rotate about; see
    /// <see cref="AppVolumeGestureMappingRule.TranslationAxes"/> and
    /// <see cref="AppVolumeGestureMappingRule.RotationAxes"/>.
    /// </summary>
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

    /// <summary>
    /// What the library applies a mapped gesture to by itself. Either way the gesture still
    /// reaches the volume's gesture subscribers; <see cref="AppVolumeGestureEvent.WasMappedToAppTransform"/>
    /// tells whether it was already applied.
    /// </summary>
    public enum GestureAppTransformMapping
    {
        /// <summary>Nothing: the gesture is left to the application.</summary>
        None = 0,
        /// <summary>The deltas move the volume's app pose (built-in move).</summary>
        AppPose = 1,
        /// <summary>The deltas change the volume's app scale (built-in zoom).</summary>
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

    /// <summary>
    /// Which onboarding path an onboarding invite carries. More may be added, so do not treat
    /// "not <see cref="Pake"/>" as <see cref="Password"/>.
    /// </summary>
    public enum OnboardingMethod
    {
        /// <summary>Code-based EC-JPAKE.</summary>
        Pake = 0,
        /// <summary>Shared-password channel.</summary>
        Password = 1
    }

    /// <summary>
    /// Native structure type tag of a <see cref="Structure"/>. Each extension class sets its own
    /// in its constructor.
    /// </summary>
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
        RuntimeOnboardLocalAppExt = 0x100A,
        // 0x100B and 0x100C (the retired member join request and complete inits) are never reused.
        RuntimeCreateMemberHubInvitationInit = 0x100D,
        RuntimeJoinHubAsMemberInit = 0x100E,
        RuntimeOnboardLocalAppTagExt = 0x100F,
        RuntimeEphemeralStorageExt = 0x1010,
        RuntimeMemberHubInvitationAppGrantExt = 0x1011,
        RuntimeOnboardRemoteAppGrantExt = 0x1012,
        SessionGraphicsDeviceId = 0x2004,
        // 0x2005 was the untagged request-invite init; the runtime refuses it ("app tag required").
        SessionRequestInviteInit = 0x200A,
        PeerPresentation = 0x2006,
        PeerDeviceIdentity = 0x2009,
        SessionLifecycleInfo = 0x200B,
        SessionInviteTargetAppInit = 0x2007,
        // 0x2008 (the removed request-onboarding-codes init) is never reused.
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

    /// <summary>Minimum severity the library logs; see <see cref="LibraryOptions.LogSeverity"/>.</summary>
    public enum LogSeverity
    {
        Trace = 0,
        Debug = 1,
        Info = 2,
        Warn = 3,
        Error = 4,
        Fatal = 5
    }

    /// <summary>
    /// What a GUI panel shows. Fixed at creation: <see cref="GuiPanelControl.NavigateToUri"/> on a
    /// stream panel fails with <see cref="StatusCode.GuiPanelChangeNotPermitted"/>.
    /// </summary>
    public enum GuiPanelContentType
    {
        WebsiteUri = 0,
        /// <summary>The video of a <see cref="PrimarylessVideoSender"/> owned by the same session.</summary>
        StreamId = 1
    }

    /// <summary>
    /// Field of view of one render view, in view space: angles in radians from the view's -Z
    /// axis, with OpenXR signs - left and down negative, right and up positive.
    /// </summary>
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

    /// <summary>
    /// Near and far clip distances along a view's -Z, in the units of the poses the view was
    /// rendered with: app units for a render sender. A near plane greater than the far plane
    /// declares reverse-Z.
    /// </summary>
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

    /// <summary>
    /// Wall-clock instant: nanoseconds since the Unix epoch (UTC). Instants stamped on
    /// different hosts are only as comparable as those hosts' clocks are synchronized.
    /// Converts to a UTC <see cref="DateTime"/> implicitly or with <see cref="ToDateTime"/>,
    /// truncated to its 100 ns tick.
    /// </summary>
    public readonly struct TimePoint : IEquatable<TimePoint>, IComparable<TimePoint>
    {
        private const long NanosecondsPerTick = 100;
        private static readonly DateTime UnixEpoch = new DateTime(1970, 1, 1, 0, 0, 0, DateTimeKind.Utc);

        public TimePoint(long unixNanoseconds)
        {
            UnixNanoseconds = unixNanoseconds;
        }

        /// <summary>Nanoseconds since 1970-01-01T00:00:00Z.</summary>
        public long UnixNanoseconds { get; }

        /// <summary>The instant as a UTC <see cref="DateTime"/>, truncated to its 100 ns tick.</summary>
        public DateTime ToDateTime()
        {
            return UnixEpoch.AddTicks(UnixNanoseconds / NanosecondsPerTick);
        }

        /// <summary>The instant as a <see cref="DateTimeOffset"/> at UTC, truncated to its 100 ns tick.</summary>
        public DateTimeOffset ToDateTimeOffset()
        {
            return new DateTimeOffset(ToDateTime());
        }

        /// <summary>From a <see cref="DateTime"/>; a local or unspecified kind is converted to UTC first.</summary>
        public static TimePoint FromDateTime(DateTime value)
        {
            return new TimePoint((value.ToUniversalTime() - UnixEpoch).Ticks * NanosecondsPerTick);
        }

        public static TimePoint FromDateTimeOffset(DateTimeOffset value)
        {
            return FromDateTime(value.UtcDateTime);
        }

        public static implicit operator DateTime(TimePoint value) { return value.ToDateTime(); }

        public static TimeSpan operator -(TimePoint left, TimePoint right)
        {
            return TimeSpan.FromTicks((left.UnixNanoseconds - right.UnixNanoseconds) / NanosecondsPerTick);
        }

        public static TimePoint operator +(TimePoint value, TimeSpan offset)
        {
            return new TimePoint(value.UnixNanoseconds + offset.Ticks * NanosecondsPerTick);
        }

        public static bool operator ==(TimePoint left, TimePoint right) { return left.Equals(right); }
        public static bool operator !=(TimePoint left, TimePoint right) { return !left.Equals(right); }
        public static bool operator <(TimePoint left, TimePoint right) { return left.UnixNanoseconds < right.UnixNanoseconds; }
        public static bool operator >(TimePoint left, TimePoint right) { return left.UnixNanoseconds > right.UnixNanoseconds; }
        public static bool operator <=(TimePoint left, TimePoint right) { return left.UnixNanoseconds <= right.UnixNanoseconds; }
        public static bool operator >=(TimePoint left, TimePoint right) { return left.UnixNanoseconds >= right.UnixNanoseconds; }

        public bool Equals(TimePoint other) { return UnixNanoseconds == other.UnixNanoseconds; }
        public override bool Equals(object? obj) { return obj is TimePoint other && Equals(other); }
        public override int GetHashCode() { return UnixNanoseconds.GetHashCode(); }
        public int CompareTo(TimePoint other) { return UnixNanoseconds.CompareTo(other.UnixNanoseconds); }
        public override string ToString() { return ToDateTime().ToString("O"); }

        internal static TimePoint FromNative(TimePointNative value)
        {
            return new TimePoint(value.UnixNs);
        }
    }

    /// <summary>
    /// A peer's 16-byte UUID. Compared by content; <c>default(PeerId)</c> equals
    /// <see cref="Default"/>.
    /// </summary>
    public readonly struct PeerId : IEquatable<PeerId>
    {
        private readonly byte[]? _data;

        private PeerId(byte[] data)
        {
            _data = IdBytes.Normalize(data);
        }

        /// <summary>The all-zero id, meaning "no peer".</summary>
        public static PeerId Default { get { return new PeerId(new byte[NativeConstants.MaxIdLength]); } }
        /// <summary>
        /// A fresh id that identifies no peer in any session. Never use it as a render target
        /// (<see cref="RenderSenderOptions.PeerId"/>): a stream created with one never produces
        /// a frame.
        /// </summary>
        public static PeerId Unique() { return FromNative(NativeMethods.qar_impl_peer_id_unique()); }
        /// <summary>Parses the UUID text <see cref="ToString"/> produces.</summary>
        /// <exception cref="FormatException"><paramref name="text"/> is not a UUID string.</exception>
        public static PeerId Parse(string text) { PeerId id; if (!TryParse(text, out id)) throw new FormatException("QAR IDs must use the native UUID string format."); return id; }
        /// <summary>Parses the UUID text <see cref="ToString"/> produces; false, with a default id, when it is not one.</summary>
        public static bool TryParse(string text, out PeerId id) { byte[] data; if (!IdBytes.TryParse(text, out data)) { id = default(PeerId); return false; } id = new PeerId(data); return true; }
        /// <summary>A copy of the 16 bytes.</summary>
        public byte[] ToByteArray() { return IdBytes.Copy(_data); }
#if NETSTANDARD2_1_OR_GREATER || NET8_0_OR_GREATER
        public ReadOnlySpan<byte> AsSpan() { return IdBytes.AsSpan(_data); }
        /// <summary>Copies the 16 bytes; false, copying nothing, when <paramref name="destination"/> is shorter.</summary>
        public bool TryCopyTo(Span<byte> destination) { return IdBytes.TryCopyTo(_data, destination); }
#endif
        /// <summary>The 36-character UUID text, which <see cref="Parse"/> accepts back.</summary>
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

    /// <summary>
    /// A session's 16-byte UUID. Compared by content; <c>default(SessionId)</c> equals
    /// <see cref="Default"/>.
    /// </summary>
    public readonly struct SessionId : IEquatable<SessionId>
    {
        private readonly byte[]? _data;
        private SessionId(byte[] data) { _data = IdBytes.Normalize(data); }
        /// <summary>The all-zero id, meaning "none".</summary>
        public static SessionId Default { get { return new SessionId(new byte[NativeConstants.MaxIdLength]); } }
        public static SessionId Unique() { return FromNative(NativeMethods.qar_impl_session_id_unique()); }
        /// <summary>Parses the UUID text <see cref="ToString"/> produces.</summary>
        /// <exception cref="FormatException"><paramref name="text"/> is not a UUID string.</exception>
        public static SessionId Parse(string text) { SessionId id; if (!TryParse(text, out id)) throw new FormatException("QAR IDs must use the native UUID string format."); return id; }
        /// <summary>Parses the UUID text <see cref="ToString"/> produces; false, with a default id, when it is not one.</summary>
        public static bool TryParse(string text, out SessionId id) { byte[] data; if (!IdBytes.TryParse(text, out data)) { id = default(SessionId); return false; } id = new SessionId(data); return true; }
        /// <summary>A copy of the 16 bytes.</summary>
        public byte[] ToByteArray() { return IdBytes.Copy(_data); }
#if NETSTANDARD2_1_OR_GREATER || NET8_0_OR_GREATER
        public ReadOnlySpan<byte> AsSpan() { return IdBytes.AsSpan(_data); }
        /// <summary>Copies the 16 bytes; false, copying nothing, when <paramref name="destination"/> is shorter.</summary>
        public bool TryCopyTo(Span<byte> destination) { return IdBytes.TryCopyTo(_data, destination); }
#endif
        /// <summary>The 36-character UUID text, which <see cref="Parse"/> accepts back.</summary>
        public override string ToString() { return IdBytes.ToAscii(_data); }
        public bool Equals(SessionId other) { return IdBytes.Equals(_data, other._data); }
        public override bool Equals(object? obj) { return obj is SessionId other && Equals(other); }
        public override int GetHashCode() { return IdBytes.GetHashCode(_data); }
        public static bool operator ==(SessionId left, SessionId right) { return left.Equals(right); }
        public static bool operator !=(SessionId left, SessionId right) { return !left.Equals(right); }
        internal SessionIdNative ToNative() { return new SessionIdNative { Data = ToByteArray() }; }
        internal static SessionId FromNative(SessionIdNative value) { return new SessionId(value.Data); }
    }

    /// <summary>
    /// A GUI panel's 16-byte UUID, derived from the panel's common name by
    /// <see cref="GuiPanelControl.GetOrCreate"/>. Compared by content; <c>default(GuiPanelId)</c>
    /// equals <see cref="Default"/>.
    /// </summary>
    public readonly struct GuiPanelId : IEquatable<GuiPanelId>
    {
        private readonly byte[]? _data;
        private GuiPanelId(byte[] data) { _data = IdBytes.Normalize(data); }
        /// <summary>The all-zero id, meaning "none".</summary>
        public static GuiPanelId Default { get { return new GuiPanelId(new byte[NativeConstants.MaxIdLength]); } }
        public static GuiPanelId Unique() { return FromNative(NativeMethods.qar_impl_gui_panel_id_unique()); }
        /// <summary>Parses the UUID text <see cref="ToString"/> produces.</summary>
        /// <exception cref="FormatException"><paramref name="text"/> is not a UUID string.</exception>
        public static GuiPanelId Parse(string text) { GuiPanelId id; if (!TryParse(text, out id)) throw new FormatException("QAR IDs must use the native UUID string format."); return id; }
        /// <summary>Parses the UUID text <see cref="ToString"/> produces; false, with a default id, when it is not one.</summary>
        public static bool TryParse(string text, out GuiPanelId id) { byte[] data; if (!IdBytes.TryParse(text, out data)) { id = default(GuiPanelId); return false; } id = new GuiPanelId(data); return true; }
        /// <summary>A copy of the 16 bytes.</summary>
        public byte[] ToByteArray() { return IdBytes.Copy(_data); }
#if NETSTANDARD2_1_OR_GREATER || NET8_0_OR_GREATER
        public ReadOnlySpan<byte> AsSpan() { return IdBytes.AsSpan(_data); }
        /// <summary>Copies the 16 bytes; false, copying nothing, when <paramref name="destination"/> is shorter.</summary>
        public bool TryCopyTo(Span<byte> destination) { return IdBytes.TryCopyTo(_data, destination); }
#endif
        /// <summary>The 36-character UUID text, which <see cref="Parse"/> accepts back.</summary>
        public override string ToString() { return IdBytes.ToAscii(_data); }
        public bool Equals(GuiPanelId other) { return IdBytes.Equals(_data, other._data); }
        public override bool Equals(object? obj) { return obj is GuiPanelId other && Equals(other); }
        public override int GetHashCode() { return IdBytes.GetHashCode(_data); }
        public static bool operator ==(GuiPanelId left, GuiPanelId right) { return left.Equals(right); }
        public static bool operator !=(GuiPanelId left, GuiPanelId right) { return !left.Equals(right); }
        internal GuiPanelIdNative ToNative() { return new GuiPanelIdNative { Data = ToByteArray() }; }
        internal static GuiPanelId FromNative(GuiPanelIdNative value) { return new GuiPanelId(value.Data); }
    }

    /// <summary>
    /// An app volume's 16-byte UUID, derived from the volume's common name by
    /// <see cref="AppVolumeControl.GetOrCreate"/>. Compared by content; <c>default(AppVolumeId)</c>
    /// equals <see cref="Default"/>.
    /// </summary>
    public readonly struct AppVolumeId : IEquatable<AppVolumeId>
    {
        private readonly byte[]? _data;
        private AppVolumeId(byte[] data) { _data = IdBytes.Normalize(data); }
        /// <summary>The all-zero id, meaning "none".</summary>
        public static AppVolumeId Default { get { return new AppVolumeId(new byte[NativeConstants.MaxIdLength]); } }
        /// <summary>Parses the UUID text <see cref="ToString"/> produces.</summary>
        /// <exception cref="FormatException"><paramref name="text"/> is not a UUID string.</exception>
        public static AppVolumeId Parse(string text) { AppVolumeId id; if (!TryParse(text, out id)) throw new FormatException("QAR IDs must use the native UUID string format."); return id; }
        /// <summary>Parses the UUID text <see cref="ToString"/> produces; false, with a default id, when it is not one.</summary>
        public static bool TryParse(string text, out AppVolumeId id) { byte[] data; if (!IdBytes.TryParse(text, out data)) { id = default(AppVolumeId); return false; } id = new AppVolumeId(data); return true; }
        /// <summary>A copy of the 16 bytes.</summary>
        public byte[] ToByteArray() { return IdBytes.Copy(_data); }
#if NETSTANDARD2_1_OR_GREATER || NET8_0_OR_GREATER
        public ReadOnlySpan<byte> AsSpan() { return IdBytes.AsSpan(_data); }
        /// <summary>Copies the 16 bytes; false, copying nothing, when <paramref name="destination"/> is shorter.</summary>
        public bool TryCopyTo(Span<byte> destination) { return IdBytes.TryCopyTo(_data, destination); }
#endif
        /// <summary>The 36-character UUID text, which <see cref="Parse"/> accepts back.</summary>
        public override string ToString() { return IdBytes.ToAscii(_data); }
        public bool Equals(AppVolumeId other) { return IdBytes.Equals(_data, other._data); }
        public override bool Equals(object? obj) { return obj is AppVolumeId other && Equals(other); }
        public override int GetHashCode() { return IdBytes.GetHashCode(_data); }
        public static bool operator ==(AppVolumeId left, AppVolumeId right) { return left.Equals(right); }
        public static bool operator !=(AppVolumeId left, AppVolumeId right) { return !left.Equals(right); }
        internal AppVolumeIdNative ToNative() { return new AppVolumeIdNative { Data = ToByteArray() }; }
        internal static AppVolumeId FromNative(AppVolumeIdNative value) { return new AppVolumeId(value.Data); }
    }

    /// <summary>
    /// A logical video stream's 16-byte UUID, generated by the library for each
    /// <see cref="PrimarylessVideoSender"/> (<see cref="PrimarylessVideoSender.GetStreamId"/>).
    /// Compared by content; <c>default(StreamId)</c> equals <see cref="Default"/>.
    /// </summary>
    public readonly struct StreamId : IEquatable<StreamId>
    {
        private readonly byte[]? _data;
        private StreamId(byte[] data) { _data = IdBytes.Normalize(data); }
        /// <summary>The all-zero id, meaning "none".</summary>
        public static StreamId Default { get { return new StreamId(new byte[NativeConstants.MaxIdLength]); } }
        /// <summary>Parses the UUID text <see cref="ToString"/> produces.</summary>
        /// <exception cref="FormatException"><paramref name="text"/> is not a UUID string.</exception>
        public static StreamId Parse(string text) { StreamId id; if (!TryParse(text, out id)) throw new FormatException("QAR IDs must use the native UUID string format."); return id; }
        /// <summary>Parses the UUID text <see cref="ToString"/> produces; false, with a default id, when it is not one.</summary>
        public static bool TryParse(string text, out StreamId id) { byte[] data; if (!IdBytes.TryParse(text, out data)) { id = default(StreamId); return false; } id = new StreamId(data); return true; }
        /// <summary>A copy of the 16 bytes.</summary>
        public byte[] ToByteArray() { return IdBytes.Copy(_data); }
#if NETSTANDARD2_1_OR_GREATER || NET8_0_OR_GREATER
        public ReadOnlySpan<byte> AsSpan() { return IdBytes.AsSpan(_data); }
        /// <summary>Copies the 16 bytes; false, copying nothing, when <paramref name="destination"/> is shorter.</summary>
        public bool TryCopyTo(Span<byte> destination) { return IdBytes.TryCopyTo(_data, destination); }
#endif
        /// <summary>The 36-character UUID text, which <see cref="Parse"/> accepts back.</summary>
        public override string ToString() { return IdBytes.ToAscii(_data); }
        public bool Equals(StreamId other) { return IdBytes.Equals(_data, other._data); }
        public override bool Equals(object? obj) { return obj is StreamId other && Equals(other); }
        public override int GetHashCode() { return IdBytes.GetHashCode(_data); }
        public static bool operator ==(StreamId left, StreamId right) { return left.Equals(right); }
        public static bool operator !=(StreamId left, StreamId right) { return !left.Equals(right); }
        internal StreamIdNative ToNative() { return new StreamIdNative { Data = ToByteArray() }; }
        internal static StreamId FromNative(StreamIdNative value) { return new StreamId(value.Data); }
    }

    /// <summary>
    /// Base of the extension structures chained onto an options object with its
    /// <c>WithExtension</c> method. The options are copied into native memory for the duration
    /// of the call that takes them.
    /// </summary>
    /// <remarks>
    /// An instance belongs to one chain at a time: appending it to a second options object cuts
    /// off whatever followed it in the first, so create a new instance per options object.
    /// </remarks>
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
    /// Onboarding mode: join the QAROS hub installed on this PC as a local source
    /// app, with no code. Select with <see cref="OnboardOptions.UseLocalApp"/>.
    /// The hub asks the user once; later starts join silently while the app still
    /// matches. Stateless: the returned onboarding id is the zero id, so rejoin and
    /// forget do not apply. Not combinable with a host or peer id extension.
    /// Windows only.
    /// </summary>
    public sealed class OnboardLocalApp : Structure
    {
        private string? _tag;

        public OnboardLocalApp()
            : base(StructureType.RuntimeOnboardLocalAppExt)
        {
        }

        /// <summary>Max wait for the user's first-time approval; zero takes the hub default (120 s).</summary>
        public TimeSpan ApprovalTimeout { get; set; } = TimeSpan.Zero;

        /// <summary>
        /// The app tag telling several instances of one approved app apart (C
        /// <c>QarOnboardLocalAppTagExt</c>); null, the default, is the app's default tag.
        /// </summary>
        /// <remarks>
        /// <para>
        /// The hub remembers a peer id per approval and tag: the next start with the same tag gets
        /// the same peer id while no other live process holds it, and a one-off id otherwise. A
        /// second onboard with the same tag in this process shares the first one's session (same
        /// peer); different tags are different peers.
        /// </para>
        /// <para>
        /// An identity key like an app volume's common name: non-empty printable ASCII, no space at
        /// either end, at most 256 bytes, compared exactly, not starting with the reserved
        /// <c>member/</c> prefix, kept constant across versions of the
        /// app. Recommended: a DNS-style name scoped by the app, such as
        /// <c>left.terraexplorer.skyline</c>. Put non-ASCII text in
        /// <see cref="PeerPresentationOptions.DisplayName"/>.
        /// </para>
        /// <para>
        /// A runtime older than the tag ignores it and joins with the default tag.
        /// </para>
        /// </remarks>
        /// <exception cref="ArgumentException">The value breaks the rules above (checked as a courtesy; the runtime checks too).</exception>
        public string? Tag
        {
            get { return _tag; }
            set { _tag = value == null ? null : AppTagRules.Validate(value, nameof(value)); }
        }

        /// <summary>
        /// Optional remote-app grant to redeem while onboarding (C
        /// <c>QarOnboardRemoteAppGrantExt</c>): a <see cref="MemberHubInvitation"/> the primary's
        /// app created with <see cref="MemberHubInvitationOptions.AppTag"/> set, as
        /// <see cref="MemberHubInvitation.Deserialize"/> returned it. Null, the default, redeems
        /// none.
        /// </summary>
        /// <remarks>
        /// <para>
        /// The hub on this PC must already be a member of the invitation's primary
        /// (<see cref="Runtime.JoinHubAsMember"/>). When no approval admits this executable there
        /// yet, the primary approves it as the app the grant admits, without a prompt, and later
        /// starts onboard with no grant and no prompt. The first executable that redeems the grant
        /// wins: once another one has, the onboarding fails with
        /// <see cref="StatusCode.OnboardingApprovalDenied"/>. When an approval already admits this
        /// executable, the invitation is not used.
        /// </para>
        /// <para>
        /// The grant's app tag applies; <see cref="Tag"/> is not needed. Only the local-app mode
        /// takes a grant.
        /// </para>
        /// <para>
        /// The invitation is a secret bearer credential: receive it only over a private channel and
        /// never log it. It is borrowed, not owned: the caller keeps ownership and disposes it, and
        /// the binding holds a reference on it for the duration of the onboard call, so disposing
        /// it meanwhile takes effect once the call is done.
        /// </para>
        /// </remarks>
        public MemberHubInvitation? RemoteAppGrant { get; set; }
    }

    /// <summary>
    /// Client-side check of an app tag against the C API rules (<c>qar::ValidateAppTag</c>), so a
    /// mistake fails at the line that made it. The runtime checks again and stays authoritative.
    /// </summary>
    internal static class AppTagRules
    {
        /// <summary>Returns <paramref name="tag"/> when it is valid; the message never echoes the tag.</summary>
        internal static string Validate(string tag, string paramName)
        {
            if (tag == null)
                throw new ArgumentNullException(paramName);
            if (tag.Length == 0)
                throw new ArgumentException("An app tag must not be empty.", paramName);
            if (tag.Length > NativeConstants.MaxAppTagLength)
                throw new ArgumentException("An app tag is at most " + NativeConstants.MaxAppTagLength + " bytes.", paramName);
            foreach (var c in tag)
            {
                if (c < ' ' || c > '~')
                    throw new ArgumentException("An app tag must be printable ASCII; put other text in the display name.", paramName);
            }
            if (tag[0] == ' ' || tag[tag.Length - 1] == ' ')
                throw new ArgumentException("An app tag must not start or end with a space.", paramName);
            if (tag.StartsWith(ReservedPrefix, StringComparison.Ordinal))
                throw new ArgumentException("App tags starting with \"" + ReservedPrefix + "\" are reserved for the primary hub's member apps.", paramName);
            return tag;
        }

        /// <summary>
        /// The prefix the primary hub keeps for the tags it remembers its member hubs' apps under
        /// (<c>qar::kReservedAppTagPrefix</c>); an app passing one would select a member PC's peer.
        /// </summary>
        internal const string ReservedPrefix = "member/";
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

        /// <summary>Selects the adapter by its LUID, as on Windows; see <see cref="Luid"/> for the packing.</summary>
        public static GraphicsDeviceId FromLuid(long luid)
        {
            return new GraphicsDeviceId { IdType = GpuDeviceIdType.Luid, Luid = luid };
        }

        /// <summary>Selects the adapter by its 16-byte UUID, as on non-Windows platforms. The array is copied.</summary>
        /// <exception cref="ArgumentNullException"><paramref name="uuid"/> is null.</exception>
        /// <exception cref="ArgumentException"><paramref name="uuid"/> is not 16 bytes long.</exception>
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
    /// <remarks>
    /// A pointer obtained with <c>Marshal.GetFunctionPointerForDelegate</c> is valid only while
    /// its delegate is reachable: keep the delegates alive for as long as the sender exists.
    /// </remarks>
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

        /// <summary>Passed unchanged to every callback; may be zero.</summary>
        public IntPtr UserState { get; }
        /// <summary>
        /// <c>void* (void* userState, const char* name)</c>: resolves a GL entry point, as
        /// eglGetProcAddress, wglGetProcAddress or glXGetProcAddressARB do. Zero uses the
        /// platform loader.
        /// </summary>
        public IntPtr GetProcAddress { get; }
        /// <summary>
        /// <c>bool (void* userState)</c>: binds the extension's context to the calling thread with
        /// the handles passed in the extension.
        /// </summary>
        public IntPtr MakeCurrent { get; }
        /// <summary>
        /// <c>QarOpenGlContextThreadState (void* userState)</c>: reads the thread's current binding
        /// before <see cref="MakeCurrent"/> replaces it.
        /// </summary>
        public IntPtr CaptureCurrent { get; }
        /// <summary>
        /// <c>bool (void* userState, const QarOpenGlContextThreadState* state)</c>: re-binds what
        /// <see cref="CaptureCurrent"/> read once the library is done. When the state's
        /// <c>has_context</c> is false the thread had nothing bound, so bind nothing.
        /// </summary>
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

        /// <summary>Optional overrides of how the library binds the context; the default uses the library's own.</summary>
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

    /// <summary>
    /// Extension: adopt the application's D3D11.1 device, so the sender's frame textures are
    /// shared D3D11 textures it renders into directly, with no CPU copy. Windows only.
    /// </summary>
    public sealed class StreamParamsD3D11 : Structure
    {
        public StreamParamsD3D11()
            : base(StructureType.StreamD3D11ParamsExt)
        {
            AcquireKeyedMutexSync = true;
        }

        /// <summary>ID3D11DeviceContext1*.</summary>
        public IntPtr D3D11Context { get; set; }
        /// <summary>ID3D11Device1*.</summary>
        public IntPtr D3D11Device { get; set; }
        /// <summary>D3D11_BIND_FLAG bits of the colour textures, for example RENDER_TARGET | SHADER_RESOURCE.</summary>
        public uint ColorBindFlags { get; set; }
        /// <summary>D3D11_BIND_FLAG bits of the depth textures, for example DEPTH_STENCIL.</summary>
        public uint DepthBindFlags { get; set; }
        /// <summary>
        /// True (the default) lets the library acquire and release the textures' keyed mutexes.
        /// Set false only to drive the acquire/release key chain yourself, for example when copying
        /// through a chain of shared textures; the keys are in <see cref="SyncFrameD3D11"/>.
        /// </summary>
        public bool AcquireKeyedMutexSync { get; set; }
    }

    /// <summary>
    /// Extension of <see cref="RenderFrameShow"/>: the pose or field of view a view was really
    /// rendered with, when it differs from what <see cref="RenderSender.BeginFrame"/> handed out.
    /// <see cref="RenderFrameShow.WithViewOverrides"/> chains one.
    /// </summary>
    public sealed class RenderFrameShowViewOverrides : Structure
    {
        public RenderFrameShowViewOverrides()
            : base(StructureType.RenderingEndFrameViewOverridesExt)
        {
            ViewOverrides = Array.Empty<RenderFrameViewOverride>();
        }

        /// <summary>At most 8 are sent; the rest are silently dropped.</summary>
        public RenderFrameViewOverride[] ViewOverrides { get; set; }
    }

    /// <summary>
    /// Arguments of <see cref="Library.Init"/>, which must run once per process before any other
    /// call. The defaults match <c>qar_library_init_default()</c>.
    /// </summary>
    public sealed class LibraryOptions
    {
        public LibraryOptions()
        {
            EnableConsoleLogging = true;
            LogSeverity = LogSeverity.Info;
        }

        /// <summary>Defaults to true.</summary>
        public bool EnableConsoleLogging { get; set; }
        /// <summary>
        /// Folder for <c>qar_streaming.log</c>; null (the default) writes no log file. The file
        /// starts empty on each init and rolls over once it is a day old or 256 MiB; the oldest
        /// rolled files are deleted so the log takes at most 3 GiB.
        /// </summary>
        public string? LogFolderPath { get; set; }
        /// <summary>Minimum severity logged. Defaults to <see cref="LogSeverity.Info"/>.</summary>
        public LogSeverity LogSeverity { get; set; }
        internal Structure? Extension { get; private set; }

        public LibraryOptions WithExtension(Structure extension)
        {
            Extension = StructureChainBuilder.Append(Extension, extension);
            return this;
        }
    }

    /// <summary>Arguments of <see cref="Runtime.Create"/>.</summary>
    public sealed class RuntimeOptions
    {
        /// <summary>
        /// Runtime <c>bin</c> folder holding <c>qar-runtime-launcher</c>. Leave null: the runtime
        /// uses the folder its loaded <c>qar-streaming-c.dll</c> came from, which is the installed
        /// QAROS <c>bin</c> (<see cref="InstalledRuntime.TryGetBinFolder"/>). Set it only for our
        /// own development builds. A relative path resolves against the current working directory.
        /// </summary>
        public string? RuntimeBinariesFolderPath { get; set; }
        /// <summary>
        /// Root of the device certificates and session state. Null or empty uses the platform's
        /// application data folder, on Windows <c>%LOCALAPPDATA%\Quaternar\QarOS</c>.
        /// </summary>
        public string? StorageFolderPath { get; set; }

        /// <summary>
        /// Keep the identity this runtime onboards with in memory only (C
        /// <c>QarRuntimeEphemeralStorageExt</c>), for a process that must leave nothing on disk:
        /// typically a co-process joining with an invite from
        /// <see cref="Session.RequestOnboardingInvite(string, IProgress{ActionProgress}?, System.Threading.CancellationToken)"/>.
        /// Defaults to false.
        /// </summary>
        /// <remarks>
        /// <para>
        /// <see cref="Runtime.Onboard"/> writes nothing under the storage folder and returns
        /// <see cref="OnboardingId.Default"/>; certificate renewal works as usual. The identity is
        /// gone with the runtime, so a restarted process onboards again with a fresh invite.
        /// <see cref="Runtime.Rejoin"/> and <see cref="Runtime.Forget"/> fail with
        /// <see cref="StatusCode.ArgumentNotSupported"/>: there is no persisted slot.
        /// <see cref="OnboardOptions.UseLocalApp"/> is stateless already and unaffected.
        /// </para>
        /// <para>
        /// A runtime older than this option ignores it and stores the identity on disk; check
        /// <see cref="RuntimeInfo.Version"/> before relying on it.
        /// </para>
        /// </remarks>
        public bool EphemeralStorage { get; set; }

        internal Structure? Extension { get; private set; }

        public RuntimeOptions WithExtension(Structure extension)
        {
            Extension = StructureChainBuilder.Append(Extension, extension);
            return this;
        }
    }

    /// <summary>
    /// The native <c>QarRuntimeEphemeralStorageExt</c> behind
    /// <see cref="RuntimeOptions.EphemeralStorage"/>; never chained by hand.
    /// </summary>
    internal sealed class RuntimeEphemeralStorage : Structure
    {
        internal RuntimeEphemeralStorage()
            : base(StructureType.RuntimeEphemeralStorageExt)
        {
        }
    }

    /// <summary>
    /// Arguments of a render sender: one stream rendered for one target peer, displayed in one
    /// app volume. The defaults match <c>qar_render_sender_init_default()</c>.
    /// </summary>
    /// <remarks>
    /// The stream-parameter property matching <see cref="GraphicsAPI"/> must be set and the other
    /// two left null; creating the sender throws <see cref="ArgumentException"/> otherwise.
    /// </remarks>
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
        /// <summary>
        /// True (the default) keeps the sender across network drops and target restarts: frame
        /// calls fail while it is disconnected, typically with
        /// <see cref="StatusCode.MediaStreamingStreamIsReconnecting"/>, and recover on their own.
        /// </summary>
        public bool EnableAutoReconnects { get; set; }
        /// <summary>Defaults to <see cref="FrameLayout.SideBySide"/>.</summary>
        public FrameLayout TextureLayout { get; set; }
        /// <summary>
        /// The views of one video frame; at most 8 are sent. <b>The order is guaranteed</b>: the
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
        /// <summary>Defaults to <see cref="PixelFormat.B8G8R8A8"/>.</summary>
        public PixelFormat ColorFormat { get; set; }
        /// <summary>Defaults to <see cref="PixelFormat.D32Float"/>.</summary>
        public PixelFormat DepthFormat { get; set; }
        /// <summary>Required when <see cref="GraphicsAPI"/> is <see cref="GraphicsAPI.D3D11"/>; must be null otherwise.</summary>
        public StreamParamsD3D11? StreamParamsD3D11 { get; set; }
        /// <summary>Required when <see cref="GraphicsAPI"/> is <see cref="GraphicsAPI.Vulkan"/>; must be null otherwise.</summary>
        public StreamParamsVulkan? StreamParamsVulkan { get; set; }
        /// <summary>Required when <see cref="GraphicsAPI"/> is <see cref="GraphicsAPI.OpenGL"/>; must be null otherwise.</summary>
        public StreamParamsOpenGl? StreamParamsOpenGl { get; set; }
    }

    /// <summary>
    /// Arguments of <see cref="PrimarylessVideoSender"/>: application-owned video, such as GUI
    /// panel content, under a library-generated <see cref="StreamId"/>. Unlike a render sender it
    /// answers no receiver request and may exist before any receiver connects.
    /// </summary>
    /// <remarks>
    /// The stream-parameter property matching <see cref="GraphicsAPI"/> must be set and the other
    /// two left null; creating the sender throws <see cref="ArgumentException"/> otherwise.
    /// </remarks>
    public sealed class PrimarylessVideoSenderOptions
    {
        public PrimarylessVideoSenderOptions(VideoFrameLayout layout)
        {
            Layout = layout ?? throw new ArgumentNullException(nameof(layout));
            EnableReconnects = true;
            ReconnectTimeout = TimeSpan.FromMinutes(5);
            GraphicsAPI = GraphicsAPI.Cpu;
        }

        /// <summary>Required. The views and textures the application writes; change it later with <see cref="PrimarylessVideoSender.ChangeLayout"/>.</summary>
        public VideoFrameLayout Layout { get; set; }
        /// <summary>Defaults to true.</summary>
        public bool EnableReconnects { get; set; }
        /// <summary>
        /// Defaults to 5 minutes. Must be positive and is sent in whole milliseconds; creating
        /// the sender throws <see cref="ArgumentOutOfRangeException"/> otherwise, even with
        /// <see cref="EnableReconnects"/> false.
        /// </summary>
        public TimeSpan ReconnectTimeout { get; set; }
        /// <summary>Defaults to <see cref="GraphicsAPI.Cpu"/>.</summary>
        public GraphicsAPI GraphicsAPI { get; set; }
        /// <summary>Required when <see cref="GraphicsAPI"/> is <see cref="GraphicsAPI.D3D11"/>; must be null otherwise.</summary>
        public StreamParamsD3D11? StreamParamsD3D11 { get; set; }
        /// <summary>Required when <see cref="GraphicsAPI"/> is <see cref="GraphicsAPI.Vulkan"/>; must be null otherwise.</summary>
        public StreamParamsVulkan? StreamParamsVulkan { get; set; }
        /// <summary>Required when <see cref="GraphicsAPI"/> is <see cref="GraphicsAPI.OpenGL"/>; must be null otherwise.</summary>
        public StreamParamsOpenGl? StreamParamsOpenGl { get; set; }
    }

    /// <summary>Arguments of <see cref="GuiPanelControl.GetOrCreate"/>.</summary>
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

        /// <summary>
        /// Required stable identity key the panel's id is derived from. Must be ASCII and
        /// non-empty; put any other text in <see cref="DisplayName"/>.
        /// </summary>
        public string CommonName { get; set; }
        /// <summary>
        /// Required non-empty label when creating or reopening this object; Unicode and emoji
        /// are allowed. The constructor leaves it null: set it before calling GetOrCreate.
        /// CommonName supplies identity only and is not a fallback display name.
        /// </summary>
        public string? DisplayName { get; set; }
        /// <summary>
        /// The panel's pose in room space: the midpoint of its top edge, front
        /// normal along -Z of the pose, the surface hanging below it.
        /// </summary>
        public Pose Pose { get; set; }
        /// <summary>Defaults to <see cref="GuiPanelSize.Default"/>.</summary>
        public GuiPanelSize Size { get; set; }
        /// <summary>
        /// Website to show. Set at most one of this and <see cref="ContentStreamId"/>, or
        /// <see cref="GuiPanelControl.GetOrCreate"/> throws <see cref="ArgumentException"/>; with
        /// neither, a website panel is created with the default URL. The content type is fixed at
        /// creation.
        /// </summary>
        public string? ContentUri { get; set; }
        /// <summary>
        /// Video to show: the <see cref="PrimarylessVideoSender.GetStreamId"/> of a video sender owned
        /// by the same session, whose layout sizes the panel's receiver. Excludes
        /// <see cref="ContentUri"/>.
        /// </summary>
        public StreamId? ContentStreamId { get; set; }
        /// <summary>
        /// Peers the panel is shown to. Empty, the default, means visible to every peer. A panel
        /// meant only for some peers is created with them here rather than narrowed afterwards.
        /// </summary>
        public PeerId[] VisibleToPeers { get; set; }
        internal Structure? Extension { get; private set; }

        public GuiPanelOptions WithExtension(Structure extension)
        {
            Extension = StructureChainBuilder.Append(Extension, extension);
            return this;
        }
    }

    /// <summary>
    /// Arguments of <see cref="AppVolumeControl.GetOrCreate"/>. The defaults match
    /// <c>qar_app_volume_init_default()</c>: identity poses, a 1 m cube, app scale 1, no world
    /// anchor and the default gesture configuration.
    /// </summary>
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

        /// <summary>
        /// Required stable identity key the volume's id is derived from. Must be ASCII and
        /// non-empty; put any other text in <see cref="DisplayName"/>.
        /// </summary>
        public string CommonName { get; set; }
        /// <summary>
        /// Required non-empty label when creating or reopening this object; Unicode and emoji
        /// are allowed. The constructor leaves it null: set it before calling GetOrCreate.
        /// CommonName supplies identity only and is not a fallback display name.
        /// </summary>
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
        /// <summary>Pins the volume to an Earth-fixed frame; null, the default, sets no anchor.</summary>
        public GeoAnchorFrame? AppWorldAnchor { get; set; }
        /// <summary>
        /// Rules of the default gesture mode and the allowed peers. Null, the default, gives the
        /// configuration a parameterless <see cref="AppVolumeGestureConfiguration"/> describes.
        /// </summary>
        public AppVolumeGestureConfiguration? GestureConfiguration { get; set; }
        internal Structure? Extension { get; private set; }

        public AppVolumeOptions WithExtension(Structure extension)
        {
            Extension = StructureChainBuilder.Append(Extension, extension);
            return this;
        }
    }

    /// <summary>
    /// Arguments of <see cref="RenderSender.ShowFrame"/>: the projection metadata the frame was
    /// rendered with. The defaults match <c>qar_render_frame_show_default()</c>: near 0.1, far
    /// 100, depth scale 1.
    /// </summary>
    /// <remarks>
    /// Report what was really used. The mixer re-projects the frame to the viewer's latest head
    /// pose from this metadata and the depth buffer; wrong values show up as swimming or
    /// misplaced content, not as an error.
    /// </remarks>
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

        /// <summary>Chains a <see cref="RenderFrameShowViewOverrides"/> carrying <paramref name="viewOverrides"/>.</summary>
        public RenderFrameShow WithViewOverrides(params RenderFrameViewOverride[] viewOverrides)
        {
            return WithExtension(new RenderFrameShowViewOverrides
            {
                ViewOverrides = viewOverrides ?? Array.Empty<RenderFrameViewOverride>()
            });
        }
    }

    /// <summary>One view declared in <see cref="RenderSenderOptions.FrameViews"/>.</summary>
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

    /// <summary>An image size in pixels.</summary>
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

    /// <summary>One texture of a frame layout; width and height in pixels.</summary>
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

    /// <summary>
    /// Where one view of a video frame lives: a pixel rectangle in one texture (and array layer).
    /// The end coordinates are exclusive and may be smaller than the start ones, which flips the
    /// view.
    /// </summary>
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
        /// <summary>Index into the frame's or layout's textures.</summary>
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

    /// <summary>The views of a video frame and the textures backing them.</summary>
    public sealed class VideoFrameLayout
    {
        public VideoFrameLayout()
        {
            Views = Array.Empty<VideoFrameView>();
            Textures = Array.Empty<TextureSize>();
        }

        /// <summary>At most 8 are sent; the rest are silently dropped.</summary>
        public VideoFrameView[] Views { get; set; }
        /// <summary>At most 4 are sent; the rest are silently dropped.</summary>
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

    /// <summary>
    /// A GUI panel's surface size in metres and the zoom of the content drawn on it. The
    /// surface spans panel-space X from -Width/2 to +Width/2 and hangs down from the pose to
    /// -Height.
    /// </summary>
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
        /// <summary>
        /// Zoom of the content raster on the surface: 1.0 fits the content across the full panel,
        /// 2.0 shows it at twice the size so only half fits, 0.5 at half size. It changes how
        /// contacts map to content pixels, not the panel's size.
        /// </summary>
        /// <remarks>
        /// A website panel takes it as the page zoom, like a desktop browser's Ctrl+ and Ctrl-:
        /// the page is drawn larger and reflowed to the panel width. Only the application sets
        /// it; the panel's own + and - buttons resize the panel instead. A stream panel ignores
        /// it: its video is always drawn at full panel width and scrolled vertically when taller
        /// than the panel.
        /// </remarks>
        public float ContentScale { get; }
        /// <summary>1 m by 1 m, content scale 1.</summary>
        public static GuiPanelSize Default { get { return new GuiPanelSize(1.0f, 1.0f, 1.0f); } }
        internal GuiPanelSizeNative ToNative() { return new GuiPanelSizeNative { WidthMeters = WidthMeters, HeightMeters = HeightMeters, ContentScale = ContentScale }; }
        internal static GuiPanelSize FromNative(GuiPanelSizeNative value) { return new GuiPanelSize(value.WidthMeters, value.HeightMeters, value.ContentScale); }
    }

    /// <summary>
    /// A local frame fully defined in world coordinates: its origin and unit axes in
    /// <see cref="WorldReferenceSystem"/> (ECEF WGS84, metres, double precision), plus the
    /// handedness of those axes. Pins an app volume to the Earth.
    /// </summary>
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
        /// <summary>Origin of the local frame, in ECEF metres.</summary>
        public Vector3d OriginWorld { get; }
        /// <summary>The local +X axis as a unit vector in world coordinates.</summary>
        public Vector3d AxisXWorld { get; }
        /// <summary>The local +Y axis as a unit vector in world coordinates.</summary>
        public Vector3d AxisYWorld { get; }
        /// <summary>The local +Z axis as a unit vector in world coordinates.</summary>
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

    /// <summary>An app volume's world anchor, if it has one.</summary>
    public readonly struct AppWorldAnchor
    {
        public AppWorldAnchor(bool hasAnchor, GeoAnchorFrame anchor)
        {
            HasAnchor = hasAnchor;
            Anchor = anchor;
        }

        public bool HasAnchor { get; }
        /// <summary>Valid only when <see cref="HasAnchor"/> is true.</summary>
        public GeoAnchorFrame Anchor { get; }
        internal static AppWorldAnchor FromNative(AppWorldAnchorNative value) { return new AppWorldAnchor(value.HasAnchor, GeoAnchorFrame.FromNative(value.Anchor)); }
    }

    /// <summary>
    /// An app volume cuboid's extents in metres, centred on the volume's pose: it spans plus and
    /// minus half of each extent along app volume space X, Y and Z. Note the constructor order:
    /// width (X), length (Z), height (Y).
    /// </summary>
    public readonly struct AppVolumeSize
    {
        public AppVolumeSize(float widthMeters, float lengthMeters, float heightMeters)
        {
            WidthMeters = widthMeters;
            LengthMeters = lengthMeters;
            HeightMeters = heightMeters;
        }

        /// <summary>Along app volume space X.</summary>
        public float WidthMeters { get; }
        /// <summary>Along app volume space Z, the horizontal depth.</summary>
        public float LengthMeters { get; }
        /// <summary>Along app volume space Y, the vertical rise.</summary>
        public float HeightMeters { get; }
        /// <summary>A 1 m cube.</summary>
        public static AppVolumeSize Default { get { return new AppVolumeSize(1.0f, 1.0f, 1.0f); } }
        internal AppVolumeSizeNative ToNative() { return new AppVolumeSizeNative { WidthMeters = WidthMeters, LengthMeters = LengthMeters, HeightMeters = HeightMeters }; }
        internal static AppVolumeSize FromNative(AppVolumeSizeNative value) { return new AppVolumeSize(value.WidthMeters, value.LengthMeters, value.HeightMeters); }
    }

    /// <summary>
    /// Whether a peer holds the volume's editing lock (<see cref="AppVolumeControl.StartEditing"/>)
    /// to move or resize the box itself. A peer manipulating the app with a gesture is not
    /// editing it.
    /// </summary>
    public readonly struct AppVolumeEditingStatus
    {
        public AppVolumeEditingStatus(bool isBeingEdited, PeerId editorPeer)
        {
            IsBeingEdited = isBeingEdited;
            EditorPeer = editorPeer;
        }

        public bool IsBeingEdited { get; }
        /// <summary>Meaningful only while <see cref="IsBeingEdited"/> is true.</summary>
        public PeerId EditorPeer { get; }
        internal static AppVolumeEditingStatus FromNative(AppVolumeEditingStatusNative value) { return new AppVolumeEditingStatus(value.IsBeingEdited, PeerId.FromNative(value.EditorPeer)); }
    }

    /// <summary>
    /// How the hand ray is drawn for gestures a mapping rule matches. The colour is RGBA, 0 to
    /// 255 per channel; <see cref="Default"/> is a solid opaque white line.
    /// </summary>
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

    /// <summary>
    /// How one gesture kind on an app volume is shaped and what it is applied to. Its priority is
    /// its position in <see cref="AppVolumeGestureConfiguration.MappingRules"/>, not a field.
    /// </summary>
    /// <remarks>
    /// The parameterless constructor mirrors <c>qar_app_volume_gesture_mapping_rule_default()</c>:
    /// an enabled <see cref="GestureKind.Click"/> rule, precision 1, constant sensitivity, all
    /// axes, gestures from outside the volume accepted, the default ray, no app transform.
    /// Hover and click reach the application even without a rule of their own.
    /// </remarks>
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
        /// <summary>
        /// A disabled hover or click rule does not stop delivery: those two always reach the
        /// application, through a built-in rule ranked after every configured one.
        /// </summary>
        public bool Enabled { get; set; }
        /// <summary>
        /// Hand centimetres per app-volume centimetre. With constant sensitivity 1.0 maps 1 cm of
        /// hand movement to 1 cm, 2.0 to 0.5 cm (finer control) and 0.5 to 2 cm.
        /// <see cref="AppScaleSensitivityMode"/> scales the result after precision is applied.
        /// </summary>
        public float Precision { get; set; }
        public AppScaleSensitivityMode AppScaleSensitivityMode { get; set; }
        public AppVolumeAxisFlags TranslationAxes { get; set; }
        public AppVolumeAxisFlags RotationAxes { get; set; }
        /// <summary>Whether a gesture made with the hand outside the volume's cuboid is accepted; tested at the hand's position.</summary>
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
        public AppVolumeGestureEvent(PeerId sourcePeerId, AppVolumeId targetAppVolumeId, GestureKind gestureKind, GesturePhase state, TimePoint timestamp, bool hasStartPoint, Vector3 startPoint, bool hasActionPoint, Vector3 actionPoint, Vector3 translationDelta, Quaternion rotationDelta, bool wasMappedToAppTransform, Handedness handedness, ulong gestureId, uint modeId)
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
            Handedness = handedness;
            GestureId = gestureId;
            ModeId = modeId;
        }

        /// <summary>The peer whose hand or controller produced the gesture, not the peer that recognised it.</summary>
        public PeerId SourcePeerId { get; }
        public AppVolumeId TargetAppVolumeId { get; }
        public GestureKind GestureKind { get; }
        /// <summary>Where in its lifecycle the gesture sits.</summary>
        public GesturePhase State { get; }
        /// <summary>When the recogniser emitted it, as UTC wall-clock time on its host; comparable only within one source peer.</summary>
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
        /// <summary>The hand that made the gesture; <see cref="Handedness.Unspecified"/> for two-hand gestures, which have no single hand.</summary>
        public Handedness Handedness { get; }
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
            NativeMethods.qar_impl_app_volume_gesture_get_handedness(gesture, out var handedness);
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
                handedness,
                gestureId,
                modeId);
        }
    }

    /// <summary>A position on a GUI panel in panel-content-space pixels: origin top-left as the viewer sees it, +X right, +Y down.</summary>
    /// <remarks>
    /// Clamped to the image the renderer is displaying: x in [0, width - 1], y in
    /// [0, height - 1]. For a stream panel that is the video at its sender's own resolution, and
    /// on a video the renderer has scrolled it is the pixel under the finger, not one relative
    /// to the visible part.
    /// </remarks>
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
        public GuiPanelGestureEvent(PeerId sourcePeerId, ulong gestureId, GuiPanelId panelId, GuiPanelGestureKind kind, GesturePhase phase, GuiPanelPoint position, GuiPanelPoint startPosition, TimePoint timestamp, Handedness handedness, byte tapCount)
        {
            SourcePeerId = sourcePeerId;
            GestureId = gestureId;
            PanelId = panelId;
            Kind = kind;
            Phase = phase;
            Position = position;
            StartPosition = startPosition;
            Timestamp = timestamp;
            Handedness = handedness;
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
        /// <summary>When the recogniser emitted it, as UTC wall-clock time on its host; comparable only within one source peer.</summary>
        public TimePoint Timestamp { get; }
        /// <summary>The hand that made the contact; <see cref="Handedness.Unspecified"/> when the recogniser did not tell.</summary>
        public Handedness Handedness { get; }
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
            NativeMethods.qar_impl_gui_panel_gesture_get_handedness(gesture, out var handedness);
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
                handedness,
                tapCount);
        }
    }

    /// <summary>
    /// The pose and/or field of view one view was really rendered with, when it deviates from
    /// what <see cref="RenderSender.BeginFrame"/> handed out. Only the parts whose flag is set are
    /// used.
    /// </summary>
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

        /// <summary>Index of the view in the sender's layout order.</summary>
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

    /// <summary>One sender-owned CPU texture. The pixel memory is not copied: <see cref="TextureData"/> points into the sender.</summary>
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
        /// <summary>Row stride in bytes.</summary>
        public uint Pitch { get; }
        /// <summary>
        /// The sender's pixel buffer, <see cref="TextureDataSize"/> bytes. Write into it only
        /// inside a frame cycle, and never after the sender is disposed.
        /// </summary>
        public IntPtr TextureData { get; }
        /// <summary>Size of <see cref="TextureData"/> in bytes.</summary>
        public UIntPtr TextureDataSize { get; }
#if NETSTANDARD2_1_OR_GREATER || NET8_0_OR_GREATER
        /// <summary>
        /// Read-only view of <see cref="TextureData"/>; empty when there is none. Write pixels
        /// through <see cref="TextureData"/>.
        /// </summary>
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

    /// <summary>
    /// A snapshot of a sender's CPU frame: its views and textures, in layout order. The pixel
    /// memory stays the sender's.
    /// </summary>
    /// <remarks>
    /// A render sender's frame exists from creation, so it can be read before the first
    /// <see cref="RenderSender.BeginFrame"/> to size resources; its contents may be written only
    /// between a successful begin-frame and <see cref="RenderSender.ShowFrame"/>. A
    /// <see cref="PrimarylessVideoSender"/> hands it out during the send cycle
    /// <see cref="PrimarylessVideoSender.TryBeginFrame"/> reserved.
    /// </remarks>
    public readonly struct VideoFrameCpu
    {
        public VideoFrameCpu(VideoFrameView[] textureViews, VideoTextureCpu[] textures)
        {
            TextureViews = textureViews;
            Textures = textures;
        }

        /// <summary>The views in layout order; each view's texture index names its texture.</summary>
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
        /// <summary>ID3D11Texture2D*, a shared texture created on the session's adapter.</summary>
        public IntPtr Texture { get; }
        internal static VideoTextureD3D11 FromNative(VideoTextureD3D11Native value) { return new VideoTextureD3D11(TextureSize.FromNative(value.Size), value.Texture); }
    }

    /// <summary>
    /// The keyed mutex of one shared D3D11 texture and the keys for its AcquireSync and
    /// ReleaseSync. Needed only when <see cref="StreamParamsD3D11.AcquireKeyedMutexSync"/> is
    /// false and the application drives the key chain itself.
    /// </summary>
    public readonly struct DXGIKeyedMutexSync
    {
        public DXGIKeyedMutexSync(IntPtr mutex, uint acquire, uint release)
        {
            Mutex = mutex;
            Acquire = acquire;
            Release = release;
        }

        /// <summary>IDXGIKeyedMutex*.</summary>
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

        /// <summary>One entry per frame texture; empty when no external synchronization is needed.</summary>
        public DXGIKeyedMutexSync[] TextureSync { get; }
        internal static SyncFrameD3D11 FromNative(SyncFrameD3D11Native value)
        {
            return new SyncFrameD3D11(NativeCopy.CopyFromFixed(value.TextureSync, value.TextureSyncCount, NativeConstants.MaxFrameTextures, DXGIKeyedMutexSync.FromNative));
        }
    }

    /// <summary>
    /// A snapshot of a sender's D3D11 frame: its views, its shared textures and their keyed-mutex
    /// synchronization. The textures stay the sender's; render into them only inside a frame
    /// cycle. A render sender's frame is readable from creation, before the first begin-frame.
    /// </summary>
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

    /// <summary>
    /// A snapshot of a sender's OpenGL frame: its views and textures. The textures stay the
    /// sender's; render into them only inside a frame cycle. A render sender's frame is readable
    /// from creation, before the first begin-frame.
    /// </summary>
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
        /// <summary>Queue family that owns the image.</summary>
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
        /// <summary>Bits of <see cref="HandJointLocationFlags"/>.</summary>
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
        /// <summary>26 entries, indexed by <see cref="HandJoint"/>.</summary>
        public HandJointLocation[] JointLocations { get; }
        /// <summary>Whether <see cref="JointVelocities"/> carries data.</summary>
        public bool HasVelocity { get; }
        /// <summary>26 entries, indexed by <see cref="HandJoint"/>; meaningful only when <see cref="HasVelocity"/> is true.</summary>
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
        /// <summary>The peer whose device sent the sample.</summary>
        public PeerId DeviceId { get; }
        /// <summary>When the device sampled the hands, as UTC wall-clock time on the device's host.</summary>
        public TimePoint DeviceTimestamp { get; }
        public HandJoints LeftHand { get; }
        public HandJoints RightHand { get; }
        internal static DeviceHandsWithJoints FromNative(DeviceHandsWithJointsNative value)
        {
            return new DeviceHandsWithJoints(PeerId.FromNative(value.DeviceId), TimePoint.FromNative(value.DeviceTimestamp), HandJoints.FromNative(value.LeftHand), HandJoints.FromNative(value.RightHand));
        }
    }
}
