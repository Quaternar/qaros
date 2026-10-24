using System;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    /// <summary>
    /// A member-hub invitation: a primary hub's signed invitation that joins the QAROS hub on
    /// another PC to its session as a member. Created on the primary's PC with
    /// <see cref="Runtime.CreateMemberHubInvitation"/>, carried by your own channel as
    /// <see cref="Serialize"/> bytes, and used on the joining PC with
    /// <see cref="Runtime.JoinHubAsMember"/>.
    /// </summary>
    /// <remarks>
    /// The invitation is a secret bearer credential: whoever holds it can join a hub to the primary
    /// until it expires or is used up. Send it only over a trusted channel and never log it. The
    /// caller owns every instance and disposes it; the calls that take one only borrow it.
    /// </remarks>
    public sealed class MemberHubInvitation : IDisposable
    {
        private readonly MemberHubInvitationSafeHandle _handle;
        private volatile bool _disposed;

        internal MemberHubInvitation(MemberHubInvitationSafeHandle handle)
        {
            _handle = handle ?? throw new ArgumentNullException(nameof(handle));
        }

        internal IntPtr DangerousHandle { get { ThrowIfDisposed(); return _handle.DangerousGetHandle(); } }

        /// <summary>The native handle, referenced by <paramref name="scope"/> until it is disposed.</summary>
        /// <exception cref="ObjectDisposedException">The invitation is disposed.</exception>
        internal IntPtr BorrowHandle(NativeMemoryScope scope)
        {
            ThrowIfDisposed();
            return scope.Borrow(_handle);
        }

        /// <summary>
        /// The invitation as bytes for your channel (UTF-8 JSON, at most 16 KiB); the other PC turns
        /// them back with <see cref="Deserialize"/>. The bytes carry the secret: never log them.
        /// </summary>
        public Result<byte[]> Serialize()
        {
            ThrowIfDisposed();
            UIntPtr size;
            var sizeInfo = Results.ToInfo(NativeMethods.qar_impl_member_hub_invitation_serialized_size(_handle.DangerousGetHandle(), out size));
            if (!sizeInfo.IsSuccess)
                return Result<byte[]>.Failure(sizeInfo);
            var buffer = new byte[NativeConvert.ToInt32(size)];
            UIntPtr written;
            var info = Results.ToInfo(NativeMethods.qar_impl_member_hub_invitation_serialize(_handle.DangerousGetHandle(), buffer, (UIntPtr)buffer.Length, out written));
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
        /// Parses an invitation from <see cref="Serialize"/> bytes. Strict: bytes with an unknown or
        /// missing field, of another kind or version, out of bounds or over 16 KiB fail with
        /// <see cref="StatusCode.MemberInvitationInvalid"/>. The signature is checked by
        /// <see cref="Runtime.JoinHubAsMember"/>, not here.
        /// </summary>
        public static Result<MemberHubInvitation> Deserialize(byte[] data)
        {
            if (data == null)
                throw new ArgumentNullException(nameof(data));
            IntPtr handle;
            var info = Results.ToInfo(NativeMethods.qar_impl_member_hub_invitation_deserialize(data, (UIntPtr)data.Length, out handle));
            return Results.FromHandle(info, handle, MemberHubInvitationNative.FromHandle);
        }

        public void Dispose()
        {
            // A native call borrowing the handle defers its release, so the handle may stay open
            // after this; the flag makes the invitation unusable from here on all the same.
            _disposed = true;
            _handle.Dispose();
        }

        private void ThrowIfDisposed()
        {
            if (_disposed || _handle.IsClosed || _handle.IsInvalid)
                throw new ObjectDisposedException(nameof(MemberHubInvitation));
        }
    }

    /// <summary>What a member-hub invitation lets the joining hub do at the primary.</summary>
    [Flags]
    public enum MemberHubInvitationPermissions : uint
    {
        /// <summary>The primary's defaults: <see cref="Join"/>, <see cref="Rejoin"/> and <see cref="BringDevices"/>.</summary>
        None = 0,
        /// <summary>A hub the primary does not know joins as a new member.</summary>
        Join = 0x1,
        /// <summary>A member the primary admitted before (and has not removed) is answered again.</summary>
        Rejoin = 0x2,
        /// <summary>A member the primary's operator removed is admitted again.</summary>
        Readmit = 0x4,
        /// <summary>A hub that was a primary of its own moves its devices, launcher children and approved apps along.</summary>
        BringDevices = 0x8
    }

    /// <summary>What <see cref="Runtime.CreateMemberHubInvitation"/> asks the primary hub for.</summary>
    public sealed class MemberHubInvitationOptions
    {
        /// <summary>How long the invitation can be used. Unset: 24 h. At most 7 days.</summary>
        public TimeSpan? Lifetime { get; set; }

        /// <summary>How many hubs may join with it. 0: 1. At most 64, and never more than the app's quota left.</summary>
        public uint MaxUses { get; set; }

        /// <summary>What it permits. <see cref="MemberHubInvitationPermissions.None"/>: the defaults.</summary>
        public MemberHubInvitationPermissions Permissions { get; set; }

        private string? _appTag;

        /// <summary>Optional UTF-8 label of at most 64 bytes, shown in the primary's visualizer.</summary>
        public string? Label { get; set; }

        /// <summary>
        /// Optional remote-app grant (C <c>QarMemberHubInvitationAppGrantExt</c>): set, the
        /// invitation also admits one app on the joining PC, bound to this app's approval and this
        /// tag. Null, the default, makes a plain member-hub invitation.
        /// </summary>
        /// <remarks>
        /// <para>
        /// On a member hub of this primary, the first executable that onboards as a local app with
        /// the invitation as <see cref="OnboardLocalApp.RemoteAppGrant"/> is approved without a
        /// prompt, recorded by its publisher or image hash, and every later start of that executable
        /// onboards with no invitation and no prompt under the same peer id while no other live
        /// process holds it. The first executable that redeems the grant wins: any other one is
        /// refused it afterwards, also with a later invitation of the same app and tag.
        /// </para>
        /// <para>
        /// One grant admits one app on one member PC however many hubs <see cref="MaxUses"/> lets
        /// join: issue one tag per remote app. With a grant this app's own approval is not extended
        /// to the member hub. Revoking this app's approval, or the granted one, on the primary
        /// revokes the granted app and forgets its peer id.
        /// </para>
        /// <para>
        /// The invitation then also is a bearer credential for that app slot: whoever redeems it
        /// first owns it. Hand it over privately, to the app it is meant for only, and never log it.
        /// </para>
        /// <para>
        /// Same rules as <see cref="OnboardLocalApp.Tag"/>: non-empty printable ASCII, no space at
        /// either end, at most 256 bytes, compared exactly, and not starting with the reserved
        /// <c>member/</c> prefix.
        /// </para>
        /// </remarks>
        /// <exception cref="ArgumentException">The value breaks the rules above (checked as a courtesy; the runtime checks too).</exception>
        public string? AppTag
        {
            get { return _appTag; }
            set { _appTag = value == null ? null : AppTagRules.Validate(value, nameof(value)); }
        }
    }

    /// <summary>What <see cref="Runtime.JoinHubAsMember"/> hands to the hub on this PC.</summary>
    public sealed class JoinHubAsMemberOptions
    {
        public JoinHubAsMemberOptions(MemberHubInvitation invitation)
        {
            Invitation = invitation ?? throw new ArgumentNullException(nameof(invitation));
        }

        /// <summary>The invitation, as <see cref="MemberHubInvitation.Deserialize"/> returned it. Borrowed for the call.</summary>
        public MemberHubInvitation Invitation { get; set; }

        /// <summary>How long to wait for a hub this call started. Unset: 60 s.</summary>
        public TimeSpan? HubStartTimeout { get; set; }

        /// <summary>
        /// How long to wait for the hub to come back as a member after its restart. Unset: 120 s. On
        /// a timeout the join stays committed; calling again succeeds once the hub is back.
        /// </summary>
        public TimeSpan? RestartTimeout { get; set; }
    }
}

