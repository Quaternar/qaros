using System;
using System.Runtime.InteropServices;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    /// <summary>
    /// One side of a sponsored member join: a hub's signed join request
    /// (<see cref="Runtime.RequestMemberJoin"/>) or a primary hub's signed reply to it
    /// (<see cref="Session.SponsorMember"/>), carried between the two PCs by your own channel.
    /// </summary>
    /// <remarks>
    /// The caller owns every instance and disposes it; the calls that take one only borrow it. A
    /// blob carries no secret, so it may cross an unauthenticated channel. Pass the primary's key
    /// fingerprint to <see cref="Runtime.CompleteMemberJoin"/> when the operator can read it out,
    /// because such a channel lets an attacker swap blobs.
    /// </remarks>
    public sealed class MemberJoinBlob : IDisposable
    {
        private readonly MemberJoinBlobSafeHandle _handle;

        internal MemberJoinBlob(MemberJoinBlobSafeHandle handle)
        {
            _handle = handle ?? throw new ArgumentNullException(nameof(handle));
        }

        internal IntPtr DangerousHandle { get { ThrowIfDisposed(); return _handle.DangerousGetHandle(); } }

        public bool IsValid()
        {
            ThrowIfDisposed();
            return NativeMethods.qar_impl_member_join_blob_handle_is_valid(_handle.DangerousGetHandle());
        }

        /// <summary>
        /// The blob as bytes for your channel (UTF-8 JSON, at most 128 KiB); the other PC turns
        /// them back with <see cref="Deserialize"/>.
        /// </summary>
        public Result<byte[]> Serialize()
        {
            ThrowIfDisposed();
            UIntPtr size;
            var sizeInfo = Results.ToInfo(NativeMethods.qar_impl_member_join_blob_serialized_size(_handle.DangerousGetHandle(), out size));
            if (!sizeInfo.IsSuccess)
                return Result<byte[]>.Failure(sizeInfo);
            var buffer = new byte[NativeConvert.ToInt32(size)];
            UIntPtr written;
            var info = Results.ToInfo(NativeMethods.qar_impl_member_join_blob_serialize(_handle.DangerousGetHandle(), buffer, (UIntPtr)buffer.Length, out written));
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
        /// Parses a request or a reply from <see cref="Serialize"/> bytes. Strict: bytes of another
        /// version, with an unknown or missing field, or that are no blob at all fail with
        /// <see cref="StatusCode.ArgumentNotSupported"/>. Signatures are checked by the calls that
        /// use the blob.
        /// </summary>
        public static Result<MemberJoinBlob> Deserialize(byte[] data)
        {
            if (data == null)
                throw new ArgumentNullException(nameof(data));
            IntPtr handle;
            var info = Results.ToInfo(NativeMethods.qar_impl_member_join_blob_deserialize(data, (UIntPtr)data.Length, out handle));
            return Results.FromHandle(info, handle, h => new MemberJoinBlob(new MemberJoinBlobSafeHandle(h)));
        }

        public void Dispose()
        {
            _handle.Dispose();
        }

        private void ThrowIfDisposed()
        {
            if (_handle.IsClosed || _handle.IsInvalid)
                throw new ObjectDisposedException(nameof(MemberJoinBlob));
        }
    }

    /// <summary>What <see cref="Session.SponsorMember"/> hands to the primary hub.</summary>
    public sealed class SponsorMemberOptions
    {
        public SponsorMemberOptions(MemberJoinBlob request)
        {
            Request = request ?? throw new ArgumentNullException(nameof(request));
        }

        /// <summary>The joining hub's request, as <see cref="Runtime.RequestMemberJoin"/> returned it on the joining PC.</summary>
        public MemberJoinBlob Request { get; set; }

        /// <summary>
        /// The primary hub's peer id. Unset: the hub on this PC, which must be the session's primary
        /// (the sponsoring app runs on the primary's PC).
        /// </summary>
        public PeerId? PrimaryPeerId { get; set; }

        /// <summary>Optional: the address the joining PC is known by, e.g. its IP. The primary records and shows it with the new member.</summary>
        public string? ClientAddress { get; set; }

        /// <summary>Optional: the joining PC's name as your app knows it. Recorded like <see cref="ClientAddress"/>.</summary>
        public string? ClientName { get; set; }
    }

    /// <summary>What <see cref="Runtime.CompleteMemberJoin"/> hands to the hub on this PC.</summary>
    public sealed class MemberJoinCompleteOptions
    {
        public MemberJoinCompleteOptions(MemberJoinBlob reply)
        {
            Reply = reply ?? throw new ArgumentNullException(nameof(reply));
        }

        /// <summary>The primary hub's reply, as <see cref="Session.SponsorMember"/> returned it on the primary's PC.</summary>
        public MemberJoinBlob Reply { get; set; }

        /// <summary>
        /// Optional: the SHA-256 fingerprint of the primary hub's trust-authority key, 64 hex digits
        /// in either case, optionally grouped by ':' or spaces. Unset: the key the reply names is
        /// trusted on first use. A reply signed by another key fails with
        /// <see cref="StatusCode.OnboardingHubNotAuthentic"/>.
        /// </summary>
        public string? ExpectedPrimaryFingerprint { get; set; }
    }

}

