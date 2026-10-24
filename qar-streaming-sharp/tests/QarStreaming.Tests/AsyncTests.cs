using System.Threading.Tasks;
using System.Threading;
using Quaternar.Qaros;
using Quaternar.Qaros.Interop;
using Xunit;

namespace Quaternar.Qaros.Tests
{
    public sealed class AsyncTests
    {
        [Fact]
        public async Task RejoinCompletionDoesNotInlineCallersOnNativeCallbackThread()
        {
            var state = new RuntimeOnboardAsyncState(null, false);
            var callbackThread = Thread.CurrentThread.ManagedThreadId;
            var insideCallback = true;
            var inlined = false;
            var continuation = state.SessionTask.ContinueWith(_ =>
            {
                inlined = Thread.CurrentThread.ManagedThreadId == callbackThread && insideCallback;
            }, CancellationToken.None, TaskContinuationOptions.ExecuteSynchronously, TaskScheduler.Default);
            state.CompleteRejoin(Result<Session>.Failure(new ResultInfo(StatusCode.LogicError, 0, "test failed rejoin")));
            insideCallback = false;
            await continuation;
            Assert.False(inlined);
        }

        [Fact]
        public async Task ResultTasksCompleteWithResultObject()
        {
            var info = new ResultInfo(StatusCode.Success, 0, string.Empty);
            var task = Task.FromResult(Result<int>.Success(info, 7));
            var result = await task;
            Assert.True(result.IsSuccess);
            Assert.Equal(7, result.Value);
        }

#if NET8_0_OR_GREATER
        [Fact]
        public async Task AsyncEventQueueYieldsQueuedItems()
        {
            var queue = new AsyncEventQueue<int>();
            queue.Enqueue(7);
            queue.Complete();
            await foreach (var value in queue.ReadAllAsync())
            {
                Assert.Equal(7, value);
                return;
            }
            Assert.Fail("Expected one queued value.");
        }
#endif

        [Fact]
        public void StandardCancellationOverloadsAreExposed()
        {
            Assert.NotNull(typeof(RenderSenderControl).GetMethod(nameof(RenderSenderControl.CreateAsync), new[] { typeof(RenderSenderOptions), typeof(System.Threading.CancellationToken) }));
            Assert.NotNull(typeof(RenderSender).GetMethod(nameof(RenderSender.BeginFrameAsync), new[] { typeof(System.Threading.CancellationToken) }));
        }

        [Fact]
        public void UncancelableStandardCancellationUsesNullNativeHandle()
        {
            var state = new BeginFrameAsyncState();
            var cancelHandle = state.UseStandardCancellation(System.Threading.CancellationToken.None);
            Assert.True(cancelHandle.IsSuccess);
            Assert.Equal(System.IntPtr.Zero, cancelHandle.Value);
            state.Complete(Result<RenderFrameInfo>.Failure(new ResultInfo(StatusCode.Timeout, 0, "completed")));
        }
    }
}
