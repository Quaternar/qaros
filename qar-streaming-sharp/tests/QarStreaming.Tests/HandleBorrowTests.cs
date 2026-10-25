using System;
using System.Reflection;
using System.Runtime.InteropServices;
using System.Threading;
using System.Threading.Tasks;
using Quaternar.Qaros;
using Quaternar.Qaros.Interop;
using Quaternar.Qaros.Tests.Ext;
using Xunit;

namespace Quaternar.Qaros.Tests
{
    /// <summary>
    /// Hub G56: every native call on a session, a sender, a frame info or a cancel token holds a
    /// reference on its handle until the call is done (an async call until its native completion
    /// fires), so a concurrent Dispose or the finalizer defers the native release instead of
    /// freeing the object under the call. A disposed wrapper refuses new calls at once. The final
    /// release of an object disposed during a borrow runs on the thread pool, never on the borrowing
    /// thread, which may be a native thread the release would join.
    /// </summary>
    public sealed class CancelTokenBorrowTests
    {
        private static ResultInfo Failed()
        {
            return new ResultInfo(StatusCode.LogicError, 0, "test completion");
        }

        // The library still knows the token: unlike is_cancelled, which reads false for a destroyed
        // token too, cancel_after fails for one. The timeout is far beyond any test.
        internal static bool IsLive(IntPtr token)
        {
            return Results.FromNative(NativeMethods.qar_impl_cancel_token_cancel_after(token, uint.MaxValue)).IsSuccess;
        }

        [Fact]
        public void ACallHoldsTheCancelTokenUntilItIsDone()
        {
            var token = CancelToken.Create().ValueOrThrow();
            var handle = NativeHandles.Of(token);
            var borrow = token.Borrow();
            Assert.Equal(handle.DangerousGetHandle(), borrow.Handle);
            token.Dispose();
            Assert.False(handle.IsClosed);
            Assert.Throws<ObjectDisposedException>(() => token.Cancel());
            Assert.Throws<ObjectDisposedException>(() => token.IsCancelled());
            Assert.Throws<ObjectDisposedException>(() => token.Borrow());
            // The call that borrowed it still has a live native token.
            Assert.True(IsLive(borrow.Handle));
            borrow.Dispose();
            Assert.True(SpinWait.SpinUntil(() => handle.IsClosed, QarNativeAssert.Budget));
        }

        // Not owned here, so only the subscription's own reference keeps the token alive after
        // its owner disposes it; the subscription still cancels through that reference.
        [Fact]
        public void ASubscriptionKeepsItsCancelTokenAliveUntilItIsDisposed()
        {
            var token = CancelToken.Create().ValueOrThrow();
            var handle = NativeHandles.Of(token);
            var subscription = PeerUpdateSubscription.Create(token, false, (sender, args) => { });
            Assert.Equal(handle.DangerousGetHandle(), subscription.TokenHandle);
            token.Dispose();
            Assert.False(handle.IsClosed);
            Assert.True(IsLive(subscription.TokenHandle));
            subscription.Dispose();
            Assert.True(SpinWait.SpinUntil(() => handle.IsClosed, QarNativeAssert.Budget));
        }

        [Fact]
        public void ASubscriptionReleasesTheTokenItOwnsWhenDisposed()
        {
            var token = CancelToken.Create().ValueOrThrow();
            var handle = NativeHandles.Of(token);
            var subscription = PeerUpdateSubscription.Create(token, true, (sender, args) => { });
            Assert.False(handle.IsClosed);
            subscription.Dispose();
            Assert.True(handle.IsClosed);
            subscription.Dispose();
        }

        [Fact]
        public void AFailedSubscriptionStartReleasesItsToken()
        {
            var token = CancelToken.Create().ValueOrThrow();
            var handle = NativeHandles.Of(token);
            var subscription = PeerUpdateSubscription.Create(token, true, (sender, args) => { });
            subscription.DisposeAfterFailedStart();
            Assert.True(handle.IsClosed);
        }

        [Fact]
        public void ACancellationScopeHoldsItsTokenForTheCall()
        {
            using var cancellation = new CancellationTokenSource();
            var scope = NativeCancellationScope.Create(cancellation.Token, true).ValueOrThrow();
            var token = scope.Token!;
            var handle = NativeHandles.Of(token);
            Assert.Equal(handle.DangerousGetHandle(), scope.Handle);
            token.Dispose();
            Assert.False(handle.IsClosed);
            Assert.True(IsLive(scope.Handle));
            scope.Dispose();
            Assert.True(SpinWait.SpinUntil(() => handle.IsClosed, QarNativeAssert.Budget));
            scope.Dispose();
        }

