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
        // Orders Dispose against the release of an async call's borrow (ReleaseAsyncBorrow).
        private readonly object _releaseGate = new object();
        private volatile bool _disposed;

        // Cancelled by Dispose: the native token of every call that may wait is linked to it, so a
        // Dispose ends the call instead of the call's borrow keeping the disposed runtime alive
        // (an async call's native completion would otherwise wait for the runtime's teardown).
        private readonly CancellationTokenSource _disposing = new CancellationTokenSource();

        internal Runtime(RuntimeSafeHandle handle)
        {
            _handle = handle ?? throw new ArgumentNullException(nameof(handle));
        }

        /// <summary>
        /// The native handle, referenced by <paramref name="scope"/> until it is disposed, so neither
        /// a concurrent <see cref="Dispose"/> nor the finalizer frees the runtime while a native call
        /// still uses it.
        /// </summary>
        /// <exception cref="ObjectDisposedException">The runtime is disposed.</exception>
        internal IntPtr BorrowHandle(NativeMemoryScope scope)
        {
            ThrowIfDisposed();
            return scope.Borrow(_handle);
        }

        /// <summary>
        /// Releases the borrow an async call held until its native completion, on that completion's
        /// thread. While the runtime is not disposed this only drops a reference: the caller's own
        /// <see cref="Dispose"/> releases the runtime later, on its thread. When it was disposed
        /// while the call ran, the borrow is the last reference and its release destroys the
        /// runtime, which joins the native task this completion runs on: it is handed to the thread
        /// pool instead. The gate keeps a concurrent Dispose from falling between the check and the
        /// release.
        /// </summary>
        internal void ReleaseAsyncBorrow(NativeMemoryScope borrow)
        {
            lock (_releaseGate)
            {
                if (!_disposed)
                {
                    borrow.Dispose();
                    return;
                }
            }
            ThreadPool.QueueUserWorkItem(DisposeBorrow, borrow);
        }

        private static void DisposeBorrow(object? borrow)
        {
            ((NativeMemoryScope)borrow!).Dispose();
        }

        /// <summary>
        /// Creates the runtime. Set <see cref="RuntimeOptions.RuntimeBinariesFolderPath"/> to the
        /// SDK <c>bin</c> folder the native library was loaded from: left empty, sessions still
        /// work but <see cref="Session.InviteTargetApp"/> never completes.
        /// </summary>
        public static Result<Runtime> Create(RuntimeOptions? options = null)
        {
            options = options ?? new RuntimeOptions();
            using (var scope = new NativeMemoryScope())
            using (var chain = NativeStructureChain.From(options.EphemeralStorage ? new RuntimeEphemeralStorage() : null, options.Extension))
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
        /// <see cref="OnboardingId.Default"/>, or the runtime was created with
        /// <see cref="RuntimeOptions.EphemeralStorage"/>, which persists no slot.
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
            var cancellationResult = NativeCancellationScope.Create(cancellationToken, _disposing.Token);
            if (!cancellationResult.IsSuccess)
                return Result<Session>.Failure(cancellationResult.Info);
            using (var cancellation = cancellationResult.Value)
            using (var scope = new NativeMemoryScope())
            using (var chain = NativeStructureChain.From(options.Extension))
            using (var presentationChain = NativeStructureChain.From(options.Presentation == null ? null : options.Presentation.Extension))
            using (var progressState = ProgressState.Create(progress))
            {
                var native = NativeInit.Rejoin(options, scope, chain.Head, presentationChain.Head);
                IntPtr session;
                var info = Results.ToInfo(NativeMethods.qar_impl_runtime_rejoin(
                    scope.Borrow(_handle),
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
            var cancelHandle = state.UseStandardCancellation(cancellationToken, _disposing.Token);
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
                // The call's own borrow: the completion may fire, and drop the state's borrow,
                // before the call returns (hub G56).
                using var call = new NativeMemoryScope();
                BorrowHandle(call);
                var info = Results.ToInfo(NativeMethods.qar_impl_runtime_rejoin_async(
                    state.BorrowRuntime(this),
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
        /// <see cref="OnboardOptions.WithPeerId"/> id, a host port of 0, an
        /// <see cref="OnboardLocalApp.Tag"/> the runtime refuses, an invalid
        /// <see cref="OnboardLocalApp.RemoteAppGrant"/>, or a GPU the session cannot create its
        /// devices on. <see cref="StatusCode.OnboardingApprovalDenied"/> with a remote-app grant
        /// means another executable already redeemed it.
        /// </para>
        /// <para>
        /// With <see cref="RuntimeOptions.EphemeralStorage"/> nothing is persisted and the returned
        /// id is <see cref="OnboardingId.Default"/>; so it is for <see cref="OnboardOptions.UseLocalApp"/>.
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
            var cancellationResult = NativeCancellationScope.Create(cancellationToken, _disposing.Token);
            if (!cancellationResult.IsSuccess)
                return Result<OnboardSession>.Failure(cancellationResult.Info);
            using (var cancellation = cancellationResult.Value)
            using (var scope = new NativeMemoryScope())
            using (var chain = NativeStructureChain.From(options.Mode, options.Extension))
            using (var presentationChain = NativeStructureChain.From(options.Presentation == null ? null : options.Presentation.Extension))
            using (var progressState = ProgressState.Create(progress))
            {
                var native = NativeInit.Onboard(options, scope, chain.Head, presentationChain.Head);
                OnboardingIdNative onboardingId;
                IntPtr session;
                var info = Results.ToInfo(NativeMethods.qar_impl_runtime_onboard(
                    scope.Borrow(_handle),
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
            var cancelHandle = state.UseStandardCancellation(cancellationToken, _disposing.Token);
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
                // The call's own borrow: the completion may fire, and drop the state's borrow,
                // before the call returns (hub G56).
                using var call = new NativeMemoryScope();
                BorrowHandle(call);
                var info = Results.ToInfo(NativeMethods.qar_impl_runtime_onboard_async(
                    state.BorrowRuntime(this),
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
        /// <see cref="StatusCode.OnboardingSessionNotFound"/>. A runtime created with
        /// <see cref="RuntimeOptions.EphemeralStorage"/> has no slots and fails with
        /// <see cref="StatusCode.ArgumentNotSupported"/>.
        /// </remarks>
        public Result<Unit> Forget(ForgetOptions options)
        {
            if (options == null)
                throw new ArgumentNullException(nameof(options));
            ThrowIfDisposed();
            using (var scope = new NativeMemoryScope())
            using (var chain = NativeStructureChain.From(options.Extension))
            {
                var native = NativeInit.Forget(options, chain.Head);
                return Results.FromNative(NativeMethods.qar_impl_runtime_forget(scope.Borrow(_handle), ref native));
            }
        }

        /// <summary>
        /// Member-hub invitation, on the primary hub's PC: asks the QAROS hub on this PC, the
        /// session's primary, for an invitation that joins the hub on another PC as its member. Carry
        /// its <see cref="MemberHubInvitation.Serialize()"/> bytes to your app on that PC, which calls
        /// <see cref="JoinHubAsMember"/>. Blocks until the hub answers. Needs no session.
        /// </summary>
        /// <remarks>
        /// The hub identifies this process from Windows and answers only an app it approved with
        /// "May bring in member hubs" and quota left; the quota is spent per hub admitted. It never
        /// prompts. The invitation is a secret bearer credential: send it only over a trusted channel
        /// and never log it. Expected failures: <see cref="StatusCode.OnboardingHubUnreachable"/>,
        /// <see cref="StatusCode.OnboardingApprovalDenied"/>,
        /// <see cref="StatusCode.MemberSponsorNotAllowed"/>,
        /// <see cref="StatusCode.MemberSponsorQuotaExceeded"/>,
        /// <see cref="StatusCode.ArgumentNotSupported"/> (an option out of range, or a
        /// <see cref="MemberHubInvitationOptions.AppTag"/> the runtime refuses). The caller owns
        /// and disposes the invitation. Windows only.
        /// </remarks>
        public Result<MemberHubInvitation> CreateMemberHubInvitation(MemberHubInvitationOptions options, CancellationToken cancellationToken = default(CancellationToken))
        {
            if (options == null)
                throw new ArgumentNullException(nameof(options));
            ThrowIfDisposed();
            var cancellationResult = NativeCancellationScope.Create(cancellationToken, _disposing.Token);
            if (!cancellationResult.IsSuccess)
                return Result<MemberHubInvitation>.Failure(cancellationResult.Info);
            using (var cancellation = cancellationResult.Value)
            using (var scope = new NativeMemoryScope())
            {
                var native = MemberHubInvitationNative.Create(options, scope);
                IntPtr invitation;
                var info = Results.ToInfo(NativeMethods.qar_impl_runtime_create_member_hub_invitation(scope.Borrow(_handle), ref native, cancellation.Handle, out invitation));
                return Results.FromHandle(info, invitation, MemberHubInvitationNative.FromHandle);
            }
        }

        /// <summary>
        /// Member-hub invitation, on the joining PC: joins the QAROS hub on this PC to the
        /// invitation's primary hub as a member. It does not onboard this app: call
        /// <see cref="Onboard"/> with a local-app onboarding afterwards, which then needs no prompt.
        /// </summary>
        /// <remarks>
        /// Starts the installed hub when none runs, checks the invitation (signature, expiry, then
        /// the administrator's allow-list or the primary this PC's hub joined first), joins, rejoins
        /// or readmits the hub, and blocks through its restart into the primary's session. Called
        /// again on a hub that already is a member of that primary, it succeeds at once. Any process
        /// of this PC's user may call it: the signed invitation authorizes the join. Expected
        /// failures: <see cref="StatusCode.MemberInvitationInvalid"/>,
        /// <see cref="StatusCode.MemberInvitationExpired"/>,
        /// <see cref="StatusCode.MemberInvitationUsedUp"/>,
        /// <see cref="StatusCode.MemberInvitationRevoked"/>,
        /// <see cref="StatusCode.MemberInvitationPrimaryNotAllowed"/>,
        /// <see cref="StatusCode.MemberInvitationPolicyNotProtected"/>,
        /// <see cref="StatusCode.MemberJoinRefused"/>, <see cref="StatusCode.MemberRemoved"/>,
        /// <see cref="StatusCode.PrimaryUnreachable"/>,
        /// <see cref="StatusCode.HubLifecycleOperationRunning"/>,
        /// <see cref="StatusCode.OnboardingHubUnreachable"/> and <see cref="StatusCode.Timeout"/>
        /// (the join stays committed; calling again succeeds once the hub is back). Windows only.
        /// </remarks>
        public Result<Unit> JoinHubAsMember(JoinHubAsMemberOptions options, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default(CancellationToken))
        {
            if (options == null)
                throw new ArgumentNullException(nameof(options));
            if (options.Invitation == null)
                throw new ArgumentNullException(nameof(options.Invitation));
            ThrowIfDisposed();
            var cancellationResult = NativeCancellationScope.Create(cancellationToken, _disposing.Token);
            if (!cancellationResult.IsSuccess)
                return Result<Unit>.Failure(cancellationResult.Info);
            using (var cancellation = cancellationResult.Value)
            using (var scope = new NativeMemoryScope())
            using (var progressState = ProgressState.Create(progress))
            {
                var native = MemberHubInvitationNative.Join(options, scope);
                return Results.FromNative(NativeMethods.qar_impl_runtime_join_hub_as_member(
                    scope.Borrow(_handle),
                    ref native,
                    progressState.Callback,
                    progressState.UserState,
                    cancellation.Handle));
            }
        }

        /// <summary>
        /// Releases this process's runtime handle. Dispose its sessions first.
        /// Does not shut down the shared Hub or other applications using it.
        /// </summary>
        public void Dispose()
        {
            // A call borrowing the handle (Onboard, Rejoin, JoinHubAsMember, Forget, and the async
            // calls until their completion fires) defers its release until it is done; the flag makes the runtime unusable from here on all the same.
            lock (_releaseGate)
                _disposed = true;
            try
            {
                _disposing.Cancel();
            }
            catch (AggregateException)
            {
            }
            _handle.Dispose();
        }

        private void ThrowIfDisposed()
        {
            if (_disposed || _handle.IsClosed || _handle.IsInvalid)
                throw new ObjectDisposedException(nameof(Runtime));
        }
    }
}
