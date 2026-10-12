using System;
using System.Runtime.InteropServices;
using System.Threading;
using System.Threading.Tasks;
using Qar;
using Xunit;

namespace QarStreaming.Tests
{
    [Collection("QAR Native")]
    public sealed class RenderSenderIntegrationTests : IClassFixture<QarNativeFixture>
    {
        private readonly QarNativeFixture _fixture;

        public RenderSenderIntegrationTests(QarNativeFixture fixture)
        {
            _fixture = fixture;
        }

        [Fact]
        public async Task RenderSenderCreateValidatesArgumentsSupportsCancellationAndThrowsAfterSessionDispose()
        {
            using var scope = _fixture.CreateSessionScope();
            var peerId = PeerId.Unique();
            Assert.Throws<ArgumentNullException>(() => scope.Session.RenderSenders.Create(null!));
            await Assert.ThrowsAsync<ArgumentNullException>(() => scope.Session.RenderSenders.CreateAsync(null!));
            Assert.Throws<ArgumentException>(() => scope.Session.RenderSenders.Create(new RenderSenderOptions(peerId, GraphicsAPI.D3D11)
            {
                TextureLayout = FrameLayout.SideBySide,
                FrameViews = CreateStereoFrameViews()
            }));
            Assert.Throws<ArgumentException>(() => scope.Session.RenderSenders.Create(new RenderSenderOptions(peerId, GraphicsAPI.Cpu)
            {
                StreamParamsD3D11 = new StreamParamsD3D11()
            }));
            using var cts = new CancellationTokenSource();
            cts.Cancel();
            await Assert.ThrowsAnyAsync<OperationCanceledException>(() => scope.Session.RenderSenders.CreateAsync(CreateCpuOptions(peerId), cts.Token));
            var renderSenders = scope.Session.RenderSenders;
            scope.Session.Dispose();
            Assert.Throws<ObjectDisposedException>(() => renderSenders.Create(CreateCpuOptions(peerId)));
            await Assert.ThrowsAsync<ObjectDisposedException>(() => renderSenders.CreateAsync(CreateCpuOptions(peerId)));
        }

        [Fact]
        public async Task CpuRenderSenderCompletesFullLoopAndExposesCpuFrame()
        {
            using var scope = _fixture.CreateSessionScope();
            var target = scope.StartRenderTarget();
            var visualizerPeerId = target.PeerId;
            using var sender = CreateSender(scope.Session, CreateCpuOptions(visualizerPeerId, scope.CreateAppVolume()));
            Assert.NotEqual(StreamId.Default, sender.GetStreamId().ValueOrThrow());
            var initialLayout = sender.GetLayout().ValueOrThrow();
            Assert.NotEmpty(initialLayout.Views);
            Assert.NotEmpty(initialLayout.Textures);
            var separatedLayout = CreateAlternateLayout(initialLayout);
            sender.ChangeLayout(separatedLayout).EnsureSuccess();
            var changedLayout = sender.GetLayout().ValueOrThrow();
            Assert.Equal(separatedLayout.Textures.Length, changedLayout.Textures.Length);
            var sideBySideLayout = CreateAlternateLayout(changedLayout);
            sender.ChangeLayout(sideBySideLayout).EnsureSuccess();
            var restoredLayout = sender.GetLayout().ValueOrThrow();
            Assert.Equal(sideBySideLayout.Textures.Length, restoredLayout.Textures.Length);
            using var frame = BeginFrameEventually(sender);
            Assert.True(frame.GetViewPose(0).IsSuccess);
            Assert.True(frame.GetViewFov(0).IsSuccess);
            Assert.False(frame.GetViewPose(999).IsSuccess);
            Assert.False(frame.GetViewFov(999).IsSuccess);
            var cpuFrame = sender.GetCpuFrame().ValueOrThrow();
            Assert.NotEmpty(cpuFrame.TextureViews);
            Assert.NotEmpty(cpuFrame.Textures);
            WriteCpuFrameMarker(cpuFrame);
            Assert.False(sender.GetD3D11Frame().IsSuccess);
            sender.ShowFrame().EnsureSuccess();
            Assert.True(target.WaitForFrames(1, TimeSpan.FromSeconds(20)), "the ext render target received no frame");
            var received = target.LastColor!;
            for (var i = 0; i < 16; ++i)
                Assert.Equal(MarkerByte(i), received[i]);
            frame.Dispose();
            Assert.Throws<ObjectDisposedException>(() => frame.GetViewPose(0));
            Assert.Throws<ObjectDisposedException>(() => frame.GetViewFov(0));
            AssertDisposedRenderSender(sender);
        }

