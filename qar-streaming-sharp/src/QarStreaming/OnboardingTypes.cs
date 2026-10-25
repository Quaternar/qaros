using System;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    /// <summary>
    /// The key of an identity slot, minted by the first <see cref="Runtime.Onboard"/>. It is the one
    /// thing an application persists: hand it back to <see cref="Runtime.Rejoin"/> on every start
    /// and to <see cref="Runtime.Forget"/> to unpair.
    /// </summary>
    /// <remarks>
    /// Store it as text: <see cref="ToString"/> writes the UUID string <see cref="Parse"/> reads.
    /// Equality is by byte content.
    /// </remarks>
    public readonly struct OnboardingId : IEquatable<OnboardingId>
    {
        private readonly byte[]? _data;

        private OnboardingId(byte[] data)
        {
            _data = IdBytes.Normalize(data);
        }

        /// <summary>The all-zero id, meaning "no id". Rejoin and forget reject it.</summary>
        public static OnboardingId Default { get { return new OnboardingId(new byte[NativeConstants.MaxIdLength]); } }
        public static OnboardingId Parse(string text) { OnboardingId id; if (!TryParse(text, out id)) throw new FormatException("QAR IDs must use the native UUID string format."); return id; }
        public static bool TryParse(string text, out OnboardingId id) { byte[] data; if (!IdBytes.TryParse(text, out data)) { id = default(OnboardingId); return false; } id = new OnboardingId(data); return true; }
        public byte[] ToByteArray() { return IdBytes.Copy(_data); }
#if NETSTANDARD2_1_OR_GREATER || NET8_0_OR_GREATER
        public ReadOnlySpan<byte> AsSpan() { return IdBytes.AsSpan(_data); }
        public bool TryCopyTo(Span<byte> destination) { return IdBytes.TryCopyTo(_data, destination); }
#endif
        public override string ToString() { return IdBytes.ToAscii(_data); }
        public bool Equals(OnboardingId other) { return IdBytes.Equals(_data, other._data); }
        public override bool Equals(object? obj) { return obj is OnboardingId other && Equals(other); }
        public override int GetHashCode() { return IdBytes.GetHashCode(_data); }
        public static bool operator ==(OnboardingId left, OnboardingId right) { return left.Equals(right); }
        public static bool operator !=(OnboardingId left, OnboardingId right) { return !left.Equals(right); }
        internal OnboardingIdNative ToNative() { return new OnboardingIdNative { Data = ToByteArray() }; }
        internal static OnboardingId FromNative(OnboardingIdNative value) { return new OnboardingId(value.Data); }
        internal static OnboardingId FromNativePointer(IntPtr ptr) { return FromNative(System.Runtime.InteropServices.Marshal.PtrToStructure<OnboardingIdNative>(ptr)); }
    }

    /// <summary>
    /// How this peer presents itself to the others. Never a storage key. A property left null keeps
    /// its persisted value, or gets a generated default on the first onboard.
    /// </summary>
    public sealed class PeerPresentationOptions
    {
        /// <summary>
        /// Human label shown to other peers; any UTF-8. It also feeds a sanitized debug label in the
        /// device certificate and the storage slot folder name.
        /// </summary>
        public string? DisplayName { get; set; }
        public string? AppVersion { get; set; }
        /// <summary>
        /// Application-defined metadata, for example a JSON blob to coordinate instances of your
        /// own application across the session. Read back as <see cref="PeerSpec.AppCustomPeerInfo"/>.
        /// </summary>
        public string? AppCustomPeerInfo { get; set; }
        internal Structure? Extension { get; private set; }

        /// <summary>
        /// Describes the device this application runs as, for example a camera. Without it the
        /// application joins as a desktop PC with the host OS' fixed id.
        /// </summary>
        public PeerPresentationOptions WithDeviceIdentity(PeerDeviceIdentity identity)
        {
            return WithExtension(identity ?? throw new ArgumentNullException(nameof(identity)));
        }

        public PeerPresentationOptions WithExtension(Structure extension)
        {
            Extension = StructureChainBuilder.Append(Extension, extension);
            return this;
        }
    }

    /// <summary>
    /// The device an application describes itself as (C <c>QarPeerDeviceIdentity</c>). Pass it at
    /// onboard or rejoin with <see cref="PeerPresentationOptions.WithDeviceIdentity"/>, or later
    /// with <see cref="PeerControl.UpdateDeviceIdentity"/>.
    /// </summary>
    /// <remarks>
    /// An application is always a <see cref="PeerRole.Source"/>: it describes its device, never its
    /// role. <see cref="Category"/> must be <see cref="DeviceCategory.DesktopPC"/>,
    /// <see cref="DeviceCategory.Server"/> or <see cref="DeviceCategory.Camera"/>; any other is
    /// refused with <see cref="StatusCode.PeerDeviceCategoryNotAllowed"/>. Ids and families are open
    /// strings ("LuxonisOakD", "Luxonis"); one the hub's device catalog does not know is fine.
    /// </remarks>
    public sealed class PeerDeviceIdentity : Structure
    {
        public PeerDeviceIdentity(DeviceCategory category, string deviceId)
            : base(StructureType.PeerDeviceIdentity)
        {
            Category = category;
            DeviceId = deviceId ?? throw new ArgumentNullException(nameof(deviceId));
        }

        public DeviceCategory Category { get; set; }
        /// <summary>Required, non-empty; empty is refused with <see cref="StatusCode.ArgumentNotSupported"/>.</summary>
        public string DeviceId { get; set; }
        /// <summary>Optional; the rendering fallback for an id a viewer does not know.</summary>
        public string? DeviceFamily { get; set; }
        /// <summary>Optional human name of the device; any UTF-8.</summary>
        public string? DeviceName { get; set; }
    }

    /// <summary>Options of <see cref="Runtime.Rejoin"/>.</summary>
    public sealed class RejoinOptions
    {
        public RejoinOptions(OnboardingId onboardingId)
        {
            OnboardingId = onboardingId;
        }

        /// <summary>
        /// The id a previous onboard returned. <see cref="OnboardingId.Default"/> is rejected.
        /// </summary>
        public OnboardingId OnboardingId { get; set; }
        /// <summary>Presentation updates; null fields keep the persisted values.</summary>
        public PeerPresentationOptions? Presentation { get; set; }
        internal Structure? Extension { get; private set; }

        public RejoinOptions WithPresentation(PeerPresentationOptions presentation)
        {
            Presentation = presentation ?? throw new ArgumentNullException(nameof(presentation));
            return this;
        }

        /// <summary>Create the session's GPU devices on this adapter.</summary>
        public RejoinOptions WithGraphicsDevice(GraphicsDeviceId graphicsDevice)
        {
            return WithExtension(graphicsDevice ?? throw new ArgumentNullException(nameof(graphicsDevice)));
        }

        public RejoinOptions WithExtension(Structure extension)
        {
            Extension = StructureChainBuilder.Append(Extension, extension);
            return this;
        }
    }

    /// <summary>
    /// Options of <see cref="Runtime.Onboard"/>. Choose exactly one mode: <see cref="UseLocalApp"/>,
    /// <see cref="UseCode"/> or <see cref="UseInvite"/>; the last call wins.
    /// </summary>
    public sealed class OnboardOptions
    {
        /// <summary>
        /// Unset to create a new identity slot; an id an earlier onboard returned re-enrolls that
        /// slot.
        /// </summary>
        public OnboardingId? OnboardingId { get; set; }
        public PeerPresentationOptions? Presentation { get; set; }
        internal Structure Mode { get; private set; } = new OnboardCode(string.Empty);

        public OnboardOptions WithPresentation(PeerPresentationOptions presentation)
        {
            Presentation = presentation ?? throw new ArgumentNullException(nameof(presentation));
            return this;
        }

        /// <summary>
        /// Onboards with a pairing code: the short code on the hub's onboarding screen, or the text
        /// of any code the hub minted (<see cref="OnboardingInvite.GetCode"/>). Without
        /// <see cref="WithHost"/> it finds this user's hub on this PC by itself.
        /// </summary>
        /// <remarks>
        /// The code is a secret: do not log it or pass it on a command line. Replaces an earlier
        /// mode, including a <see cref="WithHost"/> made before this call.
        /// </remarks>
        public OnboardOptions UseCode(string code)
        {
            Mode = new OnboardCode(code ?? throw new ArgumentNullException(nameof(code)));
            return this;
        }

        /// <summary>
        /// Onboards from a full invite: a co-process invite the parent app obtained with
        /// <see cref="Session.RequestOnboardingInvite(string, IProgress{ActionProgress}?, System.Threading.CancellationToken)"/>
        /// (pair it with <see cref="RuntimeOptions.EphemeralStorage"/>), or the launch invite QAROS writes as the
        /// first stdin line of an API app it starts where local enrollment is not available (not
        /// Windows; <see cref="OnboardingInvite.Deserialize"/>).
        /// No discovery, no code.
        /// </summary>
        /// <remarks>
        /// The invite is borrowed: the caller keeps ownership and must not dispose it before the
        /// onboard call has completed. Replaces an earlier mode.
        /// </remarks>
        public OnboardOptions UseInvite(OnboardingInvite invite)
        {
            Mode = new OnboardInvite(invite ?? throw new ArgumentNullException(nameof(invite)));
            return this;
        }

        /// <summary>
        /// Join the QAROS hub on this PC as a local source app: no code, nothing
        /// persisted. The default for a source app running next to the hub. Replaces an earlier
        /// mode.
        /// </summary>
        public OnboardOptions UseLocalApp(OnboardLocalApp? localApp = null)
        {
            Mode = localApp ?? new OnboardLocalApp();
            return this;
        }

        /// <summary>
        /// Code onboarding against the hub at <paramref name="hostname"/>:<paramref name="port"/>.
        /// The port is required: hubs claim their onboarding port from a range, so there is no
        /// default. Without WithHost, code onboarding finds this user's hub on its own.
        /// </summary>
        /// <exception cref="InvalidOperationException">The mode is an invite (<see cref="UseInvite"/>).</exception>
        public OnboardOptions WithHost(string hostname, ushort port)
        {
            if (!(Mode is OnboardCode))
                throw new InvalidOperationException("OnboardOptions.WithHost can only be used with code onboarding. Call UseCode(...) first.");
            Mode = StructureChainBuilder.Append(Mode, new OnboardHost(hostname ?? throw new ArgumentNullException(nameof(hostname)), port))!;
            return this;
        }

        /// <summary>Optional extensions chained after the mode: <see cref="OnboardPeerId"/>, <see cref="GraphicsDeviceId"/>.</summary>
        internal Structure? Extension { get; private set; }

        /// <summary>
        /// Join the session under <paramref name="peerId"/> instead of a fresh id;
        /// it is persisted with the slot and kept by every later rejoin. For a
        /// process whose peer id was planned by someone else. Not with
        /// <see cref="UseLocalApp"/>: the hub picks a local app's peer id.
        /// </summary>
        public OnboardOptions WithPeerId(PeerId peerId)
        {
            return WithExtension(new OnboardPeerId(peerId));
        }

        /// <summary>Create the session's GPU devices on this adapter.</summary>
        public OnboardOptions WithGraphicsDevice(GraphicsDeviceId graphicsDevice)
        {
            return WithExtension(graphicsDevice ?? throw new ArgumentNullException(nameof(graphicsDevice)));
        }

        public OnboardOptions WithExtension(Structure extension)
        {
            Extension = StructureChainBuilder.Append(Extension, extension);
            return this;
        }
    }

    /// <summary>
    /// Onboarding mode extension behind <see cref="OnboardOptions.UseCode"/>; prefer that method.
    /// </summary>
    public sealed class OnboardCode : Structure
    {
        public OnboardCode(string code)
            : base(StructureType.RuntimeOnboardCodeExt)
        {
            Code = code ?? throw new ArgumentNullException(nameof(code));
        }

        public string Code { get; set; }
    }

    /// <summary>
    /// The hub to code-onboard against, behind <see cref="OnboardOptions.WithHost"/>; prefer that
    /// method.
    /// </summary>
    public sealed class OnboardHost : Structure
    {
        public OnboardHost(string hostname, ushort port)
            : base(StructureType.RuntimeOnboardHostExt)
        {
            Hostname = hostname ?? throw new ArgumentNullException(nameof(hostname));
            Port = port;
        }

        /// <summary>A plain host name or IP address, not a URL.</summary>
        public string Hostname { get; set; }
        /// <summary>The hub's onboarding port, shown with its code. Required: 0 is rejected.</summary>
        public ushort Port { get; set; }
    }

    /// <summary>
    /// Onboarding mode extension behind <see cref="OnboardOptions.UseInvite"/>; prefer that method.
    /// The invite is borrowed, not owned.
    /// </summary>
    public sealed class OnboardInvite : Structure
    {
        public OnboardInvite(OnboardingInvite invite)
            : base(StructureType.RuntimeOnboardInviteExt)
        {
            Invite = invite ?? throw new ArgumentNullException(nameof(invite));
        }

        public OnboardingInvite Invite { get; }
    }

    /// <summary>
    /// Arguments of <see cref="Session.RequestOnboardingInvite(RequestOnboardingInviteOptions, IProgress{ActionProgress}?, System.Threading.CancellationToken)"/>
    /// (C <c>QarRequestInviteInit</c>): a co-process invite for one app tag.
    /// </summary>
    public sealed class RequestOnboardingInviteOptions
    {
        private string _appTag;

        /// <summary>Creates the options for the co-process tagged <paramref name="appTag"/>.</summary>
        /// <exception cref="ArgumentNullException"><paramref name="appTag"/> is null.</exception>
        /// <exception cref="ArgumentException"><paramref name="appTag"/> breaks the rules of <see cref="AppTag"/>.</exception>
        public RequestOnboardingInviteOptions(string appTag)
        {
            _appTag = AppTagRules.Validate(appTag, nameof(appTag));
        }

        /// <summary>
        /// Required: the app tag of the co-process the invite is for. The hub binds the invite to
        /// the peer id it remembers for this session's approval and this tag, so the co-process
        /// joins as the same peer on every start while no other live process holds that id.
        /// </summary>
        /// <remarks>
        /// Same rules as <see cref="OnboardLocalApp.Tag"/>: non-empty printable ASCII, no space at
        /// either end, at most 256 bytes, compared exactly; an identity key like an app volume's
        /// common name, for example <c>companion.terraexplorer.skyline</c>.
        /// </remarks>
        /// <exception cref="ArgumentNullException">The value is null.</exception>
        /// <exception cref="ArgumentException">The value breaks the rules above (checked as a courtesy; the runtime checks too).</exception>
        public string AppTag
        {
            get { return _appTag; }
            set { _appTag = AppTagRules.Validate(value, nameof(value)); }
        }

        internal Structure? Extension { get; private set; }

        /// <summary>Chains an extension structure onto the request.</summary>
        public RequestOnboardingInviteOptions WithExtension(Structure extension)
        {
            Extension = StructureChainBuilder.Append(Extension, extension);
            return this;
        }
    }

    public sealed class ForgetOptions
    {
        public ForgetOptions(OnboardingId onboardingId)
        {
            OnboardingId = onboardingId;
        }

        public OnboardingId OnboardingId { get; set; }
        internal Structure? Extension { get; private set; }

        public ForgetOptions WithExtension(Structure extension)
        {
            Extension = StructureChainBuilder.Append(Extension, extension);
            return this;
        }
    }

    public readonly struct ActionProgress
    {
        public ActionProgress(ActionSeverity severity, float percent, string message)
        {
            Severity = severity;
            Percent = percent;
            Message = message ?? string.Empty;
        }

        public ActionSeverity Severity { get; }
        public float Percent { get; }
        public string Message { get; }
    }

    /// <summary>
    /// What <see cref="Runtime.Onboard"/> produces. Persist <see cref="OnboardingId"/> before
    /// anything else; the caller owns and disposes <see cref="Session"/>.
    /// </summary>
    public readonly struct OnboardSession
    {
        public OnboardSession(OnboardingId onboardingId, Session session)
        {
            OnboardingId = onboardingId;
            Session = session;
        }

        public OnboardingId OnboardingId { get; }
        public Session Session { get; }
    }

    public readonly struct RenderStreamRequest
    {
        public RenderStreamRequest(PeerId targetPeerId, StreamId streamId)
        {
            TargetPeerId = targetPeerId;
            StreamId = streamId;
        }

        public PeerId TargetPeerId { get; }
        public StreamId StreamId { get; }
    }
}
