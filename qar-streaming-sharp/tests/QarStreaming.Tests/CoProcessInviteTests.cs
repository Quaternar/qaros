using System;
using System.IO;
using System.Linq;
using System.Text;
using Quaternar.Qaros;
using Xunit;

namespace Quaternar.Qaros.Tests
{
    /// <summary>
    /// App tags, co-process invites and ephemeral storage through the C# binding against real hubs
    /// of qar-hub-test-ext (GitHub #268, stories A1 to A3): a local app joins with a tag, asks for a
    /// tagged invite, and a co-process with an in-memory identity joins with it. Every runtime here
    /// lives in this one process.
    /// </summary>
    [Collection("QAR Native")]
    public sealed class CoProcessInviteTests : IClassFixture<QarNativeFixture>
    {
        private const string ParentTag = "parent.sharp-tests.quaternar";
        private const string CompanionTag = "companion.sharp-tests.quaternar";

        public CoProcessInviteTests(QarNativeFixture fixture)
        {
            _ = fixture;
        }

        [HubTestExtFact]
        public void ALocalAppJoinsWithATagAndOneProcessSharesThePeerOfATag()
        {
            using var hub = new InvitationTestHub("app-tag");
            using var storage = new TempFolder();
            using var runtime = Runtime.Create(new RuntimeOptions { StorageFolderPath = storage.Path }).ValueOrThrow();
            try
            {
                hub.ApproveThisProcess(false, 0);
                hub.UseAsHubOnThisPc();
                var first = OnboardLocal(runtime, ParentTag);
                using var firstSession = first.Session;
                Assert.Equal(OnboardingId.Default, first.OnboardingId);
                var firstPeer = firstSession.Peers.GetMySpec().ValueOrThrow().Id;
                // A2: the same tag in the same process is the same session, so the same peer.
                using var sameTag = OnboardLocal(runtime, ParentTag).Session;
                Assert.Equal(firstPeer, sameTag.Peers.GetMySpec().ValueOrThrow().Id);
                // A1: another tag is another instance of the app, so another peer.
                using var otherTag = OnboardLocal(runtime, "other.sharp-tests.quaternar").Session;
                Assert.NotEqual(firstPeer, otherTag.Peers.GetMySpec().ValueOrThrow().Id);
            }
            finally
            {
                InvitationTestHub.ClearHubOnThisPc();
            }
        }

        [HubTestExtFact]
        public void ACoProcessJoinsByATaggedInviteAndWritesNothing()
        {
            using var hub = new InvitationTestHub("co-process");
            using var parentStorage = new TempFolder();
            using var coProcessStorage = new TempFolder();
            using var parentRuntime = Runtime.Create(new RuntimeOptions { StorageFolderPath = parentStorage.Path }).ValueOrThrow();
            try
            {
                hub.ApproveThisProcess(false, 0);
                hub.UseAsHubOnThisPc();
                using var parent = OnboardLocal(parentRuntime, ParentTag).Session;
                var parentPeer = parent.Peers.GetMySpec().ValueOrThrow().Id;
                byte[] wire;
                using (var minted = parent.RequestOnboardingInvite(CompanionTag).ValueOrThrow())
                    wire = minted.Serialize().ValueOrThrow();
                using var coProcessRuntime = Runtime.Create(new RuntimeOptions
                {
                    StorageFolderPath = coProcessStorage.Path,
                    EphemeralStorage = true
                }).ValueOrThrow();
                using var invite = OnboardingInvite.Deserialize(wire).ValueOrThrow();
                var joined = coProcessRuntime.Onboard(new OnboardOptions()
                    .UseInvite(invite)
                    .WithPresentation(new PeerPresentationOptions { DisplayName = "QarStreamingSharpTests co-process" })).ValueOrThrow();
                using var coProcess = joined.Session;
                Assert.Equal(OnboardingId.Default, joined.OnboardingId);
                Assert.NotEqual(parentPeer, coProcess.Peers.GetMySpec().ValueOrThrow().Id);
                Assert.Empty(coProcessStorage.Files());
                // Nothing persisted, so there is nothing to rejoin or forget.
                var slot = OnboardingId.Parse(Guid.NewGuid().ToString());
                Assert.Equal(StatusCode.ArgumentNotSupported, coProcessRuntime.Rejoin(new RejoinOptions(slot)).Info.StatusCode);
                Assert.Equal(StatusCode.ArgumentNotSupported, coProcessRuntime.Forget(new ForgetOptions(slot)).Info.StatusCode);
            }
            finally
            {
                InvitationTestHub.ClearHubOnThisPc();
            }
        }

