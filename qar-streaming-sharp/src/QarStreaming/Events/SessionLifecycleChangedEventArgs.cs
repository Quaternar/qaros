using System;
using System.Runtime.InteropServices;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    /// <summary>
    /// One state of a session's lifecycle (C <c>QarSessionLifecycleInfo</c>): a snapshot, safe to
    /// keep. Delivered by <see cref="Session.SubscribeLifecycle(EventHandler{SessionLifecycleChangedEventArgs})"/>
    /// and returned by <see cref="Session.GetLifecycleState"/>.
    /// </summary>
    public sealed class SessionLifecycleChangedEventArgs : EventArgs
    {
        public SessionLifecycleChangedEventArgs(SessionLifecycleState state, SessionChangeCause cause, SessionId sessionId, SessionId? previousSessionId, string reason)
        {
            State = state;
            Cause = cause;
            SessionId = sessionId;
            PreviousSessionId = previousSessionId;
            Reason = reason ?? string.Empty;
        }

        /// <summary>Where the session stands now.</summary>
        public SessionLifecycleState State { get; }

        /// <summary>
        /// Why the latest move happened; <see cref="SessionChangeCause.None"/> on a state that is not
        /// part of a move.
        /// </summary>
        public SessionChangeCause Cause { get; }

        /// <summary>
        /// The session the app is in now: during <see cref="SessionLifecycleState.Migrating"/> still
        /// the old one, the new one once the move completed. <see cref="Session.GetId"/> returns the
        /// same.
        /// </summary>
        public SessionId SessionId { get; }

        /// <summary>
        /// The session it moved from, on the state that ends a move; null otherwise.
        /// </summary>
        public SessionId? PreviousSessionId { get; }

        /// <summary>Text for logs and the operator, never a secret; empty from <see cref="Session.GetLifecycleState"/>.</summary>
        public string Reason { get; }

        internal static SessionLifecycleChangedEventArgs FromNative(SessionLifecycleInfoNative native)
        {
            SessionId? previous = null;
            if (native.HasPreviousSession)
                previous = SessionId.FromNative(native.PreviousSessionId);
            var reason = NativeString.DecodeNullTerminated(native.Reason) ?? string.Empty;
            return new SessionLifecycleChangedEventArgs(native.State, native.Cause, SessionId.FromNative(native.SessionId), previous, reason);
        }

        internal static SessionLifecycleChangedEventArgs FromNativePointer(IntPtr info)
        {
            return FromNative(Marshal.PtrToStructure<SessionLifecycleInfoNative>(info));
        }
    }
}
