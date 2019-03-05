using AdventOfCode.CSharp;
using System.Collections.Generic;
using Xunit;
using System.Linq;
using AdventOfCode.FSharp;

namespace AdventOfCode.Tests
{
    public abstract class DayTest<TData>
        where TData : IDayTestData, new()
    {
        [Theory]
        [MemberData(nameof(PartAData))]
        public void CSharpPartA(IEnumerable<string> input, string expected)
        {
            var answer = GetDay<CSharpDayFactory>().RunPartA(input);
            Assert.Equal(expected, answer);
        }

        [Theory]
        [MemberData(nameof(PartBData))]
        public void CSharpPartB(IEnumerable<string> input, string expected)
        {
            var answer = GetDay<CSharpDayFactory>().RunPartB(input);
            Assert.Equal(expected, answer);
        }

        [Theory]
        [MemberData(nameof(PartAData))]
        public void FSharpPartA(IEnumerable<string> input, string expected)
        {
            var answer = GetDay<FSharpDayFactory>().RunPartA(input);
            Assert.Equal(expected, answer);
        }

        [Theory]
        [MemberData(nameof(PartBData))]
        public void FSharpPartB(IEnumerable<string> input, string expected)
        {
            var answer = GetDay<FSharpDayFactory>().RunPartB(input);
            Assert.Equal(expected, answer);
        }

        public static IEnumerable<object[]> PartAData
            => new TData().PartATests.Select(o => new object[] { o.Input.Split(","), o.Expected });

        public static IEnumerable<object[]> PartBData
            => new TData().PartBTests.Select(o => new object[] { o.Input.Split(","), o.Expected });

        private static IDay GetDay<TDayFactory>()
            where TDayFactory : IDayFactory, new()
        {
            IDayFactory dayFactory = new TDayFactory();
            IDayTestData data = new TData();
            var day = dayFactory.GetDay(data.DayNumber);
            return day;
        }
    }
}