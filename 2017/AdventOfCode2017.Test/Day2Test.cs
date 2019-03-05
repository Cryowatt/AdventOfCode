using Xunit;

namespace AdventOfCode2017.Test
{
    [Trait("Day", "02")]
    public class Day2Test : DayTest<Day2>
    {
        [Theory]
        [InlineData(@"5 1 9 5", "8")]
        [InlineData(@"7 5 3", "4")]
        [InlineData(@"2 4 6 8", "6")]
        [InlineData(@"5 1 9 5
7 5 3
2 4 6 8", "18")]
        public void Part1(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart1(input));
        }

        [Theory]
        [InlineData(@"5 9 2 8", "4")]
        [InlineData(@"9 4 7 3", "3")]
        [InlineData(@"3 8 6 5", "2")]
        [InlineData(@"5 9 2 8
9 4 7 3
3 8 6 5", "9")]
        public void Part2(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart2(input));
        }
    }
}
