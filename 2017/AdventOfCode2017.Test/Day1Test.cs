using Xunit;

namespace AdventOfCode2017.Test
{
    [Trait("Day", "01")]
    public class Day1Test : DayTest<Day1>
    {
        [Theory]
        [InlineData("1122", "3")]
        [InlineData("1111", "4")]
        [InlineData("1234", "0")]
        [InlineData("91212129", "9")]
        public void Part1(string input, string expectedOutput)
        {
            Assert.Equal(expectedOutput, RunPart1(input));
        }

        [Theory]
        [InlineData("1212", "6")]
        [InlineData("1221", "0")]
        [InlineData("123425", "4")]
        [InlineData("123123", "12")]
        [InlineData("12131415", "4")]
        public void Part2(string input, string expectedOutput)
        {
            Assert.Equal(expectedOutput, RunPart2(input));
        }
    }
}
