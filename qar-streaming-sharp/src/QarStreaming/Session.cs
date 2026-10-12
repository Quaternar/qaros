using System;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Qar.Native;

namespace Qar
{
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

        public Result<OnboardingInvite> RequestOnboardingInvite(RequestOnboardingInviteOptions? options = null, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default(CancellationToken))
        {
            _context.ThrowIfDisposed();
            options = options ?? new RequestOnboardingInviteOptions();
            var cancellationResult = NativeCancellationScope.Create(cancellationToken, false);
            if (!cancellationResult.IsSuccess)
                return Result<OnboardingInvite>.Failure(cancellationResult.Info);
            using (var chain = NativeStructureChain.From(options.Extension))
            using (var cancellation = cancellationResult.Value)
            using (var progressState = ProgressState.Create(progress))
            {
                var native = NativeInit.RequestInvite(options, chain.Head);
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
        /// Ask the hub for a new set of one-time onboarding codes, for several devices to onboard at once.
        /// Codes other callers hold and the hub's onboarding screen code stay valid. The caller owns and
        /// disposes every returned invite.
        /// </summary>
        public Result<IReadOnlyList<OnboardingInvite>> RequestOnboardingCodes(RequestOnboardingCodesOptions? options = null, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default(CancellationToken))
        {
            _context.ThrowIfDisposed();
            options = options ?? new RequestOnboardingCodesOptions();
            var cancellationResult = NativeCancellationScope.Create(cancellationToken, false);
            if (!cancellationResult.IsSuccess)
                return Result<IReadOnlyList<OnboardingInvite>>.Failure(cancellationResult.Info);
            using (var cancellation = cancellationResult.Value)
            using (var progressState = ProgressState.Create(progress))
            {
                var native = NativeInit.RequestOnboardingCodes(options, IntPtr.Zero);
                var handles = new IntPtr[Math.Max(1u, options.Count)];
                var info = Results.ToInfo(NativeMethods.qar_impl_session_request_onboarding_codes(
                    _context.DangerousHandle,
                    ref native,
                    progressState.Callback,
                    progressState.UserState,
                    cancellation.Handle,
                    handles));
                if (!info.IsSuccess)
                    return Result<IReadOnlyList<OnboardingInvite>>.Failure(info);
                var invites = new List<OnboardingInvite>(handles.Length);
                foreach (var handle in handles)
                    invites.Add(new OnboardingInvite(new OnboardingInviteSafeHandle(handle)));
                return Result<IReadOnlyList<OnboardingInvite>>.Success(info, invites);
            }
        }

        public Task<Result<OnboardingInvite>> RequestOnboardingInviteAsync(RequestOnboardingInviteOptions? options = null, IProgress<ActionProgress>? progress = null, CancellationToken cancellationToken = default(CancellationToken))
        {
            _context.ThrowIfDisposed();
            options = options ?? new RequestOnboardingInviteOptions();
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
        public Result<PeerId> FindRenderTarget(string connection)
        {
            if (connection == null)
                throw new ArgumentNullException(nameof(connection));
            using (var scope = new NativeMemoryScope())
            {
                var peer = PeerIdNative.CreateDefault();
                var info = Results.ToInfo(NativeMethods.qar_impl_session_find_render_target(_context.DangerousHandle, scope.AllocString(connection), ref peer));
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
            using (var nativeHost = NativeUtf8String.FromString(host))
                return NativeString.GetString((buffer, size) => build(nativeHost.Pointer, buffer, size));
        }
    }
}
