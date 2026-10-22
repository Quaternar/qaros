using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
#if NET8_0_OR_GREATER
using System.Runtime.CompilerServices;
#endif
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Quaternar.Qaros;

namespace Quaternar.Qaros.Interop
{
    internal static class NativeCallbackRegistry
    {
        private static readonly object Gate = new object();
        private static readonly Dictionary<long, NativeSubscription> Subscriptions = new Dictionary<long, NativeSubscription>();
        private static long _nextId;

        internal static IntPtr Register(NativeSubscription subscription)
        {
            var id = Interlocked.Increment(ref _nextId);
            lock (Gate)
                Subscriptions.Add(id, subscription);
            return new IntPtr(id);
        }

        internal static void Unregister(IntPtr userState)
        {
            var id = userState.ToInt64();
            lock (Gate)
                Subscriptions.Remove(id);
        }

        internal static bool TryGet<T>(IntPtr userState, out T subscription)
            where T : NativeSubscription
        {
            NativeSubscription? value;
            lock (Gate)
            {
                if (!Subscriptions.TryGetValue(userState.ToInt64(), out value))
                {
                    subscription = null!;
                    return false;
                }
            }
            if (value is T typed)
            {
                subscription = typed;
                return true;
            }
            subscription = null!;
            return false;
        }
    }

    internal abstract class NativeSubscription : IDisposable
    {
        private int _disposed;

        private readonly CancelToken _token;

        private readonly bool _ownsToken;

        private CancellationTokenRegistration _standardCancellationRegistration;

        private readonly bool _hasStandardCancellationRegistration;

        protected NativeSubscription(CancelToken token, bool ownsToken)
            : this(token, ownsToken, default(CancellationTokenRegistration), false)
        {
        }

        protected NativeSubscription(CancelToken token, bool ownsToken, CancellationTokenRegistration standardCancellationRegistration)
            : this(token, ownsToken, standardCancellationRegistration, true)
        {
        }

        private NativeSubscription(CancelToken token, bool ownsToken, CancellationTokenRegistration standardCancellationRegistration, bool hasStandardCancellationRegistration)
        {
            _token = token;
            _ownsToken = ownsToken;
            _standardCancellationRegistration = standardCancellationRegistration;
            _hasStandardCancellationRegistration = hasStandardCancellationRegistration;
            UserState = NativeCallbackRegistry.Register(this);
        }

        public IntPtr UserState { get; }

        protected bool IsDisposed { get { return Volatile.Read(ref _disposed) != 0; } }

        internal void DisposeAfterFailedStart()
        {
            if (Interlocked.Exchange(ref _disposed, 1) != 0)
                return;
            NativeCallbackRegistry.Unregister(UserState);
            if (_hasStandardCancellationRegistration)
                _standardCancellationRegistration.Dispose();
            if (_ownsToken)
                _token.Dispose();
        }

        /// <summary>
        /// Ends delivery. When this returns, a handler call running on another
        /// thread has finished and no further one starts: the native cancel
        /// waits for it (ADR-0283), and only then are the registry entry and
        /// the callback delegate released. Called from inside this
        /// subscription's own handler it cannot wait for that call: it returns
        /// at once, the handler finishes and no further call starts. Do not
        /// dispose while holding a lock the handler takes.
        /// </summary>
        public void Dispose()
        {
            if (Interlocked.Exchange(ref _disposed, 1) != 0)
                return;
            // Blocks until no native callback of this subscription runs on
            // another thread; after it, none starts.
            _token.Cancel();
            NativeCallbackRegistry.Unregister(UserState);
            if (_hasStandardCancellationRegistration)
                _standardCancellationRegistration.Dispose();
            if (_ownsToken)
                _token.Dispose();
        }
    }

    internal sealed class PeerUpdateSubscription : NativeSubscription
    {
        private readonly EventHandler<PeerSpecUpdatedEventArgs> _handler;

        public PeerUpdateCallback Callback { get; }

        private PeerUpdateSubscription(CancelToken token, bool ownsToken, EventHandler<PeerSpecUpdatedEventArgs> handler) : base(token, ownsToken)
        {
            _handler = handler;
            Callback = OnPeerUpdated;
        }

        private PeerUpdateSubscription(CancelToken token, CancellationTokenRegistration standardCancellationRegistration, EventHandler<PeerSpecUpdatedEventArgs> handler) : base(token, true, standardCancellationRegistration)
        {
            _handler = handler;
            Callback = OnPeerUpdated;
        }

