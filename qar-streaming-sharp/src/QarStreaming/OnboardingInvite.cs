using System;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    /// <summary>
    /// A one-time onboarding invite minted by the hub: everything another instance needs to
    /// onboard with <see cref="OnboardOptions.UseInvite"/>, or a code to hand out by
    /// <see cref="GetCode()"/>.
    /// </summary>
    /// <remarks>
    /// The caller owns every instance and disposes it; an onboard only borrows it. Invites expire
    /// (<see cref="GetExpiresUnix"/>) and work once: mint them when needed and do not stockpile
    /// them. They carry a secret, so never log their bytes or code.
    /// </remarks>
    public sealed class OnboardingInvite : IDisposable
    {
        private readonly OnboardingInviteSafeHandle _handle;
        private volatile bool _disposed;

        internal OnboardingInvite(OnboardingInviteSafeHandle handle)
        {
            _handle = handle ?? throw new ArgumentNullException(nameof(handle));
        }

        /// <summary>
        /// The native handle, referenced by <paramref name="scope"/> until it is disposed: every
        /// native call that takes the invite borrows it this way, so neither a concurrent
        /// <see cref="Dispose"/> nor the finalizer can free it while the call still reads it.
        /// </summary>
        /// <exception cref="ObjectDisposedException">The invite is disposed.</exception>
        internal IntPtr BorrowHandle(NativeMemoryScope scope)
        {
            ThrowIfDisposed();
            return scope.Borrow(_handle);
        }

        public bool IsValid()
        {
            using (var scope = new NativeMemoryScope())
                return NativeMethods.qar_impl_onboarding_invite_handle_is_valid(BorrowHandle(scope));
        }

        public Result<OnboardingMethod> GetMethod()
        {
            using (var scope = new NativeMemoryScope())
            {
                OnboardingMethod method;
                var info = Results.ToInfo(NativeMethods.qar_impl_onboarding_invite_get_method(BorrowHandle(scope), out method));
                return info.IsSuccess ? Result<OnboardingMethod>.Success(info, method) : Result<OnboardingMethod>.Failure(info);
            }
        }

        /// <summary>Expiry in Unix seconds, so a UI can warn before handing out a stale invite.</summary>
        public Result<long> GetExpiresUnix()
        {
            using (var scope = new NativeMemoryScope())
            {
                long expiresUnix;
                var info = Results.ToInfo(NativeMethods.qar_impl_onboarding_invite_get_expires_unix(BorrowHandle(scope), out expiresUnix));
                return info.IsSuccess ? Result<long>.Success(info, expiresUnix) : Result<long>.Failure(info);
            }
        }

        /// <summary>
        /// The code text to onboard with (OnboardCode). It names its own invitation, so it works for any
        /// code the hub minted. It is a secret: never log it.
        /// </summary>
        /// <remarks>
        /// Only a <see cref="OnboardingMethod.Pake"/> invite has one; any other fails with
        /// <see cref="StatusCode.ArgumentNotSupported"/>.
        /// </remarks>
        public Result<string> GetCode()
        {
            using (var scope = new NativeMemoryScope())
                return GetCode(BorrowHandle(scope));
        }

        private static Result<string> GetCode(IntPtr handle)
        {
            UIntPtr length;
            var sizeInfo = Results.ToInfo(NativeMethods.qar_impl_onboarding_invite_get_code(handle, null, UIntPtr.Zero, out length));
            if (!sizeInfo.IsSuccess)
                return Result<string>.Failure(sizeInfo);
            var buffer = new byte[NativeConvert.ToInt32(length) + 1];
            var info = Results.ToInfo(NativeMethods.qar_impl_onboarding_invite_get_code(handle, buffer, (UIntPtr)buffer.Length, out length));
            if (!info.IsSuccess)
                return Result<string>.Failure(info);
            return Result<string>.Success(info, System.Text.Encoding.UTF8.GetString(buffer, 0, NativeConvert.ToInt32(length)));
        }

        /// <summary>
        /// The invite as bytes for your IPC boundary (pipe, shared memory, config field); the
        /// receiving instance turns them back with <see cref="Deserialize"/>.
        /// </summary>
        public Result<byte[]> Serialize()
        {
            using (var scope = new NativeMemoryScope())
                return Serialize(BorrowHandle(scope));
        }

        private static Result<byte[]> Serialize(IntPtr handle)
        {
            UIntPtr size;
            var sizeInfo = Results.ToInfo(NativeMethods.qar_impl_onboarding_invite_serialized_size(handle, out size));
            if (!sizeInfo.IsSuccess)
                return Result<byte[]>.Failure(sizeInfo);
            var buffer = new byte[NativeConvert.ToInt32(size)];
            UIntPtr written;
            var info = Results.ToInfo(NativeMethods.qar_impl_onboarding_invite_serialize(handle, buffer, (UIntPtr)buffer.Length, out written));
            if (!info.IsSuccess)
                return Result<byte[]>.Failure(info);
            var count = NativeConvert.ToInt32(written);
            if (count == buffer.Length)
                return Result<byte[]>.Success(info, buffer);
            var result = new byte[count];
            Array.Copy(buffer, result, count);
            return Result<byte[]>.Success(info, result);
        }

        /// <summary>
        /// Parses invite bytes from <see cref="Serialize()"/>, or the launch invite QAROS writes as
        /// the first stdin line of an API app it starts where local enrollment is not available
        /// (not Windows): pass that JSON line's UTF-8 bytes unchanged.
        /// Bytes that are not an invite fail with <see cref="StatusCode.PakeError"/>.
        /// </summary>
        public static Result<OnboardingInvite> Deserialize(byte[] inviteData)
        {
            if (inviteData == null)
                throw new ArgumentNullException(nameof(inviteData));
            IntPtr handle;
            var info = Results.ToInfo(NativeMethods.qar_impl_onboarding_invite_deserialize(inviteData, (UIntPtr)inviteData.Length, out handle));
            return Results.FromHandle(info, handle, h => new OnboardingInvite(new OnboardingInviteSafeHandle(h)));
        }

        public void Dispose()
        {
            // A native call borrowing the handle defers its release, so the handle may stay open
            // after this; the flag makes the invite unusable from here on all the same.
            _disposed = true;
            _handle.Dispose();
        }

        private void ThrowIfDisposed()
        {
            if (_disposed || _handle.IsClosed || _handle.IsInvalid)
                throw new ObjectDisposedException(nameof(OnboardingInvite));
        }
    }
}
