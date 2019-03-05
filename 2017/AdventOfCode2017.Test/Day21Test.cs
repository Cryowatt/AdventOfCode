using Xunit;

namespace AdventOfCode2017.Test
{
    [Trait("Day", "21")]
    public class Day21Test : DayTest<Day21>
    {
        [Theory]
        [InlineData(@"../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#", "12")]
        public void Part1(string input, string expected)
        {
            this.Day.Iterations = 2;
            Assert.Equal(expected, this.RunPart1(input));
        }

        [Theory]
        public void Part2(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart2(input));
        }
    }
}
