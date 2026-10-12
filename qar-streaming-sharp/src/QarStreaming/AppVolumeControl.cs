using System;
using System.Threading;
#if NET8_0_OR_GREATER
using System.Collections.Generic;
using System.Runtime.CompilerServices;
#endif
using Qar.Native;

namespace Qar
{
    public sealed class AppVolumeControl
    {
        private readonly SessionNativeContext _context;

        internal AppVolumeControl(SessionNativeContext context)
        {
            _context = context ?? throw new ArgumentNullException(nameof(context));
        }

        public event EventHandler<AppVolumeUpdatedEventArgs>? Updated;

        /// <summary>
        /// Returns the volume identified by <see cref="AppVolumeOptions.CommonName"/>,
        /// creating it from <paramref name="options"/> if it does not exist yet.
        /// </summary>
        public Result<AppVolumeId> GetOrCreate(AppVolumeOptions options)
        {
            if (options == null)
                throw new ArgumentNullException(nameof(options));
            if (options.CommonName == null)
                throw new ArgumentException("AppVolumeOptions.CommonName is required.", nameof(options));
            using (var scope = new NativeMemoryScope())
            using (var chain = NativeStructureChain.From(options.Extension))
            {
                var native = NativeInit.AppVolume(options, scope, chain.Head);
                var id = AppVolumeIdNative.CreateDefault();
                var info = Results.ToInfo(NativeMethods.qar_impl_app_volumes_get_or_create(_context.DangerousHandle, ref native, ref id));
                return info.IsSuccess ? Result<AppVolumeId>.Success(info, AppVolumeId.FromNative(id)) : Result<AppVolumeId>.Failure(info);
            }
        }

        public Result<Unit> ChangeDisplayName(AppVolumeId id, string displayName)
        {
            var nativeId = id.ToNative();
            using (var str = NativeUtf8String.FromString(displayName))
                return Results.FromNative(NativeMethods.qar_impl_app_volumes_change_display_name(_context.DangerousHandle, ref nativeId, str.Pointer));
        }

        public Result<Unit> ChangeSize(AppVolumeId id, AppVolumeSize size)
        {
            var nativeId = id.ToNative();
            var nativeSize = size.ToNative();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_change_size(_context.DangerousHandle, ref nativeId, ref nativeSize));
        }

