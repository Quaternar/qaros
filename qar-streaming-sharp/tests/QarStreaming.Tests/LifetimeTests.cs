using Qar;
using Qar.Native;
using Xunit;

namespace QarStreaming.Tests
{
    public sealed class LifetimeTests
    {
        [Fact]
        public void EnsureSuccessThrowsForFailedResultInfo()
        {
            var info = new ResultInfo(StatusCode.Timeout, 42, "timeout");
            var exception = Assert.Throws<NativeCallException>(() => info.EnsureSuccess());
            Assert.Equal(42u, exception.Result.ErrorHandle);
        }

        [Fact]
        public void QueryHandlesReturnsSnapshotFailure()
        {
            var handle = new SessionSafeHandle(new System.IntPtr(1));
            var context = new SessionNativeContext(handle);
            var success = new ResultNative { Code = StatusCode.Success };
            var snapshotFailure = new ResultInfo(StatusCode.LogicError, 3, "snapshot failed");
            var result = context.QueryHandles(
                (System.IntPtr session, out System.UIntPtr count) =>
                {
                    count = (System.UIntPtr)2;
                    return success;
                },
                (System.IntPtr session, System.IntPtr[] handles, System.UIntPtr bufferSize, out System.UIntPtr written) =>
                {
                    handles[0] = new System.IntPtr(10);
                    handles[1] = new System.IntPtr(20);
                    written = (System.UIntPtr)2;
                    return success;
                },
                nativeHandle => nativeHandle.ToInt64() == 20 ? Result<int>.Failure(snapshotFailure) : Result<int>.Success(new ResultInfo(StatusCode.Success, 0, string.Empty), 10));
            handle.SetHandleAsInvalid();
            context.Dispose();
            Assert.False(result.IsSuccess);
            Assert.Equal(StatusCode.LogicError, result.Info.StatusCode);
            Assert.Equal(3u, result.Info.ErrorHandle);
        }
    }
}
