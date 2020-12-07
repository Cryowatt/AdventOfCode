using AdventOfCode.Test;
using System.Collections.Generic;
using Xunit;

namespace AdventOfCode.Calendar.Day06
{
    public class TestData : ITestData
    {
        public IEnumerable<(string Input, string Expected)> PartAData => new[]
        {
            (@"abc

a
b
c

ab
ac

a
a
a
a

b", "11"),
        };

        public IEnumerable<(string Input, string Expected)> PartBData => new[]
        {
            (@"BFFFBBFRRR
BFFFBBFRLR", "566"),
        };
    }

    public class FunctionalTest : DayTest<Functional, TestData> { }
}
