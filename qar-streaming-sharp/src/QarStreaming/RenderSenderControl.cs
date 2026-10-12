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
            var cancellationResult = NativeCancellationScope.Create(cancellationToken, false);
            if (!cancellationResult.IsSuccess)
                return Result<RenderSender>.Failure(cancellationResult.Info);
            using (var scope = new NativeMemoryScope())
            using (var cancellation = cancellationResult.Value)
            {
                var native = NativeInit.RenderSender(options, scope);
                IntPtr stream;
                var info = Results.ToInfo(NativeMethods.qar_impl_render_sender_create(_context.DangerousHandle, ref native, cancellation.Handle, out stream));
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
            var cancelHandle = state.UseStandardCancellation(cancellationToken);
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
                var info = Results.ToInfo(NativeMethods.qar_impl_render_sender_create_async(_context.DangerousHandle, ref native, state.Callback, state.UserState, cancelHandle));
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
            _context.ThrowIfDisposed();
            var tokenResult = CancelToken.Create();
            if (!tokenResult.IsSuccess)
                return Result<IDisposable>.Failure(tokenResult.Info);
            var token = tokenResult.Value;
            CancellationTokenRegistration registration = default(CancellationTokenRegistration);
            if (cancellationToken.CanBeCanceled)
            {
                if (cancellationToken.IsCancellationRequested)
                    token.Cancel();
                else
                    registration = cancellationToken.Register(state => ((CancelToken)state!).Cancel(), token);
            }
            var subscription = cancellationToken.CanBeCanceled
                ? RenderRequestSubscription.Create(token, registration, handler)
                : RenderRequestSubscription.Create(token, false, handler);
            var info = Results.ToInfo(NativeMethods.qar_impl_render_sender_subscribe_requests(_context.DangerousHandle, subscription.Callback, subscription.UserState, token.DangerousHandle));
            return Results.SubscriptionResult(info, subscription);
        }
    }
}
