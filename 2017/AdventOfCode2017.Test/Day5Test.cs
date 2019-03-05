using Xunit;

namespace AdventOfCode2017.Test
{
    [Trait("Day", "05")]
    public class Day5Test :DayTest<Day5>
    {
        [Theory]
        [InlineData(@"0
3
0
1
-3", "5")]
        public void Part1(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart1(input));
        }

        [Theory]
        [InlineData(@"0
3
0
1
-3", "10")]
        public void Part2(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart2(input));
        }
    }
}
