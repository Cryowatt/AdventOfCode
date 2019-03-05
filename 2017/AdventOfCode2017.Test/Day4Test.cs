using Xunit;

namespace AdventOfCode2017.Test
{
    [Trait("Day", "04")]
    public class Day4Test : DayTest<Day4>
    {
        [Theory]
        [InlineData("aa bb cc dd ee", "1")]
        [InlineData("aa bb cc dd aa", "0")]
        [InlineData("aa bb cc dd aaa", "1")]
        [InlineData(@"aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa", "2")]
        public void Part1(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart1(input));
        }

        [Theory]
        [InlineData("abcde fghij", "1")]
        [InlineData("abcde xyz ecdab", "0")]
        [InlineData("a ab abc abd abf abj", "1")]
        [InlineData("iiii oiii ooii oooi oooo", "1")]
        [InlineData("oiii ioii iioi iiio", "0")]
        public void Part2(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart2(input));
        }
    }
}
