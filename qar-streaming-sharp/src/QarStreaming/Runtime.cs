using System;
using System.Threading;
using System.Threading.Tasks;
using Qar.Native;

namespace Qar
{
    public sealed class Runtime : IDisposable
    {
        private readonly RuntimeSafeHandle _handle;

        internal Runtime(RuntimeSafeHandle handle)
        {
            _handle = handle ?? throw new ArgumentNullException(nameof(handle));
        }

        internal IntPtr DangerousHandle { get { ThrowIfDisposed(); return _handle.DangerousGetHandle(); } }

        public static Result<Runtime> Create(RuntimeOptions? options = null)
        {
            options = options ?? new RuntimeOptions();
            using (var scope = new NativeMemoryScope())
            using (var chain = NativeStructureChain.From(options.Extension))
            {
                var native = new RuntimeInitNative
                {
                    Header = NativeDefaults.Header(StructureType.RuntimeInit, chain.Head),
                    RuntimeBinariesFolderPath = scope.AllocString(options.RuntimeBinariesFolderPath),
                    StorageFolderPath = scope.AllocString(options.StorageFolderPath)
                };
                IntPtr handle;
                var info = Results.ToInfo(NativeMethods.qar_impl_runtime_create(ref native, out handle));
                return Results.FromHandle(info, handle, h => new Runtime(new RuntimeSafeHandle(h)));
            }
        }

        public Result<Session> Rejoin(RejoinOptions options, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default(CancellationToken))
        {
            if (options == null)
                throw new ArgumentNullException(nameof(options));
            ThrowIfDisposed();
            var cancellationResult = NativeCancellationScope.Create(cancellationToken, false);
            if (!cancellationResult.IsSuccess)
                return Result<Session>.Failure(cancellationResult.Info);
            using (var scope = new NativeMemoryScope())
            using (var chain = NativeStructureChain.From(options.Extension))
            using (var presentationChain = NativeStructureChain.From(options.Presentation == null ? null : options.Presentation.Extension))
            using (var cancellation = cancellationResult.Value)
            using (var progressState = ProgressState.Create(progress))
            {
                var native = NativeInit.Rejoin(options, scope, chain.Head, presentationChain.Head);
                IntPtr session;
                var info = Results.ToInfo(NativeMethods.qar_impl_runtime_rejoin(
                    _handle.DangerousGetHandle(),
                    ref native,
                    progressState.Callback,
                    progressState.UserState,
                    cancellation.Handle,
                    out session));
                return Results.FromHandle(info, session, h => new Session(new SessionSafeHandle(h)));
            }
        }

        public Task<Result<Session>> RejoinAsync(RejoinOptions options, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default(CancellationToken))
        {
            if (options == null)
                throw new ArgumentNullException(nameof(options));
            ThrowIfDisposed();
            var state = new RuntimeOnboardAsyncState(progress, false);
            var cancelHandle = state.UseStandardCancellation(cancellationToken);
            if (!cancelHandle.IsSuccess)
            {
                state.CompleteRejoin(Result<Session>.Failure(cancelHandle.Info));
                return state.SessionTask;
            }
            return RejoinAsync(options, state, cancelHandle.Value);
        }

        private Task<Result<Session>> RejoinAsync(RejoinOptions options, RuntimeOnboardAsyncState state, IntPtr cancelHandle)
        {
            try
            {
                var native = state.CreateNative(options);
                var info = Results.ToInfo(NativeMethods.qar_impl_runtime_rejoin_async(
                    _handle.DangerousGetHandle(),
                    ref native,
                    state.ResultCallback,
                    state.ProgressCallback,
                    state.UserState,
                    cancelHandle));
                if (!info.IsSuccess)
                    state.CompleteRejoin(Result<Session>.Failure(info));
            }
            catch (Exception ex)
            {
                state.FaultRejoin(ex);
            }
            return state.SessionTask;
        }

        public Result<OnboardSession> Onboard(OnboardOptions options, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default(CancellationToken))
        {
            if (options == null)
                throw new ArgumentNullException(nameof(options));
            if (options.Mode == null)
                throw new ArgumentNullException(nameof(options.Mode));
            ThrowIfDisposed();
            var cancellationResult = NativeCancellationScope.Create(cancellationToken, false);
            if (!cancellationResult.IsSuccess)
                return Result<OnboardSession>.Failure(cancellationResult.Info);
            using (var scope = new NativeMemoryScope())
            using (var chain = NativeStructureChain.From(options.Mode, options.Extension))
            using (var presentationChain = NativeStructureChain.From(options.Presentation == null ? null : options.Presentation.Extension))
            using (var cancellation = cancellationResult.Value)
            using (var progressState = ProgressState.Create(progress))
            {
                var native = NativeInit.Onboard(options, scope, chain.Head, presentationChain.Head);
                OnboardingIdNative onboardingId;
                IntPtr session;
                var info = Results.ToInfo(NativeMethods.qar_impl_runtime_onboard(
                    _handle.DangerousGetHandle(),
                    ref native,
                    progressState.Callback,
                    progressState.UserState,
                    cancellation.Handle,
                    out onboardingId,
                    out session));
                if (!info.IsSuccess)
                    return Result<OnboardSession>.Failure(info);
                if (session == IntPtr.Zero)
                    return Result<OnboardSession>.Failure(new ResultInfo(StatusCode.LogicError, 0, "Native call succeeded but returned a null session handle."));
                return Result<OnboardSession>.Success(info, new OnboardSession(OnboardingId.FromNative(onboardingId), new Session(new SessionSafeHandle(session))));
            }
        }

        public Task<Result<OnboardSession>> OnboardAsync(OnboardOptions options, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default(CancellationToken))
        {
            if (options == null)
                throw new ArgumentNullException(nameof(options));
            if (options.Mode == null)
                throw new ArgumentNullException(nameof(options.Mode));
            ThrowIfDisposed();
            var state = new RuntimeOnboardAsyncState(progress, true);
            var cancelHandle = state.UseStandardCancellation(cancellationToken);
            if (!cancelHandle.IsSuccess)
            {
                state.CompleteOnboard(Result<OnboardSession>.Failure(cancelHandle.Info));
                return state.OnboardTask;
            }
            return OnboardAsync(options, state, cancelHandle.Value);
        }

        private Task<Result<OnboardSession>> OnboardAsync(OnboardOptions options, RuntimeOnboardAsyncState state, IntPtr cancelHandle)
        {
            try
            {
                var native = state.CreateNative(options);
                var info = Results.ToInfo(NativeMethods.qar_impl_runtime_onboard_async(
                    _handle.DangerousGetHandle(),
                    ref native,
                    state.ResultCallback,
                    state.ProgressCallback,
                    state.UserState,
                    cancelHandle));
                if (!info.IsSuccess)
                    state.CompleteOnboard(Result<OnboardSession>.Failure(info));
            }
            catch (Exception ex)
            {
                state.Fault(ex);
            }
            return state.OnboardTask;
        }

        public Result<Unit> Forget(ForgetOptions options)
        {
            if (options == null)
                throw new ArgumentNullException(nameof(options));
            ThrowIfDisposed();
            using (var chain = NativeStructureChain.From(options.Extension))
            {
                var native = NativeInit.Forget(options, chain.Head);
                return Results.FromNative(NativeMethods.qar_impl_runtime_forget(_handle.DangerousGetHandle(), ref native));
            }
        }

        public void Dispose()
        {
            _handle.Dispose();
        }

        private void ThrowIfDisposed()
        {
            if (_handle.IsClosed || _handle.IsInvalid)
                throw new ObjectDisposedException(nameof(Runtime));
        }
    }
}
