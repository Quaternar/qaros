using Quaternar.Qaros;
using Xunit;

namespace Quaternar.Qaros.Tests
{
    public sealed class DeviceTypeTests
    {
        // DeviceType crosses the C ABI as QarDeviceType: the values are the contract.
        [Theory]
        [InlineData(DeviceType.Internal, 0)]
        [InlineData(DeviceType.DesktopWindows, 1)]
        [InlineData(DeviceType.VisualizerControl, 5)]
        [InlineData(DeviceType.AndroidArHeadset, 150)]
        [InlineData(DeviceType.HoloLensArHeadset, 151)]
        [InlineData(DeviceType.MetaQuestHeadset, 152)]
        [InlineData(DeviceType.Visualizer, 153)]
        [InlineData(DeviceType.AndroidStreamer, 155)]
        [InlineData(DeviceType.TestTarget, 157)]
        public void ValuesMatchTheCApi(DeviceType type, int value)
        {
            Assert.Equal(value, (int)type);
        }

        [Fact]
        public void APeerSpecWithoutADeviceTypeReadsInternal()
        {
            var peer = new PeerSpec(PeerId.Default, "display", "1.0", "custom", AppState.Running, "v", "room", true);
            Assert.Equal(DeviceType.Internal, peer.DeviceType);
            var headset = new PeerSpec(PeerId.Default, "HL", "1.0", "", AppState.Running, "v", "", true, DeviceType.HoloLensArHeadset);
            Assert.Equal(DeviceType.HoloLensArHeadset, headset.DeviceType);
        }
    }
}
