using System;
using System.Collections.Generic;
using System.Threading;
using Qar.Native;

namespace Qar
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

        internal Result<T[]> QueryHandles<T>(
            CountFunc countFunc,
            QueryFunc queryFunc,
            Func<IntPtr, Result<T>> snapshot)
        {
            UIntPtr count;
            var countInfo = Results.ToInfo(countFunc(DangerousHandle, out count));
            if (!countInfo.IsSuccess)
                return Result<T[]>.Failure(countInfo);
            var handles = new IntPtr[NativeConvert.ToInt32(count)];
            UIntPtr written;
            var queryInfo = Results.ToInfo(queryFunc(DangerousHandle, handles, count, out written));
            if (!queryInfo.IsSuccess)
                return Result<T[]>.Failure(queryInfo);
            var writtenCount = NativeConvert.ToInt32(written);
            if (writtenCount > handles.Length)
                return Result<T[]>.Failure(new ResultInfo(StatusCode.LogicError, 0, "Native query wrote more handles than the provided buffer can hold."));
            var result = new List<T>(writtenCount);
            ResultInfo? firstSnapshotFailure = null;
            for (var i = 0; i < writtenCount; i++)
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

        internal delegate ResultNative CountFunc(IntPtr session, out UIntPtr count);

        internal delegate ResultNative QueryFunc(IntPtr session, IntPtr[] handles, UIntPtr bufferSize, out UIntPtr written);
    }
}
