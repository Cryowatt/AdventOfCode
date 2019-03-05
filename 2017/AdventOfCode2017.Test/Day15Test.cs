using Xunit;

namespace AdventOfCode2017.Test
{
    [Trait("Day", "15")]
    public class Day15Test : DayTest<Day15>
    {
        [Theory]
        [InlineData(@"Generator A starts with 65
Generator B starts with 8921", "588")]
        public void Part1(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart1(input));
        }

        [Theory]
        [InlineData(@"Generator A starts with 65
Generator B starts with 8921", "309")]
        public void Part2(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart2(input));
        }
    }
}