        private static SafeHandle OwnedTokenOf(BeginFrameAsyncState state)
        {
            var field = typeof(AsyncStateBase<Result<RenderFrameInfo>>).GetField("_ownedCancellationToken", BindingFlags.NonPublic | BindingFlags.Instance);
            Assert.NotNull(field);
            return NativeHandles.Of(Assert.IsType<CancelToken>(field!.GetValue(state)));
        }

        // A task cancelled by its token completes at the cancel; the native call still runs, so its
        // token stays alive until the native completion fires.
        [Fact]
        public void AnAsyncCallHoldsItsCancelTokenUntilTheCompletionFires()
        {
            using var cancellation = new CancellationTokenSource();
            var state = new BeginFrameAsyncState();
            var tokenHandle = state.UseStandardCancellation(cancellation.Token).ValueOrThrow();
            var handle = OwnedTokenOf(state);
            Assert.Equal(handle.DangerousGetHandle(), tokenHandle);
            cancellation.Cancel();
            Assert.True(state.Task.IsCanceled);
            Assert.False(handle.IsClosed);
            Assert.True(NativeMethods.qar_impl_cancel_token_is_cancelled(tokenHandle));
            state.Complete(Result<RenderFrameInfo>.Failure(Failed()));
            Assert.True(handle.IsClosed);
        }

        // Disposing the object an async call was made on cancels the call's native token, so a call
        // whose completion only the object's teardown would bring still completes; the task waits
        // for that completion and reports its failure.
        [Fact]
        public void DisposingTheTargetCancelsAWaitingAsyncCall()
        {
            var target = CancelToken.Create().ValueOrThrow();
            var owner = NativeHandles.OwnerOf(target);
            var state = new BeginFrameAsyncState();
            var tokenHandle = state.UseStandardCancellation(CancellationToken.None, owner).ValueOrThrow();
            Assert.NotEqual(IntPtr.Zero, tokenHandle);
            state.HoldUntilCompletion(owner);
            Assert.False(NativeMethods.qar_impl_cancel_token_is_cancelled(tokenHandle));
            target.Dispose();
            Assert.True(NativeMethods.qar_impl_cancel_token_is_cancelled(tokenHandle));
            Assert.False(state.Task.IsCompleted);
            Assert.False(NativeHandles.Of(target).IsClosed);
            state.Complete(Result<RenderFrameInfo>.Failure(Failed()));
            Assert.True(state.Task.IsCompleted);
            Assert.True(SpinWait.SpinUntil(() => NativeHandles.Of(target).IsClosed, QarNativeAssert.Budget));
        }

        [Fact]
        public void DisposingTheTargetCancelsAWaitingCall()
        {
            var target = CancelToken.Create().ValueOrThrow();
            var owner = NativeHandles.OwnerOf(target);
            using var scope = NativeCancellationScope.Create(CancellationToken.None, owner).ValueOrThrow();
            Assert.NotEqual(IntPtr.Zero, scope.Handle);
            Assert.False(NativeMethods.qar_impl_cancel_token_is_cancelled(scope.Handle));
            target.Dispose();
            Assert.True(NativeMethods.qar_impl_cancel_token_is_cancelled(scope.Handle));
        }

        [Fact]
        public void AWaitingCallOnADisposedTargetIsRefused()
        {
            var target = CancelToken.Create().ValueOrThrow();
            var owner = NativeHandles.OwnerOf(target);
            target.Dispose();
            Assert.Throws<ObjectDisposedException>(() => NativeCancellationScope.Create(CancellationToken.None, owner));
            using var cancellation = new CancellationTokenSource();
            Assert.Throws<ObjectDisposedException>(() => NativeCancellationScope.Create(cancellation.Token, owner));
        }