        public static PeerUpdateSubscription Create(CancelToken token, bool ownsToken, EventHandler<PeerSpecUpdatedEventArgs> handler) { return new PeerUpdateSubscription(token, ownsToken, handler); }
        public static PeerUpdateSubscription Create(CancelToken token, CancellationTokenRegistration standardCancellationRegistration, EventHandler<PeerSpecUpdatedEventArgs> handler) { return new PeerUpdateSubscription(token, standardCancellationRegistration, handler); }

        private static void OnPeerUpdated(IntPtr handle, IntPtr userState)
        {
            try
            {
                if (!NativeCallbackRegistry.TryGet(userState, out PeerUpdateSubscription sub))
                    return;
                if (sub.IsDisposed) return;
                var peer = PeerSpec.FromHandle(handle, false);
                if (peer.IsSuccess) sub._handler(sub, new PeerSpecUpdatedEventArgs(peer.Value));
                GC.KeepAlive(sub);
            }
            catch
            {
            }
        }
    }

    internal sealed class GuiPanelUpdateSubscription : NativeSubscription
    {
        private readonly EventHandler<GuiPanelUpdatedEventArgs> _handler;

        public GuiPanelUpdateCallback Callback { get; }

        private GuiPanelUpdateSubscription(CancelToken token, bool ownsToken, EventHandler<GuiPanelUpdatedEventArgs> handler) : base(token, ownsToken)
        {
            _handler = handler;
            Callback = OnPanelUpdated;
        }

        private GuiPanelUpdateSubscription(CancelToken token, CancellationTokenRegistration standardCancellationRegistration, EventHandler<GuiPanelUpdatedEventArgs> handler) : base(token, true, standardCancellationRegistration)
        {
            _handler = handler;
            Callback = OnPanelUpdated;
        }

        public static GuiPanelUpdateSubscription Create(CancelToken token, bool ownsToken, EventHandler<GuiPanelUpdatedEventArgs> handler) { return new GuiPanelUpdateSubscription(token, ownsToken, handler); }
        public static GuiPanelUpdateSubscription Create(CancelToken token, CancellationTokenRegistration standardCancellationRegistration, EventHandler<GuiPanelUpdatedEventArgs> handler) { return new GuiPanelUpdateSubscription(token, standardCancellationRegistration, handler); }

        private static void OnPanelUpdated(IntPtr handle, IntPtr userState)
        {
            try
            {
                if (!NativeCallbackRegistry.TryGet(userState, out GuiPanelUpdateSubscription sub))
                    return;
                if (sub.IsDisposed) return;
                var panel = GuiPanel.FromHandle(handle, false);
                if (panel.IsSuccess) sub._handler(sub, new GuiPanelUpdatedEventArgs(panel.Value));
                GC.KeepAlive(sub);
            }
            catch
            {
            }
        }
    }

    internal sealed class AppVolumeUpdateSubscription : NativeSubscription
    {
        private readonly EventHandler<AppVolumeUpdatedEventArgs> _handler;

        public AppVolumeUpdateCallback Callback { get; }

        private AppVolumeUpdateSubscription(CancelToken token, bool ownsToken, EventHandler<AppVolumeUpdatedEventArgs> handler) : base(token, ownsToken)
        {
            _handler = handler;
            Callback = OnVolumeUpdated;
        }

        private AppVolumeUpdateSubscription(CancelToken token, CancellationTokenRegistration standardCancellationRegistration, EventHandler<AppVolumeUpdatedEventArgs> handler) : base(token, true, standardCancellationRegistration)
        {
            _handler = handler;
            Callback = OnVolumeUpdated;
        }

        public static AppVolumeUpdateSubscription Create(CancelToken token, bool ownsToken, EventHandler<AppVolumeUpdatedEventArgs> handler) { return new AppVolumeUpdateSubscription(token, ownsToken, handler); }
        public static AppVolumeUpdateSubscription Create(CancelToken token, CancellationTokenRegistration standardCancellationRegistration, EventHandler<AppVolumeUpdatedEventArgs> handler) { return new AppVolumeUpdateSubscription(token, standardCancellationRegistration, handler); }

