using Xunit;

namespace AdventOfCode2017.Test
{
    [Trait("Day", "09")]
    public class Day9Test : DayTest<Day9>
    {
        [Theory]
        [InlineData(@"{}", "1")]
        [InlineData(@"{{{}}}", "6")]
        [InlineData(@"{{},{}}", "5")]
        [InlineData(@"{{{},{},{{}}}}", "16")]
        [InlineData(@"{< a>,<a>,<a>,<a>}", "1")]
        [InlineData(@"{{<ab>},{<ab>},{<ab>},{<ab>}}", "9")]
        [InlineData(@"{{< !!>},{<!!>},{<!!>},{<!!>}}", "9")]
        [InlineData(@"{{<a!>},{<a!>},{<a!>},{<ab>}}", "3")]
        public void Part1(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart1(input));
        }

        [Theory]
        [InlineData(@"<>", "0")]
        [InlineData(@"<random characters>", "17")]
        [InlineData(@"<<<<>", "3")]
        [InlineData(@"<{!>}>", "2")]
        [InlineData(@"<!!>", "0")]
        [InlineData(@"<!!!>>", "0")]
        [InlineData(@"<{o""i!a,<{i<a>", "10")]
        public void Part2(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart2(input));
        }
    }
}
