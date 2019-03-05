using Xunit;

namespace AdventOfCode2017.Test
{
    [Trait("Day", "03")]
    public class Day3Test :DayTest<Day3>
    {
        [Theory]
        [InlineData("1", "0")]
        [InlineData("12", "3")]
        [InlineData("23", "2")]
        [InlineData("1024", "31")]
        public void Part1(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart1(input));
        }

        [Theory]
        [InlineData("1", "2")]
        [InlineData("2", "4")]
        [InlineData("4", "5")]
        [InlineData("5", "10")]
        [InlineData("10", "11")]
        [InlineData("11", "23")]
        [InlineData("23", "25")]
        [InlineData("25", "26")]
        [InlineData("26", "54")]
        public void Part2(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart2(input));
        }
    }
}
