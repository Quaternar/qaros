using System;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    /// <summary>
    /// An active session, the entry point to peers, GUI panels, app volumes and senders. Only
    /// <see cref="Runtime.Onboard"/> and <see cref="Runtime.Rejoin"/> produce one, and there is no
    /// getter for a "current" session: keep the instance.
    /// </summary>
    /// <remarks>
    /// The caller owns the session. <see cref="Dispose"/> tears the session down but keeps the
    /// persisted identity, so the next start can <see cref="Runtime.Rejoin"/> silently; erasing the
    /// identity is <see cref="Runtime.Forget"/>. Disposing ends every subscription of the session
    /// and waits for a handler running on another thread, so never dispose it from a QAROS
    /// callback or event handler, nor while holding a lock a handler takes.
    /// </remarks>
    public sealed partial class Session : IDisposable
    {
        private readonly SessionNativeContext _context;

        internal Session(SessionSafeHandle handle)
        {
            _context = new SessionNativeContext(handle);
            Peers = new PeerControl(_context);
            GuiPanels = new GuiPanelControl(_context);
            AppVolumes = new AppVolumeControl(_context);
            RenderSenders = new RenderSenderControl(_context);
            PrimarylessVideoSenders = new PrimarylessVideoSenderControl(_context);
        }

        public PeerControl Peers { get; }
        public GuiPanelControl GuiPanels { get; }
        public AppVolumeControl AppVolumes { get; }
        public RenderSenderControl RenderSenders { get; }

        public PrimarylessVideoSenderControl PrimarylessVideoSenders { get; }

        internal IntPtr DangerousHandle { get { return _context.DangerousHandle; } }

        public Result<SessionId> GetId()
        {
            var id = SessionIdNative.CreateDefault();
            var info = Results.ToInfo(NativeMethods.qar_impl_session_get_id(_context.DangerousHandle, ref id));
            return info.IsSuccess ? Result<SessionId>.Success(info, SessionId.FromNative(id)) : Result<SessionId>.Failure(info);
        }

        /// <summary>
        /// The GPU adapter the session's graphics devices were created on, or null when it is not
        /// known. It is the one <see cref="OnboardOptions.WithGraphicsDevice"/> named, else the one
        /// QAROS selected for an app it launched, else the one the library picked. Create your own
        /// graphics device on it so the frames you render need no cross-GPU copy.
        /// </summary>
        public Result<GraphicsDeviceId?> GetGraphicsDeviceId()
        {
            _context.ThrowIfDisposed();
            var native = new GraphicsDeviceId().ToNative();
            bool present;
            var info = Results.ToInfo(NativeMethods.qar_impl_session_get_graphics_device_id(_context.DangerousHandle, ref native, out present));
            if (!info.IsSuccess)
                return Result<GraphicsDeviceId?>.Failure(info);
            return Result<GraphicsDeviceId?>.Success(info, present ? GraphicsDeviceId.FromNative(native) : null);
        }

        /// <summary>
        /// Asks the hub for a co-process invite: one companion process on this PC (a C# helper of a
        /// native app, a plugin sandbox) joins with it, without a code, as the peer the hub
        /// remembers for this app's approval and <paramref name="appTag"/>. Blocks until the hub
        /// answers.
        /// </summary>
        /// <remarks>
        /// <para>
        /// Only a session that joined by <see cref="OnboardOptions.UseLocalApp"/>, or a process the
        /// QAROS launcher started, may ask; the hub refuses any other. The invite works once, for
        /// 60 s (set by the hub), and only from the hub's PC. The co-process gets the short local
        /// certificate under this app's approval; its certificate renews only while this app's peer is
        /// still in the session, and revoking the approval revokes both.
        /// </para>
        /// <para>
        /// The caller owns and disposes the invite. Hand it to the co-process over a private
        /// channel (for example its stdin, never a command line) with
        /// <see cref="OnboardingInvite.Serialize"/>; the co-process creates its runtime with
        /// <see cref="RuntimeOptions.EphemeralStorage"/>, so it writes nothing to disk, and onboards
        /// with <see cref="OnboardingInvite.Deserialize"/> and <see cref="OnboardOptions.UseInvite"/>.
        /// A restarted co-process needs a fresh invite: mint one when it is needed.
        /// </para>
        /// <para>
        /// There is no invite for another PC or for a new device: pairing codes and unbound
        /// invites are minted by the hub's operators. Fails with
        /// <see cref="StatusCode.ArgumentNotSupported"/> for a tag the runtime refuses, or when the
        /// hub cannot be resolved from this session, for example while it is not connected. A
        /// runtime older than app tags refuses the request (its structure type changed with the
        /// tag); check <see cref="RuntimeInfo.Version"/> when you may load one.
        /// </para>
        /// </remarks>
        /// <param name="appTag">The co-process's app tag; rules in <see cref="RequestOnboardingInviteOptions.AppTag"/>.</param>
        /// <param name="progress">Optional progress reports.</param>
        /// <param name="cancellationToken">Cancels the request; the call then returns a failure.</param>
        /// <exception cref="ArgumentNullException"><paramref name="appTag"/> is null.</exception>
        /// <exception cref="ArgumentException"><paramref name="appTag"/> is not a valid app tag.</exception>
        public Result<OnboardingInvite> RequestOnboardingInvite(string appTag, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default(CancellationToken))
        {
            _context.ThrowIfDisposed();
            return RequestOnboardingInvite(new RequestOnboardingInviteOptions(appTag), progress, cancellationToken);
        }

        /// <summary>
        /// <see cref="RequestOnboardingInvite(string, IProgress{ActionProgress}?, CancellationToken)"/>
        /// with the full request options.
        /// </summary>
        /// <exception cref="ArgumentNullException"><paramref name="options"/> is null.</exception>
        public Result<OnboardingInvite> RequestOnboardingInvite(RequestOnboardingInviteOptions options, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default(CancellationToken))
        {
            if (options == null)
                throw new ArgumentNullException(nameof(options));
            _context.ThrowIfDisposed();
            var cancellationResult = NativeCancellationScope.Create(cancellationToken, false);
            if (!cancellationResult.IsSuccess)
                return Result<OnboardingInvite>.Failure(cancellationResult.Info);
            using (var scope = new NativeMemoryScope())
            using (var chain = NativeStructureChain.From(options.Extension))
            using (var cancellation = cancellationResult.Value)
            using (var progressState = ProgressState.Create(progress))
            {
                var native = NativeInit.RequestInvite(options, scope, chain.Head);
                IntPtr invite;
                var info = Results.ToInfo(NativeMethods.qar_impl_session_request_onboarding_invite(
                    _context.DangerousHandle,
                    ref native,
                    progressState.Callback,
                    progressState.UserState,
                    cancellation.Handle,
                    out invite));
                return Results.FromHandle(info, invite, h => new OnboardingInvite(new OnboardingInviteSafeHandle(h)));
            }
        }

        /// <summary>
        /// Async <see cref="RequestOnboardingInvite(string, IProgress{ActionProgress}?, CancellationToken)"/>.
        /// Canceling <paramref name="cancellationToken"/> cancels the task.
        /// </summary>
        /// <exception cref="ArgumentNullException"><paramref name="appTag"/> is null.</exception>
        /// <exception cref="ArgumentException"><paramref name="appTag"/> is not a valid app tag.</exception>
        public Task<Result<OnboardingInvite>> RequestOnboardingInviteAsync(string appTag, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default(CancellationToken))
        {
            _context.ThrowIfDisposed();
            return RequestOnboardingInviteAsync(new RequestOnboardingInviteOptions(appTag), progress, cancellationToken);
        }

        /// <summary>
        /// Async <see cref="RequestOnboardingInvite(RequestOnboardingInviteOptions, IProgress{ActionProgress}?, CancellationToken)"/>.
        /// Canceling <paramref name="cancellationToken"/> cancels the task.
        /// </summary>
        /// <exception cref="ArgumentNullException"><paramref name="options"/> is null.</exception>
        public Task<Result<OnboardingInvite>> RequestOnboardingInviteAsync(RequestOnboardingInviteOptions options, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default(CancellationToken))
        {
            if (options == null)
                throw new ArgumentNullException(nameof(options));
            _context.ThrowIfDisposed();
            var state = new RequestInviteAsyncState(progress);
            var cancelHandle = state.UseStandardCancellation(cancellationToken);
            if (!cancelHandle.IsSuccess)
            {
                state.Complete(Result<OnboardingInvite>.Failure(cancelHandle.Info));
                return state.Task;
            }
            return RequestOnboardingInviteAsync(options, state, cancelHandle.Value);
        }

        private Task<Result<OnboardingInvite>> RequestOnboardingInviteAsync(RequestOnboardingInviteOptions options, RequestInviteAsyncState state, IntPtr token)
        {
            try
            {
                var native = state.CreateNative(options);
                var info = Results.ToInfo(NativeMethods.qar_impl_session_request_onboarding_invite_async(
                    _context.DangerousHandle,
                    ref native,
                    state.ResultCallback,
                    state.ProgressCallback,
                    state.UserState,
                    token));
                if (!info.IsSuccess)
                    state.Complete(Result<OnboardingInvite>.Failure(info));
            }
            catch (Exception ex)
            {
                state.Fault(ex);
            }
            return state.Task;
        }

        /// <summary>Finds an already-connected render target from its connection string.</summary>
        /// <remarks>
        /// Takes the strings <see cref="TargetAppConnectionString"/> builds. It only looks at the
        /// peers this session knows now: it neither invites nor waits, and fails with
        /// <see cref="StatusCode.ArgumentNotSupported"/> when no peer matches. To start a target
        /// app, use <see cref="InviteTargetApp"/>.
        /// </remarks>
        public Result<PeerId> FindRenderTarget(string connection)
        {
            if (connection == null)
                throw new ArgumentNullException(nameof(connection));
            using (var scope = new NativeMemoryScope())
            {
                var peer = PeerIdNative.CreateDefault();
                var info = Results.ToInfo(NativeMethods.qar_impl_session_find_render_target(_context.DangerousHandle, scope.AllocUtf8(connection), ref peer));
                return info.IsSuccess ? Result<PeerId>.Success(info, PeerId.FromNative(peer)) : Result<PeerId>.Failure(info);
            }
        }

        /// <summary>
        /// Asks this machine's hub to start the target app for one AR device and
        /// returns the target app's peer id once its peer joined the session - the
        /// id a render sender renders for. Build <paramref name="connectionString"/>
        /// with <see cref="TargetAppConnectionString"/>; a malformed one or a device
        /// kind that cannot be invited fails with
        /// <see cref="StatusCode.ArgumentNotSupported"/>.
        /// </summary>
        public Result<PeerId> InviteTargetApp(string connectionString, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default(CancellationToken))
        {
            if (connectionString == null)
                throw new ArgumentNullException(nameof(connectionString));
            _context.ThrowIfDisposed();
            var cancellationResult = NativeCancellationScope.Create(cancellationToken, false);
            if (!cancellationResult.IsSuccess)
                return Result<PeerId>.Failure(cancellationResult.Info);
            using (var scope = new NativeMemoryScope())
            using (var cancellation = cancellationResult.Value)
            using (var progressState = ProgressState.Create(progress))
            {
                var native = NativeInit.InviteTargetApp(connectionString, scope);
                var peer = PeerIdNative.CreateDefault();
                var info = Results.ToInfo(NativeMethods.qar_impl_session_invite_target_app(_context.DangerousHandle, ref native, progressState.Callback, progressState.UserState, cancellation.Handle, ref peer));
                return info.IsSuccess ? Result<PeerId>.Success(info, PeerId.FromNative(peer)) : Result<PeerId>.Failure(info);
            }
        }

        /// <summary>Async <see cref="InviteTargetApp"/>.</summary>
        public Task<Result<PeerId>> InviteTargetAppAsync(string connectionString, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default(CancellationToken))
        {
            if (connectionString == null)
                throw new ArgumentNullException(nameof(connectionString));
            _context.ThrowIfDisposed();
            var state = new TargetAppAsyncState<PeerId>(progress, PeerIdFromPointer);
            var cancelHandle = state.UseStandardCancellation(cancellationToken);
            if (!cancelHandle.IsSuccess)
            {
                state.Complete(Result<PeerId>.Failure(cancelHandle.Info));
                return state.Task;
            }
            try
            {
                var native = NativeInit.InviteTargetApp(connectionString, state.Scope);
                var info = Results.ToInfo(NativeMethods.qar_impl_session_invite_target_app_async(_context.DangerousHandle, ref native, state.ResultCallback, state.ProgressCallback, state.UserState, cancelHandle.Value));
                if (!info.IsSuccess)
                    state.Complete(Result<PeerId>.Failure(info));
            }
            catch (Exception ex)
            {
                state.Fault(ex);
            }
            return state.Task;
        }

        /// <summary>
        /// Asks this machine's hub to stop a target app it started, with the whole
        /// pipeline feeding it. A target app this hub did not start ends in
        /// <see cref="StatusCode.Timeout"/>.
        /// </summary>
        public Result<Unit> KillTargetApp(PeerId targetApp, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default(CancellationToken))
        {
            _context.ThrowIfDisposed();
            var cancellationResult = NativeCancellationScope.Create(cancellationToken, false);
            if (!cancellationResult.IsSuccess)
                return Result<Unit>.Failure(cancellationResult.Info);
            using (var cancellation = cancellationResult.Value)
            using (var progressState = ProgressState.Create(progress))
            {
                var native = targetApp.ToNative();
                return Results.FromNative(NativeMethods.qar_impl_session_kill_target_app(_context.DangerousHandle, ref native, progressState.Callback, progressState.UserState, cancellation.Handle));
            }
        }

        /// <summary>Async <see cref="KillTargetApp"/>.</summary>
        public Task<Result<Unit>> KillTargetAppAsync(PeerId targetApp, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default(CancellationToken))
        {
            _context.ThrowIfDisposed();
            var state = new TargetAppAsyncState<Unit>(progress, peer => Unit.Value);
            var cancelHandle = state.UseStandardCancellation(cancellationToken);
            if (!cancelHandle.IsSuccess)
            {
                state.Complete(Result<Unit>.Failure(cancelHandle.Info));
                return state.Task;
            }
            try
            {
                var native = targetApp.ToNative();
                var info = Results.ToInfo(NativeMethods.qar_impl_session_kill_target_app_async(_context.DangerousHandle, ref native, state.ResultCallback, state.ProgressCallback, state.UserState, cancelHandle.Value));
                if (!info.IsSuccess)
                    state.Complete(Result<Unit>.Failure(info));
            }
            catch (Exception ex)
            {
                state.Fault(ex);
            }
            return state.Task;
        }

        private static PeerId PeerIdFromPointer(IntPtr peer)
        {
            return peer == IntPtr.Zero ? PeerId.Default : PeerId.FromNativePointer(peer);
        }

        /// <summary>
        /// Tears the session down but keeps its persisted identity for a later
        /// <see cref="Runtime.Rejoin"/>. Waits for subscription handlers running on other threads;
        /// do not call it from one.
        /// </summary>
        public void Dispose()
        {
            _context.Dispose();
        }
    }

    /// <summary>
    /// Builds the connection strings <see cref="Session.InviteTargetApp"/> accepts.
    /// A host is a hostname or an IP address; it must not be empty.
    /// </summary>
    public static class TargetAppConnectionString
    {
        public static Result<string> HoloLens(string host)
        {
            return Build(host, NativeMethods.qar_impl_target_app_connection_string_hololens);
        }

        public static Result<string> Quest(string host)
        {
            return Build(host, NativeMethods.qar_impl_target_app_connection_string_quest);
        }

        public static Result<string> Android(string host)
        {
            return Build(host, NativeMethods.qar_impl_target_app_connection_string_android);
        }

        private static Result<string> Build(string host, Func<IntPtr, byte[], UIntPtr, ResultNative> build)
        {
            if (host == null)
                throw new ArgumentNullException(nameof(host));
            using (var nativeHost = NativeUtf8String.FromUtf8(host))
                return NativeString.GetString((buffer, size) => build(nativeHost.Pointer, buffer, size));
        }
    }
}
