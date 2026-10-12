using System;
using System.Threading;
#if NET8_0_OR_GREATER
using System.Collections.Generic;
using System.Runtime.CompilerServices;
#endif
using Qar.Native;

namespace Qar
{
    public sealed class PeerControl
    {
        private readonly SessionNativeContext _context;

        internal PeerControl(SessionNativeContext context)
        {
            _context = context ?? throw new ArgumentNullException(nameof(context));
        }

        public event EventHandler<PeerSpecUpdatedEventArgs>? Updated;

        public Result<PeerSpec> GetMySpec()
        {
            IntPtr handle;
            var info = Results.ToInfo(NativeMethods.qar_impl_session_get_my_spec(_context.DangerousHandle, out handle));
            return info.IsSuccess ? PeerSpec.FromHandle(handle, true) : Result<PeerSpec>.Failure(info);
        }

        public Result<PeerSpec[]> GetAll()
        {
            return _context.QueryHandles(
                NativeMethods.qar_impl_query_peer_specs,
                handle => PeerSpec.FromHandle(handle, true));
        }

        public Result<Unit> UpdateDisplayName(string name)
        {
            using (var str = NativeUtf8String.FromString(name))
                return Results.FromNative(NativeMethods.qar_impl_peer_update_display_name(_context.DangerousHandle, str.Pointer));
        }

        public Result<IDisposable> SubscribeUpdates()
        {
            return SubscribeUpdates(RaiseUpdated);
        }

        public Result<IDisposable> SubscribeUpdates(CancellationToken cancellationToken)
        {
            return SubscribeUpdates(RaiseUpdated, cancellationToken);
        }

        public Result<IDisposable> SubscribeUpdates(EventHandler<PeerSpecUpdatedEventArgs> handler)
        {
            var tokenResult = CancelToken.Create();
            if (!tokenResult.IsSuccess)
                return Result<IDisposable>.Failure(tokenResult.Info);
            return SubscribeUpdatesCore(handler, tokenResult.Value, true);
        }

        public Result<IDisposable> SubscribeUpdates(EventHandler<PeerSpecUpdatedEventArgs> handler, CancellationToken cancellationToken)
        {
            return SubscribeUpdatesCore(handler, cancellationToken);
        }

        private Result<IDisposable> SubscribeUpdatesCore(EventHandler<PeerSpecUpdatedEventArgs> handler, CancelToken cancelToken, bool ownsToken)
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            _context.ThrowIfDisposed();
            var subscription = PeerUpdateSubscription.Create(cancelToken, ownsToken, handler);
            var info = Results.ToInfo(NativeMethods.qar_impl_peer_subscribe_updates(_context.DangerousHandle, subscription.Callback, subscription.UserState, cancelToken.DangerousHandle));
            return Results.SubscriptionResult(info, subscription);
        }

        private Result<IDisposable> SubscribeUpdatesCore(EventHandler<PeerSpecUpdatedEventArgs> handler, CancellationToken cancellationToken)
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            _context.ThrowIfDisposed();
            CancellationTokenRegistration registration = default(CancellationTokenRegistration);
            try
            {
                var tokenResult = CancelToken.Create();
                if (!tokenResult.IsSuccess)
                    return Result<IDisposable>.Failure(tokenResult.Info);
                var hasRegistration = false;
                if (cancellationToken.IsCancellationRequested)
                    tokenResult.Value.Cancel();
                else if (cancellationToken.CanBeCanceled)
                {
                    registration = cancellationToken.Register(CancelNativeToken, tokenResult.Value);
                    hasRegistration = true;
                }
                var subscription = hasRegistration ? PeerUpdateSubscription.Create(tokenResult.Value, registration, handler) : PeerUpdateSubscription.Create(tokenResult.Value, true, handler);
                var info = Results.ToInfo(NativeMethods.qar_impl_peer_subscribe_updates(_context.DangerousHandle, subscription.Callback, subscription.UserState, tokenResult.Value.DangerousHandle));
                return Results.SubscriptionResult(info, subscription);
            }
            catch
            {
                registration.Dispose();
                throw;
            }
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

#if NET8_0_OR_GREATER
        public async IAsyncEnumerable<PeerSpec> WatchUpdatesAsync([EnumeratorCancellation] System.Threading.CancellationToken cancellationToken = default)
        {
            var queue = new AsyncEventQueue<PeerSpec>();
            var subscriptionResult = SubscribeUpdates((sender, args) => queue.Enqueue(args.Peer));
            if (!subscriptionResult.IsSuccess)
                throw new NativeCallException(subscriptionResult.Info);
            using (subscriptionResult.Value)
            {
                try
                {
                    await foreach (var item in queue.ReadAllAsync(cancellationToken))
                        yield return item;
                }
                finally
                {
                    queue.Complete();
                }
            }
        }
#endif

        private void RaiseUpdated(object? sender, PeerSpecUpdatedEventArgs args)
        {
            var handler = Updated;
            if (handler != null)
                handler(this, args);
        }
    }
}
