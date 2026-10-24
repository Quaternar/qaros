using System;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Quaternar.Qaros;
using Xunit;

namespace Quaternar.Qaros.Tests
{
    [Collection("QAR Native")]
    public sealed class PeerControlTests : IClassFixture<QarNativeFixture>
    {
        private readonly QarNativeFixture _fixture;

        public PeerControlTests(QarNativeFixture fixture)
        {
            _fixture = fixture;
        }

        [Fact(Skip = "Current native implementation does not propagate UpdateDisplayName back through Session.GetMySpec().")]
        public void UpdateDisplayNameRoundTripsThroughGetMySpec()
        {
            using var scope = _fixture.CreateSessionScope();
            const string updatedName = "peer-roundtrip";
            var mySpec = QarNativeAssert.WaitUntil(
                () =>
                {
                    scope.Session.Peers.UpdateDisplayName(updatedName).EnsureSuccess();
                    var result = scope.Session.Peers.GetMySpec();
                    return result.IsSuccess && result.Value.DisplayName == updatedName ? result.Value : null;
                },
                peer => peer.DisplayName == updatedName,
                "peer display name roundtrip",
                timeoutMs: 10000,
                intervalMs: 100);
            Assert.Equal(updatedName, mySpec.DisplayName);
        }

        [Fact]
        public void AnApplicationJoinsAsADesktopSource()
        {
            using var scope = _fixture.CreateSessionScope();
            var mySpec = scope.Session.Peers.GetMySpec().ValueOrThrow();
            Assert.Equal(PeerRole.Source, mySpec.Role);
            Assert.False(mySpec.IsTargetApp);
            Assert.Equal(DeviceCategory.DesktopPC, mySpec.Category);
            var expectedId = System.Runtime.InteropServices.RuntimeInformation.IsOSPlatform(System.Runtime.InteropServices.OSPlatform.Windows) ? "DesktopWindows" : "DesktopLinux";
            Assert.Equal(expectedId, mySpec.DeviceId);
        }

        [Fact]
        public void UpdateDeviceIdentityDescribesACameraWithAnUnknownId()
        {
            using var scope = _fixture.CreateSessionScope();
            var identity = new PeerDeviceIdentity(DeviceCategory.Camera, "SomeCameraNoCatalogKnows")
            {
                DeviceFamily = "Luxonis",
                DeviceName = "OAK-D Pro W \U0001F4F7"
            };
            var myId = scope.Session.Peers.GetMySpec().ValueOrThrow().Id;
            scope.Session.Peers.UpdateDeviceIdentity(identity).EnsureSuccess();
            // GetMySpec() is the spec the session joined with (it does not follow updates, see the
            // skipped display-name test); the session's peer list does.
            var mySpec = QarNativeAssert.WaitUntil(
                () =>
                {
                    var result = scope.Session.Peers.GetAll();
                    return result.IsSuccess ? Array.Find(result.Value, peer => peer.Id.Equals(myId)) : null;
                },
                peer => peer.DeviceId == identity.DeviceId,
                "device identity update",
                timeoutMs: 10000,
                intervalMs: 100);
            Assert.Equal(PeerRole.Source, mySpec.Role);
            Assert.Equal(DeviceCategory.Camera, mySpec.Category);
            Assert.Equal("Luxonis", mySpec.DeviceFamily);
            Assert.Equal(identity.DeviceName, mySpec.DeviceName);
        }

        // A joined peer is Online in the hub's view, and the deprecated AppState still reads the
        // Running every session claims when it joins.
        [Fact]
        public void AJoinedPeerReadsOnlineAndItsDeprecatedAppStateStaysRunning()
        {
            using var scope = _fixture.CreateSessionScope();
            var myId = scope.Session.Peers.GetMySpec().ValueOrThrow().Id;
            var mySpec = QarNativeAssert.WaitUntil(
                () =>
                {
                    var result = scope.Session.Peers.GetAll();
                    return result.IsSuccess ? Array.Find(result.Value, peer => peer.Id.Equals(myId)) : null;
                },
                peer => peer.Presence == PeerPresence.Online,
                "own peer online",
                timeoutMs: 10000,
                intervalMs: 100);
            Assert.Equal(PeerPresence.Online, mySpec.Presence);
#pragma warning disable CS0618 // the deprecated AppState keeps working
            Assert.Equal(AppState.Running, mySpec.AppState);
#pragma warning restore CS0618
        }

        // A peer that exits cleanly announces its leave: the session lists it as Leaving rather than
        // Online, and the deprecated AppState reads ShuttingDown.
        [Fact]
        public void APeerThatLeavesReadsLeaving()
        {
            using var scope = _fixture.CreateSessionScope();
            var peer = scope.Hub.CreatePeer("QarStreamingSharpTests leaving peer");
            var peerId = peer.Id;
            PeerSpec? Find()
            {
                var result = scope.Session.Peers.GetAll();
                return result.IsSuccess ? Array.Find(result.Value, spec => spec.Id.Equals(peerId)) : null;
            }
            QarNativeAssert.WaitUntil(Find, spec => spec.Presence == PeerPresence.Online, "peer online", timeoutMs: 15000, intervalMs: 100);
            peer.Dispose();
            var left = QarNativeAssert.WaitUntil(Find, spec => spec.Presence == PeerPresence.Leaving, "peer leaving", timeoutMs: 15000, intervalMs: 100);
#pragma warning disable CS0618 // the deprecated AppState keeps working
            Assert.Equal(AppState.ShuttingDown, left.AppState);
#pragma warning restore CS0618
        }

