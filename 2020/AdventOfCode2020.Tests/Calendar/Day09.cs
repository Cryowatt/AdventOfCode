using System;
using System.Collections.Generic;
using System.Linq;
using Xunit;

namespace AdventOfCode.Calendar.Day09
{
    public class DataStructureTest
    {
        public static IEnumerable<object[]> GetPartAData()
        {
            var longPreamble = Enumerable.Range(1, 25).Select(o => (long)o);
            yield return new object[] { longPreamble.Concat(new long[] { 100 }).ToArray(), 25, 100 };
            yield return new object[] { longPreamble.Concat(new long[] { 50 }).ToArray(), 25, 50 };
            yield return new object[] { new long[] { 35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576 }, 5, 127 };
        }

        [Theory]
        [MemberDataAttribute(nameof(GetPartAData))]
        public void DataTest(long[] input, int preambleLength, int expected)
        {
            var day = new DataStructures();
            var data = new Memory<long>(input);
            Assert.Equal(expected, day.FirstExploit(data, preambleLength));
        }
    }
}