        private static void OnVolumeUpdated(IntPtr handle, IntPtr userState)
        {
            try
            {
                if (!NativeCallbackRegistry.TryGet(userState, out AppVolumeUpdateSubscription sub))
                    return;
                if (sub.IsDisposed) return;
                var volume = AppVolume.FromHandle(handle, false);
                if (volume.IsSuccess) sub._handler(sub, new AppVolumeUpdatedEventArgs(volume.Value));
                GC.KeepAlive(sub);
            }
            catch
            {
            }
        }
    }

    internal sealed class AppVolumeGestureSubscription : NativeSubscription
    {
        private readonly EventHandler<AppVolumeGestureEventArgs> _handler;

        public AppVolumeGestureEventCallback Callback { get; }

        private AppVolumeGestureSubscription(CancelToken token, bool ownsToken, EventHandler<AppVolumeGestureEventArgs> handler) : base(token, ownsToken)
        {
            _handler = handler;
            Callback = OnGesture;
        }

        private AppVolumeGestureSubscription(CancelToken token, CancellationTokenRegistration standardCancellationRegistration, EventHandler<AppVolumeGestureEventArgs> handler) : base(token, true, standardCancellationRegistration)
        {
            _handler = handler;
            Callback = OnGesture;
        }

        public static AppVolumeGestureSubscription Create(CancelToken token, bool ownsToken, EventHandler<AppVolumeGestureEventArgs> handler) { return new AppVolumeGestureSubscription(token, ownsToken, handler); }
        public static AppVolumeGestureSubscription Create(CancelToken token, CancellationTokenRegistration standardCancellationRegistration, EventHandler<AppVolumeGestureEventArgs> handler) { return new AppVolumeGestureSubscription(token, standardCancellationRegistration, handler); }

        private static void OnGesture(IntPtr gesture, IntPtr userState)
        {
            try
            {
                if (!NativeCallbackRegistry.TryGet(userState, out AppVolumeGestureSubscription sub))
                    return;
                if (sub.IsDisposed) return;
                // The handle dies with this call, so it is read into a managed
                // copy here rather than handed to the application.
                sub._handler(sub, new AppVolumeGestureEventArgs(AppVolumeGestureEvent.FromHandle(gesture)));
                GC.KeepAlive(sub);
            }
            catch
            {
            }
        }
    }

    internal sealed class GuiPanelGestureSubscription : NativeSubscription
    {
        private readonly EventHandler<GuiPanelGestureEventArgs> _handler;

        public GuiPanelGestureEventCallback Callback { get; }

        private GuiPanelGestureSubscription(CancelToken token, bool ownsToken, EventHandler<GuiPanelGestureEventArgs> handler) : base(token, ownsToken)
        {
            _handler = handler;
            Callback = OnGesture;
        }

        private GuiPanelGestureSubscription(CancelToken token, CancellationTokenRegistration standardCancellationRegistration, EventHandler<GuiPanelGestureEventArgs> handler) : base(token, true, standardCancellationRegistration)
        {
            _handler = handler;
            Callback = OnGesture;
        }

        public static GuiPanelGestureSubscription Create(CancelToken token, bool ownsToken, EventHandler<GuiPanelGestureEventArgs> handler) { return new GuiPanelGestureSubscription(token, ownsToken, handler); }
        public static GuiPanelGestureSubscription Create(CancelToken token, CancellationTokenRegistration standardCancellationRegistration, EventHandler<GuiPanelGestureEventArgs> handler) { return new GuiPanelGestureSubscription(token, standardCancellationRegistration, handler); }

        private static void OnGesture(IntPtr gesture, IntPtr userState)
        {
            try
            {
                if (!NativeCallbackRegistry.TryGet(userState, out GuiPanelGestureSubscription sub))
                    return;
                if (sub.IsDisposed) return;
                // The handle dies with this call, so it is read into a managed
                // copy here rather than handed to the application.
                sub._handler(sub, new GuiPanelGestureEventArgs(GuiPanelGestureEvent.FromHandle(gesture)));
                GC.KeepAlive(sub);
            }
            catch
            {
            }
        }
    }

    internal sealed class RenderRequestSubscription : NativeSubscription
    {
        private readonly EventHandler<RenderStreamRequestEventArgs> _handler;

        public RenderRequestCallback Callback { get; }

        private RenderRequestSubscription(CancelToken token, bool ownsToken, EventHandler<RenderStreamRequestEventArgs> handler) : base(token, ownsToken)
        {
            _handler = handler;
            Callback = OnRequest;
        }