        [Theory]
        [InlineData(DeviceCategory.Headset)]
        [InlineData(DeviceCategory.Visualizer)]
        [InlineData(DeviceCategory.Unknown)]
        public void AnApplicationCannotDeclareATargetCategory(DeviceCategory category)
        {
            using var scope = _fixture.CreateSessionScope();
            var result = scope.Session.Peers.UpdateDeviceIdentity(new PeerDeviceIdentity(category, "MetaQuest3"));
            Assert.False(result.IsSuccess);
            Assert.Equal(StatusCode.PeerDeviceCategoryNotAllowed, result.Info.StatusCode);
            Assert.Equal(DeviceCategory.DesktopPC, scope.Session.Peers.GetMySpec().ValueOrThrow().Category);
        }

        [Fact]
        public void GlobalSubscribeUpdatesProvidesPeerPayloadContents()
        {
            using var scope = _fixture.CreateSessionScope();
            const string updatedName = "peer-payload";
            PeerSpec? observed = null;
            using var subscription = scope.Session.Peers.SubscribeUpdates((sender, args) =>
            {
                if (args.Peer.DisplayName == updatedName)
                    observed = args.Peer;
            }).ValueOrThrow();
            var payload = QarNativeAssert.WaitUntil(
                () =>
                {
                    scope.Session.Peers.UpdateDisplayName(updatedName).EnsureSuccess();
                    return observed;
                },
                peer => peer.DisplayName == updatedName,
                "peer payload update",
                timeoutMs: 10000,
                intervalMs: 100);
            Assert.NotEqual(PeerId.Default, payload.Id);
            Assert.Equal(updatedName, payload.DisplayName);
        }

        [Fact]
        public void ControlUpdatedEventIsRaisedWithPeerPayload()
        {
            using var scope = _fixture.CreateSessionScope();
            const string updatedName = "peer-control-event";
            PeerSpec? observed = null;
            void OnUpdated(object? sender, PeerSpecUpdatedEventArgs args)
            {
                if (args.Peer.DisplayName == updatedName)
                    observed = args.Peer;
            }
            scope.Session.Peers.Updated += OnUpdated;
            try
            {
                using var subscription = scope.Session.Peers.SubscribeUpdates().ValueOrThrow();
                var payload = QarNativeAssert.WaitUntil(
                    () =>
                    {
                        scope.Session.Peers.UpdateDisplayName(updatedName).EnsureSuccess();
                        return observed;
                    },
                    peer => peer.DisplayName == updatedName,
                    "peer control updated event",
                    timeoutMs: 10000,
                    intervalMs: 100);
                Assert.Equal(updatedName, payload.DisplayName);
            }
            finally
            {
                scope.Session.Peers.Updated -= OnUpdated;
            }
        }

#if NET8_0_OR_GREATER
        [Fact]
        public async Task WatchUpdatesAsyncYieldsMultipleSequentialUpdates()
        {
            using var scope = _fixture.CreateSessionScope();
            var observedNames = new List<string>();
            var sync = new object();
            using var cts = new CancellationTokenSource(TimeSpan.FromSeconds(10));
            var task = Task.Run(async () =>
            {
                await foreach (var peer in scope.Session.Peers.WatchUpdatesAsync(cts.Token))
                {
                    if (peer.DisplayName == "peer-watch-1" || peer.DisplayName == "peer-watch-2")
                    {
                        lock (sync)
                        {
                            if (!observedNames.Contains(peer.DisplayName))
                                observedNames.Add(peer.DisplayName);
                            if (observedNames.Count == 2)
                                return;
                        }
                    }
                }
            }, cts.Token);
            QarNativeAssert.WaitUntil(
                () =>
                {
                    scope.Session.Peers.UpdateDisplayName("peer-watch-1").EnsureSuccess();
                    lock (sync)
                        return observedNames.Contains("peer-watch-1");
                },
                "first peer watch update",
                timeoutMs: 10000,
                intervalMs: 100);
            QarNativeAssert.WaitUntil(
                () =>
                {
                    scope.Session.Peers.UpdateDisplayName("peer-watch-2").EnsureSuccess();
                    lock (sync)
                        return observedNames.Contains("peer-watch-2");
                },
                "second peer watch update",
                timeoutMs: 10000,
                intervalMs: 100);
            await task;
            lock (sync)
            {
                Assert.Equal(2, observedNames.Count);
                Assert.Equal("peer-watch-1", observedNames[0]);
                Assert.Equal("peer-watch-2", observedNames[1]);
            }
        }
#endif

        [Fact]
        public void PeerMethodsThrowAfterSessionDispose()
        {
            using var scope = _fixture.CreateSessionScope();
            var peers = scope.Session.Peers;
            scope.Session.Dispose();
            Assert.Throws<ObjectDisposedException>(() => peers.GetMySpec());
            Assert.Throws<ObjectDisposedException>(() => peers.GetAll());
            Assert.Throws<ObjectDisposedException>(() => peers.UpdateDisplayName("disposed"));
            Assert.Throws<ObjectDisposedException>(() => peers.SubscribeUpdates());
            Assert.Throws<ObjectDisposedException>(() => peers.SubscribeUpdates(CancellationToken.None));
            Assert.Throws<ObjectDisposedException>(() => peers.SubscribeUpdates((sender, args) => { }));
            Assert.Throws<ObjectDisposedException>(() => peers.SubscribeUpdates((sender, args) => { }, CancellationToken.None));
        }
    }
}
