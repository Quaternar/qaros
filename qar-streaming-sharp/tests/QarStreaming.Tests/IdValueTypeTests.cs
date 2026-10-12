using System;
using Qar;
using Xunit;

namespace QarStreaming.Tests
{
    public sealed class IdValueTypeTests
    {
        private const string CanonicalId = "00112233-4455-6677-8899-aabbccddeeff";
        private const string DifferentCanonicalId = "00112233-4455-6677-8899-aabbccddeefe";
        private const string ZeroId = "00000000-0000-0000-0000-000000000000";

        [Fact]
        public void SessionIdRoundTripsCanonicalUuidAndEquality()
        {
            var id = SessionId.Parse(CanonicalId);
            var same = SessionId.Parse(CanonicalId);
            Assert.Equal(CanonicalId, id.ToString());
            Assert.Equal(16, id.ToByteArray().Length);
            Assert.Equal(id, same);
            Assert.Equal(id.GetHashCode(), same.GetHashCode());
        }

#if NETSTANDARD2_1_OR_GREATER || NET8_0_OR_GREATER
        [Fact]
        public void GuiPanelIdRoundTripsCanonicalUuidAndSpanCopy()
        {
            var id = GuiPanelId.Parse(CanonicalId);
            Span<byte> buffer = stackalloc byte[16];
            Assert.True(id.TryCopyTo(buffer));
            Assert.Equal(CanonicalId, id.ToString());
            Assert.Equal(16, buffer.Length);
        }
#endif

        [Fact]
        public void AppVolumeIdRoundTripsCanonicalUuidAndEquality()
        {
            var id = AppVolumeId.Parse(CanonicalId);
            var different = AppVolumeId.Parse(DifferentCanonicalId);
            Assert.Equal(CanonicalId, id.ToString());
            Assert.NotEqual(id, different);
            Assert.True(id != different);
        }

        [Fact]
        public void StreamIdRoundTripsCanonicalUuidAndDefaultStringUsesNativeFormatting()
        {
            var id = StreamId.Parse(CanonicalId);
            Assert.Equal(CanonicalId, id.ToString());
            Assert.Equal(ZeroId, StreamId.Default.ToString());
        }

        [Fact]
        public void PeerIdSessionIdGuiPanelIdAndStreamIdTryParseRejectInvalidInput()
        {
            Assert.False(PeerId.TryParse("zzzzzzzz-zzzz-zzzz-zzzz-zzzzzzzzzzzz", out _));
            Assert.False(SessionId.TryParse("gggggggg-gggg-gggg-gggg-gggggggggggg", out _));
            Assert.False(GuiPanelId.TryParse("this-is-not-a-uuid", out _));
            Assert.False(AppVolumeId.TryParse("bad_uuid_value", out _));
            Assert.False(StreamId.TryParse("!invalid-uuid-value!", out _));
        }

        [Fact]
        public void ParseThrowsForInvalidIds()
        {
            Assert.Throws<FormatException>(() => PeerId.Parse("zzzzzzzz-zzzz-zzzz-zzzz-zzzzzzzzzzzz"));
            Assert.Throws<FormatException>(() => SessionId.Parse("gggggggg-gggg-gggg-gggg-gggggggggggg"));
            Assert.Throws<FormatException>(() => GuiPanelId.Parse("this-is-not-a-uuid"));
            Assert.Throws<FormatException>(() => AppVolumeId.Parse("bad_uuid_value"));
            Assert.Throws<FormatException>(() => StreamId.Parse("!invalid-uuid-value!"));
        }

#if NETSTANDARD2_1_OR_GREATER || NET8_0_OR_GREATER
        [Fact]
        public void TryCopyToRejectsTooSmallDestination()
        {
            var peer = PeerId.Parse(CanonicalId);
            var session = SessionId.Parse(CanonicalId);
            var panel = GuiPanelId.Parse(CanonicalId);
            var volume = AppVolumeId.Parse(CanonicalId);
            var stream = StreamId.Parse(CanonicalId);
            Span<byte> small = stackalloc byte[15];
            Assert.False(peer.TryCopyTo(small));
            Assert.False(session.TryCopyTo(small));
            Assert.False(panel.TryCopyTo(small));
            Assert.False(volume.TryCopyTo(small));
            Assert.False(stream.TryCopyTo(small));
        }
#endif

        [Fact]
        public void UniqueFactoriesProduceNonDefaultIds()
        {
            var peer1 = PeerId.Unique();
            var peer2 = PeerId.Unique();
            var session1 = SessionId.Unique();
            var session2 = SessionId.Unique();
            var panel1 = GuiPanelId.Unique();
            var panel2 = GuiPanelId.Unique();
            Assert.NotEqual(PeerId.Default, peer1);
            Assert.NotEqual(peer1, peer2);
            Assert.NotEqual(SessionId.Default, session1);
            Assert.NotEqual(session1, session2);
            Assert.NotEqual(GuiPanelId.Default, panel1);
            Assert.NotEqual(panel1, panel2);
        }

        [Fact]
        public void CanonicalUuidIdsRoundTripAcrossAllIdTypes()
        {
            Assert.Equal(CanonicalId, PeerId.Parse(CanonicalId).ToString());
            Assert.Equal(CanonicalId, SessionId.Parse(CanonicalId).ToString());
            Assert.Equal(CanonicalId, GuiPanelId.Parse(CanonicalId).ToString());
            Assert.Equal(CanonicalId, AppVolumeId.Parse(CanonicalId).ToString());
            Assert.Equal(CanonicalId, StreamId.Parse(CanonicalId).ToString());
        }
    }
}
