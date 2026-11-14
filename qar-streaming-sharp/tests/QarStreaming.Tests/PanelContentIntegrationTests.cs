using System;
using System.Text;
using Quaternar.Qaros;
using Xunit;

namespace Quaternar.Qaros.Tests
{
    [Collection("QAR Native")]
    public sealed class PanelContentIntegrationTests : IClassFixture<QarNativeFixture>
    {
        private readonly QarNativeFixture _fixture;

        public PanelContentIntegrationTests(QarNativeFixture fixture)
        {
            _fixture = fixture;
        }

        private static PanelContentFile[] SampleFiles()
        {
            return new[]
            {
                new PanelContentFile("index.html", Encoding.UTF8.GetBytes("<p>served</p>")),
                new PanelContentFile("settings.html", Encoding.UTF8.GetBytes("<p>settings</p>"), "text/html"),
            };
        }

        [Fact]
        public void PublishRefusesBadNamesPathsAndTypes()
        {
            using var scope = _fixture.CreateSessionScope();
            var content = scope.Session.PanelContent;
            Assert.Equal(StatusCode.PanelContentBundleRefused, content.PublishFiles("bad/name", SampleFiles()).Info.StatusCode);
            Assert.Equal(StatusCode.PanelContentPathRefused, content.PublishFiles("ui", new[] { new PanelContentFile("../x.html", new byte[] { 1 }) }).Info.StatusCode);
            Assert.Equal(StatusCode.PanelContentMimeRefused, content.PublishFiles("ui", new[] { new PanelContentFile("run.exe", new byte[] { 1 }) }).Info.StatusCode);
            Assert.Equal(StatusCode.PanelContentSizeRefused, content.PublishFiles("ui", Array.Empty<PanelContentFile>()).Info.StatusCode);
        }

        [Fact]
        public void ServedPanelShowsThePublishersBundleAndChangesEntryAndName()
        {
            using var scope = _fixture.CreateSessionScope();
            var bundle = scope.Session.PanelContent.PublishFiles("ui", SampleFiles()).ValueOrThrow();
            Assert.Equal("ui", bundle.Name);
            var id = GuiPanelId.Default;
            var created = QarNativeAssert.WaitForGuiPanelUpdate(
                scope.Session,
                panel => id != GuiPanelId.Default && panel.Id == id && panel.ContentType == GuiPanelContentType.Served,
                () => id = scope.Session.GuiPanels.GetOrCreate(new GuiPanelOptions(Guid.NewGuid().ToString())
                {
                    DisplayName = "Inspector",
                    ServedContent = new GuiPanelServedContent(bundle, "index.html")
                }).ValueOrThrow());
            Assert.NotNull(created.ServedContent);
            Assert.Equal(bundle, created.ServedContent!.Bundle);
            Assert.Equal("index.html", created.ServedContent.Entry);
            Assert.Null(created.ContentUri);
            Assert.Equal(StatusCode.GuiPanelChangeNotPermitted, scope.Session.GuiPanels.NavigateToUri(id, "https://example.com").Info.StatusCode);
            var foreign = new PanelContentBundleId(PeerId.Parse("00112233-4455-6677-8899-aabbccddeeff"), "ui");
            Assert.Equal(StatusCode.PanelContentNotPublisher, scope.Session.GuiPanels.SetServedContent(id, new GuiPanelServedContent(foreign, "index.html")).Info.StatusCode);
            var changed = QarNativeAssert.WaitForGuiPanelUpdate(
                scope.Session,
                panel => panel.Id == id && panel.ServedContent != null && panel.ServedContent.Entry == "settings.html",
                () => scope.Session.GuiPanels.SetServedContent(id, new GuiPanelServedContent(bundle, "settings.html")).EnsureSuccess());
            Assert.Equal(GuiPanelContentType.Served, changed.ContentType);
            Assert.Equal(StatusCode.GuiPanelDisplayNameInvalid, scope.Session.GuiPanels.ChangeDisplayName(id, new string('a', 65)).Info.StatusCode);
            var renamed = QarNativeAssert.WaitForGuiPanelUpdate(
                scope.Session,
                panel => panel.Id == id && panel.DisplayName == "Settings \U0001F3AC",
                () => scope.Session.GuiPanels.ChangeDisplayName(id, "Settings \U0001F3AC").EnsureSuccess());
            Assert.Equal("settings.html", renamed.ServedContent!.Entry);
        }

        [Fact]
        public void MessagesAndRemovalBelongToThePublisher()
        {
            using var scope = _fixture.CreateSessionScope();
            var content = scope.Session.PanelContent;
            var bundle = content.PublishFiles("chat", SampleFiles()).ValueOrThrow();
            using (content.SubscribeMessages(bundle, (_, _) => { }).ValueOrThrow())
            {
                Assert.Equal(StatusCode.PanelContentMessageRefused, content.PostMessage(bundle, "{oops").Info.StatusCode);
                content.PostMessage(bundle, "{\"t\":\"state\",\"key\":\"count\",\"value\":1}").EnsureSuccess();
                content.PostMessage(bundle, "2", GuiPanelId.Default, PeerId.Parse("00112233-4455-6677-8899-aabbccddeeff")).EnsureSuccess();
            }
            var foreign = new PanelContentBundleId(PeerId.Parse("00112233-4455-6677-8899-aabbccddeeff"), "chat");
            Assert.Equal(StatusCode.PanelContentNotPublisher, content.PostMessage(foreign, "1").Info.StatusCode);
            Assert.Equal(StatusCode.PanelContentNotPublisher, content.Remove(foreign).Info.StatusCode);
            content.Remove(bundle).EnsureSuccess();
            Assert.Equal(StatusCode.PanelContentNotFound, content.Remove(bundle).Info.StatusCode);
        }
    }
}
