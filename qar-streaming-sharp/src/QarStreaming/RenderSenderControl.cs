using System;
using System.Threading;
using System.Threading.Tasks;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    /// <summary>
    /// Creates render senders and delivers render stream requests for a session; reached through
    /// <see cref="Session.RenderSenders"/>.
    /// </summary>
    public sealed class RenderSenderControl
    {
        private readonly SessionNativeContext _context;

        internal RenderSenderControl(SessionNativeContext context)
        {
            _context = context ?? throw new ArgumentNullException(nameof(context));
        }

        /// <summary>
        /// Creates a sender that renders for <see cref="RenderSenderOptions.PeerId"/> into the app
        /// volume <see cref="RenderSenderOptions.AppVolumeId"/>, which must already exist.
        /// </summary>
        /// <remarks>
        /// <para>
        /// The sender attaches its target to the app volume, which makes the target request the
        /// content, and the call waits for that request: it does not return until the target
        /// asks for the stream. Bound the wait with <paramref name="cancellationToken"/>, or use
        /// <see cref="CreateAsync(RenderSenderOptions, CancellationToken)"/>.
        /// </para>
        /// <para>
        /// Create one sender per target served; which targets to serve is the application's
        /// decision. A <see cref="PeerId.Unique"/> target belongs to no peer, so the stream never
        /// negotiates. A missing app volume id fails with
        /// <see cref="StatusCode.AppVolumeInvalidId"/>.
        /// </para>
        /// </remarks>
        /// <exception cref="ArgumentNullException"><paramref name="options"/> is null.</exception>
        public Result<RenderSender> Create(RenderSenderOptions options, CancellationToken cancellationToken = default(CancellationToken))
        {
            if (options == null)
                throw new ArgumentNullException(nameof(options));
            var cancellationResult = NativeCancellationScope.Create(cancellationToken, _context.Owner);
            if (!cancellationResult.IsSuccess)
                return Result<RenderSender>.Failure(cancellationResult.Info);
            using (var cancellation = cancellationResult.Value)
            using (var scope = new NativeMemoryScope())
            {
                var native = NativeInit.RenderSender(options, scope);
                IntPtr stream;
                using var session = _context.Borrow();
                var info = Results.ToInfo(NativeMethods.qar_impl_render_sender_create(session.Handle, ref native, cancellation.Handle, out stream));
                return Results.FromHandle(info, stream, h => new RenderSender(new RenderSenderSafeHandle(h)));
            }
        }

        /// <summary>Async <see cref="Create"/>; completes once the target has requested the stream.</summary>
        /// <exception cref="ArgumentNullException"><paramref name="options"/> is null.</exception>
        public Task<Result<RenderSender>> CreateAsync(RenderSenderOptions options, CancellationToken cancellationToken = default(CancellationToken))
        {
            if (options == null)
                throw new ArgumentNullException(nameof(options));
            _context.ThrowIfDisposed();
            var state = new RenderSenderCreateAsyncState();
            var cancelHandle = state.UseStandardCancellation(cancellationToken, _context.Owner);
            if (!cancelHandle.IsSuccess)
            {
                state.Complete(Result<RenderSender>.Failure(cancelHandle.Info));
                return state.Task;
            }
            return CreateAsync(options, state, cancelHandle.Value);
        }

        private Task<Result<RenderSender>> CreateAsync(RenderSenderOptions options, RenderSenderCreateAsyncState state, IntPtr cancelHandle)
        {
            try
            {
                var native = state.CreateNative(options);
                using var session = _context.Borrow();
                state.HoldUntilCompletion(_context.Owner);
                var info = Results.ToInfo(NativeMethods.qar_impl_render_sender_create_async(session.Handle, ref native, state.Callback, state.UserState, cancelHandle));
                if (!info.IsSuccess)
                    state.Complete(Result<RenderSender>.Failure(info));
            }
            catch (Exception ex)
            {
                state.Fault(ex);
            }
            return state.Task;
        }

        /// <summary>
        /// Raises <paramref name="handler"/> for each render stream request for this
        /// application's content.
        /// </summary>
        /// <remarks>
        /// <para>
        /// A target requests the content only once it is attached to the application's app
        /// volume, so this does not discover targets for a new application: find those in the
        /// peer list and create a sender for each one served, which attaches it. It sees
        /// requests for a volume something else attached a target to. Repeated requests for a
        /// target already served need no further sender.
        /// </para>
        /// <para>
        /// The handler runs on a native thread: hand the target id to the render thread with
        /// proper synchronization. Exceptions thrown by the handler are swallowed. Disposing the
        /// returned subscription, or cancelling <paramref name="cancellationToken"/>, ends it;
        /// once <c>Dispose</c> returns no handler call is running on another thread and none
        /// starts.
        /// </para>
        /// </remarks>
        /// <exception cref="ArgumentNullException"><paramref name="handler"/> is null.</exception>
        public Result<IDisposable> SubscribeRequests(EventHandler<RenderStreamRequestEventArgs> handler, CancellationToken cancellationToken = default(CancellationToken))
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            return _context.StartSubscription(
                cancellationToken,
                token => RenderRequestSubscription.Create(token, true, handler),
                (token, registration) => RenderRequestSubscription.Create(token, registration, handler),
                StartRequests);
        }

        private static ResultNative StartRequests(RenderRequestSubscription subscription, IntPtr session, IntPtr token)
        {
            return NativeMethods.qar_impl_render_sender_subscribe_requests(session, subscription.Callback, subscription.UserState, token);
        }
    }
}
