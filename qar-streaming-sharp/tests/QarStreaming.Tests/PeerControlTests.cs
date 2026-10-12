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