        public Result<Unit> Close(AppVolumeId id)
        {
            var nativeId = id.ToNative();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_close_volume(_context.DangerousHandle, ref nativeId));
        }

        public Result<Unit> UpdateUsedByPeers(AppVolumeId id, PeerId[] additions, PeerId[] removals)
        {
            var nativeId = id.ToNative();
            var nativeAdds = NativeIdArrays.ToNative(additions ?? Array.Empty<PeerId>());
            var nativeRemoves = NativeIdArrays.ToNative(removals ?? Array.Empty<PeerId>());
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_update_used_by_peers(_context.DangerousHandle, ref nativeId, nativeAdds, (UIntPtr)nativeAdds.Length, nativeRemoves, (UIntPtr)nativeRemoves.Length));
        }

        public Result<Unit> StartEditing(AppVolumeId id)
        {
            var nativeId = id.ToNative();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_start_editing(_context.DangerousHandle, ref nativeId));
        }

        public Result<Unit> StopEditing(AppVolumeId id)
        {
            var nativeId = id.ToNative();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_stop_editing(_context.DangerousHandle, ref nativeId));
        }

        /// <summary>Sets the app pose: the application's origin in app volume space, in volume metres.</summary>
        public Result<Unit> ChangeAppPose(AppVolumeId id, Pose appPose)
        {
            var nativeId = id.ToNative();
            var nativePose = appPose.ToNative();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_change_app_pose(_context.DangerousHandle, ref nativeId, ref nativePose));
        }

        public Result<Unit> ChangeAppScale(AppVolumeId id, float appScale)
        {
            var nativeId = id.ToNative();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_change_app_scale(_context.DangerousHandle, ref nativeId, appScale));
        }

        public Result<Unit> ChangeAppWorldAnchor(AppVolumeId id, GeoAnchorFrame anchor)
        {
            var nativeId = id.ToNative();
            var nativeAnchor = anchor.ToNative();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_change_app_world_anchor(_context.DangerousHandle, ref nativeId, ref nativeAnchor));
        }

        public Result<Unit> ClearAppWorldAnchor(AppVolumeId id)
        {
            var nativeId = id.ToNative();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_clear_app_world_anchor(_context.DangerousHandle, ref nativeId));
        }

        /// <summary>Moves the volume. <paramref name="pose"/> is the cuboid centre in room space.</summary>
        public Result<Unit> ChangePose(AppVolumeId id, Pose pose)
        {
            var nativeId = id.ToNative();
            var nativePose = pose.ToNative();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_change_pose(_context.DangerousHandle, ref nativeId, ref nativePose));
        }

        /// <summary>Latest locally known app pose, in app volume space.</summary>
        public Result<Pose> GetLatestAppPose(AppVolumeId id)
        {
            var nativeId = id.ToNative();
            PoseNative pose;
            var info = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_latest_app_pose(_context.DangerousHandle, ref nativeId, out pose));
            return info.IsSuccess ? Result<Pose>.Success(info, Pose.FromNative(pose)) : Result<Pose>.Failure(info);
        }

        public Result<float> GetLatestAppScale(AppVolumeId id)
        {
            var nativeId = id.ToNative();
            float scale;
            var info = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_latest_app_scale(_context.DangerousHandle, ref nativeId, out scale));
            return info.IsSuccess ? Result<float>.Success(info, scale) : Result<float>.Failure(info);
        }

        /// <summary>Latest locally known volume pose, in room space.</summary>
        public Result<Pose> GetLatestPose(AppVolumeId id)
        {
            var nativeId = id.ToNative();
            PoseNative pose;
            var info = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_latest_pose(_context.DangerousHandle, ref nativeId, out pose));
            return info.IsSuccess ? Result<Pose>.Success(info, Pose.FromNative(pose)) : Result<Pose>.Failure(info);
        }

        public Result<AppVolumeSize> GetLatestSize(AppVolumeId id)
        {
            var nativeId = id.ToNative();
            AppVolumeSizeNative size;
            var info = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_latest_size(_context.DangerousHandle, ref nativeId, out size));
            return info.IsSuccess ? Result<AppVolumeSize>.Success(info, AppVolumeSize.FromNative(size)) : Result<AppVolumeSize>.Failure(info);
        }

        /// <summary>
        /// Replaces the rules of gesture mode <see cref="AppVolumeGestureState.DefaultMode"/>
        /// and the allowed peers (<see cref="AppVolumeGestureConfiguration.AllowedPeers"/>).
        /// Other modes are kept. Fails with
        /// <see cref="StatusCode.AppVolumeGestureConfigurationInvalid"/> for a rule
        /// that could never work; the volume then keeps its previous configuration.
        /// </summary>
        public Result<Unit> ChangeGestureConfiguration(AppVolumeId id, AppVolumeGestureConfiguration configuration)
        {
            if (configuration == null)
                throw new ArgumentNullException(nameof(configuration));
            var nativeId = id.ToNative();
            using (var scope = new NativeMemoryScope())
            {
                var nativeConfig = configuration.ToNative(scope);
                return Results.FromNative(NativeMethods.qar_impl_app_volumes_change_gesture_configuration(_context.DangerousHandle, ref nativeId, ref nativeConfig));
            }
        }

        /// <summary>
        /// Defines gesture mode <paramref name="modeId"/>, or replaces the mode with
        /// that id. Other modes and the allowed peers are kept;
        /// <see cref="AppVolumeGestureConfiguration.AllowedPeers"/> must be empty.
        /// </summary>
        public Result<Unit> ChangeGestureMode(AppVolumeId id, uint modeId, AppVolumeGestureConfiguration configuration)
        {
            if (configuration == null)
                throw new ArgumentNullException(nameof(configuration));
            var nativeId = id.ToNative();
            using (var scope = new NativeMemoryScope())
            {
                var nativeConfig = configuration.ToNative(scope);
                return Results.FromNative(NativeMethods.qar_impl_app_volumes_change_gesture_mode(_context.DangerousHandle, ref nativeId, modeId, ref nativeConfig));
            }
        }

        /// <summary>
        /// Removes a gesture mode; peers in it fall back to
        /// <see cref="AppVolumeGestureState.DefaultMode"/>. The default mode cannot be
        /// removed (<see cref="StatusCode.AppVolumeGestureConfigurationInvalid"/>); an
        /// unknown one fails with <see cref="StatusCode.AppVolumeGestureModeUnknown"/>.
        /// </summary>
        public Result<Unit> RemoveGestureMode(AppVolumeId id, uint modeId)
        {
            var nativeId = id.ToNative();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_remove_gesture_mode(_context.DangerousHandle, ref nativeId, modeId));
        }

        /// <summary>
        /// Switches <paramref name="peer"/> to a gesture mode, or, with a null peer,
        /// makes it the default every peer without its own mode follows. A running
        /// gesture keeps the mode it began in.
        /// </summary>
        public Result<Unit> SelectGestureMode(AppVolumeId id, PeerId? peer, uint modeId)
        {
            var nativeId = id.ToNative();
            using (var scope = new NativeMemoryScope())
            {
                var nativePeer = peer.HasValue ? scope.AllocStruct(peer.Value.ToNative()) : IntPtr.Zero;
                return Results.FromNative(NativeMethods.qar_impl_app_volumes_select_gesture_mode(_context.DangerousHandle, ref nativeId, nativePeer, modeId));
            }
        }

        /// <summary>Drops one peer's own gesture mode, so it follows the default again.</summary>
        public Result<Unit> ClearPeerGestureMode(AppVolumeId id, PeerId peer)
        {
            var nativeId = id.ToNative();
            var nativePeer = peer.ToNative();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_clear_peer_gesture_mode(_context.DangerousHandle, ref nativeId, ref nativePeer));
        }

        /// <summary>Drops every peer's own gesture mode, so all follow the default.</summary>
        public Result<Unit> ClearAllPeerGestureModes(AppVolumeId id)
        {
            var nativeId = id.ToNative();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_clear_all_peer_gesture_modes(_context.DangerousHandle, ref nativeId));
        }

        /// <summary>
        /// Subscribe to every gesture kind on one app volume through one handler.
        /// Prefer this when taking more than one kind: everything arrives in the
        /// order the producer published it. Disposing the result stops delivery.
        /// </summary>
        public Result<IDisposable> SubscribeAllGestureUpdates(AppVolumeId id, EventHandler<AppVolumeGestureEventArgs> handler, CancellationToken cancellationToken = default(CancellationToken))
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            var nativeId = id.ToNative();
            return _context.StartSubscription(
                cancellationToken,
                token => AppVolumeGestureSubscription.Create(token, true, handler),
                (token, registration) => AppVolumeGestureSubscription.Create(token, registration, handler),
                (subscription, token) => NativeMethods.qar_impl_app_volumes_subscribe_all_gesture_events(_context.DangerousHandle, ref nativeId, subscription.Callback, subscription.UserState, token));
        }

        public Result<AppVolume[]> GetAll()
        {
            return _context.QueryHandles(
                NativeMethods.qar_impl_query_app_volumes_count,
                NativeMethods.qar_impl_query_app_volumes,
                handle => AppVolume.FromHandle(handle, true));
        }

        public Result<IDisposable> SubscribeUpdates()
        {
            return SubscribeUpdates(RaiseUpdated);
        }

        public Result<IDisposable> SubscribeUpdates(CancellationToken cancellationToken)
        {
            return SubscribeUpdates(RaiseUpdated, cancellationToken);
        }

        public Result<IDisposable> SubscribeUpdates(EventHandler<AppVolumeUpdatedEventArgs> handler)
        {
            var tokenResult = CancelToken.Create();
            if (!tokenResult.IsSuccess)
                return Result<IDisposable>.Failure(tokenResult.Info);
            return SubscribeUpdatesCore(handler, tokenResult.Value, true);
        }

        public Result<IDisposable> SubscribeUpdates(EventHandler<AppVolumeUpdatedEventArgs> handler, CancellationToken cancellationToken)
        {
            return SubscribeUpdatesCore(handler, cancellationToken);
        }

        private Result<IDisposable> SubscribeUpdatesCore(EventHandler<AppVolumeUpdatedEventArgs> handler, CancelToken cancelToken, bool ownsToken)
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            _context.ThrowIfDisposed();
            var subscription = AppVolumeUpdateSubscription.Create(cancelToken, ownsToken, handler);
            var info = Results.ToInfo(NativeMethods.qar_impl_app_volumes_subscribe_updates(_context.DangerousHandle, subscription.Callback, subscription.UserState, cancelToken.DangerousHandle));
            return Results.SubscriptionResult(info, subscription);
        }

        private Result<IDisposable> SubscribeUpdatesCore(EventHandler<AppVolumeUpdatedEventArgs> handler, CancellationToken cancellationToken)
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
                var subscription = hasRegistration ? AppVolumeUpdateSubscription.Create(tokenResult.Value, registration, handler) : AppVolumeUpdateSubscription.Create(tokenResult.Value, true, handler);
                var info = Results.ToInfo(NativeMethods.qar_impl_app_volumes_subscribe_updates(_context.DangerousHandle, subscription.Callback, subscription.UserState, tokenResult.Value.DangerousHandle));
                return Results.SubscriptionResult(info, subscription);
            }
            catch
            {
                registration.Dispose();
                throw;
            }
        }

