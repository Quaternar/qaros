using System;
using System.IO;
using System.Text;
using System.Threading.Tasks;
using Quaternar.Qaros;
using Xunit;

namespace Quaternar.Qaros.Tests
{
    /// <summary>
    /// The sponsored member join surface (docs/hub-topology, GitHub #248): the blob round trip and
    /// the checks made before a hub is asked. No hub runs here, so the steps that reach one are
    /// covered by the C++ integration tests.
    /// </summary>
    [Collection("QAR Native")]
    public sealed class MemberJoinTests : IClassFixture<QarNativeFixture>
    {
        private const string FakeKey = "AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8=";

        private readonly QarNativeFixture _fixture;

        public MemberJoinTests(QarNativeFixture fixture)
        {
            _fixture = fixture;
        }

        private static byte[] RequestBytes()
        {
            return Encoding.UTF8.GetBytes(
                "{\"createdUnix\":1760000000,\"csrPem\":\"csr\",\"displayName\":\"Workshop PC\","
                + "\"hasMembers\":false,\"machineName\":\"WORKSHOP-2\",\"memberHubKeyDer\":\"" + FakeKey + "\","
                + "\"memberHubPeerId\":\"00112233-4455-6677-8899-aabbccddeeff\","
                + "\"nonce\":\"00112233445566778899aabbccddeeff\",\"operatorPeerIds\":[],\"signature\":\"c2ln\",\"ticket\":\"\",\"version\":2}");
        }

        private static byte[] ReplyBytes(string status)
        {
            return Encoding.UTF8.GetBytes(
                "{\"caBundlePem\":\"\",\"certificateChainPem\":\"\",\"csrNeeded\":null,\"generation\":3,\"message\":\"\","
                + "\"onboardingHost\":\"studio.local\",\"onboardingPort\":47000,"
                + "\"policy\":{\"mayApproveLocalApps\":true,\"mayPairHeadsets\":true,\"primaryMayLaunch\":true},"
                + "\"primaryAuthorityKeyDer\":\"" + FakeKey + "\",\"primaryDisplayName\":\"Studio PC\","
                + "\"primaryHubPeerId\":\"00112233-4455-6677-8899-aabbccddee00\",\"reasonCode\":\"\","
                + "\"redirect\":null,\"requestNonce\":\"00112233445566778899aabbccddeeff\",\"sasNonce\":\"\","
                + "\"sessionId\":\"00112233-4455-6677-8899-aabbccddee11\",\"sessionInvite\":null,"
                + "\"signature\":\"c2ln\",\"status\":\"" + status + "\",\"version\":2}");
        }

        private static Runtime CreateRuntime()
        {
            var storageRoot = Path.Combine(Path.GetTempPath(), "qar-sharp-tests", Guid.NewGuid().ToString("N"));
            return Runtime.Create(new RuntimeOptions { StorageFolderPath = storageRoot }).ValueOrThrow();
        }

        [Fact]
        public void ARequestBlobRoundTrips()
        {
            using var blob = MemberJoinBlob.Deserialize(RequestBytes()).ValueOrThrow();
            Assert.True(blob.IsValid());
            var first = blob.Serialize().ValueOrThrow();
            using var again = MemberJoinBlob.Deserialize(first).ValueOrThrow();
            Assert.Equal(first, again.Serialize().ValueOrThrow());
        }

        [Fact]
        public void AReplyBlobRoundTrips()
        {
            using var blob = MemberJoinBlob.Deserialize(ReplyBytes("accepted")).ValueOrThrow();
            var first = blob.Serialize().ValueOrThrow();
            using var again = MemberJoinBlob.Deserialize(first).ValueOrThrow();
            Assert.Equal(first, again.Serialize().ValueOrThrow());
        }

        [Fact]
        public void BytesThatAreNoBlobAreRefused()
        {
            Assert.Equal(StatusCode.ArgumentNotSupported, MemberJoinBlob.Deserialize(Encoding.UTF8.GetBytes("not json")).Info.StatusCode);
            Assert.Equal(StatusCode.ArgumentNotSupported, MemberJoinBlob.Deserialize(Encoding.UTF8.GetBytes("{}")).Info.StatusCode);
            Assert.Equal(StatusCode.ArgumentNotSupported, MemberJoinBlob.Deserialize(new byte[0]).Info.StatusCode);
            Assert.Throws<ArgumentNullException>(() => MemberJoinBlob.Deserialize(null!));
        }

        [Fact]
        public void ADisposedBlobThrows()
        {
            var blob = MemberJoinBlob.Deserialize(RequestBytes()).ValueOrThrow();
            blob.Dispose();
            Assert.Throws<ObjectDisposedException>(() => blob.Serialize());
        }

        [Fact]
        public void CompleteRefusesAReplyThatDoesNotAdmit()
        {
            using var runtime = CreateRuntime();
            using var denied = MemberJoinBlob.Deserialize(ReplyBytes("denied")).ValueOrThrow();
            using var pending = MemberJoinBlob.Deserialize(ReplyBytes("pending")).ValueOrThrow();
            Assert.Equal(StatusCode.MemberJoinDenied, runtime.CompleteMemberJoin(new MemberJoinCompleteOptions(denied)).Info.StatusCode);
            Assert.Equal(StatusCode.MemberJoinPending, runtime.CompleteMemberJoin(new MemberJoinCompleteOptions(pending)).Info.StatusCode);
        }

        [Fact]
        public void CompleteRefusesARequestBlobAndAnotherKey()
        {
            using var runtime = CreateRuntime();
            using var request = MemberJoinBlob.Deserialize(RequestBytes()).ValueOrThrow();
            using var accepted = MemberJoinBlob.Deserialize(ReplyBytes("accepted")).ValueOrThrow();
            Assert.Equal(StatusCode.ArgumentNotSupported, runtime.CompleteMemberJoin(new MemberJoinCompleteOptions(request)).Info.StatusCode);
            var otherKey = new MemberJoinCompleteOptions(accepted) { ExpectedPrimaryFingerprint = new string('a', 64) };
            Assert.Equal(StatusCode.OnboardingHubNotAuthentic, runtime.CompleteMemberJoin(otherKey).Info.StatusCode);
            var malformed = new MemberJoinCompleteOptions(accepted) { ExpectedPrimaryFingerprint = "abcd" };
            Assert.Equal(StatusCode.ArgumentNotSupported, runtime.CompleteMemberJoin(malformed).Info.StatusCode);
        }

        [Fact]
        public async Task SponsorRefusesAReplyBlobAsTheRequest()
        {
            using var scope = _fixture.CreateSessionScope();
            using var reply = MemberJoinBlob.Deserialize(ReplyBytes("accepted")).ValueOrThrow();
            var options = new SponsorMemberOptions(reply) { ClientAddress = "10.0.0.7", ClientName = "Trainee 1" };
            Assert.Equal(StatusCode.ArgumentNotSupported, scope.Session.SponsorMember(options).Info.StatusCode);
            var asyncResult = await scope.Session.SponsorMemberAsync(options);
            Assert.Equal(StatusCode.ArgumentNotSupported, asyncResult.Info.StatusCode);
        }
    }
}