        private RenderRequestSubscription(CancelToken token, CancellationTokenRegistration standardCancellationRegistration, EventHandler<RenderStreamRequestEventArgs> handler) : base(token, true, standardCancellationRegistration)
        {
            _handler = handler;
            Callback = OnRequest;
        }

        public static RenderRequestSubscription Create(CancelToken token, bool ownsToken, EventHandler<RenderStreamRequestEventArgs> handler) { return new RenderRequestSubscription(token, ownsToken, handler); }
        public static RenderRequestSubscription Create(CancelToken token, CancellationTokenRegistration standardCancellationRegistration, EventHandler<RenderStreamRequestEventArgs> handler) { return new RenderRequestSubscription(token, standardCancellationRegistration, handler); }

        private static void OnRequest(IntPtr request, IntPtr userState)
        {
            try
            {
                if (!NativeCallbackRegistry.TryGet(userState, out RenderRequestSubscription sub))
                    return;
                if (sub.IsDisposed || request == IntPtr.Zero)
                    return;
                var peerId = PeerIdNative.CreateDefault();
                var streamId = StreamIdNative.CreateDefault();
                var peerInfo = Results.ToInfo(NativeMethods.qar_impl_render_request_get_target_peer_id(request, ref peerId));
                if (!peerInfo.IsSuccess)
                    return;
                var streamInfo = Results.ToInfo(NativeMethods.qar_impl_render_request_get_stream_id(request, ref streamId));
                if (!streamInfo.IsSuccess)
                    return;
                sub._handler(sub, new RenderStreamRequestEventArgs(new RenderStreamRequest(PeerId.FromNative(peerId), StreamId.FromNative(streamId))));
                GC.KeepAlive(sub);
            }
            catch
            {
            }
            finally
            {
                if (request != IntPtr.Zero)
                    NativeMethods.qar_impl_render_request_handle_destroy(request);
            }
        }
    }

    internal abstract class AsyncStateBase<T>
    {
        private readonly GCHandle _selfHandle;

        private int _cleanupStarted;

        private CancelToken? _ownedCancellationToken;

        private CancellationTokenRegistration _ownedCancellationRegistration;

        private bool _hasOwnedCancellationRegistration;

        protected AsyncStateBase()
        {
            Completion = new TaskCompletionSource<T>(TaskCreationOptions.RunContinuationsAsynchronously);
            _selfHandle = GCHandle.Alloc(this);
            UserState = GCHandle.ToIntPtr(_selfHandle);
        }

        protected TaskCompletionSource<T> Completion { get; }

        public IntPtr UserState { get; }

        public Task<T> Task { get { return Completion.Task; } }

        public void Complete(T value) { Completion.TrySetResult(value); CleanupOnce(); }

        public void Fault(Exception ex) { Completion.TrySetException(ex); CleanupOnce(); }

        public Result<IntPtr> UseStandardCancellation(CancellationToken cancellationToken)
        {
            if (!cancellationToken.CanBeCanceled)
                return Result<IntPtr>.Success(new ResultInfo(StatusCode.Success, 0, string.Empty), IntPtr.Zero);
            var tokenResult = CancelToken.Create();
            if (!tokenResult.IsSuccess)
                return Result<IntPtr>.Failure(tokenResult.Info);
            _ownedCancellationToken = tokenResult.Value;
            if (cancellationToken.IsCancellationRequested)
            {
                CancelFromStandardCancellation();
            }
            else if (cancellationToken.CanBeCanceled)
            {
                _ownedCancellationRegistration = cancellationToken.Register(OnStandardCancellation, this);
                _hasOwnedCancellationRegistration = true;
            }
            return Result<IntPtr>.Success(tokenResult.Info, _ownedCancellationToken.DangerousHandle);
        }

        private static void OnStandardCancellation(object? state)
        {
            ((AsyncStateBase<T>)state!).CancelFromStandardCancellation();
        }

        private void CancelFromStandardCancellation()
        {
            try
            {
                if (_ownedCancellationToken != null)
                    _ownedCancellationToken.Cancel();
            }
            catch (ObjectDisposedException)
            {
            }
            Completion.TrySetCanceled();
        }

        protected void CleanupOnce()
        {
            if (Interlocked.Exchange(ref _cleanupStarted, 1) != 0)
                return;
            Cleanup();
        }

