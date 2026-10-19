using Quaternar.Qaros;
using Xunit;

namespace Quaternar.Qaros.Tests
{
    public sealed class PeerIdentityTests
    {
        // PeerRole and DeviceCategory cross the C ABI as QarPeerRole / QarDeviceCategory: the
        // values are the contract.
        [Theory]
        [InlineData(PeerRole.Internal, 0)]
        [InlineData(PeerRole.Source, 1)]
        [InlineData(PeerRole.Target, 2)]
        [InlineData(PeerRole.Runtime, 3)]
        [InlineData(PeerRole.Control, 4)]
        public void RoleValuesMatchTheCApi(PeerRole role, int value)
        {
            Assert.Equal(value, (int)role);
        }

        [Theory]
        [InlineData(DeviceCategory.Unknown, 0)]
        [InlineData(DeviceCategory.DesktopPC, 1)]
        [InlineData(DeviceCategory.Server, 2)]
        [InlineData(DeviceCategory.Headset, 3)]
        [InlineData(DeviceCategory.Visualizer, 4)]
        [InlineData(DeviceCategory.Camera, 5)]
        public void CategoryValuesMatchTheCApi(DeviceCategory category, int value)
        {
            Assert.Equal(value, (int)category);
        }

        [Fact]
        public void IsTargetAppFollowsTheRole()
        {
            var source = new PeerSpec(PeerId.Default, "App", "1.0", "", AppState.Running, "v", "", PeerRole.Source);
            Assert.False(source.IsTargetApp);
            Assert.Equal(DeviceCategory.Unknown, source.Category);
            Assert.Equal(string.Empty, source.DeviceId);

            var headset = new PeerSpec(
                PeerId.Default, "Quest", "1.0", "", AppState.Running, "v", "",
                PeerRole.Target, DeviceCategory.Headset, "MetaQuest3", "MetaQuest", "Meta Quest 3");
            Assert.True(headset.IsTargetApp);
            Assert.Equal(DeviceCategory.Headset, headset.Category);
            Assert.Equal("MetaQuest3", headset.DeviceId);
            Assert.Equal("MetaQuest", headset.DeviceFamily);
            Assert.Equal("Meta Quest 3", headset.DeviceName);
        }

        [Fact]
        public void ADeviceIdentityChainsWithItsOwnStructureType()
        {
            var identity = new PeerDeviceIdentity(DeviceCategory.Camera, "LuxonisOakD") { DeviceFamily = "Luxonis" };
            Assert.Equal(StructureType.PeerDeviceIdentity, identity.Type);
            var presentation = new PeerPresentationOptions().WithDeviceIdentity(identity);
            Assert.NotNull(presentation);
            Assert.Throws<System.ArgumentNullException>(() => new PeerDeviceIdentity(DeviceCategory.Camera, null!));
        }
    }
}
