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
    /// rejoin holds it until its native completion fires, so a <see cref="Runtime.Dispose"/> while
    /// the call runs defers the native release. Offline: the completion is driven directly.
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
        public void TheAsyncOnboardHoldsTheRuntimeUntilTheCompletionFires()
        {
            var runtime = CreateRuntime();
            var handle = NativeHandles.Of(runtime);
            var state = new RuntimeOnboardAsyncState(null, true);
            Assert.Equal(handle.DangerousGetHandle(), state.BorrowRuntime(runtime));
            runtime.Dispose();
            Assert.False(handle.IsClosed);
            Assert.Throws<ObjectDisposedException>(() => runtime.BorrowHandle(new NativeMemoryScope()));
            state.CompleteOnboard(Result<OnboardSession>.Failure(Failed()));
            // The last reference is released on the thread pool, never on the completion's thread.
            Assert.True(SpinWait.SpinUntil(() => handle.IsClosed, TimeSpan.FromSeconds(10)));
        }

        [Fact]
        public void TheAsyncRejoinHoldsTheRuntimeUntilTheCompletionFires()
        {
            var runtime = CreateRuntime();
            var handle = NativeHandles.Of(runtime);
            var state = new RuntimeOnboardAsyncState(null, false);
            state.BorrowRuntime(runtime);
            runtime.Dispose();
            Assert.False(handle.IsClosed);
            state.CompleteRejoin(Result<Session>.Failure(Failed()));
            Assert.True(SpinWait.SpinUntil(() => handle.IsClosed, TimeSpan.FromSeconds(10)));
        }

        // The completion drops the borrow before the task completes, so the caller's Dispose after
        // the await releases the runtime at once, on the caller's thread.
        [Fact]
        public void ACompletedAsyncCallLeavesTheRuntimeToItsOwner()
        {
            var runtime = CreateRuntime();
            var handle = NativeHandles.Of(runtime);
            var state = new RuntimeOnboardAsyncState(null, true);
            state.BorrowRuntime(runtime);
            state.CompleteOnboard(Result<OnboardSession>.Failure(Failed()));
            Assert.True(state.OnboardTask.IsCompleted);
            Assert.False(handle.IsClosed);
            runtime.Dispose();
            Assert.True(handle.IsClosed);
        }

        // A cancelled task completes at the cancel, before the native completion: the borrow is held
        // until that completion fires all the same, so the native call never loses its runtime.
        [Fact]
        public void ACancelledAsyncCallHoldsTheRuntimeUntilTheNativeCompletion()
        {
            var runtime = CreateRuntime();
            var handle = NativeHandles.Of(runtime);
            var state = new RuntimeOnboardAsyncState(null, true);
            using var cancellation = new CancellationTokenSource();
            Assert.True(state.UseStandardCancellation(cancellation.Token).IsSuccess);
            state.BorrowRuntime(runtime);
            cancellation.Cancel();
            Assert.True(state.OnboardTask.IsCanceled);
            runtime.Dispose();
            Assert.False(handle.IsClosed);
            // What the native completion reports after the cancel.
            state.CompleteOnboard(Result<OnboardSession>.Failure(Failed()));
            Assert.True(SpinWait.SpinUntil(() => handle.IsClosed, TimeSpan.FromSeconds(10)));
        }

        [Fact]
        public void ADisposedRuntimeIsRefusedBeforeTheAsyncCall()
        {
            var runtime = CreateRuntime();
            runtime.Dispose();
            var state = new RuntimeOnboardAsyncState(null, true);
            Assert.Throws<ObjectDisposedException>(() => state.BorrowRuntime(runtime));
            state.CompleteOnboard(Result<OnboardSession>.Failure(Failed()));
        }
    }
}
