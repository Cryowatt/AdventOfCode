using Xunit;

namespace AdventOfCode2017.Test
{
    [Trait("Day", "13")]
    public class Day13Test : DayTest<Day13>
    {
        [Theory]
        [InlineData(@"0: 3
1: 2
4: 4
6: 4", "24")]
        public void Part1(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart1(input));
        }

        [Theory]
        [InlineData(@"0: 3
1: 2
4: 4
6: 4", "10")]
        public void Part2(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart2(input));
        }
    }
}
