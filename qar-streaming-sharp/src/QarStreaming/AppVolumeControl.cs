using System;
using System.Threading;
#if NET8_0_OR_GREATER
using System.Collections.Generic;
using System.Runtime.CompilerServices;
#endif
using Quaternar.Qaros.Interop;
using Quaternar.Qaros.Mathematics;

namespace Quaternar.Qaros
{
    /// <summary>
    /// The session's app volumes: the boxes in the room that remote 3D applications render into.
    /// Reached through <see cref="Session.AppVolumes"/>.
    /// </summary>
    /// <remarks>
    /// <para>
    /// App volume state is shared session state and eventually consistent. A change method
    /// returns success once it has published the change; the change reaches the shared state,
    /// and every peer's snapshots, later, and may still be refused there. A read right after a
    /// write can return the old value: observe the result through
    /// <see cref="SubscribeUpdates(EventHandler{AppVolumeUpdatedEventArgs}, CancellationToken)"/>
    /// instead of reading back once.
    /// </para>
    /// <para>
    /// A change to a volume this session does not know yet (its id was never seen, or its
    /// creation has not arrived) fails rather than waiting for it.
    /// </para>
    /// <para>
    /// Spaces: <see cref="ChangePose"/> places the cuboid centre in room space, in metres.
    /// <see cref="ChangeAppPose"/> and <see cref="ChangeAppScale"/> place the application's own
    /// world (app content space) inside the box. See <see cref="AppVolume"/>.
    /// </para>
    /// <para>
    /// Every method throws <see cref="ObjectDisposedException"/> once the session is disposed.
    /// </para>
    /// </remarks>
    public sealed class AppVolumeControl
    {
        private readonly SessionNativeContext _context;

        internal AppVolumeControl(SessionNativeContext context)
        {
            _context = context ?? throw new ArgumentNullException(nameof(context));
        }

        /// <summary>
        /// Raised for every app volume update, but only while a subscription started by
        /// <see cref="SubscribeUpdates()"/> or
        /// <see cref="SubscribeUpdates(CancellationToken)"/> is alive.
        /// </summary>
        /// <remarks>
        /// Raised on a library thread with this control as the sender; see
        /// <see cref="SubscribeUpdates(EventHandler{AppVolumeUpdatedEventArgs}, CancellationToken)"/>
        /// for what is delivered and the threading rules. Each such subscription raises the event
        /// once per update, so two live subscriptions raise it twice.
        /// </remarks>
        public event EventHandler<AppVolumeUpdatedEventArgs>? Updated;

