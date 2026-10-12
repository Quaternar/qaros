using Quaternar.Qaros;
using Xunit;
using Quaternar.Qaros.Mathematics;

namespace Quaternar.Qaros.Tests
{
    public sealed class SubscriptionTests
    {
        [Fact]
        public void GuiPanelEventArgsCarrySnapshot()
        {
            var panel = GuiPanel.CreateWithUri(
                GuiPanelId.Default,
                "panel",
                Pose.Identity,
                GuiPanelSize.Default,
                "about:blank",
                GuiPanelState.Visible,
                new[] { PeerId.Default });
            var args = new GuiPanelUpdatedEventArgs(panel);
            Assert.Same(panel, args.Panel);
        }
    }
}
