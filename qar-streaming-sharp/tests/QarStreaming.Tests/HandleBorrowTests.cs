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
    /// Runs a Dispose on a thread of its own, so a test can hold a borrow while it waits, and checks
    /// whether it waits or returned.
    /// </summary>
    internal static class DisposeProbe
    {
        internal static Task Start(Action dispose)
        {
            return Task.Factory.StartNew(dispose, CancellationToken.None, TaskCreationOptions.LongRunning, TaskScheduler.Default);
        }

        /// <summary>Starts <paramref name="dispose"/> and returns the managed id of the thread that runs it.</summary>
        internal static Task<int> StartRecordingThread(Action dispose)
        {
            return Task.Factory.StartNew(
                () =>
                {
                    dispose();
                    return Environment.CurrentManagedThreadId;
                },
                CancellationToken.None,
                TaskCreationOptions.LongRunning,
                TaskScheduler.Default);
        }

        internal static void AssertWaiting(Task dispose)
        {
            Assert.False(dispose.Wait(ExtTime.Scaled(TimeSpan.FromMilliseconds(200))), "Dispose returned while a call still borrowed the object");
        }

        internal static void AssertReturned(Task dispose)
        {
            Assert.True(dispose.Wait(QarNativeAssert.Budget), "Dispose did not return after the last borrow was released");
        }

        /// <summary>Waits until the Dispose running on another thread marked the owner disposed.</summary>
        internal static void WaitMarkedDisposed(NativeHandleOwner owner)
        {
            Assert.True(SpinWait.SpinUntil(() => owner.IsDisposed, QarNativeAssert.Budget), "Dispose never marked the object disposed");
        }
    }

    /// <summary>A handle that records the thread its release ran on, in place of a native object.</summary>
    internal sealed class RecordingSafeHandle : SafeHandle
    {
        private int _releasedOnThread = -1;

        public RecordingSafeHandle()
            : base(IntPtr.Zero, true)
        {
            SetHandle(new IntPtr(1));
        }

        public override bool IsInvalid { get { return handle == IntPtr.Zero; } }

        /// <summary>The managed thread id the release ran on, or -1 before it.</summary>
        public int ReleasedOnThread { get { return Volatile.Read(ref _releasedOnThread); } }

        protected override bool ReleaseHandle()
        {
            Volatile.Write(ref _releasedOnThread, Environment.CurrentManagedThreadId);
            return true;
        }
    }

    /// <summary>
    /// The dispose contract of every wrapper that owns a native object (hub G56): Dispose refuses
    /// new calls, cancels the waiting ones, waits until every borrow is released and destroys the
    /// object on its own thread; from inside one of the object's own callbacks it throws. Offline:
    /// a recording handle stands in for the native object.
    /// </summary>
    public sealed class NativeHandleOwnerDisposeTests
    {
        private static ResultInfo Failed()
        {
            return new ResultInfo(StatusCode.LogicError, 0, "test completion");
        }

        [Fact]
        public async Task DisposeWaitsForTheRunningCallAndDestroysOnTheDisposingThread()
        {
            var handle = new RecordingSafeHandle();
            var owner = new NativeHandleOwner(handle, "Probe");
            var call = owner.Borrow();
            var dispose = DisposeProbe.StartRecordingThread(owner.Dispose);
            DisposeProbe.WaitMarkedDisposed(owner);
            Assert.Throws<ObjectDisposedException>(() => owner.Borrow());
            DisposeProbe.AssertWaiting(dispose);
            Assert.False(handle.IsClosed);
            call.Dispose();
            DisposeProbe.AssertReturned(dispose);
            // Destroyed before Dispose returned, on the thread that called it.
            Assert.True(handle.IsClosed);
            Assert.Equal(await dispose, handle.ReleasedOnThread);
        }

        // The native completion fires on a library thread; the release there only wakes the Dispose.
        [Fact]
        public async Task DisposeWaitsForAnAsyncCallsCompletionAndDestroysOnTheDisposingThread()
        {
            var handle = new RecordingSafeHandle();
            var owner = new NativeHandleOwner(handle, "Probe");
            var state = new BeginFrameAsyncState();
            state.HoldUntilCompletion(owner);
            var dispose = DisposeProbe.StartRecordingThread(owner.Dispose);
            DisposeProbe.WaitMarkedDisposed(owner);
            DisposeProbe.AssertWaiting(dispose);
            var completion = Task.Factory.StartNew(
                () =>
                {
                    state.Complete(Result<RenderFrameInfo>.Failure(Failed()));
                    return Environment.CurrentManagedThreadId;
                },
                CancellationToken.None,
                TaskCreationOptions.LongRunning,
                TaskScheduler.Default);
            DisposeProbe.AssertReturned(dispose);
            Assert.True(handle.IsClosed);
            Assert.Equal(await dispose, handle.ReleasedOnThread);
            var completed = await Task.WhenAny(completion, Task.Delay(QarNativeAssert.Budget)) == completion;
            Assert.True(completed, "the completion thread did not finish");
            Assert.NotEqual(await completion, handle.ReleasedOnThread);
        }

        // Dispose cancels the waiting calls before it waits: a call that ends on the cancel lets
        // the Dispose return on the same thread, without another thread's help.
        [Fact]
        public void DisposeCancelsTheWaitingCallsBeforeItWaits()
        {
            var handle = new RecordingSafeHandle();
            var owner = new NativeHandleOwner(handle, "Probe");
            var call = owner.Borrow();
            using var endsOnCancel = owner.Disposing.Register(() => call.Dispose());
            owner.Dispose();
            Assert.True(handle.IsClosed);
            Assert.Equal(Environment.CurrentManagedThreadId, handle.ReleasedOnThread);
        }

        [Fact]
        public async Task ASecondDisposeWaitsForTheFirstOnesDestroy()
        {
            var handle = new RecordingSafeHandle();
            var owner = new NativeHandleOwner(handle, "Probe");
            var call = owner.Borrow();
            var first = DisposeProbe.StartRecordingThread(owner.Dispose);
            DisposeProbe.WaitMarkedDisposed(owner);
            var second = DisposeProbe.Start(owner.Dispose);
            DisposeProbe.AssertWaiting(second);
            call.Dispose();
            DisposeProbe.AssertReturned(first);
            DisposeProbe.AssertReturned(second);
            Assert.True(handle.IsClosed);
            Assert.Equal(await first, handle.ReleasedOnThread);
        }

        // From inside one of its own callbacks Dispose would wait for itself: it throws and leaves
        // the object as it was. Another object without a running call is destroyed right there.
        [Fact]
        public void DisposeFromInsideAnOwnCallbackThrows()
        {
            var handle = new RecordingSafeHandle();
            var owner = new NativeHandleOwner(handle, "Probe");
            var other = new RecordingSafeHandle();
            var otherOwner = new NativeHandleOwner(other, "Other");
            using (NativeHandleOwner.EnterCallback(owner))
            {
                var thrown = Assert.Throws<InvalidOperationException>(() => owner.Dispose());
                Assert.Contains("Probe", thrown.Message);
                otherOwner.Dispose();
                Assert.True(other.IsClosed);
            }
            Assert.False(owner.IsDisposed);
            Assert.False(handle.IsClosed);
            owner.Dispose();
            Assert.True(handle.IsClosed);
        }

        // From another object's callback Dispose would wait on a library thread the running call
        // may need: it throws while a call runs and leaves the object usable.
        [Fact]
        public void DisposeFromAnotherObjectsCallbackWhileACallRunsThrows()
        {
            var callbackOwner = new NativeHandleOwner(new RecordingSafeHandle(), "Session");
            var handle = new RecordingSafeHandle();
            var owner = new NativeHandleOwner(handle, "Sender");
            var call = owner.Borrow();
            using (NativeHandleOwner.EnterCallback(callbackOwner))
                Assert.Throws<InvalidOperationException>(() => owner.Dispose());
            Assert.False(owner.IsDisposed);
            call.Dispose();
            owner.Dispose();
            Assert.True(handle.IsClosed);
        }

        // The first Dispose may be waiting for the very callback that repeats it: the repeat
        // returns at once instead of throwing or waiting, and the first one destroys.
        [Fact]
        public async Task ARepeatedDisposeFromACallbackReturnsAtOnce()
        {
            var handle = new RecordingSafeHandle();
            var owner = new NativeHandleOwner(handle, "Probe");
            var call = owner.Borrow();
            var first = DisposeProbe.StartRecordingThread(owner.Dispose);
            DisposeProbe.WaitMarkedDisposed(owner);
            using (NativeHandleOwner.EnterCallback(owner))
                owner.Dispose();
            Assert.False(handle.IsClosed);
            call.Dispose();
            DisposeProbe.AssertReturned(first);
            Assert.Equal(await first, handle.ReleasedOnThread);
        }

        private sealed class DisposingProgress : IProgress<ActionProgress>
        {
            private readonly Action _dispose;

            public DisposingProgress(Action dispose)
            {
                _dispose = dispose;
            }

            public Exception? Thrown { get; private set; }

            public void Report(ActionProgress value)
            {
                try
                {
                    _dispose();
                }
                catch (Exception ex)
                {
                    Thrown = ex;
                }
            }
        }

        // A progress report runs while the reporting call holds its borrow.
        [Fact]
        public void DisposeFromAProgressReportOfTheObjectsCallThrows()
        {
            var handle = new RecordingSafeHandle();
            var owner = new NativeHandleOwner(handle, "Probe");
            var progress = new DisposingProgress(owner.Dispose);
            using (var call = owner.Borrow())
            using (var state = ProgressState.Create(progress, owner))
                state.Callback(ActionSeverity.Info, 50.0f, IntPtr.Zero, state.UserState);
            Assert.IsType<InvalidOperationException>(progress.Thrown);
            Assert.False(owner.IsDisposed);
            owner.Dispose();
            Assert.True(handle.IsClosed);
        }

        [Fact]
        public void DisposeFromAnAsyncCallsProgressReportThrows()
        {
            var handle = new RecordingSafeHandle();
            var owner = new NativeHandleOwner(handle, "Probe");
            var progress = new DisposingProgress(owner.Dispose);
            var state = new RequestInviteAsyncState(progress);
            state.HoldUntilCompletion(owner);
            state.ProgressCallback(ActionSeverity.Info, 50.0f, IntPtr.Zero, state.UserState);
            Assert.IsType<InvalidOperationException>(progress.Thrown);
            Assert.False(owner.IsDisposed);
            state.Complete(Result<OnboardingInvite>.Failure(Failed()));
            owner.Dispose();
            Assert.True(handle.IsClosed);
        }
    }

    /// <summary>
    /// Hub G56: every native call on a cancel token holds a reference on its handle until the call
    /// is done (an async call until its native completion fires). A token is the one wrapper whose
    /// Dispose does not wait: its last borrow destroys it, on whatever thread lets go last.
    /// </summary>
    // Creates native objects: serialized with the fixtures whose Library.Destroy() would
    // destroy them mid-test (it ends every cancel token).
    [Collection("QAR Native")]
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
            Assert.True(handle.IsClosed);
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
            Assert.True(handle.IsClosed);
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
            Assert.True(handle.IsClosed);
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

        // Disposing the object an async call was made on cancels the call's native token first, so
        // a call whose completion only the object's teardown would bring still completes; Dispose
        // then waits for that completion, and the task reports its failure.
        [Fact]
        public void DisposingTheTargetCancelsAWaitingAsyncCallAndWaitsForItsCompletion()
        {
            var target = new RecordingSafeHandle();
            var owner = new NativeHandleOwner(target, "Probe");
            var state = new BeginFrameAsyncState();
            var tokenHandle = state.UseStandardCancellation(CancellationToken.None, owner).ValueOrThrow();
            Assert.NotEqual(IntPtr.Zero, tokenHandle);
            state.HoldUntilCompletion(owner);
            Assert.False(NativeMethods.qar_impl_cancel_token_is_cancelled(tokenHandle));
            var dispose = DisposeProbe.Start(owner.Dispose);
            Assert.True(SpinWait.SpinUntil(() => NativeMethods.qar_impl_cancel_token_is_cancelled(tokenHandle), QarNativeAssert.Budget));
            DisposeProbe.AssertWaiting(dispose);
            Assert.False(state.Task.IsCompleted);
            Assert.False(target.IsClosed);
            state.Complete(Result<RenderFrameInfo>.Failure(Failed()));
            Assert.True(state.Task.IsCompleted);
            DisposeProbe.AssertReturned(dispose);
            Assert.True(target.IsClosed);
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
            Assert.True(shared.IsClosed);
        }
    }

    /// <summary>
    /// The runtime's waiting calls are linked to its Dispose like the session's: Dispose cancels
    /// the async onboard, waits for its completion and only then releases the runtime.
    /// </summary>
    [Collection("QAR Native")]
    public sealed class RuntimeDisposeTests : IClassFixture<QarNativeFixture>
    {
        public RuntimeDisposeTests(QarNativeFixture fixture)
        {
            _ = fixture;
        }

        [Fact]
        public void DisposingTheRuntimeCancelsAWaitingAsyncOnboardAndWaitsForIt()
        {
            var storageRoot = System.IO.Path.Combine(System.IO.Path.GetTempPath(), "qar-sharp-tests", Guid.NewGuid().ToString("N"));
            var runtime = Runtime.Create(new RuntimeOptions { StorageFolderPath = storageRoot }).ValueOrThrow();
            var handle = NativeHandles.Of(runtime);
            var owner = NativeHandles.OwnerOf(runtime);
            var state = new RuntimeOnboardAsyncState(null, true);
            var tokenHandle = state.UseStandardCancellation(CancellationToken.None, owner).ValueOrThrow();
            state.HoldUntilCompletion(owner);
            Assert.False(NativeMethods.qar_impl_cancel_token_is_cancelled(tokenHandle));
            var dispose = DisposeProbe.Start(runtime.Dispose);
            Assert.True(SpinWait.SpinUntil(() => NativeMethods.qar_impl_cancel_token_is_cancelled(tokenHandle), QarNativeAssert.Budget));
            DisposeProbe.AssertWaiting(dispose);
            Assert.False(handle.IsClosed);
            state.CompleteOnboard(Result<OnboardSession>.Failure(new ResultInfo(StatusCode.LogicError, 0, "test completion")));
            DisposeProbe.AssertReturned(dispose);
            Assert.True(handle.IsClosed);
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
        public void DisposeWaitsForACallOnTheSession()
        {
            using var scope = _fixture.CreateSessionScope();
            var session = scope.Session;
            var handle = NativeHandles.Of(session);
            var context = NativeHandles.ContextOf(session);
            var borrow = context.Borrow();
            Assert.Equal(handle.DangerousGetHandle(), borrow.Handle);
            var dispose = DisposeProbe.Start(session.Dispose);
            DisposeProbe.WaitMarkedDisposed(context.Owner);
            Assert.Throws<ObjectDisposedException>(() => session.GetId());
            Assert.Throws<ObjectDisposedException>(() => session.Peers.GetMySpec());
            Assert.Throws<ObjectDisposedException>(() => session.Peers.GetAll());
            Assert.Throws<ObjectDisposedException>(() => session.AppVolumes.GetLatestPose(default(AppVolumeId)));
            Assert.Throws<ObjectDisposedException>(() => session.GuiPanels.Close(default(GuiPanelId)));
            Assert.Throws<ObjectDisposedException>(() => session.Peers.SubscribeUpdates((sender, args) => { }));
            Assert.Throws<ObjectDisposedException>(() => context.Borrow());
            DisposeProbe.AssertWaiting(dispose);
            // The call that borrowed the session still reads a live native session.
            var id = SessionIdNative.CreateDefault();
            Assert.True(Results.ToInfo(NativeMethods.qar_impl_session_get_id(borrow.Handle, ref id)).IsSuccess);
            borrow.Dispose();
            DisposeProbe.AssertReturned(dispose);
            Assert.True(handle.IsClosed);
        }

        [Fact]
        public void DisposeWaitsForTheAsyncRenderSenderCreationsCompletion()
        {
            using var scope = _fixture.CreateSessionScope();
            var session = scope.Session;
            var handle = NativeHandles.Of(session);
            var state = new RenderSenderCreateAsyncState();
            Assert.Equal(handle.DangerousGetHandle(), state.HoldUntilCompletion(NativeHandles.ContextOf(session).Owner));
            var dispose = DisposeProbe.Start(session.Dispose);
            DisposeProbe.AssertWaiting(dispose);
            Assert.False(handle.IsClosed);
            state.Complete(Result<RenderSender>.Failure(Failed()));
            DisposeProbe.AssertReturned(dispose);
            Assert.True(handle.IsClosed);
        }

        // A cancelled task completes at the cancel: the borrow is held until the native completion
        // all the same, so the native invite never loses its session, and Dispose waits for it.
        [Fact]
        public void DisposeAfterACancelledTargetAppInviteWaitsForTheNativeCompletion()
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
            var dispose = DisposeProbe.Start(session.Dispose);
            DisposeProbe.AssertWaiting(dispose);
            Assert.False(handle.IsClosed);
            state.Complete(Result<PeerId>.Failure(Failed()));
            DisposeProbe.AssertReturned(dispose);
            Assert.True(handle.IsClosed);
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
            // Both throw before they return a task, so the call itself is checked, without waiting.
            Assert.Throws<ObjectDisposedException>(() => { _ = session.InviteTargetAppAsync("hololens://127.0.0.1"); });
            Assert.Throws<ObjectDisposedException>(() => { _ = renderSenders.CreateAsync(SenderBorrowTests.CpuOptions(PeerId.Unique())); });
        }

        // A handler running on another thread holds the session's teardown: when Dispose returns
        // it has finished and no handler call starts afterwards.
        [Fact]
        public async Task TheSessionsSubscriptionsHaveEndedWhenItsDisposeReturns()
        {
            using var scope = _fixture.CreateSessionScope();
            var session = scope.Session;
            using var entered = new ManualResetEventSlim(false);
            using var release = new ManualResetEventSlim(false);
            var running = 0;
            var calls = 0;
            var callsWhenDisposed = -1;
            using var subscription = session.Peers.SubscribeUpdates(
                (sender, args) =>
                {
                    Interlocked.Increment(ref calls);
                    Interlocked.Increment(ref running);
                    entered.Set();
                    release.Wait(QarNativeAssert.Budget);
                    Interlocked.Decrement(ref running);
                }).ValueOrThrow();
            QarNativeAssert.WaitUntil(
                () =>
                {
                    session.Peers.UpdateDisplayName("dispose-waits-for-handler").EnsureSuccess();
                    return entered.IsSet;
                },
                "handler entered");
            var dispose = DisposeProbe.Start(
                () =>
                {
                    session.Dispose();
                    // Read by the disposing thread the moment Dispose returns.
                    Volatile.Write(ref callsWhenDisposed, Volatile.Read(ref calls));
                    Assert.Equal(0, Volatile.Read(ref running));
                });
            DisposeProbe.AssertWaiting(dispose);
            release.Set();
            DisposeProbe.AssertReturned(dispose);
            // Rethrows an assertion the disposing thread failed.
            await dispose;
            Thread.Sleep(ExtTime.Scaled(TimeSpan.FromMilliseconds(300)));
            Assert.Equal(Volatile.Read(ref callsWhenDisposed), Volatile.Read(ref calls));
        }

        // Dispose from the session's own handler would wait for the handler: it throws and the
        // session stays usable.
        [Fact]
        public async Task DisposingTheSessionFromItsOwnSubscriptionHandlerThrows()
        {
            using var scope = _fixture.CreateSessionScope();
            var session = scope.Session;
            var thrown = new TaskCompletionSource<Exception>();
            using var subscription = session.Peers.SubscribeUpdates(
                (sender, args) =>
                {
                    if (thrown.Task.IsCompleted)
                        return;
                    try
                    {
                        session.Dispose();
                    }
                    catch (Exception ex)
                    {
                        thrown.TrySetResult(ex);
                    }
                }).ValueOrThrow();
            QarNativeAssert.WaitUntil(
                () =>
                {
                    session.Peers.UpdateDisplayName("dispose-from-handler").EnsureSuccess();
                    return thrown.Task.IsCompleted;
                },
                "handler tried to dispose the session");
            Assert.IsType<InvalidOperationException>(await thrown.Task);
            Assert.True(session.GetId().IsSuccess);
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
        public void DisposeWaitsForACallOnTheRenderSender()
        {
            using var scope = _fixture.CreateSessionScope();
            var target = scope.StartRenderTarget();
            var sender = CreateSender(scope, target.PeerId);
            var handle = NativeHandles.Of(sender);
            var owner = NativeHandles.OwnerOf(sender);
            var borrow = owner.Borrow();
            Assert.Equal(handle.DangerousGetHandle(), borrow.Handle);
            var dispose = DisposeProbe.Start(sender.Dispose);
            DisposeProbe.WaitMarkedDisposed(owner);
            Assert.Throws<ObjectDisposedException>(() => sender.GetLayout());
            Assert.Throws<ObjectDisposedException>(() => sender.TryBeginFrame());
            Assert.Throws<ObjectDisposedException>(() => sender.ShowFrame());
            // Throws before it returns a task, so the call itself is checked, without waiting.
            Assert.Throws<ObjectDisposedException>(() => { _ = sender.BeginFrameAsync(); });
            DisposeProbe.AssertWaiting(dispose);
            StreamIdNative streamId;
            Assert.True(Results.ToInfo(NativeMethods.qar_impl_render_sender_stream_id(borrow.Handle, out streamId)).IsSuccess);
            borrow.Dispose();
            DisposeProbe.AssertReturned(dispose);
            Assert.True(handle.IsClosed);
        }

        [Fact]
        public void DisposeWaitsForTheAsyncBeginFramesCompletion()
        {
            using var scope = _fixture.CreateSessionScope();
            var target = scope.StartRenderTarget();
            var sender = CreateSender(scope, target.PeerId);
            var handle = NativeHandles.Of(sender);
            var state = new BeginFrameAsyncState();
            Assert.Equal(handle.DangerousGetHandle(), state.HoldUntilCompletion(NativeHandles.OwnerOf(sender)));
            var dispose = DisposeProbe.Start(sender.Dispose);
            DisposeProbe.AssertWaiting(dispose);
            Assert.False(handle.IsClosed);
            state.Complete(Result<RenderFrameInfo>.Failure(Failed()));
            DisposeProbe.AssertReturned(dispose);
            Assert.True(handle.IsClosed);
        }

        // The widest window of the gap: a render thread waits in BeginFrame through a paused
        // stream, with no token of its own, while another thread disposes the sender (Unity's
        // OnDestroy). The sender is unusable at once, the Dispose ends the waiting call, waits for
        // it and destroys the native sender before it returns.
        [Fact]
        public async Task DisposingTheSenderDuringABlockedBeginFrameEndsItAndDestroysBeforeReturning()
        {
            using var scope = _fixture.CreateSessionScope();
            var peer = scope.CreatePeer("QarStreamingSharpTests paused target");
            using var receiver = peer.CreateRenderReceiver(ExtLayouts.SideBySide(64, 64));
            var sender = CreateSender(scope, peer.Id);
            var handle = NativeHandles.Of(sender);
            Assert.True(receiver.Video.WaitConnected(QarNativeAssert.Budget), "the render receiver never connected");
            receiver.Video.Pause().EnsureSuccess();
            var call = StartBlockedBeginFrame(sender);
            var dispose = DisposeProbe.Start(sender.Dispose);
            DisposeProbe.AssertReturned(dispose);
            Assert.True(handle.IsClosed, "Dispose returned before the native sender was destroyed");
            Assert.Throws<ObjectDisposedException>(() => sender.GetLayout());
            Assert.Throws<ObjectDisposedException>(() => sender.BeginFrame());
            var ended = await Task.WhenAny(call, Task.Delay(QarNativeAssert.Budget)) == call;
            Assert.True(ended, "the Dispose did not end the waiting BeginFrame");
            var result = await call;
            if (result.IsSuccess)
                result.Value.Dispose();
            Assert.False(result.IsSuccess);
        }

        // A cancelled task completes at the cancel, before the native completion; the Dispose after
        // it waits for that completion, which the cancel brings, and returns with the sender gone.
        [Fact]
        public void DisposeAfterACancelledBeginFrameAsyncWaitsForItsCompletion()
        {
            using var scope = _fixture.CreateSessionScope();
            var peer = scope.CreatePeer("QarStreamingSharpTests paused target");
            using var receiver = peer.CreateRenderReceiver(ExtLayouts.SideBySide(64, 64));
            var sender = CreateSender(scope, peer.Id);
            var handle = NativeHandles.Of(sender);
            Assert.True(receiver.Video.WaitConnected(QarNativeAssert.Budget), "the render receiver never connected");
            receiver.Video.Pause().EnsureSuccess();
            using var cancellation = new CancellationTokenSource();
            var call = StartBlockedBeginFrameAsync(sender, cancellation.Token);
            cancellation.Cancel();
            Assert.True(call.IsCanceled);
            var dispose = DisposeProbe.Start(sender.Dispose);
            DisposeProbe.AssertReturned(dispose);
            Assert.True(handle.IsClosed);
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

        private static Task<Result<RenderFrameInfo>> StartBlockedBeginFrameAsync(RenderSender sender, CancellationToken cancellationToken)
        {
            var deadline = DateTime.UtcNow + QarNativeAssert.Budget;
            while (DateTime.UtcNow < deadline)
            {
                var call = sender.BeginFrameAsync(cancellationToken);
                if (!call.Wait(ExtTime.Scaled(TimeSpan.FromMilliseconds(500))))
                    return call;
                if (call.Result.IsSuccess)
                    call.Result.Value.Dispose();
            }
            throw new Xunit.Sdk.XunitException("BeginFrameAsync never waited on the paused stream.");
        }

        [Fact]
        public void DisposeWaitsForACallOnTheFrameInfo()
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
            var owner = NativeHandles.OwnerOf(frame);
            var borrow = owner.Borrow();
            Assert.Equal(handle.DangerousGetHandle(), borrow.Handle);
            var dispose = DisposeProbe.Start(frame.Dispose);
            DisposeProbe.WaitMarkedDisposed(owner);
            Assert.Throws<ObjectDisposedException>(() => frame.GetViewPose(0));
            Assert.Throws<ObjectDisposedException>(() => frame.GetViewFov(0));
            DisposeProbe.AssertWaiting(dispose);
            PoseNative pose;
            Assert.True(Results.ToInfo(NativeMethods.qar_impl_render_frame_info_get_view_pose(borrow.Handle, UIntPtr.Zero, out pose)).IsSuccess);
            borrow.Dispose();
            DisposeProbe.AssertReturned(dispose);
            Assert.True(handle.IsClosed);
        }

        [Fact]
        public void DisposeWaitsForACallOnThePrimarylessVideoSender()
        {
            using var scope = _fixture.CreateSessionScope();
            var sender = CreateVideoSender(scope.Session);
            var handle = NativeHandles.Of(sender);
            var owner = NativeHandles.OwnerOf(sender);
            var borrow = owner.Borrow();
            Assert.Equal(handle.DangerousGetHandle(), borrow.Handle);
            var dispose = DisposeProbe.Start(sender.Dispose);
            DisposeProbe.WaitMarkedDisposed(owner);
            Assert.Throws<ObjectDisposedException>(() => sender.GetStreamId());
            Assert.Throws<ObjectDisposedException>(() => sender.TryBeginFrame());
            Assert.Throws<ObjectDisposedException>(() => sender.ShowFrame());
            DisposeProbe.AssertWaiting(dispose);
            StreamIdNative streamId;
            Assert.True(Results.ToInfo(NativeMethods.qar_impl_video_sender_stream_id(borrow.Handle, out streamId)).IsSuccess);
            borrow.Dispose();
            DisposeProbe.AssertReturned(dispose);
            Assert.True(handle.IsClosed);
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
