using System;
using System.Threading;
using System.Threading.Tasks;
using Qar;
using Xunit;

namespace QarStreaming.Tests
{
    [Collection("QAR Native")]
    public sealed class SubscriptionLifecycleTests : IClassFixture<QarNativeFixture>
    {
        private readonly QarNativeFixture _fixture;

        public SubscriptionLifecycleTests(QarNativeFixture fixture)
        {
            _fixture = fixture;
        }

        [Fact]
        public void PeerSubscriptionsStopDeliveringAfterDispose()
        {
            using var scope = _fixture.CreateSessionScope();
            var count = 0;
            using var subscription = scope.Session.Peers.SubscribeUpdates((sender, args) => Interlocked.Increment(ref count)).ValueOrThrow();
            QarNativeAssert.WaitUntil(
                () =>
                {
                    scope.Session.Peers.UpdateDisplayName("peer-one").EnsureSuccess();
                    return Volatile.Read(ref count) > 0;
                },
                "first peer update");
            subscription.Dispose();
            var before = Volatile.Read(ref count);
            scope.Session.Peers.UpdateDisplayName("peer-two").EnsureSuccess();
            Thread.Sleep(300);
            Assert.Equal(before, Volatile.Read(ref count));
        }

        [Fact]
        public void MultiplePeerSubscriptionsReceiveSameUpdate()
        {
            using var scope = _fixture.CreateSessionScope();
            var first = 0;
            var second = 0;
            using var sub1 = scope.Session.Peers.SubscribeUpdates((sender, args) => Interlocked.Increment(ref first)).ValueOrThrow();
            using var sub2 = scope.Session.Peers.SubscribeUpdates((sender, args) => Interlocked.Increment(ref second)).ValueOrThrow();
            QarNativeAssert.WaitUntil(
                () =>
                {
                    scope.Session.Peers.UpdateDisplayName("fanout").EnsureSuccess();
                    return Volatile.Read(ref first) > 0 && Volatile.Read(ref second) > 0;
                },
                "fanout peer update");
            Assert.True(first > 0);
            Assert.True(second > 0);
        }

#if NET8_0_OR_GREATER
        [Fact]
        public async Task PeerWatchUpdatesAsyncEmitsItems()
        {
            using var scope = _fixture.CreateSessionScope();
            var peer = await QarNativeAssert.ReadSingleAsync(
                scope.Session.Peers.WatchUpdatesAsync(),
                item => item.DisplayName == "peer-watch",
                () => scope.Session.Peers.UpdateDisplayName("peer-watch").EnsureSuccess());
            Assert.Equal("peer-watch", peer.DisplayName);
        }

        [Fact]
        public async Task GuiPanelWatchUpdatesAsyncEmitsItems()
        {
            using var scope = _fixture.CreateSessionScope();
            var id = scope.Session.GuiPanels.GetOrCreate(new GuiPanelOptions(Guid.NewGuid().ToString()) { DisplayName = "watch-panel" }).ValueOrThrow();
            var panel = await QarNativeAssert.ReadSingleAsync(
                scope.Session.GuiPanels.WatchUpdatesAsync(id),
                item => item.ContentUri == "https://watch.example",
                () => scope.Session.GuiPanels.NavigateToUri(id, "https://watch.example").EnsureSuccess());
            Assert.Equal(id, panel.Id);
            Assert.Equal("https://watch.example", panel.ContentUri);
        }

        [Fact]
        public async Task AppVolumeWatchUpdatesAsyncEmitsItems()
        {
            using var scope = _fixture.CreateSessionScope();
            var id = scope.Session.AppVolumes.GetOrCreate(new AppVolumeOptions(Guid.NewGuid().ToString()) { DisplayName = "watch-volume" }).ValueOrThrow();
            var volume = await QarNativeAssert.ReadSingleAsync(
                scope.Session.AppVolumes.WatchUpdatesAsync(),
                item => item.Id == id && Math.Abs(item.AppScale - 3.5f) < 0.001f,
                () => scope.Session.AppVolumes.ChangeAppScale(id, 3.5f).EnsureSuccess());
            Assert.Equal(id, volume.Id);
            Assert.Equal(3.5f, volume.AppScale, 5);
        }

        [Fact]
        public async Task WatchersRespectCancellation()
        {
            using var scope = _fixture.CreateSessionScope();
            using var peerCancellation = new CancellationTokenSource();
            peerCancellation.Cancel();
            await Assert.ThrowsAnyAsync<OperationCanceledException>(async () =>
            {
                await foreach (var _ in scope.Session.Peers.WatchUpdatesAsync(peerCancellation.Token))
                {
                }
            });
        }
#endif

        [Fact]
        public void AppVolumeSubscriptionStopsAfterDispose()
        {
            using var scope = _fixture.CreateSessionScope();
            var id = scope.Session.AppVolumes.GetOrCreate(new AppVolumeOptions(Guid.NewGuid().ToString()) { DisplayName = "sub-volume" }).ValueOrThrow();
            var count = 0;
            using var subscription = scope.Session.AppVolumes.SubscribeUpdates((sender, args) =>
            {
                if (args.Volume.Id == id)
                    Interlocked.Increment(ref count);
            }).ValueOrThrow();
            QarNativeAssert.WaitUntil(
                () =>
                {
                    scope.Session.AppVolumes.ChangeAppScale(id, 1.5f).EnsureSuccess();
                    return Volatile.Read(ref count) > 0;
                },
                "first app volume update");
            subscription.Dispose();
            var before = Volatile.Read(ref count);
            scope.Session.AppVolumes.ChangeAppScale(id, 2.5f).EnsureSuccess();
            Thread.Sleep(300);
            Assert.Equal(before, Volatile.Read(ref count));
        }
    }
}
