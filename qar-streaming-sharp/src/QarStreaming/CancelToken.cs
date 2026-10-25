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
        private readonly NativeHandleOwner _owner;

        internal CancelToken(CancelTokenSafeHandle handle)
        {
            // Destroyed by its last user, no wait: it has no thread affinity and an owner's Dispose
            // ends its shared dispose token while calls still borrow it.
            _owner = new NativeHandleOwner(handle, nameof(CancelToken), false);
        }

        /// <summary>
        /// The native token, referenced until the returned borrow is disposed: a call or a
        /// subscription that hands the token to native code holds it this way, so a concurrent
        /// <see cref="Dispose"/> or the finalizer cannot free it under them.
        /// </summary>
        /// <exception cref="ObjectDisposedException">The token is disposed.</exception>
        internal HandleBorrow Borrow()
        {
            return _owner.Borrow();
        }

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
            using (var token = _owner.Borrow())
                return Results.FromNative(NativeMethods.qar_impl_cancel_token_cancel(token.Handle));
        }

        public Result<Unit> CancelAfter(uint timeoutMs)
        {
            using (var token = _owner.Borrow())
                return Results.FromNative(NativeMethods.qar_impl_cancel_token_cancel_after(token.Handle, timeoutMs));
        }

        public bool IsCancelled()
        {
            using (var token = _owner.Borrow())
                return NativeMethods.qar_impl_cancel_token_is_cancelled(token.Handle);
        }

        public bool IsTimeout()
        {
            using (var token = _owner.Borrow())
                return NativeMethods.qar_impl_cancel_token_is_timeout(token.Handle);
        }

        public void Dispose()
        {
            _owner.Dispose();
        }
    }
}
