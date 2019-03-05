using Xunit;

namespace AdventOfCode2017.Test
{
    [Trait("Day", "10")]
    public class Day10Test : DayTest<Day10>
    {
        [Theory]
        [InlineData(@"3", "2")]
        [InlineData(@"3,4", "12")]
        [InlineData(@"3,4,1", "12")]
        [InlineData(@"3,4,1,5", "12")]
        public void Part1(string input, string expected)
        {
            this.Day.ChainLength = 5;
            Assert.Equal(expected, this.RunPart1(input));
        }

        [Theory]
        [InlineData(@"", "a2582a3a0e66e6e86e3812dcb672a272")]
        [InlineData(@"AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd")]
        [InlineData(@"1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d")]
        [InlineData(@"1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e")]
        public void Part2(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart2(input));
        }
    }
}
