using System.Collections.Generic;
using System.Linq;
using Xunit;

namespace AdventOfCode
{
    public abstract class DayTest<TTest, TDay>
        where TDay : IDay, new()
        where TTest : DayTest<TTest, TDay>, new()
    {
        public abstract IEnumerable<(string Input, string Expected)> PartATests { get; }
        public abstract IEnumerable<(string Input, string Expected)> PartBTests { get; }

        public string RunPartA(string input) => CreateDay(input).PartA().ToString();

        public string RunPartB(string input) => CreateDay(input).PartB().ToString();

        private TDay CreateDay(string input)
        {
            var day = new TDay();
            day.Parse(input);
            return day;
        }

        public static IEnumerable<object[]> PartATestData => new TTest().PartATests.Select(o => new object[] { o.Input, o.Expected });
        public static IEnumerable<object[]> PartBTestData => new TTest().PartBTests.Select(o => new object[] { o.Input, o.Expected });

        [Theory]
        [MemberData(nameof(PartATestData))]
        public void PartA(string input, string expected)
        {
            Assert.Equal(expected, new TTest().RunPartA(input));
        }

        [Theory]
        [MemberData(nameof(PartBTestData))]
        public void PartB(string input, string expected)
        {
            Assert.Equal(expected, new TTest().RunPartB(input));
        }
    }
}
