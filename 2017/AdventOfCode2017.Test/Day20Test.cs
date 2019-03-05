using Xunit;

namespace AdventOfCode2017.Test
{
    [Trait("Day", "20")]
    public class Day20Test : DayTest<Day20>
    {
        [Theory]
        [InlineData(@"p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>", "0")]
        public void Part1(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart1(input));
        }

        [Theory]
        [InlineData(@"p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>", "1")]
        [InlineData(@"p=<-6,-4,-2>, v=<3,2,1>, a=<0,0,0>
p=<-4,-6,-2>, v=<2,3,1>, a=<0,0,0>", "0")]
        public void Part2(string input, string expected)
        {
            Assert.Equal(expected, this.RunPart2(input));
        }
    }
}