        /// <summary>
        /// Returns the volume identified by <see cref="AppVolumeOptions.CommonName"/>,
        /// creating it from <paramref name="options"/> if it does not exist yet.
        /// </summary>
        /// <remarks>
        /// <para>
        /// The id is derived from the common name alone, so the same common name gives the
        /// same volume in every session, from every peer and after a restart: call this on every
        /// start instead of persisting the id. The common name is the identity key and must be
        /// ASCII; put user-facing text, which may be any UTF-8, in
        /// <see cref="AppVolumeOptions.DisplayName"/>. Set a non-empty DisplayName when
        /// creating or reopening a volume; CommonName does not supply a default label.
        /// </para>
        /// <para>
        /// When an open volume of that name exists, the rest of <paramref name="options"/> is
        /// ignored and the existing volume keeps its pose, size, app transform and gesture
        /// configuration; change them with the <c>Change*</c> methods. A closed volume
        /// (<see cref="Close"/>) is reopened with <paramref name="options"/>.
        /// </para>
        /// <para>
        /// An existing volume is taken over by this session (a previous run of the app may have
        /// left it) and carried into the hub's next session like one this session created. Call
        /// GetOrCreate again for every common name the app owns when the session id changes and
        /// after onboarding again on <see cref="SessionLifecycleState.RePairRequired"/>
        /// (<see cref="Session.SubscribeLifecycle(EventHandler{SessionLifecycleChangedEventArgs})"/>):
        /// it returns the same id and re-creates the volume when it is missing.
        /// </para>
        /// <para>
        /// A gesture configuration whose rule could never work fails with
        /// <see cref="StatusCode.AppVolumeGestureConfigurationInvalid"/>.
        /// </para>
        /// </remarks>
        /// <exception cref="ArgumentNullException"><paramref name="options"/> is null.</exception>
        /// <exception cref="ArgumentException">
        /// <see cref="AppVolumeOptions.CommonName"/> is null.
        /// </exception>
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
                using var session = _context.Borrow();
                var info = Results.ToInfo(NativeMethods.qar_impl_app_volumes_get_or_create(session.Handle, ref native, ref id));
                return info.IsSuccess ? Result<AppVolumeId>.Success(info, AppVolumeId.FromNative(id)) : Result<AppVolumeId>.Failure(info);
            }
        }

        /// <summary>
        /// Changes the label users see. Any UTF-8 text, emoji included; the common name, and with
        /// it the id, never changes.
        /// </summary>
        public Result<Unit> ChangeDisplayName(AppVolumeId id, string displayName)
        {
            var nativeId = id.ToNative();
            using (var str = NativeUtf8String.FromUtf8(displayName))
            using (var session = _context.Borrow())
                return Results.FromNative(NativeMethods.qar_impl_app_volumes_change_display_name(session.Handle, ref nativeId, str.Pointer));
        }

        /// <summary>
        /// Resizes the box around its centre, in metres. A fast-path change:
        /// <see cref="GetLatestSize"/> returns it at once.
        /// </summary>
        public Result<Unit> ChangeSize(AppVolumeId id, AppVolumeSize size)
        {
            var nativeId = id.ToNative();
            var nativeSize = size.ToNative();
            using var session = _context.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_change_size(session.Handle, ref nativeId, ref nativeSize));
        }

        /// <summary>
        /// Closes the volume for every peer: its <see cref="AppVolume.LifetimeStatus"/> becomes
        /// <see cref="AppVolumeLifetimeStatus.Closed"/>.
        /// </summary>
        /// <remarks>
        /// The volume is not forgotten: <see cref="GetOrCreate"/> with the same common name
        /// reopens it with new options. A render sender bound to a closed volume fails to begin
        /// frames with <see cref="StatusCode.RenderingProducerAppVolumeUnresolved"/>.
        /// </remarks>
        public Result<Unit> Close(AppVolumeId id)
        {
            var nativeId = id.ToNative();
            using var session = _context.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_close_volume(session.Handle, ref nativeId));
        }

        /// <summary>
        /// Adds and removes peers in <see cref="AppVolume.UsedByPeers"/>. Null arrays mean none.
        /// </summary>
        /// <remarks>
        /// Each peer is a separate change, additions first. The first failure stops the call and
        /// the changes before it stay published.
        /// </remarks>
        public Result<Unit> UpdateUsedByPeers(AppVolumeId id, PeerId[] additions, PeerId[] removals)
        {
            var nativeId = id.ToNative();
            var nativeAdds = NativeIdArrays.ToNative(additions ?? Array.Empty<PeerId>());
            var nativeRemoves = NativeIdArrays.ToNative(removals ?? Array.Empty<PeerId>());
            using var session = _context.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_update_used_by_peers(session.Handle, ref nativeId, nativeAdds, (UIntPtr)nativeAdds.Length, nativeRemoves, (UIntPtr)nativeRemoves.Length));
        }

        /// <summary>
        /// Marks the volume's box as being moved or resized by this session's own peer, which
        /// every peer reads in <see cref="AppVolume.EditingStatus"/>, for example to show who is
        /// moving it.
        /// </summary>
        /// <remarks>
        /// It replaces whatever editor was set before. A peer manipulating the app with a gesture
        /// is not an editor; see <see cref="AppVolumeGestureState.Holder"/>.
        /// </remarks>
        public Result<Unit> StartEditing(AppVolumeId id)
        {
            var nativeId = id.ToNative();
            using var session = _context.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_start_editing(session.Handle, ref nativeId));
        }

        /// <summary>Clears the editing status, whichever peer set it.</summary>
        public Result<Unit> StopEditing(AppVolumeId id)
        {
            var nativeId = id.ToNative();
            using var session = _context.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_stop_editing(session.Handle, ref nativeId));
        }

        /// <summary>Sets the app pose: the application's origin in app volume space, in volume metres.</summary>
        /// <remarks>
        /// Pans the content inside the box without the renderer changing anything: the render
        /// sender's view poses follow. Not scaled by the app scale. A fast-path change:
        /// <see cref="GetLatestAppPose"/> returns it at once. Gestures mapped to the app
        /// transform change it too.
        /// </remarks>
        public Result<Unit> ChangeAppPose(AppVolumeId id, Pose appPose)
        {
            var nativeId = id.ToNative();
            var nativePose = appPose.ToNative();
            using var session = _context.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_change_app_pose(session.Handle, ref nativeId, ref nativePose));
        }

        /// <summary>
        /// Sets the app scale: room metres per app metre, so 0.01 shows a 100 m model as 1 m.
        /// Must be greater than 0, otherwise the call fails with
        /// <see cref="StatusCode.ArgumentNotSupported"/>.
        /// </summary>
        /// <remarks>
        /// Zooms the content inside the box without the renderer changing anything. A fast-path
        /// change: <see cref="GetLatestAppScale"/> returns it at once. Gestures mapped to the app
        /// transform change it too.
        /// </remarks>
        public Result<Unit> ChangeAppScale(AppVolumeId id, float appScale)
        {
            var nativeId = id.ToNative();
            using var session = _context.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_change_app_scale(session.Handle, ref nativeId, appScale));
        }

        /// <summary>
        /// Pins the volume to an Earth-fixed frame (ECEF WGS84, metres). Cleared with
        /// <see cref="ClearAppWorldAnchor"/>.
        /// </summary>
        public Result<Unit> ChangeAppWorldAnchor(AppVolumeId id, GeoAnchorFrame anchor)
        {
            var nativeId = id.ToNative();
            var nativeAnchor = anchor.ToNative();
            using var session = _context.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_change_app_world_anchor(session.Handle, ref nativeId, ref nativeAnchor));
        }

        public Result<Unit> ClearAppWorldAnchor(AppVolumeId id)
        {
            var nativeId = id.ToNative();
            using var session = _context.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_clear_app_world_anchor(session.Handle, ref nativeId));
        }

        /// <summary>Moves the volume. <paramref name="pose"/> is the cuboid centre in room space.</summary>
        /// <remarks>
        /// The centre, not a corner or a point on the floor. A fast-path change:
        /// <see cref="GetLatestPose"/> returns it at once.
        /// </remarks>
        public Result<Unit> ChangePose(AppVolumeId id, Pose pose)
        {
            var nativeId = id.ToNative();
            var nativePose = pose.ToNative();
            using var session = _context.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_change_pose(session.Handle, ref nativeId, ref nativePose));
        }

        /// <summary>Latest locally known app pose, in app volume space.</summary>
        /// <remarks>
        /// The <c>GetLatest*</c> reads answer from this session's local view without a network
        /// round trip: the latest fast-path value from this or any other peer, else the stored
        /// state. They can be newer than the last <see cref="AppVolume"/> snapshot delivered, and
        /// fail while this session does not know the volume yet.
        /// </remarks>
        public Result<Pose> GetLatestAppPose(AppVolumeId id)
        {
            var nativeId = id.ToNative();
            PoseNative pose;
            using var session = _context.Borrow();
            var info = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_latest_app_pose(session.Handle, ref nativeId, out pose));
            return info.IsSuccess ? Result<Pose>.Success(info, Pose.FromNative(pose)) : Result<Pose>.Failure(info);
        }

        /// <summary>Latest locally known app scale; see <see cref="GetLatestAppPose"/>.</summary>
        public Result<float> GetLatestAppScale(AppVolumeId id)
        {
            var nativeId = id.ToNative();
            float scale;
            using var session = _context.Borrow();
            var info = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_latest_app_scale(session.Handle, ref nativeId, out scale));
            return info.IsSuccess ? Result<float>.Success(info, scale) : Result<float>.Failure(info);
        }

        /// <summary>
        /// Latest locally known volume pose (the cuboid centre), in room space; see
        /// <see cref="GetLatestAppPose"/>.
        /// </summary>
        public Result<Pose> GetLatestPose(AppVolumeId id)
        {
            var nativeId = id.ToNative();
            PoseNative pose;
            using var session = _context.Borrow();
            var info = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_latest_pose(session.Handle, ref nativeId, out pose));
            return info.IsSuccess ? Result<Pose>.Success(info, Pose.FromNative(pose)) : Result<Pose>.Failure(info);
        }

        /// <summary>Latest locally known size; see <see cref="GetLatestAppPose"/>.</summary>
        public Result<AppVolumeSize> GetLatestSize(AppVolumeId id)
        {
            var nativeId = id.ToNative();
            AppVolumeSizeNative size;
            using var session = _context.Borrow();
            var info = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_latest_size(session.Handle, ref nativeId, out size));
            return info.IsSuccess ? Result<AppVolumeSize>.Success(info, AppVolumeSize.FromNative(size)) : Result<AppVolumeSize>.Failure(info);
        }

        /// <summary>
        /// Replaces the rules of gesture mode <see cref="AppVolumeGestureState.DefaultMode"/>
        /// and the allowed peers (<see cref="AppVolumeGestureConfiguration.AllowedPeers"/>).
        /// Other modes are kept. Fails with
        /// <see cref="StatusCode.AppVolumeGestureConfigurationInvalid"/> for a rule
        /// that could never work; the volume then keeps its previous configuration.
        /// </summary>
        /// <remarks>
        /// Rule order is priority: an earlier rule can suppress a later one when both are active
        /// for the same volume and source peer. The call reads the volume back from the shared
        /// storage before writing, so it takes about as long as <see cref="GetAll"/>.
        /// </remarks>
        /// <exception cref="ArgumentNullException"><paramref name="configuration"/> is null.</exception>
        public Result<Unit> ChangeGestureConfiguration(AppVolumeId id, AppVolumeGestureConfiguration configuration)
        {
            if (configuration == null)
                throw new ArgumentNullException(nameof(configuration));
            var nativeId = id.ToNative();
            using (var scope = new NativeMemoryScope())
            {
                var nativeConfig = configuration.ToNative(scope);
                using var session = _context.Borrow();
                return Results.FromNative(NativeMethods.qar_impl_app_volumes_change_gesture_configuration(session.Handle, ref nativeId, ref nativeConfig));
            }
        }

        /// <summary>
        /// Defines gesture mode <paramref name="modeId"/>, or replaces the mode with
        /// that id. Other modes and the allowed peers are kept;
        /// <see cref="AppVolumeGestureConfiguration.AllowedPeers"/> must be empty.
        /// </summary>
        /// <remarks>
        /// A mode is a set of mapping rules peers are switched to with
        /// <see cref="SelectGestureMode"/>, so a user's gestures and rays change with one small
        /// call. Any id other than <see cref="AppVolumeGestureState.DefaultMode"/> is the
        /// application's to choose. Non-empty allowed peers fail with
        /// <see cref="StatusCode.ArgumentNotSupported"/>, a rule that could never work with
        /// <see cref="StatusCode.AppVolumeGestureConfigurationInvalid"/>.
        /// </remarks>
        /// <exception cref="ArgumentNullException"><paramref name="configuration"/> is null.</exception>
        public Result<Unit> ChangeGestureMode(AppVolumeId id, uint modeId, AppVolumeGestureConfiguration configuration)
        {
            if (configuration == null)
                throw new ArgumentNullException(nameof(configuration));
            var nativeId = id.ToNative();
            using (var scope = new NativeMemoryScope())
            {
                var nativeConfig = configuration.ToNative(scope);
                using var session = _context.Borrow();
                return Results.FromNative(NativeMethods.qar_impl_app_volumes_change_gesture_mode(session.Handle, ref nativeId, modeId, ref nativeConfig));
            }
        }

        /// <summary>
        /// Removes a gesture mode; peers in it fall back to
        /// <see cref="AppVolumeGestureState.DefaultMode"/>. The default mode cannot be
        /// removed (<see cref="StatusCode.AppVolumeGestureConfigurationInvalid"/>); an
        /// unknown one fails with <see cref="StatusCode.AppVolumeGestureModeUnknown"/>.
        /// </summary>
        /// <remarks>A gesture already running in the removed mode finishes in it.</remarks>
        public Result<Unit> RemoveGestureMode(AppVolumeId id, uint modeId)
        {
            var nativeId = id.ToNative();
            using var session = _context.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_remove_gesture_mode(session.Handle, ref nativeId, modeId));
        }

        /// <summary>
        /// Switches <paramref name="peer"/> to a gesture mode, or, with a null peer,
        /// makes it the default every peer without its own mode follows. A running
        /// gesture keeps the mode it began in.
        /// </summary>
        /// <remarks>
        /// Changing the default leaves peers switched individually in their own mode; clear them
        /// with <see cref="ClearPeerGestureMode"/> or <see cref="ClearAllPeerGestureModes"/>. A
        /// mode the volume does not define fails with
        /// <see cref="StatusCode.AppVolumeGestureModeUnknown"/>.
        /// </remarks>
        public Result<Unit> SelectGestureMode(AppVolumeId id, PeerId? peer, uint modeId)
        {
            var nativeId = id.ToNative();
            using (var scope = new NativeMemoryScope())
            {
                var nativePeer = peer.HasValue ? scope.AllocStruct(peer.Value.ToNative()) : IntPtr.Zero;
                using var session = _context.Borrow();
                return Results.FromNative(NativeMethods.qar_impl_app_volumes_select_gesture_mode(session.Handle, ref nativeId, nativePeer, modeId));
            }
        }

        /// <summary>Drops one peer's own gesture mode, so it follows the default again.</summary>
        public Result<Unit> ClearPeerGestureMode(AppVolumeId id, PeerId peer)
        {
            var nativeId = id.ToNative();
            var nativePeer = peer.ToNative();
            using var session = _context.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_clear_peer_gesture_mode(session.Handle, ref nativeId, ref nativePeer));
        }

        /// <summary>Drops every peer's own gesture mode, so all follow the default.</summary>
        public Result<Unit> ClearAllPeerGestureModes(AppVolumeId id)
        {
            var nativeId = id.ToNative();
            using var session = _context.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_app_volumes_clear_all_peer_gesture_modes(session.Handle, ref nativeId));
        }

        /// <summary>
        /// Subscribe to every gesture kind on one app volume through one handler.
        /// Prefer this when taking more than one kind: everything arrives in the
        /// order the producer published it. Disposing the result stops delivery.
        /// </summary>
        /// <remarks>
        /// Dispatch on <see cref="AppVolumeGestureEvent.GestureKind"/>. The volume's mapping
        /// rules still decide what is published at all; only the kind filter is gone. Everything
        /// else - mapping, several producers, threading and disposal - is as for
        /// <see cref="SubscribeGestureUpdates(AppVolumeId, GestureKind, EventHandler{AppVolumeGestureEventArgs}, CancellationToken)"/>.
        /// </remarks>
        /// <exception cref="ArgumentNullException"><paramref name="handler"/> is null.</exception>
        public Result<IDisposable> SubscribeAllGestureUpdates(AppVolumeId id, EventHandler<AppVolumeGestureEventArgs> handler, CancellationToken cancellationToken = default(CancellationToken))
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            var nativeId = id.ToNative();
            return _context.StartSubscription(
                cancellationToken,
                token => AppVolumeGestureSubscription.Create(token, true, handler),
                (token, registration) => AppVolumeGestureSubscription.Create(token, registration, handler),
                (subscription, session, token) => NativeMethods.qar_impl_app_volumes_subscribe_all_gesture_events(session, ref nativeId, subscription.Callback, subscription.UserState, token));
        }

        /// <summary>
        /// Snapshot of every app volume the session's shared storage holds. Blocks for a network
        /// round trip of about 2 s on every call.
        /// </summary>
        /// <remarks>
        /// Use it for a one-off listing. To track volumes, subscribe with
        /// <see cref="SubscribeUpdates(EventHandler{AppVolumeUpdatedEventArgs}, CancellationToken)"/>,
        /// which delivers the same snapshot first; for one volume's live transform, use the
        /// <c>GetLatest*</c> reads.
        /// </remarks>
        public Result<AppVolume[]> GetAll()
        {
            return _context.QueryHandles(
                NativeMethods.qar_impl_query_app_volumes,
                handle => AppVolume.FromHandle(handle, true));
        }

        /// <summary>
        /// Starts delivering app volume updates through the <see cref="Updated"/> event until the
        /// result is disposed.
        /// </summary>
        /// <remarks>
        /// Same delivery and disposal rules as
        /// <see cref="SubscribeUpdates(EventHandler{AppVolumeUpdatedEventArgs}, CancellationToken)"/>.
        /// </remarks>
        public Result<IDisposable> SubscribeUpdates()
        {
            return SubscribeUpdates(RaiseUpdated);
        }

        /// <inheritdoc cref="SubscribeUpdates()"/>
        public Result<IDisposable> SubscribeUpdates(CancellationToken cancellationToken)
        {
            return SubscribeUpdates(RaiseUpdated, cancellationToken);
        }

        /// <inheritdoc cref="SubscribeUpdates(EventHandler{AppVolumeUpdatedEventArgs}, CancellationToken)"/>
        public Result<IDisposable> SubscribeUpdates(EventHandler<AppVolumeUpdatedEventArgs> handler)
        {
            var tokenResult = CancelToken.Create();
            if (!tokenResult.IsSuccess)
                return Result<IDisposable>.Failure(tokenResult.Info);
            return SubscribeUpdatesCore(handler, tokenResult.Value, true);
        }

        /// <summary>
        /// Calls <paramref name="handler"/> once for every app volume already stored, then again
        /// for every change to any volume, until the result is disposed.
        /// </summary>
        /// <remarks>
        /// <para>
        /// Each call carries an immutable <see cref="AppVolume"/> snapshot that is safe to keep.
        /// Its poses are in the publishing peer's room space.
        /// </para>
        /// <para>
        /// The handler runs on a library thread: do not block in it, do not call blocking QAROS
        /// methods from it, and marshal to your own thread before touching single-threaded
        /// state. An exception it throws is caught and dropped.
        /// </para>
        /// <para>
        /// Disposing the result ends delivery: when <c>Dispose</c> returns, a handler call
        /// running on another thread has finished and no further one starts. Called from inside
        /// the handler, <c>Dispose</c> returns at once and no call starts after the current one.
        /// Never dispose while holding a lock the handler takes: it deadlocks. Cancelling
        /// <paramref name="cancellationToken"/> ends delivery the same way, and
        /// <see cref="CancellationTokenSource.Cancel()"/> waits the same way.
        /// </para>
        /// </remarks>
        /// <exception cref="ArgumentNullException"><paramref name="handler"/> is null.</exception>
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
            return _context.StartSubscription(subscription, StartUpdates);
        }

        private Result<IDisposable> SubscribeUpdatesCore(EventHandler<AppVolumeUpdatedEventArgs> handler, CancellationToken cancellationToken)
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            _context.ThrowIfDisposed();
            return _context.StartSubscription(
                cancellationToken,
                token => AppVolumeUpdateSubscription.Create(token, true, handler),
                (token, registration) => AppVolumeUpdateSubscription.Create(token, registration, handler),
                StartUpdates);
        }

