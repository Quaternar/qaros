using System;
using System.IO;
using System.Threading;
using Quaternar.Qaros;
using Quaternar.Qaros.Tests.Ext;
using Xunit;

namespace Quaternar.Qaros.Tests
{
    /// <summary>
    /// A session onboarded with <see cref="OnboardOptions.WithoutGraphicsDevice"/> joins and
    /// drives app volumes, and refuses media streaming with
    /// <see cref="StatusCode.GraphicsDeviceNotInitialized"/>.
    /// </summary>
    [Collection("QAR Native")]
    public sealed class GraphicsDeviceDisabledTests : IClassFixture<QarNativeFixture>
    {
        public GraphicsDeviceDisabledTests(QarNativeFixture fixture)
        {
            _ = fixture;
        }

        [Fact]
        public void ASessionWithoutAGraphicsDeviceConnectsAndRefusesRenderSenders()
        {
            var storageRoot = Path.Combine(Path.GetTempPath(), "qar-sharp-tests", Guid.NewGuid().ToString("N"));
            using var runtime = Runtime.Create(new RuntimeOptions { StorageFolderPath = storageRoot }).ValueOrThrow();
            using var hub = ExtHub.Create();
            using var invite = OnboardingInvite.Deserialize(hub.MintInvite()).ValueOrThrow();
            var options = new OnboardOptions()
                .UseInvite(invite)
                .WithPresentation(new PeerPresentationOptions { DisplayName = "No graphics device" })
                .WithoutGraphicsDevice();
            var onboarded = runtime.Onboard(options).ValueOrThrow();
            var session = onboarded.Session;
            try
            {
                var volume = session.AppVolumes.GetOrCreate(new AppVolumeOptions("qar-sharp-no-gpu-volume") { DisplayName = "No GPU" }).ValueOrThrow();
                QarNativeAssert.WaitForVolumeInQuery(session, volume);

                var sender = new RenderSenderOptions(PeerId.Unique(), GraphicsAPI.Cpu)
                {
                    AppVolumeId = volume,
                    TextureLayout = FrameLayout.SideBySide
                };
                using var cts = new CancellationTokenSource(TimeSpan.FromSeconds(5));
                QarNativeAssert.AssertFailure(session.RenderSenders.Create(sender, cts.Token), StatusCode.GraphicsDeviceNotInitialized);
                QarNativeAssert.AssertFailure(session.GetGraphicsDeviceId(), StatusCode.GraphicsDeviceNotInitialized);
            }
            finally
            {
                session.Dispose();
                try
                {
                    Directory.Delete(storageRoot, true);
                }
                catch (IOException)
                {
                }
                catch (UnauthorizedAccessException)
                {
                }
            }
        }
    }
}