        // Calls made without a cancelable token share one native token per object, so the
        // per-frame BeginFrame creates none; it lives until the last call using it is done.
        [Fact]
        public void WaitingCallsWithoutATokenShareTheTargetsDisposeToken()
        {
            var target = CancelToken.Create().ValueOrThrow();
            var owner = NativeHandles.OwnerOf(target);
            var first = NativeCancellationScope.Create(CancellationToken.None, owner).ValueOrThrow();
            var second = NativeCancellationScope.Create(CancellationToken.None, owner).ValueOrThrow();
            Assert.Equal(first.Handle, second.Handle);
            var shared = NativeHandles.Of(first.Token!);
            first.Dispose();
            Assert.False(shared.IsClosed);
            Assert.True(IsLive(second.Handle));
            Assert.False(NativeMethods.qar_impl_cancel_token_is_cancelled(second.Handle));
            target.Dispose();
            Assert.True(NativeMethods.qar_impl_cancel_token_is_cancelled(second.Handle));
            Assert.False(shared.IsClosed);
            second.Dispose();
            Assert.True(SpinWait.SpinUntil(() => shared.IsClosed, QarNativeAssert.Budget));
        }
    }

    /// <summary>
    /// The runtime's waiting calls are linked to its Dispose like the session's: the async
    /// onboard's borrow would otherwise keep a disposed runtime alive until the hub answers.
    /// </summary>
    [Collection("QAR Native")]
    public sealed class RuntimeDisposeTests : IClassFixture<QarNativeFixture>
    {
        public RuntimeDisposeTests(QarNativeFixture fixture)
        {
            _ = fixture;
        }

        [Fact]
        public void DisposingTheRuntimeCancelsAWaitingAsyncOnboard()
        {
            var storageRoot = System.IO.Path.Combine(System.IO.Path.GetTempPath(), "qar-sharp-tests", Guid.NewGuid().ToString("N"));
            var runtime = Runtime.Create(new RuntimeOptions { StorageFolderPath = storageRoot }).ValueOrThrow();
            var handle = NativeHandles.Of(runtime);
            var field = typeof(Runtime).GetField("_disposing", BindingFlags.NonPublic | BindingFlags.Instance);
            Assert.NotNull(field);
            var disposing = Assert.IsType<CancellationTokenSource>(field!.GetValue(runtime)).Token;
            var state = new RuntimeOnboardAsyncState(null, true);
            var tokenHandle = state.UseStandardCancellation(CancellationToken.None, disposing).ValueOrThrow();
            state.BorrowRuntime(runtime);
            Assert.False(NativeMethods.qar_impl_cancel_token_is_cancelled(tokenHandle));
            runtime.Dispose();
            Assert.True(NativeMethods.qar_impl_cancel_token_is_cancelled(tokenHandle));
            Assert.False(handle.IsClosed);
            state.CompleteOnboard(Result<OnboardSession>.Failure(new ResultInfo(StatusCode.LogicError, 0, "test completion")));
            Assert.True(SpinWait.SpinUntil(() => handle.IsClosed, QarNativeAssert.Budget));
        }
    }

    [Collection("QAR Native")]
    public sealed class SessionBorrowTests : IClassFixture<QarNativeFixture>
    {
        private readonly QarNativeFixture _fixture;

        public SessionBorrowTests(QarNativeFixture fixture)
        {
            _fixture = fixture;
        }

        private static ResultInfo Failed()
        {
            return new ResultInfo(StatusCode.LogicError, 0, "test completion");
        }

        [Fact]
        public void ACallHoldsTheSessionUntilItIsDone()
        {
            using var scope = _fixture.CreateSessionScope();
            var session = scope.Session;
            var handle = NativeHandles.Of(session);
            var context = NativeHandles.ContextOf(session);
            var borrow = context.Borrow();
            Assert.Equal(handle.DangerousGetHandle(), borrow.Handle);
            session.Dispose();
            Assert.False(handle.IsClosed);
            Assert.Throws<ObjectDisposedException>(() => session.GetId());
            Assert.Throws<ObjectDisposedException>(() => session.Peers.GetMySpec());
            Assert.Throws<ObjectDisposedException>(() => session.Peers.GetAll());
            Assert.Throws<ObjectDisposedException>(() => session.AppVolumes.GetLatestPose(default(AppVolumeId)));
            Assert.Throws<ObjectDisposedException>(() => session.GuiPanels.Close(default(GuiPanelId)));
            Assert.Throws<ObjectDisposedException>(() => session.Peers.SubscribeUpdates((sender, args) => { }));
            Assert.Throws<ObjectDisposedException>(() => context.Borrow());
            // The call that borrowed the session still reads a live native session.
            var id = SessionIdNative.CreateDefault();
            Assert.True(Results.ToInfo(NativeMethods.qar_impl_session_get_id(borrow.Handle, ref id)).IsSuccess);
            borrow.Dispose();
            Assert.True(SpinWait.SpinUntil(() => handle.IsClosed, QarNativeAssert.Budget));
        }

