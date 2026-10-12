using System;
using Quaternar.Qaros;
using Xunit;

namespace Quaternar.Qaros.Tests
{
    public sealed class TimePointTests
    {
        [Fact]
        public void ConvertsToUtcDateTimeTruncatedToTheTick()
        {
            var time = new TimePoint(1_700_000_000_123_456_789L);

            DateTime converted = time;

            Assert.Equal(DateTimeKind.Utc, converted.Kind);
            Assert.Equal(new DateTime(2023, 11, 14, 22, 13, 20, DateTimeKind.Utc).AddTicks(1_234_567), converted);
            Assert.Equal(converted, time.ToDateTime());
            Assert.Equal(new DateTimeOffset(converted), time.ToDateTimeOffset());
        }

        [Fact]
        public void RoundTripsThroughDateTimeAtTickResolution()
        {
            var time = new TimePoint(1_700_000_000_123_456_700L);

            Assert.Equal(time, TimePoint.FromDateTime(time.ToDateTime()));
            Assert.Equal(time, TimePoint.FromDateTimeOffset(time.ToDateTimeOffset()));
        }

        [Fact]
        public void ComparesAndSubtracts()
        {
            var earlier = new TimePoint(1_000_000_000L);
            var later = earlier + TimeSpan.FromMilliseconds(16);

            Assert.True(earlier < later);
            Assert.Equal(TimeSpan.FromMilliseconds(16), later - earlier);
            Assert.Equal(1_016_000_000L, later.UnixNanoseconds);
        }
    }
}
