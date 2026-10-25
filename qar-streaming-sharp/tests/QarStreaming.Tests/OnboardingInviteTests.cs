using System;
using System.Reflection;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Text;
using Quaternar.Qaros;
using Quaternar.Qaros.Interop;
using Xunit;

namespace Quaternar.Qaros.Tests
{
    /// <summary>
    /// The safe handle a wrapper owns, read by reflection so the product keeps no raw-handle
    /// accessor for the tests: a native call borrows a handle only through a scope (hub G49).
    /// </summary>
    internal static class NativeHandles
    {
        internal static SafeHandle Of(object owner)
        {
            var field = owner.GetType().GetField("_handle", BindingFlags.NonPublic | BindingFlags.Instance);
            Assert.NotNull(field);
            return Assert.IsAssignableFrom<SafeHandle>(field!.GetValue(owner));
        }
    }

    /// <summary>
    /// The onboarding invite handle (hub G49): an onboard with <see cref="OnboardOptions.UseInvite"/>
    /// holds a reference on it for the whole native call, so neither a concurrent
    /// <see cref="OnboardingInvite.Dispose"/> nor the finalizer can free it while the call reads it.
    /// Offline: the invite is parsed, never redeemed.
    /// </summary>
    [Collection("QAR Native")]
    public sealed class OnboardingInviteTests : IClassFixture<QarNativeFixture>
    {
        public OnboardingInviteTests(QarNativeFixture fixture)
        {
            _ = fixture;
        }

        // A well-formed PAKE invite; nothing listens on its endpoint.
        private static byte[] InviteWire()
        {
            var expires = DateTimeOffset.UtcNow.ToUnixTimeSeconds() + 3600;
            return Encoding.UTF8.GetBytes("{\"Hostname\":\"localhost\",\"IpAddress\":\"127.0.0.1\",\"Port\":47000,"
                + "\"ExpiresUnix\":" + expires + ",\"Method\":\"Pake\",\"InvitationId\":\"inv-sharp-tests\","
                + "\"ServerId\":\"server-sharp-tests\",\"Code\":\"123456\"}");
        }

        [Fact]
        public void AnInviteRoundTrips()
        {
            using var invite = OnboardingInvite.Deserialize(InviteWire()).ValueOrThrow();
            Assert.True(invite.IsValid());
            Assert.Equal(OnboardingMethod.Pake, invite.GetMethod().ValueOrThrow());
            Assert.NotEmpty(invite.GetCode().ValueOrThrow());
            var first = invite.Serialize().ValueOrThrow();
            using var again = OnboardingInvite.Deserialize(first).ValueOrThrow();
            Assert.Equal(first, again.Serialize().ValueOrThrow());
        }

        [Fact]
        public void TheOnboardChainHoldsTheInviteUntilItIsDisposed()
        {
            var invite = OnboardingInvite.Deserialize(InviteWire()).ValueOrThrow();
            var handle = NativeHandles.Of(invite);
            var chain = NativeStructureChain.From(new OnboardInvite(invite));
            var native = Marshal.PtrToStructure<OnboardInviteExtNative>(chain.Head);
            Assert.Equal(StructureType.RuntimeOnboardInviteExt, native.Header.Type);
            Assert.Equal(handle.DangerousGetHandle(), native.Invite);
            invite.Dispose();
            Assert.False(handle.IsClosed);
            Assert.Throws<ObjectDisposedException>(() => invite.Serialize());
            Assert.Throws<ObjectDisposedException>(() => invite.IsValid());
            chain.Dispose();
            Assert.True(handle.IsClosed);
        }

        // The single-threaded case: the caller keeps no reference to the invite after building the
        // options, so only the chain's reference stops the finalizer during the blocking call.
        [Fact]
        public void TheOnboardChainKeepsAnUnreferencedInviteFromTheFinalizer()
        {
            var chain = ChainOfAnUnreferencedInvite(out var handle);
            try
            {
                GC.Collect();
                GC.WaitForPendingFinalizers();
                GC.Collect();
                Assert.True(handle.IsAlive);
                Assert.False(((SafeHandle)handle.Target!).IsClosed);
            }
            finally
            {
                chain.Dispose();
            }
        }

        [Fact]
        public void ADisposedInviteIsRefusedWhenMarshalled()
        {
            var invite = OnboardingInvite.Deserialize(InviteWire()).ValueOrThrow();
            var options = new OnboardInvite(invite);
            invite.Dispose();
            Assert.Throws<ObjectDisposedException>(() => NativeStructureChain.From(options));
            Assert.Throws<ObjectDisposedException>(() => invite.GetCode());
            Assert.Throws<ObjectDisposedException>(() => invite.GetExpiresUnix());
        }

        [MethodImpl(MethodImplOptions.NoInlining)]
        private static NativeStructureChain ChainOfAnUnreferencedInvite(out WeakReference handle)
        {
            var invite = OnboardingInvite.Deserialize(InviteWire()).ValueOrThrow();
            handle = new WeakReference(NativeHandles.Of(invite));
            return NativeStructureChain.From(new OnboardInvite(invite));
        }
    }
}
