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
    /// The session's GUI panels: shared 2D surfaces in the room showing a web page or a video
    /// stream. Reached through <see cref="Session.GuiPanels"/>.
    /// </summary>
    /// <remarks>
    /// <para>
    /// Panel state is shared session state and eventually consistent. A change method returns
    /// success once it has published the change; the change reaches the shared state, and every
    /// peer's snapshots, later, and may still be refused there. A read right after a write can
    /// return the old value: observe the result through
    /// <see cref="SubscribeUpdates(EventHandler{GuiPanelUpdatedEventArgs}, CancellationToken)"/>
    /// instead of reading back once. A change to a panel this session does not know yet fails
    /// rather than waiting for it.
    /// </para>
    /// <para>
    /// Every method throws <see cref="ObjectDisposedException"/> once the session is disposed.
    /// </para>
    /// </remarks>
    public sealed class GuiPanelControl
    {
        private readonly SessionNativeContext _context;

        internal GuiPanelControl(SessionNativeContext context)
        {
            _context = context ?? throw new ArgumentNullException(nameof(context));
        }

        /// <summary>
        /// Raised for every panel update, but only while a subscription started by one of the
        /// <c>SubscribeUpdates</c> overloads without a handler is alive.
        /// </summary>
        /// <remarks>
        /// Raised on a library thread with this control as the sender; see
        /// <see cref="SubscribeUpdates(EventHandler{GuiPanelUpdatedEventArgs}, CancellationToken)"/>
        /// for what is delivered and the threading rules. Each such subscription raises the event
        /// once per update it delivers, so overlapping subscriptions raise it more than once.
        /// </remarks>
        public event EventHandler<GuiPanelUpdatedEventArgs>? Updated;

        /// <summary>
        /// Returns the panel identified by <see cref="GuiPanelOptions.CommonName"/>,
        /// creating it from <paramref name="options"/> if it does not exist yet.
        /// </summary>
        /// <remarks>
        /// <para>
        /// The id is derived from the common name alone, so the same common name gives the
        /// same panel in every session, from every peer and after a restart: call this on every
        /// start instead of persisting the id. The common name is the identity key and must be
        /// ASCII; put user-facing text, which may be any UTF-8, in
        /// <see cref="GuiPanelOptions.DisplayName"/>. Set a non-empty DisplayName when
        /// creating or reopening a panel; CommonName does not supply a default label.
        /// </para>
        /// <para>
        /// When an open panel of that name exists, the rest of <paramref name="options"/> is
        /// ignored. A closed panel is reopened, visible, with <paramref name="options"/>.
        /// </para>
        /// <para>
        /// An existing panel is taken over by this session (a previous run of the app may have
        /// left it) and carried into the hub's next session like one this session created. Call
        /// GetOrCreate again for every common name the app owns when the session id changes and
        /// after onboarding again on <see cref="SessionLifecycleState.RePairRequired"/>
        /// (<see cref="Session.SubscribeLifecycle(EventHandler{SessionLifecycleChangedEventArgs})"/>):
        /// it returns the same id and re-creates the panel when it is missing.
        /// </para>
        /// <para>
        /// The content is a web page (<see cref="GuiPanelOptions.ContentUri"/>) or a video stream
        /// (<see cref="GuiPanelOptions.ContentStreamId"/>, a video sender owned by this session)
        /// and cannot change type later; with neither the panel shows the default web page. A
        /// panel meant only for some peers is best created with them in
        /// <see cref="GuiPanelOptions.VisibleToPeers"/> rather than narrowed afterwards.
        /// </para>
        /// </remarks>
        /// <exception cref="ArgumentNullException"><paramref name="options"/> is null.</exception>
        /// <exception cref="ArgumentException">
        /// <see cref="GuiPanelOptions.CommonName"/> is null, or both a URI and a stream id are set.
        /// </exception>
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
                using var session = _context.Borrow();
                var info = Results.ToInfo(NativeMethods.qar_impl_gui_panels_get_or_create(session.Handle, ref native, ref id));
                return info.IsSuccess ? Result<GuiPanelId>.Success(info, GuiPanelId.FromNative(id)) : Result<GuiPanelId>.Failure(info);
            }
        }

        /// <summary>
        /// Subscribe to one kind of gesture the rendering peers report for one
        /// panel. Several producers publish, one per user: key lifecycles on
        /// (source peer, gesture id). Disposing the result stops delivery.
        /// </summary>
        /// <remarks>
        /// <para>
        /// The peer that draws the panel recognises hover, tap, long press and drag from its
        /// user's hands; positions arrive in panel-content-space pixels. Every panel reports
        /// gestures, web panels included.
        /// </para>
        /// <para>
        /// The gesture id is unique only within a source peer: two users' first drags both carry
        /// id 1. Order holds within one producer, not across producers, and a source peer never
        /// seen before is a new user. A live lifecycle is republished every 250 ms even when
        /// nothing changed; treat one silent for three intervals as cancelled, which is what
        /// recovers from a crashed producer. A missed started phase is recoverable because
        /// every update carries its start position.
        /// </para>
        /// <para>
        /// Separate per-kind subscriptions have no order relative to each other; to take more
        /// than one kind, prefer <see cref="SubscribeAllGestureEvents"/>. The handler runs on a
        /// library thread and an exception it throws is dropped. Disposal waits for a running
        /// handler call as described for
        /// <see cref="SubscribeUpdates(EventHandler{GuiPanelUpdatedEventArgs}, CancellationToken)"/>.
        /// The default all-zero id fails with <see cref="StatusCode.GuiPanelInvalidId"/>.
        /// </para>
        /// </remarks>
        /// <exception cref="ArgumentNullException"><paramref name="handler"/> is null.</exception>
        public Result<IDisposable> SubscribeGestureEvents(GuiPanelId id, GuiPanelGestureKind gestureKind, EventHandler<GuiPanelGestureEventArgs> handler, CancellationToken cancellationToken = default(CancellationToken))
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            var nativeId = id.ToNative();
            return _context.StartSubscription(
                cancellationToken,
                token => GuiPanelGestureSubscription.Create(token, true, handler),
                (token, registration) => GuiPanelGestureSubscription.Create(token, registration, handler),
                (subscription, session, token) => NativeMethods.qar_impl_gui_panels_subscribe_gesture_events(session, ref nativeId, gestureKind, subscription.Callback, subscription.UserState, token));
        }

        /// <summary>
        /// Subscribe to every gesture kind on one panel through one handler. Prefer
        /// this when taking more than one kind: everything arrives in the order
        /// the producer published it.
        /// </summary>
        /// <remarks>
        /// A hover and the drag that grows out of the same contact keep their order here. Dispatch
        /// on <see cref="GuiPanelGestureEvent.Kind"/>. Everything else - several producers,
        /// lifecycle keys, threading and disposal - is as for
        /// <see cref="SubscribeGestureEvents"/>.
        /// </remarks>
        /// <exception cref="ArgumentNullException"><paramref name="handler"/> is null.</exception>
        public Result<IDisposable> SubscribeAllGestureEvents(GuiPanelId id, EventHandler<GuiPanelGestureEventArgs> handler, CancellationToken cancellationToken = default(CancellationToken))
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            var nativeId = id.ToNative();
            return _context.StartSubscription(
                cancellationToken,
                token => GuiPanelGestureSubscription.Create(token, true, handler),
                (token, registration) => GuiPanelGestureSubscription.Create(token, registration, handler),
                (subscription, session, token) => NativeMethods.qar_impl_gui_panels_subscribe_all_gesture_events(session, ref nativeId, subscription.Callback, subscription.UserState, token));
        }

        /// <summary>Moves a panel. <paramref name="pose"/> is in room space.</summary>
        /// <remarks>
        /// The pose is the midpoint of the panel's top edge, not its centre, with the front
        /// normal along -Z. A fast-path change that this session's <see cref="GetAll"/> shows at
        /// once.
        /// </remarks>
        public Result<Unit> UpdatePose(GuiPanelId id, Pose pose)
        {
            var nativeId = id.ToNative();
            var nativePose = pose.ToNative();
            using var session = _context.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_gui_panels_update_pose(session.Handle, ref nativeId, ref nativePose));
        }

        /// <summary>
        /// Changes the panel's size in metres and its content scale. A fast-path change that
        /// this session's <see cref="GetAll"/> shows at once.
        /// </summary>
        public Result<Unit> ChangeSize(GuiPanelId id, GuiPanelSize size)
        {
            var nativeId = id.ToNative();
            var nativeSize = size.ToNative();
            using var session = _context.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_gui_panels_change_size(session.Handle, ref nativeId, ref nativeSize));
        }

        /// <summary>
        /// Shows, minimizes or hides the panel for every peer. This, not the visible-to list,
        /// is how a panel is hidden. <see cref="GuiPanelState.Closed"/> is the same as
        /// <see cref="Close"/>.
        /// </summary>
        public Result<Unit> SetState(GuiPanelId id, GuiPanelState state)
        {
            var nativeId = id.ToNative();
            using var session = _context.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_gui_panels_set_state(session.Handle, ref nativeId, state));
        }

        /// <summary>
        /// Closes the panel for every peer: its <see cref="GuiPanel.State"/> becomes
        /// <see cref="GuiPanelState.Closed"/>. <see cref="GetOrCreate"/> with the same common
        /// name reopens it.
        /// </summary>
        public Result<Unit> Close(GuiPanelId id)
        {
            var nativeId = id.ToNative();
            using var session = _context.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_gui_panels_close_panel(session.Handle, ref nativeId));
        }

        /// <summary>
        /// Points a web panel at another URI. A stream panel fails with
        /// <see cref="StatusCode.GuiPanelChangeNotPermitted"/>: content type is fixed at creation.
        /// </summary>
        public Result<Unit> NavigateToUri(GuiPanelId id, string uri)
        {
            var nativeId = id.ToNative();
            using (var nativeUri = NativeUtf8String.FromUtf8(uri))
            using (var session = _context.Borrow())
                return Results.FromNative(NativeMethods.qar_impl_gui_panels_navigate_to_uri(session.Handle, ref nativeId, nativeUri.Pointer));
        }

        /// <summary>
        /// Switches a stream panel to another video stream, which must be a video sender owned
        /// by this session. A web panel fails with
        /// <see cref="StatusCode.GuiPanelChangeNotPermitted"/>: content type is fixed at creation.
        /// </summary>
        public Result<Unit> SetStream(GuiPanelId id, StreamId streamId)
        {
            var nativeId = id.ToNative();
            var nativeStreamId = streamId.ToNative();
            using var session = _context.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_gui_panels_set_stream(session.Handle, ref nativeId, ref nativeStreamId));
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
                using var session = _context.Borrow();
                return Results.FromNative(NativeMethods.qar_impl_gui_panels_update_visible_to(session.Handle, ref nativeId, addPtr, (UIntPtr)(additions == null ? 0 : additions.Length), removePtr, (UIntPtr)(removals == null ? 0 : removals.Length)));
            }
        }

        /// <summary>
        /// Snapshot of every panel the session's shared storage holds, with this session's own
        /// pending pose and size changes applied. Blocks for a network round trip of about 2 s
        /// on every call.
        /// </summary>
        /// <remarks>
        /// Use it for a one-off listing. To track panels, subscribe with
        /// <see cref="SubscribeUpdates(EventHandler{GuiPanelUpdatedEventArgs}, CancellationToken)"/>,
        /// which delivers the same snapshot first.
        /// </remarks>
        public Result<GuiPanel[]> GetAll()
        {
            return _context.QueryHandles(
                NativeMethods.qar_impl_query_gui_panels,
                handle => GuiPanel.FromHandle(handle, true));
        }

        /// <summary>
        /// Starts delivering updates of every panel through the <see cref="Updated"/> event until
        /// the result is disposed.
        /// </summary>
        /// <remarks>
        /// Same delivery and disposal rules as
        /// <see cref="SubscribeUpdates(EventHandler{GuiPanelUpdatedEventArgs}, CancellationToken)"/>.
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

        /// <inheritdoc cref="SubscribeUpdates(EventHandler{GuiPanelUpdatedEventArgs}, CancellationToken)"/>
        public Result<IDisposable> SubscribeUpdates(EventHandler<GuiPanelUpdatedEventArgs> handler)
        {
            var tokenResult = CancelToken.Create();
            if (!tokenResult.IsSuccess)
                return Result<IDisposable>.Failure(tokenResult.Info);
            return SubscribeUpdatesCore(handler, tokenResult.Value, true);
        }

        /// <summary>
        /// Calls <paramref name="handler"/> once for every panel already stored, then again for
        /// every change to any panel, until the result is disposed.
        /// </summary>
        /// <remarks>
        /// <para>
        /// Each call carries an immutable <see cref="GuiPanel"/> snapshot that is safe to keep.
        /// Its pose is in the publishing peer's room space. Changes come from any peer or app,
        /// including a user grabbing the panel in the room.
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
            return _context.StartSubscription(subscription, StartUpdates);
        }

        private Result<IDisposable> SubscribeUpdatesCore(EventHandler<GuiPanelUpdatedEventArgs> handler, CancellationToken cancellationToken)
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            _context.ThrowIfDisposed();
            return _context.StartSubscription(
                cancellationToken,
                token => GuiPanelUpdateSubscription.Create(token, true, handler),
                (token, registration) => GuiPanelUpdateSubscription.Create(token, registration, handler),
                StartUpdates);
        }

