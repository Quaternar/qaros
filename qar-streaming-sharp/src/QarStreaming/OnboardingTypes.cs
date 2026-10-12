using System;
using Qar.Native;

namespace Qar
{
    public readonly struct OnboardingId : IEquatable<OnboardingId>
    {
        private readonly byte[]? _data;

        private OnboardingId(byte[] data)
        {
            _data = IdBytes.Normalize(data);
        }

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

    public sealed class PeerPresentationOptions
    {
        public string? DisplayName { get; set; }
        public string? AppVersion { get; set; }
        public string? AppCustomPeerInfo { get; set; }
        internal Structure? Extension { get; private set; }

        public PeerPresentationOptions WithExtension(Structure extension)
        {
            Extension = StructureChainBuilder.Append(Extension, extension);
            return this;
        }
    }

    public sealed class RejoinOptions
    {
        public RejoinOptions(OnboardingId onboardingId)
        {
            OnboardingId = onboardingId;
        }

        public OnboardingId OnboardingId { get; set; }
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

    public sealed class OnboardOptions
    {
        public OnboardingId? OnboardingId { get; set; }
        public PeerPresentationOptions? Presentation { get; set; }
        internal Structure Mode { get; private set; } = new OnboardCode(string.Empty);

        public OnboardOptions WithPresentation(PeerPresentationOptions presentation)
        {
            Presentation = presentation ?? throw new ArgumentNullException(nameof(presentation));
            return this;
        }

        public OnboardOptions UseCode(string code)
        {
            Mode = new OnboardCode(code ?? throw new ArgumentNullException(nameof(code)));
            return this;
        }

        public OnboardOptions UseInvite(OnboardingInvite invite)
        {
            Mode = new OnboardInvite(invite ?? throw new ArgumentNullException(nameof(invite)));
            return this;
        }

        /// <summary>
        /// Code onboarding against the hub at <paramref name="hostname"/>:<paramref name="port"/>.
        /// The port is required: hubs claim their onboarding port from a range, so there is no
        /// default. Without WithHost, code onboarding finds this user's hub on its own.
        /// </summary>
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
        /// process whose peer id was planned by someone else, such as the runtime
        /// launcher.
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

    public sealed class OnboardCode : Structure
    {
        public OnboardCode(string code)
            : base(StructureType.RuntimeOnboardCodeExt)
        {
            Code = code ?? throw new ArgumentNullException(nameof(code));
        }

        public string Code { get; set; }
    }

    public sealed class OnboardHost : Structure
    {
        public OnboardHost(string hostname, ushort port)
            : base(StructureType.RuntimeOnboardHostExt)
        {
            Hostname = hostname ?? throw new ArgumentNullException(nameof(hostname));
            Port = port;
        }

        public string Hostname { get; set; }
        public ushort Port { get; set; }
    }

    public sealed class OnboardInvite : Structure
    {
        public OnboardInvite(OnboardingInvite invite)
            : base(StructureType.RuntimeOnboardInviteExt)
        {
            Invite = invite ?? throw new ArgumentNullException(nameof(invite));
        }

        public OnboardingInvite Invite { get; }
    }

    public sealed class RequestOnboardingInviteOptions
    {
        internal Structure? Extension { get; private set; }

        public RequestOnboardingInviteOptions WithExtension(Structure extension)
        {
            Extension = StructureChainBuilder.Append(Extension, extension);
            return this;
        }
    }

    /// <summary>
    /// A request for a new set of one-time onboarding codes. Every request gets codes of its own;
    /// codes other callers hold and the hub's onboarding screen code stay valid.
    /// </summary>
    public sealed class RequestOnboardingCodesOptions
    {
        /// <summary>How many codes, 1 to 64.</summary>
        public uint Count { get; set; } = 1;

        /// <summary>How long each code stays valid; zero takes the hub default (120 s). At most 600 s.</summary>
        public TimeSpan Lifetime { get; set; } = TimeSpan.Zero;

        /// <summary>When set, a device pairing with one of these codes can only join under this peer id.</summary>
        public PeerId? PlannedPeerId { get; set; }
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
