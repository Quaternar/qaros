using System;
using System.IO;
using System.Text;
using Quaternar.Qaros;
using Xunit;

namespace Quaternar.Qaros.Tests
{
    /// <summary>
    /// The member-hub invitation surface (docs/hub-topology §5, GitHub #262): the invitation round
    /// trip, strict parsing, the options, and the checks made before a hub is asked. The joins
    /// against real hubs are <see cref="HubMemberInvitationTests"/>.
    /// </summary>
    [Collection("QAR Native")]
    public sealed class MemberHubInvitationTests : IClassFixture<QarNativeFixture>
    {
        public MemberHubInvitationTests(QarNativeFixture fixture)
        {
            _ = fixture;
        }

        // Well formed and within every bound; the signature is checked by the join, not the parse.
        private static string InvitationJson(string kind = "member-hub-invitation", string extra = "")
        {
            var now = DateTimeOffset.UtcNow.ToUnixTimeSeconds();
            return "{\"app\":{\"displayName\":\"Simulation server\",\"imageSha256\":\"\",\"originalFilename\":\"\","
                + "\"productName\":\"\",\"publisher\":\"\",\"rule\":\"hash\",\"signerSubject\":\"\",\"trustId\":\"trust-1\"},"
                + "\"authorityCertificateChainPem\":\"\",\"authorityKey\":\"AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8=\","
                + "\"endpoints\":[{\"host\":\"studio.local\",\"port\":47000}],"
                + "\"expiresUnix\":" + (now + 3600) + ",\"generation\":3,"
                + "\"id\":\"6f1c1d0e-8a51-4b8e-9d0b-0f4a3c2b1a90\",\"issuedAtUnix\":" + now + ","
                + "\"kind\":\"" + kind + "\",\"label\":\"Workshop\",\"maxUses\":1,"
                + "\"memberCommonName\":\"member.session.qaros\","
                + "\"permissions\":{\"bringDevices\":true,\"join\":true,\"readmit\":false,\"rejoin\":true},"
                + "\"primaryDisplayName\":\"Studio PC\",\"primaryHubPeerId\":\"00112233-4455-6677-8899-aabbccddee00\","
                + "\"sessionId\":\"00112233-4455-6677-8899-aabbccddee11\",\"signature\":\"c2ln\","
                + "\"token\":\"AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8=\"" + extra + "}";
        }

        private static Runtime CreateRuntime()
        {
            var storageRoot = Path.Combine(Path.GetTempPath(), "qar-sharp-tests", Guid.NewGuid().ToString("N"));
            return Runtime.Create(new RuntimeOptions { StorageFolderPath = storageRoot }).ValueOrThrow();
        }

        [Fact]
        public void AnInvitationRoundTrips()
        {
            using var invitation = MemberHubInvitation.Deserialize(Encoding.UTF8.GetBytes(InvitationJson())).ValueOrThrow();
            var first = invitation.Serialize().ValueOrThrow();
            using var again = MemberHubInvitation.Deserialize(first).ValueOrThrow();
            Assert.Equal(first, again.Serialize().ValueOrThrow());
        }

        [Fact]
        public void BytesThatAreNoInvitationAreRefused()
        {
            Assert.Equal(StatusCode.MemberInvitationInvalid, MemberHubInvitation.Deserialize(Encoding.UTF8.GetBytes("not json")).Info.StatusCode);
            Assert.Equal(StatusCode.MemberInvitationInvalid, MemberHubInvitation.Deserialize(Encoding.UTF8.GetBytes("{}")).Info.StatusCode);
            Assert.Equal(StatusCode.MemberInvitationInvalid, MemberHubInvitation.Deserialize(Encoding.UTF8.GetBytes(InvitationJson("member-join-request"))).Info.StatusCode);
            Assert.Equal(StatusCode.MemberInvitationInvalid, MemberHubInvitation.Deserialize(Encoding.UTF8.GetBytes(InvitationJson(extra: ",\"sponsor\":\"me\""))).Info.StatusCode);
            Assert.Equal(StatusCode.ArgumentNotSupported, MemberHubInvitation.Deserialize(new byte[0]).Info.StatusCode);
            Assert.Throws<ArgumentNullException>(() => MemberHubInvitation.Deserialize(null!));
        }

        [Fact]
        public void ADisposedInvitationThrows()
        {
            var invitation = MemberHubInvitation.Deserialize(Encoding.UTF8.GetBytes(InvitationJson())).ValueOrThrow();
            invitation.Dispose();
            Assert.Throws<ObjectDisposedException>(() => invitation.Serialize());
            Assert.Throws<ObjectDisposedException>(() => new JoinHubAsMemberOptions(invitation).Invitation.Serialize());
        }

        [Fact]
        public void OptionsOutOfRangeAreRefusedBeforeTheHubIsAsked()
        {
            using var runtime = CreateRuntime();
            Assert.Equal(StatusCode.ArgumentNotSupported, runtime.CreateMemberHubInvitation(new MemberHubInvitationOptions { Lifetime = TimeSpan.FromDays(8) }).Info.StatusCode);
            Assert.Equal(StatusCode.ArgumentNotSupported, runtime.CreateMemberHubInvitation(new MemberHubInvitationOptions { MaxUses = 65 }).Info.StatusCode);
            Assert.Equal(StatusCode.ArgumentNotSupported, runtime.CreateMemberHubInvitation(new MemberHubInvitationOptions { Label = new string('x', 65) }).Info.StatusCode);
            Assert.Equal(StatusCode.ArgumentNotSupported, runtime.CreateMemberHubInvitation(new MemberHubInvitationOptions { Permissions = MemberHubInvitationPermissions.BringDevices }).Info.StatusCode);
            Assert.Throws<ArgumentOutOfRangeException>(() => runtime.CreateMemberHubInvitation(new MemberHubInvitationOptions { Lifetime = TimeSpan.FromSeconds(-1) }));
            Assert.Throws<ArgumentNullException>(() => runtime.CreateMemberHubInvitation(null!));
            Assert.Throws<ArgumentNullException>(() => runtime.JoinHubAsMember(null!));
            Assert.Throws<ArgumentNullException>(() => new JoinHubAsMemberOptions(null!));
        }

        [Fact]
        public void PermissionsMirrorC()
        {
            Assert.Equal(0x1u, (uint)MemberHubInvitationPermissions.Join);
            Assert.Equal(0x2u, (uint)MemberHubInvitationPermissions.Rejoin);
            Assert.Equal(0x4u, (uint)MemberHubInvitationPermissions.Readmit);
            Assert.Equal(0x8u, (uint)MemberHubInvitationPermissions.BringDevices);
        }
    }
}
