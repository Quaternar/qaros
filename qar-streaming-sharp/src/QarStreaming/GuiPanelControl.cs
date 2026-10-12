using System;
using System.Threading;
#if NET8_0_OR_GREATER
using System.Collections.Generic;
using System.Runtime.CompilerServices;
#endif
using Qar.Native;

namespace Qar
{
    public sealed class GuiPanelControl
    {
        private readonly SessionNativeContext _context;

        internal GuiPanelControl(SessionNativeContext context)
        {
            _context = context ?? throw new ArgumentNullException(nameof(context));
        }

        public event EventHandler<GuiPanelUpdatedEventArgs>? Updated;

        /// <summary>
        /// Returns the panel identified by <see cref="GuiPanelOptions.CommonName"/>,
        /// creating it from <paramref name="options"/> if it does not exist yet.
        /// </summary>
        public Result<GuiPanelId> GetOrCreate(GuiPanelOptions options)
        {
            if (options == null)
                throw new ArgumentNullException(nameof(options));
            if (options.CommonName == null)
                throw new ArgumentException("GuiPanelOptions.CommonName is required.", nameof(options));
            if (options.ContentUri != null && options.ContentStreamId.HasValue)
                throw new ArgumentException("A GUI panel can have either URI or stream content, but not both.", nameof(options));
            using (var scope = new NativeMemoryScope())
            using (var chain = NativeStructureChain.From(options.Extension))
            {
                var content = CreateContentExtension(options, scope, chain.Head);
                var native = NativeInit.GuiPanel(options, scope, content);
                var id = GuiPanelIdNative.CreateDefault();
                var info = Results.ToInfo(NativeMethods.qar_impl_gui_panels_get_or_create(_context.DangerousHandle, ref native, ref id));
                return info.IsSuccess ? Result<GuiPanelId>.Success(info, GuiPanelId.FromNative(id)) : Result<GuiPanelId>.Failure(info);
            }
        }

        /// <summary>
        /// Subscribe to one kind of gesture the rendering peers report for one
        /// panel. Several producers publish, one per user: key lifecycles on
        /// (source peer, gesture id). Disposing the result stops delivery.
        /// </summary>
        public Result<IDisposable> SubscribeGestureEvents(GuiPanelId id, GuiPanelGestureKind gestureKind, EventHandler<GuiPanelGestureEventArgs> handler, CancellationToken cancellationToken = default(CancellationToken))
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            var nativeId = id.ToNative();
            return _context.StartSubscription(
                cancellationToken,
                token => GuiPanelGestureSubscription.Create(token, true, handler),
                (token, registration) => GuiPanelGestureSubscription.Create(token, registration, handler),
                (subscription, token) => NativeMethods.qar_impl_gui_panels_subscribe_gesture_events(_context.DangerousHandle, ref nativeId, gestureKind, subscription.Callback, subscription.UserState, token));
        }

