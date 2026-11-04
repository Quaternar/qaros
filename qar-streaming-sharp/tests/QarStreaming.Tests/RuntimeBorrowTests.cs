using System;
using System.IO;
using System.Threading;
using Quaternar.Qaros;
using Quaternar.Qaros.Interop;
using Xunit;

namespace Quaternar.Qaros.Tests
{
    /// <summary>
    /// The runtime handle is borrowed for every native call that takes it: an async onboard or
    /// rejoin holds it until its native completion fires, and a <see cref="Runtime.Dispose"/> while
    /// the call runs waits for that completion, then releases the runtime on its own thread before
    /// it returns. Offline: the completion is driven directly.
    /// </summary>
    [Collection("QAR Native")]
    public sealed class RuntimeBorrowTests : IClassFixture<QarNativeFixture>
    {
        public RuntimeBorrowTests(QarNativeFixture fixture)
        {
            _ = fixture;
        }

        private static Runtime CreateRuntime()
        {
            var storageRoot = Path.Combine(Path.GetTempPath(), "qar-sharp-tests", Guid.NewGuid().ToString("N"));
            return Runtime.Create(new RuntimeOptions { StorageFolderPath = storageRoot }).ValueOrThrow();
        }

        private static ResultInfo Failed()
        {
            return new ResultInfo(StatusCode.LogicError, 0, "test completion");
        }

        [Fact]
        public void DisposeWaitsForTheAsyncOnboardsCompletion()
        {
            var runtime = CreateRuntime();
            var handle = NativeHandles.Of(runtime);
            var owner = NativeHandles.OwnerOf(runtime);
            var state = new RuntimeOnboardAsyncState(null, true);
            Assert.Equal(handle.DangerousGetHandle(), state.HoldUntilCompletion(owner));
            var dispose = DisposeProbe.Start(runtime.Dispose);
            DisposeProbe.WaitMarkedDisposed(owner);
            Assert.Throws<ObjectDisposedException>(() => owner.Borrow());
            DisposeProbe.AssertWaiting(dispose);
            Assert.False(handle.IsClosed);
            state.CompleteOnboard(Result<OnboardSession>.Failure(Failed()));
            DisposeProbe.AssertReturned(dispose);
            Assert.True(handle.IsClosed);
        }

        [Fact]
        public void DisposeWaitsForTheAsyncRejoinsCompletion()
        {
            var runtime = CreateRuntime();
            var handle = NativeHandles.Of(runtime);
            var state = new RuntimeOnboardAsyncState(null, false);
            state.HoldUntilCompletion(NativeHandles.OwnerOf(runtime));
            var dispose = DisposeProbe.Start(runtime.Dispose);
            DisposeProbe.AssertWaiting(dispose);
            Assert.False(handle.IsClosed);
            state.CompleteRejoin(Result<Session>.Failure(Failed()));
            DisposeProbe.AssertReturned(dispose);
            Assert.True(handle.IsClosed);
        }

        // The completion drops the borrow before the task completes, so the caller's Dispose after
        // the await releases the runtime at once, on the caller's thread.
        [Fact]
        public void ACompletedAsyncCallLeavesTheRuntimeToItsOwner()
        {
            var runtime = CreateRuntime();
            var handle = NativeHandles.Of(runtime);
            var state = new RuntimeOnboardAsyncState(null, true);
            state.HoldUntilCompletion(NativeHandles.OwnerOf(runtime));
            state.CompleteOnboard(Result<OnboardSession>.Failure(Failed()));
            Assert.True(state.OnboardTask.IsCompleted);
            Assert.False(handle.IsClosed);
            runtime.Dispose();
            Assert.True(handle.IsClosed);
        }

        // A cancelled task completes at the cancel, before the native completion: the borrow is held
        // until that completion fires all the same, so the native call never loses its runtime and
        // the Dispose waits for it.
        [Fact]
        public void DisposeAfterACancelledAsyncCallWaitsForTheNativeCompletion()
        {
            var runtime = CreateRuntime();
            var handle = NativeHandles.Of(runtime);
            var state = new RuntimeOnboardAsyncState(null, true);
            using var cancellation = new CancellationTokenSource();
            Assert.True(state.UseStandardCancellation(cancellation.Token).IsSuccess);
            state.HoldUntilCompletion(NativeHandles.OwnerOf(runtime));
            cancellation.Cancel();
            Assert.True(state.OnboardTask.IsCanceled);
            var dispose = DisposeProbe.Start(runtime.Dispose);
            DisposeProbe.AssertWaiting(dispose);
            Assert.False(handle.IsClosed);
            // What the native completion reports after the cancel.
            state.CompleteOnboard(Result<OnboardSession>.Failure(Failed()));
            DisposeProbe.AssertReturned(dispose);
            Assert.True(handle.IsClosed);
        }

        [Fact]
        public void ADisposedRuntimeIsRefusedBeforeTheAsyncCall()
        {
            var runtime = CreateRuntime();
            var owner = NativeHandles.OwnerOf(runtime);
            runtime.Dispose();
            var state = new RuntimeOnboardAsyncState(null, true);
            Assert.Throws<ObjectDisposedException>(() => state.HoldUntilCompletion(owner));
            state.CompleteOnboard(Result<OnboardSession>.Failure(Failed()));
            Assert.Throws<ObjectDisposedException>(() => runtime.Forget(new ForgetOptions(OnboardingId.Default)));
        }
    }
}