namespace Quaternar.Qaros.Interop
{
    internal static class MemberJoinNative
    {
        internal static SponsorMemberInitNative Sponsor(SponsorMemberOptions options, NativeMemoryScope scope)
        {
            return new SponsorMemberInitNative
            {
                Header = NativeDefaults.Header(StructureType.SessionSponsorMemberInit),
                Request = options.Request.DangerousHandle,
                PrimaryPeerId = options.PrimaryPeerId.HasValue ? options.PrimaryPeerId.Value.ToNative() : PeerIdNative.CreateDefault(),
                ClientAddress = scope.AllocUtf8(options.ClientAddress),
                ClientName = scope.AllocUtf8(options.ClientName)
            };
        }

        internal static MemberJoinCompleteInitNative Complete(MemberJoinCompleteOptions options, NativeMemoryScope scope)
        {
            return new MemberJoinCompleteInitNative
            {
                Header = NativeDefaults.Header(StructureType.RuntimeMemberJoinCompleteInit),
                Reply = options.Reply.DangerousHandle,
                ExpectedPrimaryFingerprint = scope.AllocUtf8(options.ExpectedPrimaryFingerprint)
            };
        }

        internal static MemberJoinBlob FromHandle(IntPtr handle)
        {
            return new MemberJoinBlob(new MemberJoinBlobSafeHandle(handle));
        }
    }

    /// <summary>
    /// Rooted state of one qar_session_sponsor_member_async call; the result callback fires
    /// exactly once, also on cancellation.
    /// </summary>
    internal sealed class SponsorMemberAsyncState : AsyncStateBase<Result<MemberJoinBlob>>
    {
        private readonly NativeMemoryScope _scope = new NativeMemoryScope();

        private readonly IProgress<ActionProgress>? _progress;

        public SessionSponsorMemberResultCallback ResultCallback { get; }

        public ProgressCallback ProgressCallback { get; }

        public SponsorMemberAsyncState(IProgress<ActionProgress>? progress)
        {
            _progress = progress;
            ResultCallback = new SessionSponsorMemberResultCallback(OnResult);
            ProgressCallback = new ProgressCallback(OnProgress);
        }

        public SponsorMemberInitNative CreateNative(SponsorMemberOptions options)
        {
            return MemberJoinNative.Sponsor(options, _scope);
        }

        private static void OnResult(ResultNative status, IntPtr reply, IntPtr userState)
        {
            SponsorMemberAsyncState? state = null;
            try
            {
                state = (SponsorMemberAsyncState)GCHandle.FromIntPtr(userState).Target!;
                var info = Results.ToInfo(status);
                var result = Results.FromHandle(info, reply, MemberJoinNative.FromHandle);
                // Cancelled before the native answer: nobody will read the result, so the reply
                // blob it holds is released here instead of leaking.
                if (!state.Completion.TrySetResult(result) && result.IsSuccess)
                    result.Value.Dispose();
                state.CleanupOnce();
            }
            catch (Exception ex)
            {
                if (state != null)
                    state.Fault(ex);
            }
        }

        private static void OnProgress(ActionSeverity severity, float percent, IntPtr message, IntPtr userState)
        {
            try
            {
                var state = (SponsorMemberAsyncState)GCHandle.FromIntPtr(userState).Target!;
                if (state._progress != null)
                    state._progress.Report(new ActionProgress(severity, percent, NativeString.DecodeNullTerminated(message) ?? string.Empty));
            }
            catch
            {
            }
        }

        protected override void Cleanup()
        {
            _scope.Dispose();
            base.Cleanup();
        }
    }
}
