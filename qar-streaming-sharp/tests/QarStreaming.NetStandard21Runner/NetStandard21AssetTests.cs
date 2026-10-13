using System;
using System.Runtime.InteropServices;
using Quaternar.Qaros;
using Xunit;

namespace Quaternar.Qaros.Tests.NetStandard21Runner
{
    public sealed class NetStandard21AssetTests
    {
        private const string CanonicalId = "00112233-4455-6677-8899-aabbccddeeff";

        [Fact]
        public void TargetSpecificSurfaceMatchesNetStandard21()
        {
            Assert.NotNull(typeof(PeerId).GetMethod("AsSpan"));
            Assert.NotNull(typeof(PeerId).GetMethod("TryCopyTo"));
            Assert.NotNull(typeof(VideoTextureCpu).GetProperty("TextureDataSpan"));
            Assert.Null(typeof(PeerControl).GetMethod("WatchUpdatesAsync"));
            Assert.Null(typeof(GuiPanelControl).GetMethod("WatchUpdatesAsync"));
            Assert.Null(typeof(AppVolumeControl).GetMethod("WatchUpdatesAsync"));
        }

        [Fact]
        public void SpanApisWorkThroughNetStandard21Asset()
        {
            var peerId = PeerId.Parse(CanonicalId);
            Span<byte> destination = stackalloc byte[16];
            Assert.True(peerId.TryCopyTo(destination));
            Assert.Equal(16, peerId.AsSpan().Length);
            Assert.Equal(16, destination.Length);
            var pointer = Marshal.AllocHGlobal(3);
            try
            {
                Marshal.WriteByte(pointer, 0, 1);
                Marshal.WriteByte(pointer, 1, 2);
                Marshal.WriteByte(pointer, 2, 3);
                var texture = new VideoTextureCpu(default(TextureSize), 0, pointer, (UIntPtr)3);
                var span = texture.TextureDataSpan;
                Assert.Equal(3, span.Length);
                Assert.Equal((byte)1, span[0]);
                Assert.Equal((byte)2, span[1]);
                Assert.Equal((byte)3, span[2]);
            }
            finally
            {
                Marshal.FreeHGlobal(pointer);
            }
        }

        [Fact]
        public void NativeSmokeWorksThroughNetStandard21Asset()
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
