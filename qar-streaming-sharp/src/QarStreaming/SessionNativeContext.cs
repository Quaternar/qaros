using System;
using System.Collections.Generic;
using System.Threading;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    internal sealed class SessionNativeContext : IDisposable
    {
        private readonly NativeHandleOwner _owner;

        internal SessionNativeContext(SessionSafeHandle handle)
        {
            _owner = new NativeHandleOwner(handle, nameof(Session));
        }

        /// <summary>
        /// The session handle for an async call to hold until its native completion
        /// (AsyncStateBase.HoldUntilCompletion).
        /// </summary>
        internal NativeHandleOwner Owner { get { return _owner; } }

        /// <summary>
        /// The session handle, referenced until the returned borrow is disposed: every native call
        /// that takes the session runs inside <c>using (var session = context.Borrow())</c>, so
        /// neither a concurrent <see cref="Dispose"/> nor the finalizer frees the session while the
        /// call still uses it (hub G56).
        /// </summary>
        /// <exception cref="ObjectDisposedException">The session is disposed.</exception>
        internal HandleBorrow Borrow()
        {
            return _owner.Borrow();
        }

        internal void ThrowIfDisposed()
        {
            _owner.ThrowIfDisposed();
        }

        // Every native query is a full shared-object reconcile that waits out its
        // reply window, so a list is read with one query call, not a count call
        // followed by a query. A buffer the query fills completely may have cut
        // the list short: its handles are released and the query repeated with a
        // larger one.
        private const int InitialQueryCapacity = 32;

        internal Result<T[]> QueryHandles<T>(
            QueryFunc queryFunc,
            Func<IntPtr, Result<T>> snapshot)
        {
            using (var session = Borrow())
                return QueryHandles(session.Handle, queryFunc, snapshot);
        }

        private static Result<T[]> QueryHandles<T>(
            IntPtr session,
            QueryFunc queryFunc,
            Func<IntPtr, Result<T>> snapshot)
        {
            var capacity = InitialQueryCapacity;
            while (true)
            {
                var handles = new IntPtr[capacity];
                UIntPtr written;
                var queryInfo = Results.ToInfo(queryFunc(session, handles, (UIntPtr)capacity, out written));
                if (!queryInfo.IsSuccess)
                    return Result<T[]>.Failure(queryInfo);
                var writtenCount = NativeConvert.ToInt32(written);
                if (writtenCount > handles.Length)
                    return Result<T[]>.Failure(new ResultInfo(StatusCode.LogicError, 0, "Native query wrote more handles than the provided buffer can hold."));
                var snapshots = SnapshotHandles(queryInfo, handles, writtenCount, snapshot);
                if (writtenCount < capacity)
                    return snapshots;
                capacity *= 2;
            }
        }

        // Reads and releases every handle, so none leaks when the first one fails.
        private static Result<T[]> SnapshotHandles<T>(ResultInfo queryInfo, IntPtr[] handles, int count, Func<IntPtr, Result<T>> snapshot)
        {
            var result = new List<T>(count);
            ResultInfo? firstSnapshotFailure = null;
            for (var i = 0; i < count; i++)
            {
                var item = snapshot(handles[i]);
                if (item.IsSuccess)
                    result.Add(item.Value);
                else if (!firstSnapshotFailure.HasValue)
                    firstSnapshotFailure = item.Info;
            }
            if (firstSnapshotFailure.HasValue)
                return Result<T[]>.Failure(firstSnapshotFailure.Value);
            return Result<T[]>.Success(queryInfo, result.ToArray());
        }

        /// <summary>
        /// Starts a continuous native subscription that owns its native cancel
        /// token, bridged to <paramref name="cancellationToken"/> when it can be
        /// canceled; disposing the subscription stops native delivery.
        /// <paramref name="start"/> gets the subscription, the borrowed session
        /// handle and the subscription's cancel token handle.
        /// </summary>
        internal Result<IDisposable> StartSubscription<T>(
            CancellationToken cancellationToken,
            Func<CancelToken, T> createOwned,
            Func<CancelToken, CancellationTokenRegistration, T> createRegistered,
            Func<T, IntPtr, IntPtr, ResultNative> start)
            where T : NativeSubscription
        {
            ThrowIfDisposed();
            var tokenResult = CancelToken.Create();
            if (!tokenResult.IsSuccess)
                return Result<IDisposable>.Failure(tokenResult.Info);
            var token = tokenResult.Value;
            if (cancellationToken.IsCancellationRequested)
                token.Cancel();
            var subscription = cancellationToken.CanBeCanceled && !cancellationToken.IsCancellationRequested
                ? createRegistered(token, cancellationToken.Register(CancelNativeToken, token))
                : createOwned(token);
            return StartSubscription(subscription, start);
        }

        /// <summary>
        /// Starts <paramref name="subscription"/> with the session borrowed for the call; a failed
        /// start, a disposed session included, disposes the subscription and its owned token.
        /// </summary>
        internal Result<IDisposable> StartSubscription<T>(T subscription, Func<T, IntPtr, IntPtr, ResultNative> start)
            where T : NativeSubscription
        {
            ResultInfo info;
            try
            {
                using (var session = Borrow())
                    info = Results.ToInfo(start(subscription, session.Handle, subscription.TokenHandle));
            }
            catch
            {
                subscription.DisposeAfterFailedStart();
                throw;
            }
            return Results.SubscriptionResult(info, subscription);
        }

        private static void CancelNativeToken(object? state)
        {
            try
            {
                ((CancelToken)state!).Cancel();
            }
            catch (ObjectDisposedException)
            {
            }
        }

        /// <summary>
        /// Makes the session unusable and releases it. A call still borrowing the session defers the
        /// native release until it is done.
        /// </summary>
        public void Dispose()
        {
            _owner.Dispose();
        }

        internal delegate ResultNative QueryFunc(IntPtr session, IntPtr[] handles, UIntPtr bufferSize, out UIntPtr written);
    }
}