        protected virtual void Cleanup()
        {
            if (_hasOwnedCancellationRegistration)
            {
                _ownedCancellationRegistration.Dispose();
                _hasOwnedCancellationRegistration = false;
            }
            if (_ownedCancellationToken != null)
            {
                _ownedCancellationToken.Dispose();
                _ownedCancellationToken = null;
            }
            if (_selfHandle.IsAllocated)
                _selfHandle.Free();
        }
    }

    internal sealed class ProgressState : IDisposable
    {
        private readonly IProgress<ActionProgress>? _progress;

        public ProgressCallback Callback { get; }

        public IntPtr UserState { get; }

        private ProgressState(IProgress<ActionProgress>? progress)
        {
            _progress = progress;
            Callback = new ProgressCallback(OnProgress);
            UserState = progress == null ? IntPtr.Zero : GCHandle.ToIntPtr(GCHandle.Alloc(this));
        }

        public static ProgressState Create(IProgress<ActionProgress>? progress)
        {
            return new ProgressState(progress);
        }

        private static void OnProgress(ActionSeverity severity, float percent, IntPtr message, IntPtr userState)
        {
            try
            {
                if (userState == IntPtr.Zero)
                    return;
                var state = (ProgressState)GCHandle.FromIntPtr(userState).Target!;
                if (state._progress != null)
                    state._progress.Report(new ActionProgress(severity, percent, NativeString.DecodeNullTerminated(message) ?? string.Empty));
            }
            catch
            {
            }
        }

        public void Dispose()
        {
            if (UserState != IntPtr.Zero)
                GCHandle.FromIntPtr(UserState).Free();
        }
    }

    internal sealed class RuntimeOnboardAsyncState : AsyncStateBase<Result<OnboardSession>>
    {
        private readonly NativeMemoryScope _scope = new NativeMemoryScope();

        private NativeStructureChain? _chain;

        private NativeStructureChain? _presentationChain;

        private readonly bool _returnsOnboardSession;

        private readonly IProgress<ActionProgress>? _progress;

        private readonly TaskCompletionSource<Result<Session>>? _sessionCompletion;

        public RuntimeOnboardResultCallback ResultCallback { get; }

        public ProgressCallback ProgressCallback { get; }

        public Task<Result<OnboardSession>> OnboardTask { get { return Task; } }

        public Task<Result<Session>> SessionTask { get { return _sessionCompletion == null ? System.Threading.Tasks.Task.FromResult(Result<Session>.Failure(new ResultInfo(StatusCode.LogicError, 0, "Invalid async state usage."))) : _sessionCompletion.Task; } }

        public RuntimeOnboardAsyncState(IProgress<ActionProgress>? progress, bool returnsOnboardSession)
        {
            _progress = progress;
            _returnsOnboardSession = returnsOnboardSession;
            if (!returnsOnboardSession)
                // Callers may dispose the runtime after a failed rejoin. Never
                // run that continuation on the native callback's own worker:
                // runtime disposal joins the native background tasks.
                _sessionCompletion = new TaskCompletionSource<Result<Session>>(TaskCreationOptions.RunContinuationsAsynchronously);
            ResultCallback = new RuntimeOnboardResultCallback(OnResult);
            ProgressCallback = new ProgressCallback(OnProgress);
        }

        public RejoinInitNative CreateNative(RejoinOptions options)
        {
            _chain = NativeStructureChain.From(options.Extension);
            _presentationChain = NativeStructureChain.From(options.Presentation == null ? null : options.Presentation.Extension);
            return NativeInit.Rejoin(options, _scope, _chain.Head, _presentationChain.Head);
        }

        public OnboardInitNative CreateNative(OnboardOptions options)
        {
            _chain = NativeStructureChain.From(options.Mode, options.Extension);
            _presentationChain = NativeStructureChain.From(options.Presentation == null ? null : options.Presentation.Extension);
            return NativeInit.Onboard(options, _scope, _chain.Head, _presentationChain.Head);
        }

        public void CompleteRejoin(Result<Session> result)
        {
            if (_sessionCompletion != null)
                _sessionCompletion.TrySetResult(result);
            CleanupOnce();
        }

        public void CompleteOnboard(Result<OnboardSession> result)
        {
            Complete(result);
        }

        public void FaultRejoin(Exception ex)
        {
            if (_sessionCompletion != null)
                _sessionCompletion.TrySetException(ex);
            CleanupOnce();
        }

