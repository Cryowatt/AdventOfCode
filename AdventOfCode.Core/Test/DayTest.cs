using System.Collections.Generic;
using System.Linq;
using Xunit;

namespace AdventOfCode.Test
{
    public abstract class DayTest<TDay, TTestData>
        where TDay : IDay, new()
        where TTestData : ITestData, new()
    {
        public string RunPartA(string input) => CreateDay(input).PartA().ToString();

        public string RunPartB(string input) => CreateDay(input).PartB().ToString();

        private TDay CreateDay(string input)
        {
            var day = new TDay();
            day.Parse(input);
            return day;
        }

        public static IEnumerable<object[]> PartATestData => new TTestData().PartAData.Select(o => new object[] { o.Input, o.Expected });
        public static IEnumerable<object[]> PartBTestData => new TTestData().PartBData.Select(o => new object[] { o.Input, o.Expected });

        [Theory]
        [MemberData(nameof(PartATestData))]
        public void PartA(string input, string expected)
        {
            Assert.Equal(expected, RunPartA(input));
        }

        [Theory]
        [MemberData(nameof(PartBTestData))]
        public void PartB(string input, string expected)
        {
            Assert.Equal(expected, RunPartB(input));
        }
    }
}
