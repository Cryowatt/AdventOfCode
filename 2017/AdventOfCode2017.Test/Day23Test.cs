using Xunit;

namespace AdventOfCode2017.Test
{
    [Trait("Day", "23")]
    public class Day23Test : DayTest<Day23>
    {
        [Theory]
        public void Part1(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart1(input));
        }

        [Theory]
        public void Part2(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart2(input));
        }
    }
}
