using System;
using System.Threading;
using System.Threading.Tasks;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    /// <summary>
    /// Hosts the identity slots a process onboards with and produces its sessions. Create it after
    /// <see cref="Library.Init"/>; every <see cref="Session"/> comes from <see cref="Onboard"/> or
    /// <see cref="Rejoin"/> on it.
    /// </summary>
    /// <remarks>
    /// The canonical startup: try <see cref="Rejoin"/> with the persisted
    /// <see cref="OnboardingId"/>; on <see cref="StatusCode.OnboardingSessionNotFound"/> run
    /// <see cref="Onboard"/> and persist the id it returns. Dispose sessions before the runtime,
    /// and the runtime before <see cref="Library.Destroy"/>.
    /// </remarks>
    public sealed class Runtime : IDisposable
    {
        private readonly RuntimeSafeHandle _handle;

        internal Runtime(RuntimeSafeHandle handle)
        {
            _handle = handle ?? throw new ArgumentNullException(nameof(handle));
        }

        internal IntPtr DangerousHandle { get { ThrowIfDisposed(); return _handle.DangerousGetHandle(); } }

        /// <summary>
        /// Creates the runtime. Set <see cref="RuntimeOptions.RuntimeBinariesFolderPath"/> to the
        /// SDK <c>bin</c> folder the native library was loaded from: left empty, sessions still
        /// work but <see cref="Session.InviteTargetApp"/> never completes.
        /// </summary>
        public static Result<Runtime> Create(RuntimeOptions? options = null)
        {
            options = options ?? new RuntimeOptions();
            using (var scope = new NativeMemoryScope())
            using (var chain = NativeStructureChain.From(options.Extension))
            {
                var native = new RuntimeInitNative
                {
                    Header = NativeDefaults.Header(StructureType.RuntimeInit, chain.Head),
                    RuntimeBinariesFolderPath = scope.AllocUtf8(options.RuntimeBinariesFolderPath),
                    StorageFolderPath = scope.AllocUtf8(options.StorageFolderPath)
                };
                IntPtr handle;
                var info = Results.ToInfo(NativeMethods.qar_impl_runtime_create(ref native, out handle));
                return Results.FromHandle(info, handle, h => new Runtime(new RuntimeSafeHandle(h)));
            }
        }

        /// <summary>
        /// Restores the session last persisted for <see cref="RejoinOptions.OnboardingId"/>. Pops no
        /// UI, so call it optimistically on every start. Blocks until the restore validated or
        /// failed.
        /// </summary>
        /// <remarks>
        /// <para>
        /// The session keeps the peer id the onboard persisted. Presentation fields left null keep
        /// their persisted values.
        /// </para>
        /// <para>
        /// Expected failures: <see cref="StatusCode.OnboardingSessionNotFound"/> means this slot has
        /// nothing persisted here, fall back to <see cref="Onboard"/>.
        /// <see cref="StatusCode.ArgumentNotSupported"/> means the onboarding id is
        /// <see cref="OnboardingId.Default"/>.
        /// </para>
        /// <para>
        /// <paramref name="progress"/> is reported from a library thread while this call blocks.
        /// A <see cref="Progress{T}"/> created on a UI thread posts there, so do not block that
        /// thread on this call if you want to see the reports.
        /// </para>
        /// </remarks>
        /// <param name="options">The onboarding id to restore, and optional presentation and GPU.</param>
        /// <param name="progress">Optional progress reports.</param>
        /// <param name="cancellationToken">Cancels the restore; the call then returns a failure.</param>
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

        /// <summary>
        /// Async <see cref="Rejoin"/>: returns at once, and the task completes on a library thread
        /// with the same results.
        /// </summary>
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

        /// <summary>
        /// Runs full onboarding with the mode chosen by <see cref="OnboardOptions.UseCode"/> or
        /// <see cref="OnboardOptions.UseInvite"/>, and blocks until it succeeded or failed. Persist
        /// the returned <see cref="OnboardSession.OnboardingId"/>: it is the ticket for every later
        /// <see cref="Rejoin"/> and <see cref="Forget"/>.
        /// </summary>
        /// <remarks>
        /// <para>
        /// With <see cref="OnboardOptions.OnboardingId"/> unset a new identity slot is created; set
        /// to an id an earlier onboard returned, that slot is re-enrolled.
        /// </para>
        /// <para>
        /// Expected failures: <see cref="StatusCode.PakeError"/> means the code was wrong or
        /// expired, or the invite was rejected or expired: re-prompt for a code or mint a fresh
        /// invite. <see cref="StatusCode.OnboardingHubUnreachable"/> means no hub answered: check
        /// that it runs and pass its host and port with <see cref="OnboardOptions.WithHost"/>.
        /// <see cref="StatusCode.ArgumentNotSupported"/> means a zero
        /// <see cref="OnboardOptions.WithPeerId"/> id, a host port of 0, or a GPU the session
        /// cannot create its devices on.
        /// </para>
        /// <para>
        /// <paramref name="progress"/> is reported from a library thread while this call blocks.
        /// </para>
        /// </remarks>
        /// <param name="options">The onboarding mode and its optional extensions.</param>
        /// <param name="progress">Optional progress reports, for an onboarding UI.</param>
        /// <param name="cancellationToken">
        /// Cancels the onboarding; the call then returns a failure.
        /// </param>
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

        /// <summary>
        /// Async <see cref="Onboard"/>: returns at once and keeps the UI responsive; the task
        /// completes with the same results. Canceling <paramref name="cancellationToken"/> cancels
        /// the task.
        /// </summary>
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

        /// <summary>
        /// Erases everything stored for an identity slot: certificate, persisted peer id and session
        /// state. Afterwards only a full <see cref="Onboard"/> gets back in. Needs no session, so it
        /// also serves a "forget this hub" action while disconnected.
        /// </summary>
        /// <remarks>
        /// Fails with <see cref="StatusCode.OnboardingSessionStillActive"/> while a session of the
        /// slot is alive: dispose it first. A slot with nothing persisted fails with
        /// <see cref="StatusCode.OnboardingSessionNotFound"/>.
        /// </remarks>
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

        /// <summary>
        /// Releases this process's runtime handle. Dispose its sessions first.
        /// Does not shut down the shared Hub or other applications using it.
        /// </summary>
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