#if NET8_0_OR_GREATER
        /// <summary>
        /// The snapshots of
        /// <see cref="SubscribeUpdates(EventHandler{AppVolumeUpdatedEventArgs}, CancellationToken)"/>
        /// as an async stream: every stored volume first, then every change.
        /// </summary>
        /// <remarks>
        /// The subscription starts with the enumeration and ends when the enumerator is disposed.
        /// Updates are queued without bound until read, so a consumer that stops reading without
        /// disposing keeps accumulating them.
        /// </remarks>
        /// <exception cref="NativeCallException">
        /// Thrown on the first <c>MoveNextAsync</c> when the subscription cannot be started.
        /// </exception>
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

        /// <inheritdoc cref="SubscribeGestureUpdates(AppVolumeId, GestureKind, EventHandler{AppVolumeGestureEventArgs}, CancellationToken)"/>
        public Result<IDisposable> SubscribeGestureUpdates(AppVolumeId id, GestureKind gestureKind, Action<AppVolumeGestureEvent> handler)
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            return SubscribeGestureUpdates(id, gestureKind, (sender, args) => handler(args.GestureEvent));
        }

        /// <inheritdoc cref="SubscribeGestureUpdates(AppVolumeId, GestureKind, EventHandler{AppVolumeGestureEventArgs}, CancellationToken)"/>
        public Result<IDisposable> SubscribeGestureUpdates(AppVolumeId id, GestureKind gestureKind, Action<AppVolumeGestureEvent> handler, CancellationToken cancellationToken)
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            return SubscribeGestureUpdates(id, gestureKind, (sender, args) => handler(args.GestureEvent), cancellationToken);
        }

        /// <inheritdoc cref="SubscribeGestureUpdates(AppVolumeId, GestureKind, EventHandler{AppVolumeGestureEventArgs}, CancellationToken)"/>
        public Result<IDisposable> SubscribeGestureUpdates(AppVolumeId id, GestureKind gestureKind, EventHandler<AppVolumeGestureEventArgs> handler)
        {
            var tokenResult = CancelToken.Create();
            if (!tokenResult.IsSuccess)
                return Result<IDisposable>.Failure(tokenResult.Info);
            return SubscribeGestureUpdatesCore(id, gestureKind, handler, tokenResult.Value, true);
        }

        /// <summary>
        /// Calls <paramref name="handler"/> for every gesture of one kind that a peer performs
        /// against one app volume, until the result is disposed.
        /// </summary>
        /// <remarks>
        /// <para>
        /// Each gesture is already mapped through the volume's gesture configuration: filtered
        /// by its rules, its
        /// points in app content space, its deltas in app volume space. When
        /// <see cref="AppVolumeGestureEvent.WasMappedToAppTransform"/> is true the library
        /// already applied the delta to the app pose or scale; applying it again moves the
        /// content twice.
        /// </para>
        /// <para>
        /// Several producers publish, one per user who can reach the volume. Order holds within
        /// one producer, not across producers, and a source peer never seen before is a new user.
        /// Keep per-user state per <see cref="AppVolumeGestureEvent.SourcePeerId"/>. Separate
        /// per-kind subscriptions have no order relative to each other; to take more than one
        /// kind, prefer <see cref="SubscribeAllGestureUpdates"/>.
        /// </para>
        /// <para>
        /// Threading, exceptions and disposal follow
        /// <see cref="SubscribeUpdates(EventHandler{AppVolumeUpdatedEventArgs}, CancellationToken)"/>:
        /// the handler runs on a library thread, and <c>Dispose</c> waits for a running call.
        /// </para>
        /// </remarks>
        /// <exception cref="ArgumentNullException"><paramref name="handler"/> is null.</exception>
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
            return _context.StartSubscription(
                subscription,
                (started, session, token) => NativeMethods.qar_impl_app_volumes_subscribe_gesture_events(session, ref nativeId, gestureKind, started.Callback, started.UserState, token));
        }

        private Result<IDisposable> SubscribeGestureUpdatesCore(AppVolumeId id, GestureKind gestureKind, EventHandler<AppVolumeGestureEventArgs> handler, CancellationToken cancellationToken)
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            _context.ThrowIfDisposed();
            var nativeId = id.ToNative();
            return _context.StartSubscription(
                cancellationToken,
                token => AppVolumeGestureSubscription.Create(token, true, handler),
                (token, registration) => AppVolumeGestureSubscription.Create(token, registration, handler),
                (started, session, token) => NativeMethods.qar_impl_app_volumes_subscribe_gesture_events(session, ref nativeId, gestureKind, started.Callback, started.UserState, token));
        }

        private static ResultNative StartUpdates(AppVolumeUpdateSubscription subscription, IntPtr session, IntPtr token)
        {
            return NativeMethods.qar_impl_app_volumes_subscribe_updates(session, subscription.Callback, subscription.UserState, token);
        }

#if NET8_0_OR_GREATER
        /// <summary>
        /// The gestures of
        /// <see cref="SubscribeGestureUpdates(AppVolumeId, GestureKind, EventHandler{AppVolumeGestureEventArgs}, CancellationToken)"/>
        /// as an async stream.
        /// </summary>
        /// <remarks>
        /// The subscription starts with the enumeration and ends when the enumerator is disposed.
        /// Gestures are queued without bound until read.
        /// </remarks>
        /// <exception cref="NativeCallException">
        /// Thrown on the first <c>MoveNextAsync</c> when the subscription cannot be started.
        /// </exception>
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
