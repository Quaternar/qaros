using System;
using Qar;
using Xunit;

namespace QarStreaming.Tests
{
    [Collection("QAR Native")]
    public sealed class AppVolumeIntegrationTests : IClassFixture<QarNativeFixture>
    {
        private readonly QarNativeFixture _fixture;

        public AppVolumeIntegrationTests(QarNativeFixture fixture)
        {
            _fixture = fixture;
        }

        [Fact]
        public void AddAppVolumeProducesSnapshotViaUpdateSubscription()
        {
            using var scope = _fixture.CreateSessionScope();
            var expectedPose = new Pose(new Quaternion(0.1f, 0.2f, 0.3f, 0.9f), new Vector3(2.0f, 1.0f, -3.0f));
            var expectedSize = new AppVolumeSize(4.0f, 3.0f, 2.0f);
            var expectedAppPose = new Pose(Quaternion.Identity, new Vector3(0.5f, 0.25f, 0.75f));
            var expectedAnchor = new GeoAnchorFrame(
                WorldReferenceSystem.EcefWgs84Meters,
                new Vector3d(1.0, 2.0, 3.0),
                new Vector3d(1.0, 0.0, 0.0),
                new Vector3d(0.0, 1.0, 0.0),
                new Vector3d(0.0, 0.0, 1.0),
                Handedness.Right);
            var expectedGestures = new AppVolumeGestureConfiguration
            {
                MappingRules = new[]
                {
                    new AppVolumeGestureMappingRule
                    {
                        GestureKind = GestureKind.SinglePointer6Dof,
                        Enabled = true,
                        Precision = 0.02f,
                        AppScaleSensitivityMode = AppScaleSensitivityMode.InverseToAppScale,
                        TranslationAxes = AppVolumeAxisFlags.X | AppVolumeAxisFlags.Y,
                        RotationAxes = AppVolumeAxisFlags.Z,
                        EnableGesturesFromOutsideOfVolume = false,
                        ControllerRayStyle = new AppVolumeControllerRayStyle(AppVolumeControllerRayLineStyle.Dotted, 26, 51, 77, 102),
                        AppTransformMapping = GestureAppTransformMapping.AppPose
                    }
                }
            };
            var id = AppVolumeId.Default;
            var volume = QarNativeAssert.WaitForAppVolumeUpdate(
                scope.Session,
                snapshot => id != AppVolumeId.Default && snapshot.Id == id,
                () => id = scope.Session.AppVolumes.GetOrCreate(new AppVolumeOptions(Guid.NewGuid().ToString())
                {
                    DisplayName = "Integration Volume",
                    Pose = expectedPose,
                    Size = expectedSize,
                    AppPose = expectedAppPose,
                    AppScale = 4.25f,
                    AppWorldAnchor = expectedAnchor,
                    GestureConfiguration = expectedGestures
                }).ValueOrThrow());
            Assert.NotEqual(AppVolumeId.Default, id);
            Assert.Equal("Integration Volume", volume.DisplayName);
            QarNativeAssert.PosePositionEqual(expectedPose, volume.Pose);
            QarNativeAssert.AppVolumeSizeEqual(expectedSize, volume.Size);
            QarNativeAssert.PosePositionEqual(expectedAppPose, volume.AppPose);
            Assert.Equal(4.25f, volume.AppScale, 5);
            Assert.True(volume.AppWorldAnchor.HasAnchor);
            QarNativeAssert.AnchorEqual(expectedAnchor, volume.AppWorldAnchor.Anchor);
            Assert.Single(volume.GestureConfiguration.MappingRules);
            QarNativeAssert.GestureRuleEqual(expectedGestures.MappingRules[0], volume.GestureConfiguration.MappingRules[0]);
        }

