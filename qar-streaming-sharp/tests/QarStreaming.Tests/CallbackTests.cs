using Qar;
using Xunit;

namespace QarStreaming.Tests
{
    public sealed class CallbackTests
    {
        [Fact]
        public void PeerUpdateEventArgsCarrySnapshot()
        {
            var peer = new PeerSpec(PeerId.Default, "display", "1.0", "custom", AppState.Running, "v", "room", true);
            var args = new PeerSpecUpdatedEventArgs(peer);
            Assert.Same(peer, args.Peer);
        }
    }
}
