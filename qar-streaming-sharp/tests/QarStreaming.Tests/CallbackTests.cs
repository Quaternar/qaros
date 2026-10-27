using Quaternar.Qaros;
using Xunit;

#pragma warning disable CS0618 // PeerSpec is still constructed with its deprecated AppState

namespace Quaternar.Qaros.Tests
{
    public sealed class CallbackTests
    {
        [Fact]
        public void PeerUpdateEventArgsCarrySnapshot()
        {
            var peer = new PeerSpec(PeerId.Default, "display", "1.0", "custom", AppState.Running, "v", "room", PeerRole.Target);
            var args = new PeerSpecUpdatedEventArgs(peer);
            Assert.Same(peer, args.Peer);
        }
    }
}
