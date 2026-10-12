using System;
using System.Linq;
using Qar;
using Qar.Native;
using Xunit;

namespace QarStreaming.Tests
{
    [Collection("QAR Native")]
    public sealed class QueryAndMutationTests : IClassFixture<QarNativeFixture>
    {
        private readonly QarNativeFixture _fixture;

        public QueryAndMutationTests(QarNativeFixture fixture)
        {
            _fixture = fixture;
        }

        [Fact]
        public void GuiPanelGetAllEventuallyIncludesAddedPanel()
        {
            using var scope = _fixture.CreateSessionScope();
            var id = scope.Session.GuiPanels.GetOrCreate(new GuiPanelOptions(Guid.NewGuid().ToString())
            {
                DisplayName = "query-panel",
                Pose = new Pose(Quaternion.Identity, new Vector3(1, 2, 3)),
                Size = new GuiPanelSize(1.3f, 0.9f, 1.1f)
            }).ValueOrThrow();
            var panel = QarNativeAssert.WaitForPanelInQuery(scope.Session, id);
            Assert.Equal(id, panel.Id);
        }

        [Fact]
        public void AppVolumeGetAllEventuallyIncludesAddedVolume()
        {
            using var scope = _fixture.CreateSessionScope();
            var id = scope.Session.AppVolumes.GetOrCreate(new AppVolumeOptions(Guid.NewGuid().ToString())
            {
                DisplayName = "query-volume",
                Pose = new Pose(Quaternion.Identity, new Vector3(2, 3, 4)),
                Size = new AppVolumeSize(2.0f, 3.0f, 4.0f)
            }).ValueOrThrow();
            var volume = QarNativeAssert.WaitForVolumeInQuery(scope.Session, id);
            Assert.Equal(id, volume.Id);
        }

        [Fact(Skip = "Removing a panel's last visible-to peer makes it vanish from GetAll; product behaviour under review, docs/c-api-ext/01-improvements.md E4.")]
        public void GuiPanelVisibleToPeersRoundTrips()
        {
            using var scope = _fixture.CreateSessionScope();
            var myPeer = scope.Session.Peers.GetMySpec().ValueOrThrow();
            var id = scope.Session.GuiPanels.GetOrCreate(new GuiPanelOptions(Guid.NewGuid().ToString())
            {
                DisplayName = "peer-panel"
            }).ValueOrThrow();
            scope.Session.GuiPanels.UpdateVisibleTo(id, new[] { myPeer.Id }, Array.Empty<PeerId>()).EnsureSuccess();
            var created = QarNativeAssert.WaitUntil(
                () =>
                {
                    var panel = QarNativeAssert.WaitForPanelInQuery(scope.Session, id);
                    return panel.VisibleToPeers.Any(peer => peer == myPeer.Id) ? panel : null;
                },
                panel => panel.Id == id,
                "gui panel visible-to added state");
            Assert.Contains(myPeer.Id, created.VisibleToPeers);
            scope.Session.GuiPanels.UpdateVisibleTo(id, Array.Empty<PeerId>(), new[] { myPeer.Id }).EnsureSuccess();
            var cleared = QarNativeAssert.WaitUntil(
                () =>
                {
                    var panel = QarNativeAssert.WaitForPanelInQuery(scope.Session, id);
                    return panel.VisibleToPeers.Length == 0 ? panel : null;
                },
                panel => panel.Id == id,
                "gui panel visible-to cleared state");
            Assert.Empty(cleared.VisibleToPeers);
            scope.Session.GuiPanels.UpdateVisibleTo(id, new[] { myPeer.Id }, Array.Empty<PeerId>()).EnsureSuccess();
            var restored = QarNativeAssert.WaitUntil(
                () =>
                {
                    var panel = QarNativeAssert.WaitForPanelInQuery(scope.Session, id);
                    return panel.VisibleToPeers.Any(peer => peer == myPeer.Id) ? panel : null;
                },
                panel => panel.Id == id,
                "gui panel visible-to restored state");
            Assert.Contains(myPeer.Id, restored.VisibleToPeers);
        }