        [Fact]
        public void PrimarylessCpuVideoSenderIsEagerAndReportsNoDestinations()
        {
            using var scope = _fixture.CreateSessionScope();
            var layout = new VideoFrameLayout
            {
                Views = new[]
                {
                    new VideoFrameView(0, 0, 64, 64, 0, 0, PixelFormat.B8G8R8A8, VideoFrameViewType.Color, VideoFrameViewEye.Left)
                },
                Textures = new[] { new TextureSize(PixelFormat.B8G8R8A8, 64, 64, 1) }
            };
            using var sender = scope.Session.PrimarylessVideoSenders.CreateCpu(
                new PrimarylessVideoSenderOptions(layout)
            ).ValueOrThrow();
            Assert.NotEqual(default(StreamId), sender.GetStreamId().ValueOrThrow());
            Assert.Single(sender.GetLayout().ValueOrThrow().Textures);
            Assert.Single(sender.GetCpuFrame().ValueOrThrow().Textures);
            QarNativeAssert.AssertFailure(sender.TryBeginFrame(), StatusCode.MediaStreamingNoActiveTransfers);
            sender.Dispose();
            Assert.Throws<ObjectDisposedException>(() => sender.GetStreamId());
        }

#if NET8_0_OR_GREATER
        [Fact]
        public async Task D3D11RenderSenderCompletesFullLoopAndExposesD3D11Frame()
        {
            Assert.True(OperatingSystem.IsWindows(), "D3D11 render sender tests require Windows.");
            using var scope = _fixture.CreateSessionScope();
            var target = scope.StartRenderTarget();
            var visualizerPeerId = target.PeerId;
            using var device = D3D11TestDevice.Create();
            using var sender = CreateSender(scope.Session, CreateD3D11Options(visualizerPeerId, device, scope.CreateAppVolume()));
            var initialLayout = sender.GetLayout().ValueOrThrow();
            Assert.NotEmpty(initialLayout.Views);
            Assert.NotEmpty(initialLayout.Textures);
            var changedLayout = CreateAlternateLayout(initialLayout);
            sender.ChangeLayout(changedLayout).EnsureSuccess();
            var updatedLayout = sender.GetLayout().ValueOrThrow();
            Assert.Equal(changedLayout.Textures.Length, updatedLayout.Textures.Length);
            using var frame = await BeginFrameEventuallyAsync(sender);
            Assert.True(frame.GetViewPose(0).IsSuccess);
            Assert.True(frame.GetViewFov(0).IsSuccess);
            Assert.False(frame.GetViewPose(999).IsSuccess);
            Assert.False(frame.GetViewFov(999).IsSuccess);
            var d3dFrame = sender.GetD3D11Frame().ValueOrThrow();
            Assert.NotEmpty(d3dFrame.TextureViews);
            Assert.NotEmpty(d3dFrame.Textures);
            Assert.NotEqual(IntPtr.Zero, d3dFrame.Textures[0].Texture);
            Assert.False(sender.GetCpuFrame().IsSuccess);
            sender.ShowFrame().EnsureSuccess();
            frame.Dispose();
            Assert.Throws<ObjectDisposedException>(() => frame.GetViewPose(0));
            Assert.Throws<ObjectDisposedException>(() => frame.GetViewFov(0));
        }
#endif

        private static RenderSender CreateSender(Session session, RenderSenderOptions options)
        {
            using var cts = new CancellationTokenSource(TimeSpan.FromSeconds(20));
            return session.RenderSenders.Create(options, cts.Token).ValueOrThrow();
        }

        private static RenderSenderOptions CreateCpuOptions(PeerId peerId, AppVolumeId? appVolume = null)
        {
            return new RenderSenderOptions(peerId, GraphicsAPI.Cpu)
            {
                AppVolumeId = appVolume,
                TextureLayout = FrameLayout.SideBySide,
                FrameViews = CreateStereoFrameViews()
            };
        }

#if NET8_0_OR_GREATER
        private static RenderSenderOptions CreateD3D11Options(PeerId peerId, D3D11TestDevice device, AppVolumeId appVolume)
        {
            return new RenderSenderOptions(peerId, GraphicsAPI.D3D11)
            {
                AppVolumeId = appVolume,
                TextureLayout = FrameLayout.SideBySide,
                FrameViews = CreateStereoFrameViews(),
                StreamParamsD3D11 = new StreamParamsD3D11
                {
                    D3D11Device = device.Device.NativePointer,
                    D3D11Context = device.Context.NativePointer
                }
            };
        }
#endif

        private static RenderFrameView[] CreateStereoFrameViews()
        {
            return new[]
            {
                new RenderFrameView(VideoFrameViewType.Color, VideoFrameViewEye.Left, 0),
                new RenderFrameView(VideoFrameViewType.Color, VideoFrameViewEye.Right, 0),
                new RenderFrameView(VideoFrameViewType.Depth, VideoFrameViewEye.Left, 0),
                new RenderFrameView(VideoFrameViewType.Depth, VideoFrameViewEye.Right, 0)
            };
        }

        private static RenderFrameInfo BeginFrameEventually(RenderSender sender)
        {
            return QarNativeAssert.WaitUntil(
                () =>
                {
                    var result = sender.BeginFrame();
                    return result.IsSuccess ? result.Value : null;
                },
                frame => frame != null,
                "render sender begin frame",
                timeoutMs: 15000,
                intervalMs: 100);
        }

