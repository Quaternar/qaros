using System;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    /// <summary>
    /// An onboarding invite: everything a companion process needs to onboard with
    /// <see cref="OnboardOptions.UseInvite"/>. The app requests it once per session
    /// (<see cref="Session.RequestOnboardingInvite(string, IProgress{ActionProgress}?, System.Threading.CancellationToken)"/>);
    /// on Windows it works for every start of the companion while the app's session lives.
    /// </summary>
    /// <remarks>
    /// The caller owns every instance and disposes it; an onboard only borrows it. Invites carry a
    /// secret, so never log their bytes.
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
        /// Parses invite bytes from <see cref="Serialize()"/>, unchanged.
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