        [Fact]
        public void TheAsyncRenderSenderCreationHoldsTheSessionUntilTheCompletionFires()
        {
            using var scope = _fixture.CreateSessionScope();
            var session = scope.Session;
            var handle = NativeHandles.Of(session);
            var state = new RenderSenderCreateAsyncState();
            Assert.Equal(handle.DangerousGetHandle(), state.HoldUntilCompletion(NativeHandles.ContextOf(session).Owner));
            session.Dispose();
            Assert.False(handle.IsClosed);
            state.Complete(Result<RenderSender>.Failure(Failed()));
            Assert.True(SpinWait.SpinUntil(() => handle.IsClosed, QarNativeAssert.Budget));
        }

        // A cancelled task completes at the cancel: the borrow is held until the native completion
        // all the same, so the native invite never loses its session.
        [Fact]
        public void ACancelledTargetAppInviteHoldsTheSessionUntilTheNativeCompletion()
        {
            using var scope = _fixture.CreateSessionScope();
            var session = scope.Session;
            var handle = NativeHandles.Of(session);
            var state = new TargetAppAsyncState<PeerId>(null, peer => PeerId.Default);
            using var cancellation = new CancellationTokenSource();
            Assert.True(state.UseStandardCancellation(cancellation.Token).IsSuccess);
            state.HoldUntilCompletion(NativeHandles.ContextOf(session).Owner);
            cancellation.Cancel();
            Assert.True(state.Task.IsCanceled);
            session.Dispose();
            Assert.False(handle.IsClosed);
            state.Complete(Result<PeerId>.Failure(Failed()));
            Assert.True(SpinWait.SpinUntil(() => handle.IsClosed, QarNativeAssert.Budget));
        }

        // The completion drops the borrow before the task completes, so the caller's Dispose after
        // the await releases the session at once, on the caller's thread.
        [Fact]
        public void ACompletedAsyncCallLeavesTheSessionToItsOwner()
        {
            using var scope = _fixture.CreateSessionScope();
            var session = scope.Session;
            var handle = NativeHandles.Of(session);
            var state = new RequestInviteAsyncState(null);
            state.HoldUntilCompletion(NativeHandles.ContextOf(session).Owner);
            state.Complete(Result<OnboardingInvite>.Failure(Failed()));
            Assert.True(state.Task.IsCompleted);
            Assert.False(handle.IsClosed);
            session.Dispose();
            Assert.True(handle.IsClosed);
        }

        [Fact]
        public void ADisposedSessionIsRefusedBeforeTheAsyncCall()
        {
            using var scope = _fixture.CreateSessionScope();
            var session = scope.Session;
            var renderSenders = session.RenderSenders;
            session.Dispose();
            var state = new RenderSenderCreateAsyncState();
            Assert.Throws<ObjectDisposedException>(() => state.HoldUntilCompletion(NativeHandles.ContextOf(session).Owner));
            state.Complete(Result<RenderSender>.Failure(Failed()));
            Assert.Throws<ObjectDisposedException>(() => session.InviteTargetAppAsync("hololens://127.0.0.1").GetAwaiter().GetResult());
            Assert.Throws<ObjectDisposedException>(() => renderSenders.CreateAsync(SenderBorrowTests.CpuOptions(PeerId.Unique())).GetAwaiter().GetResult());
        }
    }

    [Collection("QAR Native")]
    public sealed class SenderBorrowTests : IClassFixture<QarNativeFixture>
    {
        private readonly QarNativeFixture _fixture;

        public SenderBorrowTests(QarNativeFixture fixture)
        {
            _fixture = fixture;
        }

        private static ResultInfo Failed()
        {
            return new ResultInfo(StatusCode.LogicError, 0, "test completion");
        }

        internal static RenderSenderOptions CpuOptions(PeerId peerId, AppVolumeId? appVolume = null)
        {
            return new RenderSenderOptions(peerId, GraphicsAPI.Cpu)
            {
                AppVolumeId = appVolume,
                TextureLayout = FrameLayout.SideBySide,
                FrameViews = new[]
                {
                    new RenderFrameView(VideoFrameViewType.Color, VideoFrameViewEye.Left, 0),
                    new RenderFrameView(VideoFrameViewType.Color, VideoFrameViewEye.Right, 0),
                    new RenderFrameView(VideoFrameViewType.Depth, VideoFrameViewEye.Left, 0),
                    new RenderFrameView(VideoFrameViewType.Depth, VideoFrameViewEye.Right, 0)
                }
            };
        }