        /// <summary>
        /// Subscribe to every gesture kind on one panel through one handler. Prefer
        /// this when taking more than one kind: everything arrives in the order
        /// the producer published it.
        /// </summary>
        public Result<IDisposable> SubscribeAllGestureEvents(GuiPanelId id, EventHandler<GuiPanelGestureEventArgs> handler, CancellationToken cancellationToken = default(CancellationToken))
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            var nativeId = id.ToNative();
            return _context.StartSubscription(
                cancellationToken,
                token => GuiPanelGestureSubscription.Create(token, true, handler),
                (token, registration) => GuiPanelGestureSubscription.Create(token, registration, handler),
                (subscription, token) => NativeMethods.qar_impl_gui_panels_subscribe_all_gesture_events(_context.DangerousHandle, ref nativeId, subscription.Callback, subscription.UserState, token));
        }

        /// <summary>Moves a panel. <paramref name="pose"/> is in room space.</summary>
        public Result<Unit> UpdatePose(GuiPanelId id, Pose pose)
        {
            var nativeId = id.ToNative();
            var nativePose = pose.ToNative();
            return Results.FromNative(NativeMethods.qar_impl_gui_panels_update_pose(_context.DangerousHandle, ref nativeId, ref nativePose));
        }

        public Result<Unit> ChangeSize(GuiPanelId id, GuiPanelSize size)
        {
            var nativeId = id.ToNative();
            var nativeSize = size.ToNative();
            return Results.FromNative(NativeMethods.qar_impl_gui_panels_change_size(_context.DangerousHandle, ref nativeId, ref nativeSize));
        }

        public Result<Unit> SetState(GuiPanelId id, GuiPanelState state)
        {
            var nativeId = id.ToNative();
            return Results.FromNative(NativeMethods.qar_impl_gui_panels_set_state(_context.DangerousHandle, ref nativeId, state));
        }

        public Result<Unit> Close(GuiPanelId id)
        {
            var nativeId = id.ToNative();
            return Results.FromNative(NativeMethods.qar_impl_gui_panels_close_panel(_context.DangerousHandle, ref nativeId));
        }

        public Result<Unit> NavigateToUri(GuiPanelId id, string uri)
        {
            var nativeId = id.ToNative();
            using (var nativeUri = NativeUtf8String.FromString(uri))
                return Results.FromNative(NativeMethods.qar_impl_gui_panels_navigate_to_uri(_context.DangerousHandle, ref nativeId, nativeUri.Pointer));
        }

        public Result<Unit> SetStream(GuiPanelId id, StreamId streamId)
        {
            var nativeId = id.ToNative();
            var nativeStreamId = streamId.ToNative();
            return Results.FromNative(NativeMethods.qar_impl_gui_panels_set_stream(_context.DangerousHandle, ref nativeId, ref nativeStreamId));
        }

        /// <summary>
        /// Adds and removes peers from the panel's visible-to list. An empty list means the
        /// panel is visible to every peer: removing the last peer shows it to everyone again,
        /// it never hides it. Hide a panel through its state instead.
        /// </summary>
        public Result<Unit> UpdateVisibleTo(GuiPanelId id, PeerId[] additions, PeerId[] removals)
        {
            var nativeId = id.ToNative();
            using (var scope = new NativeMemoryScope())
            {
                IntPtr addPtr = scope.AllocPointerArray(additions ?? Array.Empty<PeerId>());
                IntPtr removePtr = scope.AllocPointerArray(removals ?? Array.Empty<PeerId>());
                return Results.FromNative(NativeMethods.qar_impl_gui_panels_update_visible_to(_context.DangerousHandle, ref nativeId, addPtr, (UIntPtr)(additions == null ? 0 : additions.Length), removePtr, (UIntPtr)(removals == null ? 0 : removals.Length)));
            }
        }

        public Result<GuiPanel[]> GetAll()
        {
            return _context.QueryHandles(
                NativeMethods.qar_impl_query_gui_panels,
                handle => GuiPanel.FromHandle(handle, true));
        }

        public Result<IDisposable> SubscribeUpdates()
        {
            return SubscribeUpdates(RaiseUpdated);
        }

        public Result<IDisposable> SubscribeUpdates(CancellationToken cancellationToken)
        {
            return SubscribeUpdates(RaiseUpdated, cancellationToken);
        }

        public Result<IDisposable> SubscribeUpdates(EventHandler<GuiPanelUpdatedEventArgs> handler)
        {
            var tokenResult = CancelToken.Create();
            if (!tokenResult.IsSuccess)
                return Result<IDisposable>.Failure(tokenResult.Info);
            return SubscribeUpdatesCore(handler, tokenResult.Value, true);
        }

        public Result<IDisposable> SubscribeUpdates(EventHandler<GuiPanelUpdatedEventArgs> handler, CancellationToken cancellationToken)
        {
            return SubscribeUpdatesCore(handler, cancellationToken);
        }

        private Result<IDisposable> SubscribeUpdatesCore(EventHandler<GuiPanelUpdatedEventArgs> handler, CancelToken cancelToken, bool ownsToken)
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            _context.ThrowIfDisposed();
            var subscription = GuiPanelUpdateSubscription.Create(cancelToken, ownsToken, handler);
            var info = Results.ToInfo(NativeMethods.qar_impl_gui_panels_subscribe_updates(_context.DangerousHandle, subscription.Callback, subscription.UserState, cancelToken.DangerousHandle));
            return Results.SubscriptionResult(info, subscription);
        }

        private Result<IDisposable> SubscribeUpdatesCore(EventHandler<GuiPanelUpdatedEventArgs> handler, CancellationToken cancellationToken)
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
                var subscription = hasRegistration ? GuiPanelUpdateSubscription.Create(tokenResult.Value, registration, handler) : GuiPanelUpdateSubscription.Create(tokenResult.Value, true, handler);
                var info = Results.ToInfo(NativeMethods.qar_impl_gui_panels_subscribe_updates(_context.DangerousHandle, subscription.Callback, subscription.UserState, tokenResult.Value.DangerousHandle));
                return Results.SubscriptionResult(info, subscription);
            }
            catch
            {
                registration.Dispose();
                throw;
            }
        }