        [HubTestExtFact]
        public void TheAsyncRequestMintsATaggedInviteToo()
        {
            using var hub = new InvitationTestHub("co-process-async");
            using var storage = new TempFolder();
            using var runtime = Runtime.Create(new RuntimeOptions { StorageFolderPath = storage.Path }).ValueOrThrow();
            try
            {
                hub.ApproveThisProcess(false, 0);
                hub.UseAsHubOnThisPc();
                using var parent = OnboardLocal(runtime, ParentTag).Session;
                using var invite = parent.RequestOnboardingInviteAsync(CompanionTag).GetAwaiter().GetResult().ValueOrThrow();
                Assert.NotEmpty(invite.Serialize().ValueOrThrow());
            }
            finally
            {
                InvitationTestHub.ClearHubOnThisPc();
            }
        }

        // Only a local app (or a launcher child) may mint a co-process invite; a session that joined
        // by an invite, as a device does, is refused.
        [HubTestExtFact]
        public void ASessionThatJoinedByAnInviteCannotMintOne()
        {
            using var hub = new HubTestExt();
            using var storage = new TempFolder();
            using var runtime = Runtime.Create(new RuntimeOptions { StorageFolderPath = storage.Path }).ValueOrThrow();
            using var invite = OnboardingInvite.Deserialize(Encoding.UTF8.GetBytes(hub.MintInvite())).ValueOrThrow();
            using var session = runtime.Onboard(new OnboardOptions()
                .UseInvite(invite)
                .WithPresentation(new PeerPresentationOptions { DisplayName = "QarStreamingSharpTests invited" })).ValueOrThrow().Session;
            var refused = session.RequestOnboardingInvite(CompanionTag);
            Assert.False(refused.IsSuccess, "an invite-onboarded session minted a co-process invite");
            Assert.Equal(StatusCode.OnboardingInviteRefused, refused.Info.StatusCode);
        }

        // The ephemeral option needs no hub: a runtime that keeps its identity in memory has no slot.
        [Fact]
        public void AnEphemeralRuntimeHasNoSlotToRejoinOrForget()
        {
            using var storage = new TempFolder();
            using var runtime = Runtime.Create(new RuntimeOptions { StorageFolderPath = storage.Path, EphemeralStorage = true }).ValueOrThrow();
            var slot = OnboardingId.Parse(Guid.NewGuid().ToString());
            Assert.Equal(StatusCode.ArgumentNotSupported, runtime.Rejoin(new RejoinOptions(slot)).Info.StatusCode);
            Assert.Equal(StatusCode.ArgumentNotSupported, runtime.Forget(new ForgetOptions(slot)).Info.StatusCode);
            Assert.Empty(storage.Files());
        }

        private static OnboardSession OnboardLocal(Runtime runtime, string tag)
        {
            return runtime.Onboard(new OnboardOptions()
                .UseLocalApp(new OnboardLocalApp { ApprovalTimeout = TimeSpan.FromSeconds(5), Tag = tag })
                .WithPresentation(new PeerPresentationOptions { DisplayName = "QarStreamingSharpTests " + tag })).ValueOrThrow();
        }

        /// <summary>A storage folder of its own, not created up front, deleted afterwards.</summary>
        private sealed class TempFolder : IDisposable
        {
            public TempFolder()
            {
                Path = System.IO.Path.Combine(System.IO.Path.GetTempPath(), "qar-sharp-tests", "co-process", Guid.NewGuid().ToString("N"));
            }
            public string Path { get; }
            public string[] Files()
            {
                return Directory.Exists(Path) ? Directory.EnumerateFiles(Path, "*", SearchOption.AllDirectories).ToArray() : Array.Empty<string>();
            }
            public void Dispose()
            {
                try
                {
                    if (Directory.Exists(Path))
                        Directory.Delete(Path, true);
                }
                catch (IOException)
                {
                }
                catch (UnauthorizedAccessException)
                {
                }
            }
        }
    }
}