#if NET8_0_OR_GREATER
        public async IAsyncEnumerable<AppVolume> WatchUpdatesAsync([EnumeratorCancellation] System.Threading.CancellationToken cancellationToken = default)
        {
            var queue = new AsyncEventQueue<AppVolume>();
            var subscriptionResult = SubscribeUpdates((sender, args) => queue.Enqueue(args.Volume));
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

        public Result<IDisposable> SubscribeGestureUpdates(AppVolumeId id, GestureKind gestureKind, Action<AppVolumeGestureEvent> handler)
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            return SubscribeGestureUpdates(id, gestureKind, (sender, args) => handler(args.GestureEvent));
        }

        public Result<IDisposable> SubscribeGestureUpdates(AppVolumeId id, GestureKind gestureKind, Action<AppVolumeGestureEvent> handler, CancellationToken cancellationToken)
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            return SubscribeGestureUpdates(id, gestureKind, (sender, args) => handler(args.GestureEvent), cancellationToken);
        }

        public Result<IDisposable> SubscribeGestureUpdates(AppVolumeId id, GestureKind gestureKind, EventHandler<AppVolumeGestureEventArgs> handler)
        {
            var tokenResult = CancelToken.Create();
            if (!tokenResult.IsSuccess)
                return Result<IDisposable>.Failure(tokenResult.Info);
            return SubscribeGestureUpdatesCore(id, gestureKind, handler, tokenResult.Value, true);
        }

        public Result<IDisposable> SubscribeGestureUpdates(AppVolumeId id, GestureKind gestureKind, EventHandler<AppVolumeGestureEventArgs> handler, CancellationToken cancellationToken)
        {
            return SubscribeGestureUpdatesCore(id, gestureKind, handler, cancellationToken);
        }

        private Result<IDisposable> SubscribeGestureUpdatesCore(AppVolumeId id, GestureKind gestureKind, EventHandler<AppVolumeGestureEventArgs> handler, CancelToken cancelToken, bool ownsToken)
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            _context.ThrowIfDisposed();
            var nativeId = id.ToNative();
            var subscription = AppVolumeGestureSubscription.Create(cancelToken, ownsToken, handler);
            var info = Results.ToInfo(NativeMethods.qar_impl_app_volumes_subscribe_gesture_events(_context.DangerousHandle, ref nativeId, gestureKind, subscription.Callback, subscription.UserState, cancelToken.DangerousHandle));
            return Results.SubscriptionResult(info, subscription);
        }

        private Result<IDisposable> SubscribeGestureUpdatesCore(AppVolumeId id, GestureKind gestureKind, EventHandler<AppVolumeGestureEventArgs> handler, CancellationToken cancellationToken)
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
                var subscription = hasRegistration ? AppVolumeGestureSubscription.Create(tokenResult.Value, registration, handler) : AppVolumeGestureSubscription.Create(tokenResult.Value, true, handler);
                var info = Results.ToInfo(NativeMethods.qar_impl_app_volumes_subscribe_gesture_events(_context.DangerousHandle, ref nativeId, gestureKind, subscription.Callback, subscription.UserState, tokenResult.Value.DangerousHandle));
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
        public async IAsyncEnumerable<AppVolumeGestureEvent> WatchGestureUpdatesAsync(AppVolumeId id, GestureKind gestureKind, [EnumeratorCancellation] System.Threading.CancellationToken cancellationToken = default)
        {
            var queue = new AsyncEventQueue<AppVolumeGestureEvent>();
            var subscriptionResult = SubscribeGestureUpdates(id, gestureKind, (sender, args) => queue.Enqueue(args.GestureEvent));
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

        private void RaiseUpdated(object? sender, AppVolumeUpdatedEventArgs args)
        {
            var handler = Updated;
            if (handler != null)
                handler(this, args);
        }
    }
}
