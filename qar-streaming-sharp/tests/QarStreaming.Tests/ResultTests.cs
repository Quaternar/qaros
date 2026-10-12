using Qar;
using Xunit;

namespace QarStreaming.Tests
{
    public sealed class ResultTests
    {
        [Fact]
        public void ResultInfoReportsSuccessFromStatusCode()
        {
            var info = new ResultInfo(StatusCode.Success, 0, string.Empty);
            Assert.True(info.IsSuccess);
        }

        [Fact]
        public void FailedResultValueOrThrowThrowsNativeCallException()
        {
            var info = new ResultInfo(StatusCode.LogicError, 17, "failed");
            var result = Result<int>.Failure(info);
            var exception = Assert.Throws<NativeCallException>(() => result.ValueOrThrow());
            Assert.Equal(StatusCode.LogicError, exception.Result.StatusCode);
        }

        [Fact]
        public void UnitResultCarriesStatusWithoutPayload()
        {
            var info = new ResultInfo(StatusCode.Success, 0, string.Empty);
            var result = Result<Unit>.Success(info, Unit.Value);
            Assert.True(result.IsSuccess);
            Assert.Equal(info, result.Info);
        }

        [Fact]
        public void LazyResultInfoDoesNotLoadMessageUntilRequested()
        {
            var calls = 0;
            var info = new ResultInfo(StatusCode.LogicError, 9, () =>
            {
                calls++;
                return "lazy-message";
            });
            Assert.False(info.IsSuccess);
            Assert.Equal(0, calls);
        }

        [Fact]
        public void LazyResultInfoCachesLoadedMessage()
        {
            var calls = 0;
            var info = new ResultInfo(StatusCode.LogicError, 9, () =>
            {
                calls++;
                return "lazy-message";
            });
            Assert.Equal("lazy-message", info.Message);
            Assert.Equal("lazy-message", info.Message);
            Assert.Equal(1, calls);
        }

        [Fact]
        public void LazyResultInfoToStringLoadsMessage()
        {
            var calls = 0;
            var info = new ResultInfo(StatusCode.LogicError, 9, () =>
            {
                calls++;
                return "lazy-message";
            });
            Assert.Equal("LogicError: lazy-message", info.ToString());
            Assert.Equal(1, calls);
        }
    }
}
