using System;
using Quaternar.Qaros;
using Xunit;
using Quaternar.Qaros.Mathematics;

namespace Quaternar.Qaros.Tests
{
    [Collection("QAR Native")]
    public sealed class GuiPanelIntegrationTests : IClassFixture<QarNativeFixture>
    {
        private readonly QarNativeFixture _fixture;

        public GuiPanelIntegrationTests(QarNativeFixture fixture)
        {
            _fixture = fixture;
        }

        [Fact]
        public void AddGuiPanelProducesSnapshotViaUpdateSubscription()
        {
            using var scope = _fixture.CreateSessionScope();
            var expectedPose = new Pose(Quaternion.Identity, new Vector3(1.0f, 2.0f, 3.0f));
            var expectedSize = new GuiPanelSize(1.5f, 1.0f, 1.25f);
            var id = GuiPanelId.Default;
            var panel = QarNativeAssert.WaitForGuiPanelUpdate(
                scope.Session,
                snapshot => id != GuiPanelId.Default && snapshot.Id == id,
                () => id = scope.Session.GuiPanels.GetOrCreate(new GuiPanelOptions(Guid.NewGuid().ToString())
                {
                    DisplayName = "Integration Panel",
                    Pose = expectedPose,
                    Size = expectedSize
                }).ValueOrThrow());
            Assert.NotEqual(GuiPanelId.Default, id);
            QarNativeAssert.PoseEqual(expectedPose, panel.Pose);
            QarNativeAssert.GuiPanelSizeEqual(expectedSize, panel.Size);
            Assert.Equal(GuiPanelState.Visible, panel.State);
        }

        [Fact]
        public void GuiPanelManipulationProducesSnapshotsViaPanelSubscription()
        {
            using var scope = _fixture.CreateSessionScope();
            var id = scope.Session.GuiPanels.GetOrCreate(new GuiPanelOptions(Guid.NewGuid().ToString())).ValueOrThrow();
            var expectedPose = new Pose(new Quaternion(0.0f, 0.25f, 0.0f, 0.96f), new Vector3(5.0f, 6.0f, 7.0f));
            var expectedSize = new GuiPanelSize(2.0f, 1.5f, 0.8f);
            const string expectedUri = "https://example.com/panel";
            var moved = QarNativeAssert.WaitForGuiPanelUpdate(
                scope.Session,
                panel => panel.Id == id && System.Math.Abs(panel.Pose.Position.X - expectedPose.Position.X) < 0.001f,
                () => scope.Session.GuiPanels.UpdatePose(id, expectedPose).EnsureSuccess());
            QarNativeAssert.PosePositionEqual(expectedPose, moved.Pose);
            var resized = QarNativeAssert.WaitForGuiPanelUpdate(
                scope.Session,
                panel => panel.Id == id && System.Math.Abs(panel.Size.WidthMeters - expectedSize.WidthMeters) < 0.001f,
                () => scope.Session.GuiPanels.ChangeSize(id, expectedSize).EnsureSuccess());
            QarNativeAssert.GuiPanelSizeEqual(expectedSize, resized.Size);
            var minimized = QarNativeAssert.WaitForGuiPanelUpdate(
                scope.Session,
                panel => panel.Id == id && panel.State == GuiPanelState.Minimized,
                () => scope.Session.GuiPanels.SetState(id, GuiPanelState.Minimized).EnsureSuccess());
            Assert.Equal(GuiPanelState.Minimized, minimized.State);
            var navigated = QarNativeAssert.WaitForGuiPanelUpdate(
                scope.Session,
                panel => panel.Id == id && panel.ContentUri == expectedUri,
                () => scope.Session.GuiPanels.NavigateToUri(id, expectedUri).EnsureSuccess());
            Assert.Equal(expectedUri, navigated.ContentUri);
            Assert.Equal(GuiPanelContentType.WebsiteUri, navigated.ContentType);
            Assert.Null(navigated.ContentStreamId);
        }

        [Fact]
        public void StreamGuiPanelProducesStreamContentSnapshotsAndCanChangeStream()
        {
            using var scope = _fixture.CreateSessionScope();
            var initialStreamId = StreamId.Parse("00112233-4455-6677-8899-aabbccddeeff");
            var replacementStreamId = StreamId.Parse("ffeeddcc-bbaa-9988-7766-554433221100");
            var id = GuiPanelId.Default;
            var panel = QarNativeAssert.WaitForGuiPanelUpdate(
                scope.Session,
                snapshot => id != GuiPanelId.Default && snapshot.Id == id,
                () => id = scope.Session.GuiPanels.GetOrCreate(new GuiPanelOptions(Guid.NewGuid().ToString())
                {
                    DisplayName = "Stream Panel",
                    ContentStreamId = initialStreamId
                }).ValueOrThrow());
            Assert.Equal(GuiPanelContentType.StreamId, panel.ContentType);
            Assert.Equal(initialStreamId, panel.ContentStreamId);
            Assert.Null(panel.ContentUri);
            var updated = QarNativeAssert.WaitForGuiPanelUpdate(
                scope.Session,
                snapshot => snapshot.Id == id && snapshot.ContentStreamId == replacementStreamId,
                () => scope.Session.GuiPanels.SetStream(id, replacementStreamId).EnsureSuccess());
            Assert.Equal(replacementStreamId, updated.ContentStreamId);
        }
    }
}
