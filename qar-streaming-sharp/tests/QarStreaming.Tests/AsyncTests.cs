using System.Threading.Tasks;
using Qar;
using Qar.Native;
using Xunit;

namespace QarStreaming.Tests
{
    public sealed class AsyncTests
    {
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