        private static void OnResult(ResultNative status, IntPtr onboardingId, IntPtr session, IntPtr userState)
        {
            RuntimeOnboardAsyncState? state = null;
            try
            {
                state = (RuntimeOnboardAsyncState)GCHandle.FromIntPtr(userState).Target!;
                var info = Results.ToInfo(status);
                if (state._returnsOnboardSession)
                {
                    if (!info.IsSuccess)
                    {
                        state.Complete(Result<OnboardSession>.Failure(info));
                        return;
                    }
                    if (session == IntPtr.Zero || onboardingId == IntPtr.Zero)
                    {
                        state.Complete(Result<OnboardSession>.Failure(new ResultInfo(StatusCode.LogicError, 0, "Native async onboard succeeded but returned incomplete output.")));
                        return;
                    }
                    state.Complete(Result<OnboardSession>.Success(info, new OnboardSession(OnboardingId.FromNativePointer(onboardingId), new Session(new SessionSafeHandle(session)))));
                    return;
                }
                if (!info.IsSuccess)
                {
                    state.CompleteRejoin(Result<Session>.Failure(info));
                    return;
                }
                state.CompleteRejoin(Results.FromHandle(info, session, h => new Session(new SessionSafeHandle(h))));
            }
            catch (Exception ex)
            {
                if (state != null)
                {
                    if (state._returnsOnboardSession)
                        state.Fault(ex);
                    else
                        state.FaultRejoin(ex);
                }
            }
        }

        private static void OnProgress(ActionSeverity severity, float percent, IntPtr message, IntPtr userState)
        {
            try
            {
                var state = (RuntimeOnboardAsyncState)GCHandle.FromIntPtr(userState).Target!;
                if (state._progress != null)
                    state._progress.Report(new ActionProgress(severity, percent, NativeString.DecodeNullTerminated(message) ?? string.Empty));
            }
            catch
            {
            }
        }

        protected override void Cleanup()
        {
            _presentationChain?.Dispose();
            _chain?.Dispose();
            _scope.Dispose();
            base.Cleanup();
        }
    }

    internal sealed class RequestInviteAsyncState : AsyncStateBase<Result<OnboardingInvite>>
    {
        private readonly IProgress<ActionProgress>? _progress;

        private NativeStructureChain? _chain;

        public SessionRequestInviteResultCallback ResultCallback { get; }

        public ProgressCallback ProgressCallback { get; }

        public RequestInviteAsyncState(IProgress<ActionProgress>? progress)
        {
            _progress = progress;
            ResultCallback = new SessionRequestInviteResultCallback(OnResult);
            ProgressCallback = new ProgressCallback(OnProgress);
        }

        public RequestInviteInitNative CreateNative(RequestOnboardingInviteOptions options)
        {
            _chain = NativeStructureChain.From(options.Extension);
            return NativeInit.RequestInvite(options, _chain.Head);
        }

        private static void OnResult(ResultNative status, IntPtr invite, IntPtr userState)
        {
            RequestInviteAsyncState? state = null;
            try
            {
                state = (RequestInviteAsyncState)GCHandle.FromIntPtr(userState).Target!;
                var info = Results.ToInfo(status);
                state.Complete(Results.FromHandle(info, invite, h => new OnboardingInvite(new OnboardingInviteSafeHandle(h))));
            }
            catch (Exception ex)
            {
                if (state != null)
                    state.Fault(ex);
            }
        }

        private static void OnProgress(ActionSeverity severity, float percent, IntPtr message, IntPtr userState)
        {
            try
            {
                var state = (RequestInviteAsyncState)GCHandle.FromIntPtr(userState).Target!;
                if (state._progress != null)
                    state._progress.Report(new ActionProgress(severity, percent, NativeString.DecodeNullTerminated(message) ?? string.Empty));
            }
            catch
            {
            }
        }

        protected override void Cleanup()
        {
            _chain?.Dispose();
            base.Cleanup();
        }
    }

    /// <summary>
    /// Rooted state of one qar_session_invite_target_app_async or
    /// qar_session_kill_target_app_async call; the result callback fires exactly
    /// once, also on cancellation.
    /// </summary>
    internal sealed class TargetAppAsyncState<TValue> : AsyncStateBase<Result<TValue>>
    {
        private readonly NativeMemoryScope _scope = new NativeMemoryScope();

        private readonly IProgress<ActionProgress>? _progress;

        private readonly Func<IntPtr, TValue> _value;

        public TargetAppResultCallback ResultCallback { get; }

        public ProgressCallback ProgressCallback { get; }

        public NativeMemoryScope Scope { get { return _scope; } }

