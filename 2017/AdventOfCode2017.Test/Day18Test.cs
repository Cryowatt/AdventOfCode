using Xunit;

namespace AdventOfCode2017.Test
{
    [Trait("Day", "18")]
    public class Day18Test : DayTest<Day18>
    {
        [Theory]
        [InlineData(@"set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2", "4")]
        public void Part1(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart1(input));
        }

        [Theory]
        [InlineData(@"snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d", "3")]
        public void Part2(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart2(input));
        }
    }
}