namespace Quaternar.Qaros.Interop
{
    internal static class MemberHubInvitationNative
    {
        internal static MemberHubInvitationInitNative Create(MemberHubInvitationOptions options, NativeMemoryScope scope)
        {
            return new MemberHubInvitationInitNative
            {
                Header = NativeDefaults.Header(StructureType.RuntimeCreateMemberHubInvitationInit, AppGrant(options.AppTag, scope)),
                LifetimeSeconds = Seconds(options.Lifetime, nameof(options.Lifetime)),
                MaxUses = options.MaxUses,
                Permissions = (uint)options.Permissions,
                Label = scope.AllocUtf8(options.Label)
            };
        }

        // QarMemberHubInvitationAppGrantExt in the init's chain; none without a tag.
        private static IntPtr AppGrant(string? appTag, NativeMemoryScope scope)
        {
            if (appTag == null)
                return IntPtr.Zero;
            return scope.AllocStruct(new MemberHubInvitationAppGrantExtNative
            {
                Header = NativeDefaults.Header(StructureType.RuntimeMemberHubInvitationAppGrantExt),
                AppTag = scope.AllocUtf8(appTag)
            });
        }

        internal static JoinHubAsMemberInitNative Join(JoinHubAsMemberOptions options)
        {
            return new JoinHubAsMemberInitNative
            {
                Header = NativeDefaults.Header(StructureType.RuntimeJoinHubAsMemberInit),
                Invitation = options.Invitation.DangerousHandle,
                HubStartTimeoutMs = Milliseconds(options.HubStartTimeout, nameof(options.HubStartTimeout)),
                RestartTimeoutMs = Milliseconds(options.RestartTimeout, nameof(options.RestartTimeout))
            };
        }

        internal static MemberHubInvitation FromHandle(IntPtr handle)
        {
            return new MemberHubInvitation(new MemberHubInvitationSafeHandle(handle));
        }

        /// <summary>Unset is 0, the native default; a negative or too long span is the caller's bug.</summary>
        private static uint Seconds(TimeSpan? span, string name)
        {
            if (!span.HasValue)
                return 0;
            var seconds = Math.Ceiling(span.Value.TotalSeconds);
            if (seconds < 1 || seconds > uint.MaxValue)
                throw new ArgumentOutOfRangeException(name, span.Value, "must be positive and fit in seconds");
            return (uint)seconds;
        }

        private static uint Milliseconds(TimeSpan? span, string name)
        {
            if (!span.HasValue)
                return 0;
            var milliseconds = Math.Ceiling(span.Value.TotalMilliseconds);
            if (milliseconds < 1 || milliseconds > uint.MaxValue)
                throw new ArgumentOutOfRangeException(name, span.Value, "must be positive and fit in milliseconds");
            return (uint)milliseconds;
        }
    }
}
