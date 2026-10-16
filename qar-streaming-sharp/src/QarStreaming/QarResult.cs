using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    /// <summary>
    /// Status codes the C API exposes (QarStatusCode), with the same values.
    /// Any internal code outside this set reaches callers as
    /// <see cref="Unclassified"/>.
    /// </summary>
    public enum StatusCode
    {
        Success = 0,
        Unclassified = 1,
        NotImplemented = 4,
        ArgumentNotSupported = 5,
        Timeout = 6,
        /// <summary>
        /// A programming error, such as an invalid handle or a missing required argument; the
        /// binding also reports a native call that succeeded without its output this way.
        /// </summary>
        LogicError = 7,
        GuiPanelChangeNotPermitted = 304,
        GuiPanelInvalidId = 305,
        AppVolumeInvalidId = 325,
        /// <summary>
        /// A gesture configuration was refused because a rule in it could never
        /// work. The volume keeps its previous configuration; the message names
        /// the rule.
        /// </summary>
        AppVolumeGestureConfigurationInvalid = 326,
        /// <summary>A gesture mode was named that the volume does not define. Nothing was changed.</summary>
        AppVolumeGestureModeUnknown = 327,
        /// <summary>The receiver paused this stream (its display is not visible); keep the sender and try again later.</summary>
        MediaStreamingStreamIsPaused = 602,
        MediaStreamingStreamIsReconnecting = 603,
        MediaStreamingNoActiveTransfers = 604,
        MediaStreamingDestinationReservationFailed = 605,
        MediaStreamingSendPipelineFailed = 606,
        MediaStreamingLayoutChangeFailed = 607,
        RenderingProducerUnableToDoBeginFrame = 803,
        RenderingProducerStreamIsClosed = 804,
        /// <summary>
        /// The render sender's app volume does not resolve - it is not known to
        /// this session yet, or it was removed - so app content space is undefined
        /// and no frame was begun. Nothing is handed out in room space instead.
        /// Recoverable: keep calling begin-frame, it succeeds once the volume
        /// resolves. A volume that was removed never does; recreate it.
        /// </summary>
        RenderingProducerAppVolumeUnresolved = 805,
        /// <summary>Code-based onboarding handshake rejected, expired or timed out. Re-check the code shown on the hub and retry.</summary>
        PakeError = 1600,
        /// <summary>No persisted state for the onboarding id; fall back to a full onboard.</summary>
        OnboardingSessionNotFound = 1800,
        /// <summary>Generic onboarding failure; the message has details.</summary>
        OnboardingFailed = 1801,
        /// <summary>Discovery could not reach a hub; verify the hub is running (and the host/port) and retry.</summary>
        OnboardingHubUnreachable = 1802,
        /// <summary>Forget was called while the slot still has an active session; dispose the session first.</summary>
        OnboardingSessionStillActive = 1803,
        /// <summary>Local app onboarding: the user denied the app in the QAROS hub, or revoked its approval. Stop and tell the user.</summary>
        OnboardingApprovalDenied = 1805,
        /// <summary>Local app onboarding: nobody answered the approval prompt in time. Offer a retry.</summary>
        OnboardingApprovalTimeout = 1806,
        /// <summary>Local app onboarding: the process serving the enrollment pipe is not the installed QAROS hub. Stop; do not retry silently.</summary>
        OnboardingHubNotAuthentic = 1807,
        /// <summary>Local app onboarding: the app no longer matches what the user approved (moved, re-signed, tampered). Ask the user to re-approve it.</summary>
        OnboardingAppIdentityMismatch = 1808,
        /// <summary>The device's certificate expired or its hub no longer renews it, and the pairing credential did not replace it. Ask the user to pair the device again with a new code.</summary>
        OnboardingRePairRequired = 1809,
        /// <summary>Member join: the primary hub has the join request and waits for its operator to allow it. Not a failure; ask again later.</summary>
        MemberJoinPending = 3000,
        /// <summary>Member join: the primary hub's operator denied the join request.</summary>
        MemberJoinDenied = 3001,
        /// <summary>Member join: the primary hub refused the join request outright, for example because the joining hub hosts members of its own. The message names the reason.</summary>
        MemberJoinRefused = 3002,
        /// <summary>Member join: the asked hub is itself a member of another hub; ask that primary hub instead.</summary>
        MemberJoinRedirected = 3003,
        /// <summary>Member join: the primary hub removed this member hub; it has to join again.</summary>
        MemberRemoved = 3004,
        /// <summary>A member hub could not reach its primary hub.</summary>
        PrimaryUnreachable = 3005,
        /// <summary>A session-wide action was asked of a member hub; only the primary hub performs it.</summary>
        SessionWideActionOnMember = 3006,
        /// <summary>Sponsored member join: the app's approval on the primary hub does not allow it to bring in member hubs.</summary>
        MemberSponsorNotAllowed = 3007,
        /// <summary>Sponsored member join: the app's approval has used its quota of member hubs.</summary>
        MemberSponsorQuotaExceeded = 3008,
        /// <summary>A hub's audit log does not verify: a record was edited, dropped or reordered.</summary>
        AuditLogCorrupt = 3009
    }

    /// <summary>
    /// The outcome of a native call: a <see cref="StatusCode"/> and a diagnostic message.
    /// </summary>
    /// <remarks>
    /// Some failures are expected control flow rather than errors, for example
    /// <see cref="StatusCode.OnboardingSessionNotFound"/> from a rejoin. Equality compares
    /// <see cref="StatusCode"/> only; the message and <see cref="ErrorHandle"/> are ignored.
    /// </remarks>
    public readonly struct ResultInfo : IEquatable<ResultInfo>
    {
        private readonly string? _message;

        private readonly Lazy<string>? _lazyMessage;

        public ResultInfo(StatusCode statusCode, uint errorHandle, string message)
        {
            StatusCode = statusCode;
            ErrorHandle = errorHandle;
            _message = message ?? string.Empty;
            _lazyMessage = null;
        }

        internal ResultInfo(StatusCode statusCode, uint errorHandle, Func<string> messageFactory)
        {
            StatusCode = statusCode;
            ErrorHandle = errorHandle;
            _message = null;
            _lazyMessage = new Lazy<string>(() => messageFactory() ?? string.Empty, LazyThreadSafetyMode.ExecutionAndPublication);
        }

        public StatusCode StatusCode { get; }

        /// <summary>The native diagnostic object behind a failure; 0 on success.</summary>
        public uint ErrorHandle { get; }

        public string Message { get { return _message ?? (_lazyMessage == null ? string.Empty : _lazyMessage.Value); } }

        public bool IsSuccess { get { return StatusCode == StatusCode.Success; } }

        /// <exception cref="NativeCallException">The result is a failure.</exception>
        public void EnsureSuccess()
        {
            if (!IsSuccess)
                throw new NativeCallException(this);
        }

        public override string ToString()
        {
            return string.IsNullOrEmpty(Message) ? StatusCode.ToString() : StatusCode + ": " + Message;
        }

        public bool Equals(ResultInfo other)
        {
            return StatusCode == other.StatusCode;
        }

        public override bool Equals(object? obj)
        {
            return obj is ResultInfo other && Equals(other);
        }

        public override int GetHashCode()
        {
            unchecked
            {
                var hash = 17;
                hash = (hash * 31) + (int)StatusCode;
                return hash;
            }
        }

        public static bool operator ==(ResultInfo left, ResultInfo right)
        {
            return left.Equals(right);
        }

        public static bool operator !=(ResultInfo left, ResultInfo right)
        {
            return !left.Equals(right);
        }
    }

    /// <summary>The value of a <see cref="Result{T}"/> whose operation returns nothing.</summary>
    public readonly struct Unit
    {
        public static readonly Unit Value = new Unit();
    }

    /// <summary>
    /// A value or a failure. A native failure is returned, never thrown: check
    /// <see cref="IsSuccess"/>, or opt into an exception with <see cref="ValueOrThrow"/> or
    /// <see cref="EnsureSuccess"/>.
    /// </summary>
    /// <remarks>
    /// The binding throws only for managed misuse: a null argument, or an
    /// <see cref="ObjectDisposedException"/> for an object already disposed. An async method
    /// completes its task with a failed result on a native failure; only managed cancellation
    /// cancels the task.
    /// </remarks>
    public readonly struct Result<T>
    {
        private readonly T _value;

        private Result(ResultInfo info, T value)
        {
            Info = info;
            _value = value;
        }

        public ResultInfo Info { get; }

        /// <exception cref="InvalidOperationException">The result is a failure.</exception>
        public T Value
        {
            get
            {
                if (!IsSuccess)
                    throw new InvalidOperationException("A failed QAR result has no value.");
                return _value;
            }
        }

        public bool IsSuccess { get { return Info.IsSuccess; } }

        public static Result<T> Success(ResultInfo info, T value)
        {
            return new Result<T>(info, value);
        }

        public static Result<T> Failure(ResultInfo info)
        {
            return new Result<T>(info, default(T)!);
        }

        /// <exception cref="NativeCallException">The result is a failure.</exception>
        public void EnsureSuccess()
        {
            Info.EnsureSuccess();
        }

        /// <exception cref="NativeCallException">The result is a failure.</exception>
        public T ValueOrThrow()
        {
            if (!IsSuccess)
                throw new NativeCallException(Info);
            return _value;
        }
    }

    /// <summary>
    /// A native failure turned into an exception on request, by <see cref="Result{T}.ValueOrThrow"/>
    /// or <c>EnsureSuccess</c>. <see cref="Result"/> carries the status code.
    /// </summary>
    public sealed class NativeCallException : Exception
    {
        public NativeCallException(ResultInfo result)

            : base(result.ToString())
        {
            Result = result;
        }

        public ResultInfo Result { get; }
    }
}
