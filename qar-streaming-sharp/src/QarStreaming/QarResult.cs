using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Qar.Native;

namespace Qar
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
        OnboardingSessionStillActive = 1803
    }

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

        public uint ErrorHandle { get; }

        public string Message { get { return _message ?? (_lazyMessage == null ? string.Empty : _lazyMessage.Value); } }

        public bool IsSuccess { get { return StatusCode == StatusCode.Success; } }

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

    public readonly struct Unit
    {
        public static readonly Unit Value = new Unit();
    }

    public readonly struct Result<T>
    {
        private readonly T _value;

        private Result(ResultInfo info, T value)
        {
            Info = info;
            _value = value;
        }

        public ResultInfo Info { get; }

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

        public void EnsureSuccess()
        {
            Info.EnsureSuccess();
        }

        public T ValueOrThrow()
        {
            if (!IsSuccess)
                throw new NativeCallException(Info);
            return _value;
        }
    }

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