        public TargetAppAsyncState(IProgress<ActionProgress>? progress, Func<IntPtr, TValue> value)
        {
            _progress = progress;
            _value = value;
            ResultCallback = new TargetAppResultCallback(OnResult);
            ProgressCallback = new ProgressCallback(OnProgress);
        }

        private static void OnResult(ResultNative status, IntPtr peerId, IntPtr userState)
        {
            TargetAppAsyncState<TValue>? state = null;
            try
            {
                state = (TargetAppAsyncState<TValue>)GCHandle.FromIntPtr(userState).Target!;
                var info = Results.ToInfo(status);
                state.Complete(info.IsSuccess ? Result<TValue>.Success(info, state._value(peerId)) : Result<TValue>.Failure(info));
            }
            catch (Exception ex)
            {
                if (state != null)
                    state.Fault(ex);
            }
        }

        private static void OnProgress(ActionSeverity severity, float percent, IntPtr message, IntPtr userState)
        {
            try
            {
                var state = (TargetAppAsyncState<TValue>)GCHandle.FromIntPtr(userState).Target!;
                if (state._progress != null)
                    state._progress.Report(new ActionProgress(severity, percent, NativeString.DecodeNullTerminated(message) ?? string.Empty));
            }
            catch
            {
            }
        }

        protected override void Cleanup()
        {
            _scope.Dispose();
            base.Cleanup();
        }
    }

    internal sealed class RenderSenderCreateAsyncState : AsyncStateBase<Result<RenderSender>>
    {
        private readonly NativeMemoryScope _scope = new NativeMemoryScope();

        public RenderSenderCreateCallback Callback { get; }

        public RenderSenderCreateAsyncState() { Callback = OnResult; }

        public RenderSenderInitNative CreateNative(RenderSenderOptions options)
        {
            return NativeInit.RenderSender(options, _scope);
        }

        private static void OnResult(ResultNative status, IntPtr stream, IntPtr userState)
        {
            RenderSenderCreateAsyncState? state = null;
            try
            {
                state = (RenderSenderCreateAsyncState)GCHandle.FromIntPtr(userState).Target!;
                var info = Results.ToInfo(status);
                state.Complete(Results.FromHandle(info, stream, h => new RenderSender(new RenderSenderSafeHandle(h))));
            }
            catch (Exception ex)
            {
                if (state != null)
                    state.Fault(ex);
            }
        }

        protected override void Cleanup() { _scope.Dispose(); base.Cleanup(); }
    }

    internal sealed class BeginFrameAsyncState : AsyncStateBase<Result<RenderFrameInfo>>
    {
        public RenderSenderBeginFrameCallback Callback { get; }

        public BeginFrameAsyncState() { Callback = OnResult; }

        private static void OnResult(ResultNative status, IntPtr frameInfo, IntPtr userState)
        {
            BeginFrameAsyncState? state = null;
            try
            {
                state = (BeginFrameAsyncState)GCHandle.FromIntPtr(userState).Target!;
                var info = Results.ToInfo(status);
                state.Complete(Results.FromHandle(info, frameInfo, h => new RenderFrameInfo(new RenderFrameInfoSafeHandle(h))));
            }
            catch (Exception ex)
            {
                if (state != null)
                    state.Fault(ex);
            }
        }
    }

#if NET8_0_OR_GREATER
    internal sealed class AsyncEventQueue<T>
    {
        private readonly object _gate = new object();
        private readonly Queue<T> _items = new Queue<T>();
        private readonly SemaphoreSlim _signal = new SemaphoreSlim(0);
        private bool _completed;

        public void Enqueue(T item)
        {
            lock (_gate)
            {
                if (_completed)
                    return;
                _items.Enqueue(item);
            }
            _signal.Release();
        }

        public void Complete()
        {
            lock (_gate)
            {
                if (_completed)
                    return;
                _completed = true;
            }
            _signal.Release();
        }

        public async IAsyncEnumerable<T> ReadAllAsync([EnumeratorCancellation] CancellationToken cancellationToken = default)
        {
            while (true)
            {
                await _signal.WaitAsync(cancellationToken).ConfigureAwait(false);
                T item;
                lock (_gate)
                {
                    if (_items.Count == 0)
                    {
                        if (_completed)
                            yield break;
                        continue;
                    }
                    item = _items.Dequeue();
                }
                yield return item;
            }
        }
    }
#endif
}