        private static RenderSender CreateSender(SessionTestScope scope, PeerId target)
        {
            using var cancellation = new CancellationTokenSource(QarNativeAssert.Budget);
            return scope.Session.RenderSenders.Create(CpuOptions(target, scope.CreateAppVolume()), cancellation.Token).ValueOrThrow();
        }

        private static PrimarylessVideoSender CreateVideoSender(Session session)
        {
            var layout = new VideoFrameLayout
            {
                Views = new[]
                {
                    new VideoFrameView(0, 0, 64, 64, 0, 0, PixelFormat.B8G8R8A8, VideoFrameViewType.Color, VideoFrameViewEye.Left)
                },
                Textures = new[] { new TextureSize(PixelFormat.B8G8R8A8, 64, 64, 1) }
            };
            return session.PrimarylessVideoSenders.CreateCpu(new PrimarylessVideoSenderOptions(layout)).ValueOrThrow();
        }

        [Fact]
        public void ACallHoldsTheRenderSenderUntilItIsDone()
        {
            using var scope = _fixture.CreateSessionScope();
            var target = scope.StartRenderTarget();
            var sender = CreateSender(scope, target.PeerId);
            var handle = NativeHandles.Of(sender);
            var borrow = NativeHandles.OwnerOf(sender).Borrow();
            Assert.Equal(handle.DangerousGetHandle(), borrow.Handle);
            sender.Dispose();
            Assert.False(handle.IsClosed);
            Assert.Throws<ObjectDisposedException>(() => sender.GetLayout());
            Assert.Throws<ObjectDisposedException>(() => sender.TryBeginFrame());
            Assert.Throws<ObjectDisposedException>(() => sender.ShowFrame());
            Assert.Throws<ObjectDisposedException>(() => sender.BeginFrameAsync().GetAwaiter().GetResult());
            StreamIdNative streamId;
            Assert.True(Results.ToInfo(NativeMethods.qar_impl_render_sender_stream_id(borrow.Handle, out streamId)).IsSuccess);
            borrow.Dispose();
            Assert.True(SpinWait.SpinUntil(() => handle.IsClosed, QarNativeAssert.Budget));
        }

        [Fact]
        public void TheAsyncBeginFrameHoldsTheSenderUntilTheCompletionFires()
        {
            using var scope = _fixture.CreateSessionScope();
            var target = scope.StartRenderTarget();
            var sender = CreateSender(scope, target.PeerId);
            var handle = NativeHandles.Of(sender);
            var state = new BeginFrameAsyncState();
            Assert.Equal(handle.DangerousGetHandle(), state.HoldUntilCompletion(NativeHandles.OwnerOf(sender)));
            sender.Dispose();
            Assert.False(handle.IsClosed);
            state.Complete(Result<RenderFrameInfo>.Failure(Failed()));
            Assert.True(SpinWait.SpinUntil(() => handle.IsClosed, QarNativeAssert.Budget));
        }

        // The widest window of the gap: a render thread waits in BeginFrame through a paused
        // stream, with no token of its own, while the main thread disposes the sender (Unity's
        // OnDestroy). The sender is unusable at once, the Dispose ends the waiting call, and the
        // native sender is released only after that call returned.
        [Fact]
        public async Task DisposingTheSenderDuringABlockedBeginFrameDefersItsRelease()
        {
            using var scope = _fixture.CreateSessionScope();
            var peer = scope.CreatePeer("QarStreamingSharpTests paused target");
            using var receiver = peer.CreateRenderReceiver(ExtLayouts.SideBySide(64, 64));
            var sender = CreateSender(scope, peer.Id);
            var handle = NativeHandles.Of(sender);
            Assert.True(receiver.Video.WaitConnected(QarNativeAssert.Budget), "the render receiver never connected");
            receiver.Video.Pause().EnsureSuccess();
            var call = StartBlockedBeginFrame(sender);
            sender.Dispose();
            // Read the handle before the call state: a call still running at the second read was
            // running at the first one, so its sender must have been alive then.
            var closed = handle.IsClosed;
            var stillRunning = !call.IsCompleted;
            Assert.False(closed && stillRunning, "the sender was released while BeginFrame still used it");
            Assert.Throws<ObjectDisposedException>(() => sender.GetLayout());
            Assert.Throws<ObjectDisposedException>(() => sender.BeginFrame());
            var ended = await Task.WhenAny(call, Task.Delay(QarNativeAssert.Budget)) == call;
            Assert.True(ended, "the Dispose did not end the waiting BeginFrame");
            var result = await call;
            if (result.IsSuccess)
                result.Value.Dispose();
            Assert.False(result.IsSuccess);
            Assert.True(SpinWait.SpinUntil(() => handle.IsClosed, QarNativeAssert.Budget));
        }