        [Fact]
        public void AppVolumeUsedByPeersRoundTrips()
        {
            using var scope = _fixture.CreateSessionScope();
            var myPeer = scope.Session.Peers.GetMySpec().ValueOrThrow();
            var id = scope.Session.AppVolumes.GetOrCreate(new AppVolumeOptions(Guid.NewGuid().ToString())
            {
                DisplayName = "peer-volume",
                InitialPeers = new[] { myPeer.Id }
            }).ValueOrThrow();
            var created = QarNativeAssert.WaitUntil(
                () =>
                {
                    var volume = QarNativeAssert.WaitForVolumeInQuery(scope.Session, id);
                    return volume.UsedByPeers.Any(peer => peer == myPeer.Id) ? volume : null;
                },
                volume => volume.Id == id,
                "app volume used-by initial state");
            Assert.Contains(myPeer.Id, created.UsedByPeers);
            var cleared = QarNativeAssert.WaitForAppVolumeUpdate(
                scope.Session,
                volume => volume.Id == id && volume.UsedByPeers.Length == 0,
                () => scope.Session.AppVolumes.UpdateUsedByPeers(id, Array.Empty<PeerId>(), new[] { myPeer.Id }).EnsureSuccess());
            Assert.Empty(cleared.UsedByPeers);
            var restored = QarNativeAssert.WaitForAppVolumeUpdate(
                scope.Session,
                volume => volume.Id == id && volume.UsedByPeers.Any(peer => peer == myPeer.Id),
                () => scope.Session.AppVolumes.UpdateUsedByPeers(id, new[] { myPeer.Id }, Array.Empty<PeerId>()).EnsureSuccess());
            Assert.Contains(myPeer.Id, restored.UsedByPeers);
        }

        [Fact]
        public void AppVolumeEditingStateRoundTrips()
        {
            using var scope = _fixture.CreateSessionScope();
            var id = scope.Session.AppVolumes.GetOrCreate(new AppVolumeOptions(Guid.NewGuid().ToString()) { DisplayName = "editable" }).ValueOrThrow();
            var started = QarNativeAssert.WaitForAppVolumeUpdate(
                scope.Session,
                volume => volume.Id == id && volume.EditingStatus.IsBeingEdited,
                () => scope.Session.AppVolumes.StartEditing(id).EnsureSuccess());
            Assert.True(started.EditingStatus.IsBeingEdited);
            var stopped = QarNativeAssert.WaitForAppVolumeUpdate(
                scope.Session,
                volume => volume.Id == id && !volume.EditingStatus.IsBeingEdited,
                () => scope.Session.AppVolumes.StopEditing(id).EnsureSuccess());
            Assert.False(stopped.EditingStatus.IsBeingEdited);
        }

        [Fact]
        public void AppWorldAnchorCanBeAddedChangedAndCleared()
        {
            using var scope = _fixture.CreateSessionScope();
            var firstAnchor = new GeoAnchorFrame(
                WorldReferenceSystem.EcefWgs84Meters,
                new Vector3d(1, 2, 3),
                new Vector3d(1, 0, 0),
                new Vector3d(0, 1, 0),
                new Vector3d(0, 0, 1),
                Handedness.Right);
            var secondAnchor = new GeoAnchorFrame(
                WorldReferenceSystem.EcefWgs84Meters,
                new Vector3d(4, 5, 6),
                new Vector3d(1, 0, 0),
                new Vector3d(0, 1, 0),
                new Vector3d(0, 0, 1),
                Handedness.Left);
            var id = scope.Session.AppVolumes.GetOrCreate(new AppVolumeOptions(Guid.NewGuid().ToString())
            {
                DisplayName = "anchored",
                AppWorldAnchor = firstAnchor
            }).ValueOrThrow();
            var created = QarNativeAssert.WaitForAppVolumeUpdate(
                scope.Session,
                volume => volume.Id == id && volume.AppWorldAnchor.HasAnchor,
                () => { });
            Assert.True(created.AppWorldAnchor.HasAnchor);
            QarNativeAssert.AnchorEqual(firstAnchor, created.AppWorldAnchor.Anchor);
            var changed = QarNativeAssert.WaitForAppVolumeUpdate(
                scope.Session,
                volume => volume.Id == id
                          && volume.AppWorldAnchor.HasAnchor
                          && Math.Abs(volume.AppWorldAnchor.Anchor.OriginWorld.X - secondAnchor.OriginWorld.X) < 0.001,
                () => scope.Session.AppVolumes.ChangeAppWorldAnchor(id, secondAnchor).EnsureSuccess());
            QarNativeAssert.AnchorEqual(secondAnchor, changed.AppWorldAnchor.Anchor);
            var cleared = QarNativeAssert.WaitForAppVolumeUpdate(
                scope.Session,
                volume => volume.Id == id && !volume.AppWorldAnchor.HasAnchor,
                () => scope.Session.AppVolumes.ClearAppWorldAnchor(id).EnsureSuccess());
            Assert.False(cleared.AppWorldAnchor.HasAnchor);
        }

