using System;
using System.IO;
using Quaternar.Qaros;
using Quaternar.Qaros.Tests.Ext;
using Xunit;

namespace Quaternar.Qaros.Tests
{
    /// <summary>
    /// <see cref="Session.OpenQarosView"/>: the arguments are checked first, and only a local-app
    /// session has a QAROS of this PC to open a view in. A session onboarded by invite to the ext
    /// hub is refused before any qaros.exe is looked for, so no test starts a real QAROS.
    /// </summary>
    [Collection("QAR Native")]
    public sealed class QarosViewTests : IClassFixture<QarNativeFixture>
    {
        public QarosViewTests(QarNativeFixture fixture)
        {
            _ = fixture;
        }

        [Fact]
        public void EnumValuesMirrorC()
        {
            Assert.Equal(1, (int)QarosView.PairHeadsetQr);
            Assert.Equal(2, (int)QarosView.RoomOriginQr);
            Assert.Equal(3, (int)QarosView.DeviceSetup);
            Assert.Equal(0, (int)QarosDeviceKind.None);
            Assert.Equal(1, (int)QarosDeviceKind.MetaQuest);
            Assert.Equal(2, (int)QarosDeviceKind.Metalens);
            Assert.Equal(3, (int)QarosDeviceKind.AndroidXr);
            Assert.Equal(4, (int)QarosDeviceKind.HoloLens);
        }

        [Fact]
        public void BadArgumentsAndAnInviteSessionAreRefused()
        {
            var storageRoot = Path.Combine(Path.GetTempPath(), "qar-sharp-tests", Guid.NewGuid().ToString("N"));
            using var runtime = Runtime.Create(new RuntimeOptions { StorageFolderPath = storageRoot }).ValueOrThrow();
            using var hub = ExtHub.Create();
            using var invite = OnboardingInvite.Deserialize(hub.MintInvite()).ValueOrThrow();
            var options = new OnboardOptions()
                .UseInvite(invite)
                .WithPresentation(new PeerPresentationOptions { DisplayName = "QAROS views" })
                .WithoutGraphicsDevice();
            var session = runtime.Onboard(options).ValueOrThrow().Session;
            try
            {
                QarNativeAssert.AssertFailure(session.OpenQarosView(QarosView.DeviceSetup), StatusCode.ArgumentNotSupported);
                QarNativeAssert.AssertFailure(session.OpenQarosView(QarosView.PairHeadsetQr, QarosDeviceKind.HoloLens), StatusCode.ArgumentNotSupported);
                QarNativeAssert.AssertFailure(session.OpenQarosView((QarosView)42), StatusCode.ArgumentNotSupported);
                QarNativeAssert.AssertFailure(session.OpenQarosView(QarosView.DeviceSetup, (QarosDeviceKind)42), StatusCode.ArgumentNotSupported);
                // Valid views: refused because the session was not onboarded as a local app.
                QarNativeAssert.AssertFailure(session.OpenQarosView(QarosView.PairHeadsetQr), StatusCode.ArgumentNotSupported);
                QarNativeAssert.AssertFailure(session.OpenQarosView(QarosView.RoomOriginQr), StatusCode.ArgumentNotSupported);
                QarNativeAssert.AssertFailure(session.OpenQarosView(QarosView.DeviceSetup, QarosDeviceKind.MetaQuest), StatusCode.ArgumentNotSupported);
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
