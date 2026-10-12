using System;
using System.Threading;
using Quaternar.Qaros;
using Quaternar.Qaros.Interop;
using Xunit;

namespace Quaternar.Qaros.Tests
{
    public sealed class CancelTokenTests
    {
        [Fact]
        public void CreateProducesActiveToken()
        {
            using var token = CancelToken.Create().ValueOrThrow();
            Assert.False(token.IsCancelled());
            Assert.False(token.IsTimeout());
        }

        [Fact]
        public void CancelMarksTokenAsCancelledWithoutTimeout()
        {
            using var token = CancelToken.Create().ValueOrThrow();
            token.Cancel().EnsureSuccess();
            Assert.True(token.IsCancelled());
            Assert.False(token.IsTimeout());
        }

        [Fact]
        public void CreateWithTimeoutEventuallyMarksTimeout()
        {
            using var token = CancelToken.CreateWithTimeout(100).ValueOrThrow();
            QarNativeAssert.WaitUntil(token.IsTimeout, "cancel token timeout");
            Assert.True(token.IsCancelled());
            Assert.True(token.IsTimeout());
        }

        [Fact]
        public void CancelAfterEventuallyMarksTimeout()
        {
            using var token = CancelToken.Create().ValueOrThrow();
            token.CancelAfter(100).EnsureSuccess();
            QarNativeAssert.WaitUntil(token.IsTimeout, "cancel-after timeout");
            Assert.True(token.IsCancelled());
            Assert.True(token.IsTimeout());
        }

        [Fact]
        public void CancelAfterCanBeCalledAgainAfterTimeout()
        {
            using var token = CancelToken.Create().ValueOrThrow();
            token.CancelAfter(50).EnsureSuccess();
            QarNativeAssert.WaitUntil(token.IsTimeout, "first timeout");
            token.CancelAfter(50).EnsureSuccess();
            Assert.True(token.IsCancelled());
            Assert.True(token.IsTimeout());
        }

        [Fact]
        public void OperationsThrowAfterDispose()
        {
            var token = CancelToken.Create().ValueOrThrow();
            token.Dispose();
            Assert.Throws<ObjectDisposedException>(() => token.Cancel());
            Assert.Throws<ObjectDisposedException>(() => token.CancelAfter(10));
            Assert.Throws<ObjectDisposedException>(() => token.IsCancelled());
            Assert.Throws<ObjectDisposedException>(() => token.IsTimeout());
        }

        [Fact]
        public void StandardCancellationTokenCancelsNativeCancelToken()
        {
            using var cts = new CancellationTokenSource();
            var scopeResult = NativeCancellationScope.Create(cts.Token, true);
            using var scope = scopeResult.ValueOrThrow();
            Assert.NotEqual(IntPtr.Zero, scope.Handle);
            Assert.False(NativeMethods.qar_impl_cancel_token_is_cancelled(scope.Handle));
            cts.Cancel();
            QarNativeAssert.WaitUntil(
                () => NativeMethods.qar_impl_cancel_token_is_cancelled(scope.Handle),
                "standard cancellation token propagation");
            Assert.True(NativeMethods.qar_impl_cancel_token_is_cancelled(scope.Handle));
        }
    }
}