        // Retries until one BeginFrame stays blocked: the receiver's pause reaches the sender
        // asynchronously, so an early call may still begin a frame. No token of the caller's: the
        // render loop's path, which only the sender's own dispose token can end.
        private static Task<Result<RenderFrameInfo>> StartBlockedBeginFrame(RenderSender sender)
        {
            var deadline = DateTime.UtcNow + QarNativeAssert.Budget;
            while (DateTime.UtcNow < deadline)
            {
                var call = Task.Run(() => sender.BeginFrame());
                if (!call.Wait(ExtTime.Scaled(TimeSpan.FromMilliseconds(500))))
                    return call;
                if (call.Result.IsSuccess)
                    call.Result.Value.Dispose();
            }
            throw new Xunit.Sdk.XunitException("BeginFrame never waited on the paused stream.");
        }

        [Fact]
        public void ACallHoldsTheFrameInfoUntilItIsDone()
        {
            using var scope = _fixture.CreateSessionScope();
            var target = scope.StartRenderTarget();
            using var sender = CreateSender(scope, target.PeerId);
            var frame = QarNativeAssert.WaitUntil(
                () =>
                {
                    var result = sender.BeginFrame();
                    return result.IsSuccess ? result.Value : null;
                },
                begun => begun != null,
                "render sender begin frame",
                timeoutMs: 15000,
                intervalMs: 100)!;
            var handle = NativeHandles.Of(frame);
            var borrow = NativeHandles.OwnerOf(frame).Borrow();
            Assert.Equal(handle.DangerousGetHandle(), borrow.Handle);
            frame.Dispose();
            Assert.False(handle.IsClosed);
            Assert.Throws<ObjectDisposedException>(() => frame.GetViewPose(0));
            Assert.Throws<ObjectDisposedException>(() => frame.GetViewFov(0));
            PoseNative pose;
            Assert.True(Results.ToInfo(NativeMethods.qar_impl_render_frame_info_get_view_pose(borrow.Handle, UIntPtr.Zero, out pose)).IsSuccess);
            borrow.Dispose();
            Assert.True(SpinWait.SpinUntil(() => handle.IsClosed, QarNativeAssert.Budget));
        }

        [Fact]
        public void ACallHoldsThePrimarylessVideoSenderUntilItIsDone()
        {
            using var scope = _fixture.CreateSessionScope();
            var sender = CreateVideoSender(scope.Session);
            var handle = NativeHandles.Of(sender);
            var borrow = NativeHandles.OwnerOf(sender).Borrow();
            Assert.Equal(handle.DangerousGetHandle(), borrow.Handle);
            sender.Dispose();
            Assert.False(handle.IsClosed);
            Assert.Throws<ObjectDisposedException>(() => sender.GetStreamId());
            Assert.Throws<ObjectDisposedException>(() => sender.TryBeginFrame());
            Assert.Throws<ObjectDisposedException>(() => sender.ShowFrame());
            StreamIdNative streamId;
            Assert.True(Results.ToInfo(NativeMethods.qar_impl_video_sender_stream_id(borrow.Handle, out streamId)).IsSuccess);
            borrow.Dispose();
            Assert.True(SpinWait.SpinUntil(() => handle.IsClosed, QarNativeAssert.Budget));
        }

        // Not disposed: releasing a borrow only drops a reference, the owner's Dispose releases
        // the sender synchronously on its own thread.
        [Fact]
        public void AReleasedBorrowLeavesTheSenderToItsOwner()
        {
            using var scope = _fixture.CreateSessionScope();
            var sender = CreateVideoSender(scope.Session);
            var handle = NativeHandles.Of(sender);
            using (NativeHandles.OwnerOf(sender).Borrow())
                Assert.True(sender.GetStreamId().IsSuccess);
            Assert.False(handle.IsClosed);
            sender.Dispose();
            Assert.True(handle.IsClosed);
            sender.Dispose();
        }
    }
}
