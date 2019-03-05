using Xunit;

namespace AdventOfCode2017.Test
{
    [Trait("Day", "17")]
    public class Day17Test : DayTest<Day17>
    {
        [Theory]
        [InlineData(@"3", "638")]
        public void Part1(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart1(input));
        }

        [Theory]
        [InlineData(@"3", 1, "1")]
        [InlineData(@"3", 2, "2")]
        [InlineData(@"3", 3, "2")]
        [InlineData(@"3", 4, "2")]
        [InlineData(@"3", 5, "5")]
        [InlineData(@"3", 6, "5")]
        [InlineData(@"3", 7, "5")]
        [InlineData(@"3", 8, "5")]
        [InlineData(@"3", 9, "9")]
        public void Part2(string input, int iterations, string expected)
        {
            this.Day.Part2Iterations = iterations;
            Assert.Equal(expected, this.RunPart2(input));
        }
    }
}
