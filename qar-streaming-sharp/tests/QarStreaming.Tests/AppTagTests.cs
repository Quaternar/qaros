using System;
using System.Runtime.InteropServices;
using Quaternar.Qaros;
using Quaternar.Qaros.Interop;
using Xunit;

namespace Quaternar.Qaros.Tests
{
    // App tags (GitHub #268): the courtesy checks of the binding and how a tag, a co-process
    // invite request and ephemeral storage reach the C structs. No native library needed.
    public sealed class AppTagTests
    {
        public static TheoryData<string> InvalidTags()
        {
            return new TheoryData<string>
            {
                string.Empty,
                " left.app",
                "left.app ",
                "léft.app",
                "left\tapp",
                "left\u007fapp",
                new string('a', NativeConstants.MaxAppTagLength + 1),
                "member/reserved",
                "member/"
            };
        }

        [Theory]
        [MemberData(nameof(InvalidTags))]
        public void LocalAppTagRefusesAnInvalidTag(string tag)
        {
            var local = new OnboardLocalApp();
            Assert.Throws<ArgumentException>(() => local.Tag = tag);
            Assert.Null(local.Tag);
        }

        [Theory]
        [MemberData(nameof(InvalidTags))]
        public void InviteOptionsRefuseAnInvalidTag(string tag)
        {
            Assert.Throws<ArgumentException>(() => new RequestOnboardingInviteOptions(tag));
            var options = new RequestOnboardingInviteOptions("companion.sharp-tests");
            Assert.Throws<ArgumentException>(() => options.AppTag = tag);
            Assert.Equal("companion.sharp-tests", options.AppTag);
        }

        [Fact]
        public void InviteOptionsRequireATag()
        {
            Assert.Throws<ArgumentNullException>(() => new RequestOnboardingInviteOptions(null!));
            var options = new RequestOnboardingInviteOptions("companion.sharp-tests");
            Assert.Throws<ArgumentNullException>(() => options.AppTag = null!);
        }

        [Theory]
        [InlineData("left.terraexplorer.skyline")]
        [InlineData("a")]
        [InlineData("with inner space ~!")]
        public void ValidTagsAreKept(string tag)
        {
            Assert.Equal(tag, new OnboardLocalApp { Tag = tag }.Tag);
            Assert.Equal(tag, new RequestOnboardingInviteOptions(tag).AppTag);
        }

        [Fact]
        public void ATagOfTheMaximumLengthIsKept()
        {
            var tag = new string('a', NativeConstants.MaxAppTagLength);
            Assert.Equal(tag, new OnboardLocalApp { Tag = tag }.Tag);
        }

        [Fact]
        public void TheDefaultTagChainsNoTagExtension()
        {
            using (var chain = NativeStructureChain.From(new OnboardLocalApp(), new OnboardPeerId(PeerId.Parse(Guid.NewGuid().ToString()))))
            {
                var local = Marshal.PtrToStructure<OnboardLocalAppExtNative>(chain.Head);
                Assert.Equal(StructureType.RuntimeOnboardLocalAppExt, local.Header.Type);
                Assert.Equal(StructureType.RuntimeOnboardPeerIdExt, Marshal.PtrToStructure<StructureHeaderNative>(local.Header.Next).Type);
            }
        }

        [Fact]
        public void ATagChainsItsExtensionRightAfterTheLocalAppAndKeepsWhatFollows()
        {
            var local = new OnboardLocalApp { Tag = "left.sharp-tests", ApprovalTimeout = TimeSpan.FromSeconds(3) };
            using (var chain = NativeStructureChain.From(local, new OnboardPeerId(PeerId.Parse(Guid.NewGuid().ToString()))))
            {
                var native = Marshal.PtrToStructure<OnboardLocalAppExtNative>(chain.Head);
                Assert.Equal(StructureType.RuntimeOnboardLocalAppExt, native.Header.Type);
                Assert.Equal(3000u, native.ApprovalTimeoutMs);
                var tag = Marshal.PtrToStructure<OnboardLocalAppTagExtNative>(native.Header.Next);
                Assert.Equal(StructureType.RuntimeOnboardLocalAppTagExt, tag.Header.Type);
                Assert.Equal("left.sharp-tests", NativeString.DecodeNullTerminated(tag.AppTag));
                var after = Marshal.PtrToStructure<OnboardPeerIdExtNative>(tag.Header.Next);
                Assert.Equal(StructureType.RuntimeOnboardPeerIdExt, after.Header.Type);
                Assert.Equal(IntPtr.Zero, after.Header.Next);
            }
        }

