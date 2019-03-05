using Xunit;

namespace AdventOfCode2017.Test
{
    [Trait("Day", "11")]
    public class Day11Test : DayTest<Day11>
    {
        [Theory]
        [InlineData(@"ne,ne,ne", "3")]
        [InlineData(@"ne,ne,sw,sw", "0")]
        [InlineData(@"ne,ne,s,s", "2")]
        [InlineData(@"se,sw,se,sw,sw", "3")]
        public void Part1(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart1(input));
        }

        [Theory]
        [InlineData(@"ne,ne,ne", "3")]
        [InlineData(@"ne,ne,sw,sw", "2")]
        [InlineData(@"ne,ne,s,s", "2")]
        [InlineData(@"se,sw,se,sw,sw", "3")]
        public void Part2(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart2(input));
        }
    }
}
