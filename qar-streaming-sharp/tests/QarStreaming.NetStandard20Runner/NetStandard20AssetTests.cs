using Quaternar.Qaros;
using Xunit;

namespace Quaternar.Qaros.Tests.NetStandard20Runner
{
    public sealed class NetStandard20AssetTests
    {
        private const string CanonicalId = "00112233-4455-6677-8899-aabbccddeeff";

        [Fact]
        public void TargetSpecificSurfaceMatchesNetStandard20()
        {
            Assert.Null(typeof(PeerId).GetMethod("AsSpan"));
            Assert.Null(typeof(PeerId).GetMethod("TryCopyTo"));
            Assert.Null(typeof(VideoTextureCpu).GetProperty("TextureDataSpan"));
            Assert.Null(typeof(PeerControl).GetMethod("WatchUpdatesAsync"));
            Assert.Null(typeof(GuiPanelControl).GetMethod("WatchUpdatesAsync"));
            Assert.Null(typeof(AppVolumeControl).GetMethod("WatchUpdatesAsync"));
        }

        [Fact]
        public void ManagedApisWorkThroughNetStandard20Asset()
        {
            var peerId = PeerId.Parse(CanonicalId);
            var sessionId = SessionId.Parse(CanonicalId);
            var panelId = GuiPanelId.Parse(CanonicalId);
            var volumeId = AppVolumeId.Parse(CanonicalId);
            var streamId = StreamId.Parse(CanonicalId);
            Assert.Equal(CanonicalId, peerId.ToString());
            Assert.Equal(16, sessionId.ToByteArray().Length);
            Assert.Equal(CanonicalId, panelId.ToString());
            Assert.Equal(CanonicalId, volumeId.ToString());
            Assert.Equal(CanonicalId, streamId.ToString());
            var options = new RenderSenderOptions(peerId, GraphicsAPI.Cpu)
            {
                TextureLayout = FrameLayout.SideBySide,
                FrameViews = new[]
                {
                    new RenderFrameView(VideoFrameViewType.Color, VideoFrameViewEye.Left, 0),
                    new RenderFrameView(VideoFrameViewType.Color, VideoFrameViewEye.Right, 0)
                }
            };
            Assert.Equal(GraphicsAPI.Cpu, options.GraphicsAPI);
            var info = new ResultInfo(StatusCode.Success, 0, string.Empty);
            var result = Result<int>.Success(info, 7);
            Assert.Equal(7, result.Value);
        }

        [Fact]
        public void NativeSmokeWorksThroughNetStandard20Asset()
        {
            Library.Init().EnsureSuccess();
            try
            {
                using (var runtime = Runtime.Create().ValueOrThrow()) { }
            }
            finally
            {
                Library.Destroy().EnsureSuccess();
            }
        }
    }
}