        [Fact]
        public void TheInviteRequestCarriesTheTagUnderTheNewStructureType()
        {
            using (var scope = new NativeMemoryScope())
            {
                var native = NativeInit.RequestInvite(new RequestOnboardingInviteOptions("companion.sharp-tests"), scope, IntPtr.Zero);
                Assert.Equal(StructureType.SessionRequestInviteInit, native.Header.Type);
                Assert.Equal(0x200A, (int)native.Header.Type);
                Assert.Equal("companion.sharp-tests", NativeString.DecodeNullTerminated(native.AppTag));
            }
        }

        [Theory]
        [MemberData(nameof(InvalidTags))]
        public void MemberHubInvitationGrantRefusesAnInvalidTag(string tag)
        {
            var options = new MemberHubInvitationOptions { AppTag = "remote.sharp-tests" };
            Assert.Throws<ArgumentException>(() => options.AppTag = tag);
            Assert.Equal("remote.sharp-tests", options.AppTag);
        }

        [Theory]
        [InlineData("remote.terraexplorer.skyline")]
        [InlineData("members.app")]
        [InlineData("Member/case-differs")]
        [InlineData("app/member/")]
        public void MemberHubInvitationGrantKeepsAValidTag(string tag)
        {
            Assert.Equal(tag, new MemberHubInvitationOptions { AppTag = tag }.AppTag);
            Assert.Equal(tag, new OnboardLocalApp { Tag = tag }.Tag);
        }

        [Fact]
        public void MemberHubInvitationGrantIsOffByDefaultAndCanBeCleared()
        {
            Assert.Null(new MemberHubInvitationOptions().AppTag);
            var options = new MemberHubInvitationOptions { AppTag = "remote.sharp-tests" };
            options.AppTag = null;
            Assert.Null(options.AppTag);
            Assert.Null(new OnboardLocalApp().RemoteAppGrant);
        }

        [Fact]
        public void AnInvitationWithoutATagChainsNoGrant()
        {
            using (var scope = new NativeMemoryScope())
            {
                var native = MemberHubInvitationNative.Create(new MemberHubInvitationOptions { MaxUses = 2 }, scope);
                Assert.Equal(StructureType.RuntimeCreateMemberHubInvitationInit, native.Header.Type);
                Assert.Equal(2u, native.MaxUses);
                Assert.Equal(IntPtr.Zero, native.Header.Next);
            }
        }

        [Fact]
        public void AnInvitationTagChainsTheGrantExtension()
        {
            using (var scope = new NativeMemoryScope())
            {
                var native = MemberHubInvitationNative.Create(new MemberHubInvitationOptions { AppTag = "remote.sharp-tests", Label = "Workshop" }, scope);
                Assert.Equal("Workshop", NativeString.DecodeNullTerminated(native.Label));
                var grant = Marshal.PtrToStructure<MemberHubInvitationAppGrantExtNative>(native.Header.Next);
                Assert.Equal(StructureType.RuntimeMemberHubInvitationAppGrantExt, grant.Header.Type);
                Assert.Equal(0x1011, (int)grant.Header.Type);
                Assert.Equal("remote.sharp-tests", NativeString.DecodeNullTerminated(grant.AppTag));
                Assert.Equal(IntPtr.Zero, grant.Header.Next);
            }
        }

        [Fact]
        public void EphemeralStorageIsOffByDefault()
        {
            Assert.False(new RuntimeOptions().EphemeralStorage);
        }

        [Fact]
        public void EphemeralStorageMarshalsToItsExtension()
        {
            using (var chain = NativeStructureChain.From(new RuntimeEphemeralStorage(), null))
            {
                var native = Marshal.PtrToStructure<RuntimeEphemeralStorageExtNative>(chain.Head);
                Assert.Equal(StructureType.RuntimeEphemeralStorageExt, native.Header.Type);
                Assert.Equal(IntPtr.Zero, native.Header.Next);
            }
        }
    }
}
