using Xunit;

namespace AdventOfCode2017.Test
{
    [Trait("Day", "16")]
    public class Day16Test : DayTest<Day16>
    {
        [Theory]
        [InlineData(@"s1", "eabcd")]
        [InlineData(@"s1,x3/4", "eabdc")]
        [InlineData(@"s1,x3/4,pe/b", "baedc")]
        public void Part1(string input, string expected)
        {
            this.Day.ChainLength = 5;
            Assert.Equal(expected, this.RunPart1(input));
        }

        [Theory]
        [InlineData(@"s1,x3/4,pe/b", "ceadb")]
        public void Part2(string input, string expected)
        {
            this.Day.ChainLength = 5;
            this.Day.Dances = 2;
            Assert.Equal(expected, this.RunPart2(input));
        }
    }
}
