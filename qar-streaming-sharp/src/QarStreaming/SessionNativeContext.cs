using System;
using System.Collections.Generic;
using System.Threading;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    internal sealed class SessionNativeContext : IDisposable
    {
        private readonly SessionSafeHandle _handle;

        internal SessionNativeContext(SessionSafeHandle handle)
        {
            _handle = handle ?? throw new ArgumentNullException(nameof(handle));
        }

        internal IntPtr DangerousHandle
        {
            get
            {
                ThrowIfDisposed();
                return _handle.DangerousGetHandle();
            }
        }

        internal void ThrowIfDisposed()
        {
            if (_handle.IsClosed || _handle.IsInvalid)
                throw new ObjectDisposedException(nameof(Session));
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
            var capacity = InitialQueryCapacity;
            while (true)
            {
                var handles = new IntPtr[capacity];
                UIntPtr written;
                var queryInfo = Results.ToInfo(queryFunc(DangerousHandle, handles, (UIntPtr)capacity, out written));
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
        /// </summary>
        internal Result<IDisposable> StartSubscription<T>(
            CancellationToken cancellationToken,
            Func<CancelToken, T> createOwned,
            Func<CancelToken, CancellationTokenRegistration, T> createRegistered,
            Func<T, IntPtr, ResultNative> start)
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
            var info = Results.ToInfo(start(subscription, token.DangerousHandle));
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

        public void Dispose()
        {
            _handle.Dispose();
        }

        internal delegate ResultNative QueryFunc(IntPtr session, IntPtr[] handles, UIntPtr bufferSize, out UIntPtr written);
    }
}