#if NET8_0_OR_GREATER
        public async IAsyncEnumerable<GuiPanel> WatchUpdatesAsync([EnumeratorCancellation] System.Threading.CancellationToken cancellationToken = default)
        {
            var queue = new AsyncEventQueue<GuiPanel>();
            var subscriptionResult = SubscribeUpdates((sender, args) => queue.Enqueue(args.Panel));
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

        public Result<IDisposable> SubscribeUpdates(GuiPanelId id)
        {
            return SubscribeUpdates(id, RaiseUpdated);
        }

        public Result<IDisposable> SubscribeUpdates(GuiPanelId id, CancellationToken cancellationToken)
        {
            return SubscribeUpdates(id, RaiseUpdated, cancellationToken);
        }

        public Result<IDisposable> SubscribeUpdates(GuiPanelId id, EventHandler<GuiPanelUpdatedEventArgs> handler)
        {
            var tokenResult = CancelToken.Create();
            if (!tokenResult.IsSuccess)
                return Result<IDisposable>.Failure(tokenResult.Info);
            return SubscribeUpdatesCore(id, handler, tokenResult.Value, true);
        }

        public Result<IDisposable> SubscribeUpdates(GuiPanelId id, EventHandler<GuiPanelUpdatedEventArgs> handler, CancellationToken cancellationToken)
        {
            return SubscribeUpdatesCore(id, handler, cancellationToken);
        }

        private Result<IDisposable> SubscribeUpdatesCore(GuiPanelId id, EventHandler<GuiPanelUpdatedEventArgs> handler, CancelToken cancelToken, bool ownsToken)
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            _context.ThrowIfDisposed();
            var nativeId = id.ToNative();
            var subscription = GuiPanelUpdateSubscription.Create(cancelToken, ownsToken, handler);
            var info = Results.ToInfo(NativeMethods.qar_impl_gui_panels_subscribe_panel_updates(_context.DangerousHandle, ref nativeId, subscription.Callback, subscription.UserState, cancelToken.DangerousHandle));
            return Results.SubscriptionResult(info, subscription);
        }

        private Result<IDisposable> SubscribeUpdatesCore(GuiPanelId id, EventHandler<GuiPanelUpdatedEventArgs> handler, CancellationToken cancellationToken)
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            _context.ThrowIfDisposed();
            var nativeId = id.ToNative();
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
                var subscription = hasRegistration ? GuiPanelUpdateSubscription.Create(tokenResult.Value, registration, handler) : GuiPanelUpdateSubscription.Create(tokenResult.Value, true, handler);
                var info = Results.ToInfo(NativeMethods.qar_impl_gui_panels_subscribe_panel_updates(_context.DangerousHandle, ref nativeId, subscription.Callback, subscription.UserState, tokenResult.Value.DangerousHandle));
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
        public async IAsyncEnumerable<GuiPanel> WatchUpdatesAsync(GuiPanelId id, [EnumeratorCancellation] System.Threading.CancellationToken cancellationToken = default)
        {
            var queue = new AsyncEventQueue<GuiPanel>();
            var subscriptionResult = SubscribeUpdates(id, (sender, args) => queue.Enqueue(args.Panel));
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

        private void RaiseUpdated(object? sender, GuiPanelUpdatedEventArgs args)
        {
            var handler = Updated;
            if (handler != null)
                handler(this, args);
        }

        private static IntPtr CreateContentExtension(GuiPanelOptions options, NativeMemoryScope scope, IntPtr next)
        {
            if (options.ContentStreamId.HasValue)
            {
                return scope.AllocStruct(new GuiPanelStreamIdExtNative
                {
                    Header = NativeDefaults.Header(StructureType.GuiPanelStreamIdExt, next),
                    StreamId = options.ContentStreamId.Value.ToNative()
                });
            }
            if (options.ContentUri != null)
            {
                return scope.AllocStruct(new GuiPanelUrlExtNative
                {
                    Header = NativeDefaults.Header(StructureType.GuiPanelUrlExt, next),
                    Url = scope.AllocString(options.ContentUri)
                });
            }
            return next;
        }
    }
}