        [Fact]
        public void AppVolumeManipulationUpdatesSnapshotsAndLatestValues()
        {
            using var scope = _fixture.CreateSessionScope();
            var id = scope.Session.AppVolumes.GetOrCreate(new AppVolumeOptions(Guid.NewGuid().ToString())
            {
                DisplayName = "Initial Volume"
            }).ValueOrThrow();
            var expectedPose = new Pose(new Quaternion(0.3f, 0.4f, 0.1f, 0.85f), new Vector3(10.0f, 11.0f, 12.0f));
            var expectedSize = new AppVolumeSize(5.0f, 4.0f, 3.0f);
            var expectedAppPose = new Pose(new Quaternion(0.0f, 0.1f, 0.0f, 0.99f), new Vector3(-1.0f, 2.0f, 3.0f));
            var expectedGestures = new AppVolumeGestureConfiguration
            {
                MappingRules = new[]
                {
                    new AppVolumeGestureMappingRule
                    {
                        GestureKind = GestureKind.DualPointerTranslateDistance,
                        Enabled = true,
                        Precision = 0.5f,
                        AppScaleSensitivityMode = AppScaleSensitivityMode.Constant,
                        TranslationAxes = AppVolumeAxisFlags.All,
                        RotationAxes = AppVolumeAxisFlags.None,
                        EnableGesturesFromOutsideOfVolume = true,
                        ControllerRayStyle = new AppVolumeControllerRayStyle(AppVolumeControllerRayLineStyle.Solid, 255, 128, 64, 255),
                        AppTransformMapping = GestureAppTransformMapping.AppScale
                    }
                }
            };
            scope.Session.AppVolumes.ChangeDisplayName(id, "Updated Volume").EnsureSuccess();
            var moved = QarNativeAssert.WaitForAppVolumeUpdate(
                scope.Session,
                volume => volume.Id == id && System.Math.Abs(volume.Pose.Position.X - expectedPose.Position.X) < 0.001f,
                () => scope.Session.AppVolumes.ChangePose(id, expectedPose).EnsureSuccess());
            QarNativeAssert.PosePositionEqual(expectedPose, moved.Pose);
            var resized = QarNativeAssert.WaitForAppVolumeUpdate(
                scope.Session,
                volume => volume.Id == id && System.Math.Abs(volume.Size.WidthMeters - expectedSize.WidthMeters) < 0.001f,
                () => scope.Session.AppVolumes.ChangeSize(id, expectedSize).EnsureSuccess());
            QarNativeAssert.AppVolumeSizeEqual(expectedSize, resized.Size);
            var appMoved = QarNativeAssert.WaitForAppVolumeUpdate(
                scope.Session,
                volume => volume.Id == id && System.Math.Abs(volume.AppPose.Position.X - expectedAppPose.Position.X) < 0.001f,
                () => scope.Session.AppVolumes.ChangeAppPose(id, expectedAppPose).EnsureSuccess());
            QarNativeAssert.PosePositionEqual(expectedAppPose, appMoved.AppPose);
            var scaled = QarNativeAssert.WaitForAppVolumeUpdate(
                scope.Session,
                volume => volume.Id == id && System.Math.Abs(volume.AppScale - 2.5f) < 0.001f,
                () => scope.Session.AppVolumes.ChangeAppScale(id, 2.5f).EnsureSuccess());
            Assert.Equal(2.5f, scaled.AppScale, 5);
            var reconfigured = QarNativeAssert.WaitForAppVolumeUpdate(
                scope.Session,
                volume => volume.Id == id
                          && volume.GestureConfiguration.MappingRules.Length == 1
                          && System.Math.Abs(volume.GestureConfiguration.MappingRules[0].Precision - expectedGestures.MappingRules[0].Precision) < 0.001f,
                () => scope.Session.AppVolumes.ChangeGestureConfiguration(id, expectedGestures).EnsureSuccess());
            Assert.Single(reconfigured.GestureConfiguration.MappingRules);
            QarNativeAssert.GestureRuleEqual(expectedGestures.MappingRules[0], reconfigured.GestureConfiguration.MappingRules[0]);
            QarNativeAssert.PosePositionEqual(expectedPose, scope.Session.AppVolumes.GetLatestPose(id).ValueOrThrow());
            QarNativeAssert.AppVolumeSizeEqual(expectedSize, scope.Session.AppVolumes.GetLatestSize(id).ValueOrThrow());
            QarNativeAssert.PosePositionEqual(expectedAppPose, scope.Session.AppVolumes.GetLatestAppPose(id).ValueOrThrow());
            Assert.Equal(2.5f, scope.Session.AppVolumes.GetLatestAppScale(id).ValueOrThrow(), 5);
        }
    }
}