        private static async Task<RenderFrameInfo> BeginFrameEventuallyAsync(RenderSender sender)
        {
            var deadline = DateTime.UtcNow.AddSeconds(15);
            while (DateTime.UtcNow < deadline)
            {
                var result = await sender.BeginFrameAsync();
                if (result.IsSuccess)
                    return result.Value;
                await Task.Delay(100);
            }
            throw new Xunit.Sdk.XunitException("Timed out waiting for render sender async begin frame.");
        }

        private static VideoFrameLayout CreateAlternateLayout(VideoFrameLayout currentLayout)
        {
            var leftColor = FindView(currentLayout, VideoFrameViewType.Color, VideoFrameViewEye.Left);
            var rightColor = FindView(currentLayout, VideoFrameViewType.Color, VideoFrameViewEye.Right);
            var leftDepth = FindView(currentLayout, VideoFrameViewType.Depth, VideoFrameViewEye.Left);
            var rightDepth = FindView(currentLayout, VideoFrameViewType.Depth, VideoFrameViewEye.Right);
            var width = leftColor.EndX - leftColor.StartX;
            var height = leftColor.EndY - leftColor.StartY;
            if (currentLayout.Textures.Length <= 2)
            {
                return new VideoFrameLayout
                {
                    Textures = new[]
                    {
                        new TextureSize(leftColor.TextureFormat, width, height, 1),
                        new TextureSize(rightColor.TextureFormat, width, height, 1),
                        new TextureSize(leftDepth.TextureFormat, width, height, 1),
                        new TextureSize(rightDepth.TextureFormat, width, height, 1)
                    },
                    Views = new[]
                    {
                        new VideoFrameView(0, 0, width, height, 0, 0, leftColor.TextureFormat, VideoFrameViewType.Color, VideoFrameViewEye.Left),
                        new VideoFrameView(0, 0, width, height, 1, 0, rightColor.TextureFormat, VideoFrameViewType.Color, VideoFrameViewEye.Right),
                        new VideoFrameView(0, 0, width, height, 2, 0, leftDepth.TextureFormat, VideoFrameViewType.Depth, VideoFrameViewEye.Left),
                        new VideoFrameView(0, 0, width, height, 3, 0, rightDepth.TextureFormat, VideoFrameViewType.Depth, VideoFrameViewEye.Right)
                    }
                };
            }
            return new VideoFrameLayout
            {
                Textures = new[]
                {
                    new TextureSize(leftColor.TextureFormat, width * 2, height, 1),
                    new TextureSize(leftDepth.TextureFormat, width * 2, height, 1)
                },
                Views = new[]
                {
                    new VideoFrameView(0, 0, width, height, 0, 0, leftColor.TextureFormat, VideoFrameViewType.Color, VideoFrameViewEye.Left),
                    new VideoFrameView(width, 0, width * 2, height, 0, 0, rightColor.TextureFormat, VideoFrameViewType.Color, VideoFrameViewEye.Right),
                    new VideoFrameView(0, 0, width, height, 1, 0, leftDepth.TextureFormat, VideoFrameViewType.Depth, VideoFrameViewEye.Left),
                    new VideoFrameView(width, 0, width * 2, height, 1, 0, rightDepth.TextureFormat, VideoFrameViewType.Depth, VideoFrameViewEye.Right)
                }
            };
        }

        private static VideoFrameView FindView(VideoFrameLayout layout, VideoFrameViewType dataType, VideoFrameViewEye eye)
        {
            return Assert.Single(layout.Views, view => view.DataType == dataType && view.Eye == eye);
        }

        /// <summary>The first 16 bytes the marker writes into colour texture 0.</summary>
        private static byte MarkerByte(int index)
        {
            return (byte)(0x40 + index);
        }

        private static void WriteCpuFrameMarker(VideoFrameCpu frame)
        {
            Assert.NotEmpty(frame.Textures);
            var texture = frame.Textures[0];
            var length = Math.Min((int)texture.TextureDataSize.ToUInt64(), 16);
            if (length <= 0 || texture.TextureData == IntPtr.Zero)
                return;
            var marker = new byte[length];
            for (var i = 0; i < marker.Length; ++i)
                marker[i] = MarkerByte(i);
            Marshal.Copy(marker, 0, texture.TextureData, marker.Length);
        }

        private static void AssertDisposedRenderSender(RenderSender sender)
        {
            sender.Dispose();
            Assert.Throws<ObjectDisposedException>(() => sender.GetLayout());
            Assert.Throws<ObjectDisposedException>(() => sender.ChangeLayout(new VideoFrameLayout()));
            Assert.Throws<ObjectDisposedException>(() => sender.GetCpuFrame());
            Assert.Throws<ObjectDisposedException>(() => sender.GetD3D11Frame());
            Assert.Throws<ObjectDisposedException>(() => sender.BeginFrame());
            Assert.Throws<ObjectDisposedException>(() => sender.BeginFrameAsync().GetAwaiter().GetResult());
            Assert.Throws<ObjectDisposedException>(() => sender.ShowFrame());
            Assert.Throws<ObjectDisposedException>(() => sender.GetLastHands());
        }
    }
}
