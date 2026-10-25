using System;
using System.Threading;
#if NET8_0_OR_GREATER
using System.Collections.Generic;
using System.Runtime.CompilerServices;
#endif
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    /// <summary>
    /// The peers of a session: every participant, devices and applications alike, this process
    /// included. Reached through <see cref="Session.Peers"/>.
    /// </summary>
    /// <remarks>
    /// Peers join, change and leave asynchronously, so <see cref="GetAll"/> is a snapshot of what
    /// this session knows at that moment. To react to changes, subscribe with
    /// <see cref="SubscribeUpdates(EventHandler{PeerSpecUpdatedEventArgs})"/>.
    /// </remarks>
    public sealed class PeerControl
    {
        private readonly SessionNativeContext _context;

        internal PeerControl(SessionNativeContext context)
        {
            _context = context ?? throw new ArgumentNullException(nameof(context));
        }

        /// <summary>
        /// Raised for every peer update, but only while a subscription from
        /// <see cref="SubscribeUpdates()"/> or <see cref="SubscribeUpdates(CancellationToken)"/> is
        /// alive: attaching a handler alone delivers nothing.
        /// </summary>
        /// <remarks>Raised on a library thread, like every subscription handler.</remarks>
        public event EventHandler<PeerSpecUpdatedEventArgs>? Updated;

        /// <summary>This process' own peer, for example to learn its peer id.</summary>
        public Result<PeerSpec> GetMySpec()
        {
            IntPtr handle;
            using var session = _context.Borrow();
            var info = Results.ToInfo(NativeMethods.qar_impl_session_get_my_spec(session.Handle, out handle));
            return info.IsSuccess ? PeerSpec.FromHandle(handle, true) : Result<PeerSpec>.Failure(info);
        }

        /// <summary>
        /// Every peer this session knows now, this process' own included (see
        /// <see cref="GetMySpec"/>).
        /// </summary>
        public Result<PeerSpec[]> GetAll()
        {
            return _context.QueryHandles(
                NativeMethods.qar_impl_query_peer_specs,
                handle => PeerSpec.FromHandle(handle, true));
        }

        /// <summary>Changes the display name other peers see for this process; any UTF-8.</summary>
        public Result<Unit> UpdateDisplayName(string name)
        {
            using (var str = NativeUtf8String.FromUtf8(name))
            using (var session = _context.Borrow())
                return Results.FromNative(NativeMethods.qar_impl_peer_update_display_name(session.Handle, str.Pointer));
        }

        /// <summary>
        /// Replaces the device this process describes itself as, for example once a camera SDK
        /// has reported the exact model it opened. Published as a normal peer update; the role
        /// stays <see cref="PeerRole.Source"/>. Fails with
        /// <see cref="StatusCode.PeerDeviceCategoryNotAllowed"/> for a category an application may
        /// not claim.
        /// </summary>
        public Result<Unit> UpdateDeviceIdentity(PeerDeviceIdentity identity)
        {
            if (identity == null)
                throw new ArgumentNullException(nameof(identity));
            using (var chain = NativeStructureChain.From(new PeerDeviceIdentity(identity.Category, identity.DeviceId)
            {
                DeviceFamily = identity.DeviceFamily,
                DeviceName = identity.DeviceName
            }))
            using (var session = _context.Borrow())
                return Results.FromNative(NativeMethods.qar_impl_peer_update_device_identity(session.Handle, chain.Head));
        }

        /// <summary>
        /// Starts delivering peer updates to <see cref="Updated"/> until the returned subscription is
        /// disposed.
        /// </summary>
        public Result<IDisposable> SubscribeUpdates()
        {
            return SubscribeUpdates(RaiseUpdated);
        }

        /// <summary>
        /// Starts delivering peer updates to <see cref="Updated"/> until the returned subscription is
        /// disposed or <paramref name="cancellationToken"/> is canceled.
        /// </summary>
        public Result<IDisposable> SubscribeUpdates(CancellationToken cancellationToken)
        {
            return SubscribeUpdates(RaiseUpdated, cancellationToken);
        }

        /// <summary>
        /// Calls <paramref name="handler"/> for every peer that joins, leaves or changes its
        /// presentation or app state, until the returned subscription is disposed. The way to learn
        /// that an invited target app joined: it is listed and <see cref="PeerSpec.IsReachable"/>.
        /// </summary>
        /// <remarks>
        /// <para>
        /// The handler runs on a library thread: marshal to your own thread before touching
        /// single-threaded state, and do not make blocking QAROS calls from it. An exception it
        /// throws is swallowed.
        /// </para>
        /// <para>
        /// When <c>Dispose</c> of the subscription returns, a handler call running on another thread
        /// has finished and no further one starts. Disposing from inside the handler returns at once.
        /// Never dispose while holding a lock the handler takes: that deadlocks.
        /// </para>
        /// </remarks>
        public Result<IDisposable> SubscribeUpdates(EventHandler<PeerSpecUpdatedEventArgs> handler)
        {
            var tokenResult = CancelToken.Create();
            if (!tokenResult.IsSuccess)
                return Result<IDisposable>.Failure(tokenResult.Info);
            return SubscribeUpdatesCore(handler, tokenResult.Value, true);
        }

        /// <summary>
        /// <see cref="SubscribeUpdates(EventHandler{PeerSpecUpdatedEventArgs})"/> that also ends
        /// when <paramref name="cancellationToken"/> is canceled. Still dispose the subscription.
        /// </summary>
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
            return _context.StartSubscription(subscription, StartUpdates);
        }

        private Result<IDisposable> SubscribeUpdatesCore(EventHandler<PeerSpecUpdatedEventArgs> handler, CancellationToken cancellationToken)
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            _context.ThrowIfDisposed();
            return _context.StartSubscription(
                cancellationToken,
                token => PeerUpdateSubscription.Create(token, true, handler),
                (token, registration) => PeerUpdateSubscription.Create(token, registration, handler),
                StartUpdates);
        }

        private static ResultNative StartUpdates(PeerUpdateSubscription subscription, IntPtr session, IntPtr token)
        {
            return NativeMethods.qar_impl_peer_subscribe_updates(session, subscription.Callback, subscription.UserState, token);
        }

#if NET8_0_OR_GREATER
        /// <summary>
        /// Peer updates as an async stream, buffered in arrival order, until
        /// <paramref name="cancellationToken"/> is canceled or the enumeration is abandoned.
        /// </summary>
        /// <exception cref="NativeCallException">The subscription could not be started.</exception>
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
