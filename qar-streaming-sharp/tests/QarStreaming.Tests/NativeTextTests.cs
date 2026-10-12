using System;
using System.Runtime.InteropServices;
using System.Text;
using Qar;
using Qar.Native;
using Xunit;

namespace QarStreaming.Tests
{
    // Every char* of the C API is UTF-8: the binding converts its UTF-16 strings itself, never
    // through the ANSI code page. Only a common name must be ASCII, and the C API checks that.
    public sealed class NativeTextTests
    {
        private const string Text = "Room \U0001F680 Žluťoučký";

        private static string ReadUtf8(IntPtr pointer)
        {
            var length = 0;
            while (Marshal.ReadByte(pointer, length) != 0)
                length++;
            var bytes = new byte[length];
            Marshal.Copy(pointer, bytes, 0, length);
            return Encoding.UTF8.GetString(bytes);
        }

        [Fact]
        public void NonAsciiTextReachesNativeAsUtf8()
        {
            using (var scope = new NativeMemoryScope())
            {
                var volume = NativeInit.AppVolume(new AppVolumeOptions("volume") { DisplayName = Text }, scope, IntPtr.Zero);
                Assert.Equal(Text, ReadUtf8(volume.DisplayName));
                var peer = NativeInit.PeerPresentation(
                    new PeerPresentationOptions { DisplayName = Text, AppVersion = "1.0-β", AppCustomPeerInfo = Text }, scope, IntPtr.Zero);
                Assert.Equal("1.0-β", ReadUtf8(peer.AppVersion));
                Assert.Equal(Text, ReadUtf8(peer.AppCustomPeerInfo));
                Assert.Equal(@"C:\Users\Lukáš\QAROS", ReadUtf8(scope.AllocUtf8(@"C:\Users\Lukáš\QAROS")));
                Assert.Equal("https://příklad.cz/stránka", ReadUtf8(scope.AllocUtf8("https://příklad.cz/stránka")));
            }
        }

        [Fact]
        public void NonAsciiCommonNameIsPassedOnForTheNativeCheck()
        {
            using (var scope = new NativeMemoryScope())
            {
                var volume = NativeInit.AppVolume(new AppVolumeOptions("Žluťoučký"), scope, IntPtr.Zero);
                Assert.Equal("Žluťoučký", ReadUtf8(volume.CommonName));
            }
        }

        [Fact]
        public void NullStaysNull()
        {
            using (var scope = new NativeMemoryScope())
            using (var text = NativeUtf8String.FromUtf8(null))
            {
                Assert.Equal(IntPtr.Zero, scope.AllocUtf8(null));
                Assert.Equal(IntPtr.Zero, text.Pointer);
            }
        }
    }
}
