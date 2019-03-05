using Xunit;

namespace AdventOfCode2017.Test
{
    [Trait("Day", "24")]
    public class Day24Test : DayTest<Day24>
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
