using System;
using System.Threading;
using System.Threading.Tasks;
using Qar.Native;

namespace Qar
{
    public sealed class RenderSenderControl
    {
        private readonly SessionNativeContext _context;

        internal RenderSenderControl(SessionNativeContext context)
        {
            _context = context ?? throw new ArgumentNullException(nameof(context));
        }

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
