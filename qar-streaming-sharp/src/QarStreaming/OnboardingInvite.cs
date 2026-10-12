using System;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    /// <summary>
    /// A one-time onboarding invite minted by the hub: everything another instance needs to
    /// onboard with <see cref="OnboardOptions.UseInvite"/>, or a code to hand out by
    /// <see cref="GetCode"/>.
    /// </summary>
    /// <remarks>
    /// The caller owns every instance and disposes it; an onboard only borrows it. Invites expire
    /// (<see cref="GetExpiresUnix"/>) and work once: mint them when needed and do not stockpile
    /// them. They carry a secret, so never log their bytes or code.
    /// </remarks>
    public sealed class OnboardingInvite : IDisposable
    {
        private readonly OnboardingInviteSafeHandle _handle;

        internal OnboardingInvite(OnboardingInviteSafeHandle handle)
        {
            _handle = handle ?? throw new ArgumentNullException(nameof(handle));
        }

        internal IntPtr DangerousHandle { get { ThrowIfDisposed(); return _handle.DangerousGetHandle(); } }

        public bool IsValid()
        {
            ThrowIfDisposed();
            return NativeMethods.qar_impl_onboarding_invite_handle_is_valid(_handle.DangerousGetHandle());
        }

        public Result<OnboardingMethod> GetMethod()
        {
            ThrowIfDisposed();
            OnboardingMethod method;
            var info = Results.ToInfo(NativeMethods.qar_impl_onboarding_invite_get_method(_handle.DangerousGetHandle(), out method));
            return info.IsSuccess ? Result<OnboardingMethod>.Success(info, method) : Result<OnboardingMethod>.Failure(info);
        }

        /// <summary>Expiry in Unix seconds, so a UI can warn before handing out a stale invite.</summary>
        public Result<long> GetExpiresUnix()
        {
            ThrowIfDisposed();
            long expiresUnix;
            var info = Results.ToInfo(NativeMethods.qar_impl_onboarding_invite_get_expires_unix(_handle.DangerousGetHandle(), out expiresUnix));
            return info.IsSuccess ? Result<long>.Success(info, expiresUnix) : Result<long>.Failure(info);
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
            ThrowIfDisposed();
            UIntPtr length;
            var sizeInfo = Results.ToInfo(NativeMethods.qar_impl_onboarding_invite_get_code(_handle.DangerousGetHandle(), null, UIntPtr.Zero, out length));
            if (!sizeInfo.IsSuccess)
                return Result<string>.Failure(sizeInfo);
            var buffer = new byte[NativeConvert.ToInt32(length) + 1];
            var info = Results.ToInfo(NativeMethods.qar_impl_onboarding_invite_get_code(_handle.DangerousGetHandle(), buffer, (UIntPtr)buffer.Length, out length));
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
            ThrowIfDisposed();
            UIntPtr size;
            var sizeInfo = Results.ToInfo(NativeMethods.qar_impl_onboarding_invite_serialized_size(_handle.DangerousGetHandle(), out size));
            if (!sizeInfo.IsSuccess)
                return Result<byte[]>.Failure(sizeInfo);
            var buffer = new byte[NativeConvert.ToInt32(size)];
            UIntPtr written;
            var info = Results.ToInfo(NativeMethods.qar_impl_onboarding_invite_serialize(_handle.DangerousGetHandle(), buffer, (UIntPtr)buffer.Length, out written));
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
        /// Parses invite bytes from <see cref="Serialize"/>, or the launch invite QAROS writes as
        /// the first stdin line of an API app it starts: pass that JSON line's UTF-8 bytes unchanged.
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
            _handle.Dispose();
        }

        private void ThrowIfDisposed()
        {
            if (_handle.IsClosed || _handle.IsInvalid)
                throw new ObjectDisposedException(nameof(OnboardingInvite));
        }
    }
}
