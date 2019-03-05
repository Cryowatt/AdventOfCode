using Xunit;

namespace AdventOfCode2017.Test
{
    [Trait("Day", "06")]
    public class Day6Test :DayTest<Day6>
    {
        [Theory]
        [InlineData(@"0 2 7 0", "5")]
        public void Part1(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart1(input));
        }

        [Theory]
        [InlineData(@"0 2 7 0", "4")]
        public void Part2(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart2(input));
        }
    }
}