        [Fact]
        public void AppVolumeGestureConfigurationDefaultsAndTruncationRoundTrip()
        {
            using var scope = _fixture.CreateSessionScope();
            var defaultId = scope.Session.AppVolumes.GetOrCreate(new AppVolumeOptions(Guid.NewGuid().ToString()) { DisplayName = "defaults" }).ValueOrThrow();
            var defaultSnapshot = QarNativeAssert.WaitForAppVolumeUpdate(
                scope.Session,
                volume => volume.Id == defaultId && volume.GestureConfiguration.MappingRules.Length >= 2,
                () => { });
            Assert.True(defaultSnapshot.GestureConfiguration.MappingRules.Length >= 2);
            var rules = Enumerable.Range(0, NativeConstants.MaxAppVolumeGestureMappingRules + 4)
                .Select(QarNativeAssert.CreateRule)
                .ToArray();
            var id = scope.Session.AppVolumes.GetOrCreate(new AppVolumeOptions(Guid.NewGuid().ToString()) { DisplayName = "gesture-limit" }).ValueOrThrow();
            var changed = QarNativeAssert.WaitForAppVolumeUpdate(
                scope.Session,
                volume => volume.Id == id
                          && volume.GestureConfiguration.MappingRules.Length == NativeConstants.MaxAppVolumeGestureMappingRules,
                () => scope.Session.AppVolumes.ChangeGestureConfiguration(id, new AppVolumeGestureConfiguration { MappingRules = rules }).EnsureSuccess());
            Assert.Equal(NativeConstants.MaxAppVolumeGestureMappingRules, changed.GestureConfiguration.MappingRules.Length);
            QarNativeAssert.GestureRuleEqual(rules[0], changed.GestureConfiguration.MappingRules[0]);
            QarNativeAssert.GestureRuleEqual(rules[NativeConstants.MaxAppVolumeGestureMappingRules - 1], changed.GestureConfiguration.MappingRules[NativeConstants.MaxAppVolumeGestureMappingRules - 1]);
        }

        [Fact]
        public void CloseOperationsTransitionSnapshots()
        {
            using var scope = _fixture.CreateSessionScope();
            var panelId = scope.Session.GuiPanels.GetOrCreate(new GuiPanelOptions(Guid.NewGuid().ToString()) { DisplayName = "close-panel" }).ValueOrThrow();
            var panelClosed = QarNativeAssert.WaitForGuiPanelUpdate(
                scope.Session,
                panel => panel.Id == panelId && panel.State == GuiPanelState.Closed,
                () => scope.Session.GuiPanels.Close(panelId).EnsureSuccess());
            Assert.Equal(GuiPanelState.Closed, panelClosed.State);
            var volumeId = scope.Session.AppVolumes.GetOrCreate(new AppVolumeOptions(Guid.NewGuid().ToString()) { DisplayName = "close-volume" }).ValueOrThrow();
            var volumeClosed = QarNativeAssert.WaitForAppVolumeUpdate(
                scope.Session,
                volume => volume.Id == volumeId && volume.LifetimeStatus == AppVolumeLifetimeStatus.Closed,
                () => scope.Session.AppVolumes.Close(volumeId).EnsureSuccess());
            Assert.Equal(AppVolumeLifetimeStatus.Closed, volumeClosed.LifetimeStatus);
        }
    }
}