#if NET8_0_OR_GREATER
        /// <summary>
        /// The snapshots of
        /// <see cref="SubscribeUpdates(EventHandler{GuiPanelUpdatedEventArgs}, CancellationToken)"/>
        /// as an async stream: every stored panel first, then every change.
        /// </summary>
        /// <remarks>
        /// The subscription starts with the enumeration and ends when the enumerator is disposed.
        /// Updates are queued without bound until read.
        /// </remarks>
        /// <exception cref="NativeCallException">
        /// Thrown on the first <c>MoveNextAsync</c> when the subscription cannot be started.
        /// </exception>
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

        /// <summary>
        /// Starts delivering one panel's updates through the <see cref="Updated"/> event until
        /// the result is disposed.
        /// </summary>
        /// <remarks>
        /// Same delivery and disposal rules as
        /// <see cref="SubscribeUpdates(GuiPanelId, EventHandler{GuiPanelUpdatedEventArgs}, CancellationToken)"/>.
        /// </remarks>
        public Result<IDisposable> SubscribeUpdates(GuiPanelId id)
        {
            return SubscribeUpdates(id, RaiseUpdated);
        }

        /// <inheritdoc cref="SubscribeUpdates(GuiPanelId)"/>
        public Result<IDisposable> SubscribeUpdates(GuiPanelId id, CancellationToken cancellationToken)
        {
            return SubscribeUpdates(id, RaiseUpdated, cancellationToken);
        }

        /// <inheritdoc cref="SubscribeUpdates(GuiPanelId, EventHandler{GuiPanelUpdatedEventArgs}, CancellationToken)"/>
        public Result<IDisposable> SubscribeUpdates(GuiPanelId id, EventHandler<GuiPanelUpdatedEventArgs> handler)
        {
            var tokenResult = CancelToken.Create();
            if (!tokenResult.IsSuccess)
                return Result<IDisposable>.Failure(tokenResult.Info);
            return SubscribeUpdatesCore(id, handler, tokenResult.Value, true);
        }

        /// <summary>
        /// Calls <paramref name="handler"/> for one panel: first with its stored state, when it
        /// already exists, then for every change to it, until the result is disposed.
        /// </summary>
        /// <remarks>
        /// Snapshots, threading, exceptions and disposal are as for
        /// <see cref="SubscribeUpdates(EventHandler{GuiPanelUpdatedEventArgs}, CancellationToken)"/>.
        /// </remarks>
        /// <exception cref="ArgumentNullException"><paramref name="handler"/> is null.</exception>
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
            return _context.StartSubscription(
                subscription,
                (started, session, token) => NativeMethods.qar_impl_gui_panels_subscribe_panel_updates(session, ref nativeId, started.Callback, started.UserState, token));
        }

        private Result<IDisposable> SubscribeUpdatesCore(GuiPanelId id, EventHandler<GuiPanelUpdatedEventArgs> handler, CancellationToken cancellationToken)
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            _context.ThrowIfDisposed();
            var nativeId = id.ToNative();
            return _context.StartSubscription(
                cancellationToken,
                token => GuiPanelUpdateSubscription.Create(token, true, handler),
                (token, registration) => GuiPanelUpdateSubscription.Create(token, registration, handler),
                (started, session, token) => NativeMethods.qar_impl_gui_panels_subscribe_panel_updates(session, ref nativeId, started.Callback, started.UserState, token));
        }

        private static ResultNative StartUpdates(GuiPanelUpdateSubscription subscription, IntPtr session, IntPtr token)
        {
            return NativeMethods.qar_impl_gui_panels_subscribe_updates(session, subscription.Callback, subscription.UserState, token);
        }

#if NET8_0_OR_GREATER
        /// <summary>
        /// The snapshots of
        /// <see cref="SubscribeUpdates(GuiPanelId, EventHandler{GuiPanelUpdatedEventArgs}, CancellationToken)"/>
        /// as an async stream.
        /// </summary>
        /// <remarks>
        /// The subscription starts with the enumeration and ends when the enumerator is disposed.
        /// Updates are queued without bound until read.
        /// </remarks>
        /// <exception cref="NativeCallException">
        /// Thrown on the first <c>MoveNextAsync</c> when the subscription cannot be started.
        /// </exception>
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
                    Url = scope.AllocUtf8(options.ContentUri)
                });
            }
            return next;
        }
    }
}
