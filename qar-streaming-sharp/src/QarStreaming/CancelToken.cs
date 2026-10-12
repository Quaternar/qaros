using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    internal sealed class CancelToken : IDisposable
    {
        private readonly CancelTokenSafeHandle _handle;

        internal CancelToken(CancelTokenSafeHandle handle)
        {
            _handle = handle ?? throw new ArgumentNullException(nameof(handle));
        }

        internal IntPtr DangerousHandle { get { ThrowIfDisposed(); return _handle.DangerousGetHandle(); } }

        public static Result<CancelToken> Create()
        {
            IntPtr handle;
            var info = Results.ToInfo(NativeMethods.qar_impl_cancel_token_create(out handle));
            return Results.FromHandle(info, handle, h => new CancelToken(new CancelTokenSafeHandle(h)));
        }

        public static Result<CancelToken> CreateWithTimeout(uint timeoutMs)
        {
            IntPtr handle;
            var info = Results.ToInfo(NativeMethods.qar_impl_cancel_token_create_with_timeout(out handle, timeoutMs));
            return Results.FromHandle(info, handle, h => new CancelToken(new CancelTokenSafeHandle(h)));
        }

        public Result<Unit> Cancel()
        {
            ThrowIfDisposed();
            return Results.FromNative(NativeMethods.qar_impl_cancel_token_cancel(_handle.DangerousGetHandle()));
        }

        public Result<Unit> CancelAfter(uint timeoutMs)
        {
            ThrowIfDisposed();
            return Results.FromNative(NativeMethods.qar_impl_cancel_token_cancel_after(_handle.DangerousGetHandle(), timeoutMs));
        }

        public bool IsCancelled()
        {
            ThrowIfDisposed();
            return NativeMethods.qar_impl_cancel_token_is_cancelled(_handle.DangerousGetHandle());
        }

        public bool IsTimeout()
        {
            ThrowIfDisposed();
            return NativeMethods.qar_impl_cancel_token_is_timeout(_handle.DangerousGetHandle());
        }

        public void Dispose()
        {
            _handle.Dispose();
        }

        private void ThrowIfDisposed()
        {
            if (_handle.IsClosed || _handle.IsInvalid)
                throw new ObjectDisposedException(nameof(CancelToken));
        }
    }
}
