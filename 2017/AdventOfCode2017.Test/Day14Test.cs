using Xunit;

namespace AdventOfCode2017.Test
{
    [Trait("Day", "14")]
    public class Day14Test : DayTest<Day14>
    {
        [Theory]
        [InlineData(@"flqrgnkx", "8108")]
        public void Part1(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart1(input));
        }

        [Theory]
        [InlineData(@"flqrgnkx", "1242")]
        public void Part2(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart2(input));
        }
    }
}
