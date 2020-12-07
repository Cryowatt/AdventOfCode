using AdventOfCode.Test;
using System.Collections.Generic;
using Xunit;

namespace AdventOfCode.Calendar.Day05
{
    public class TestData : ITestData
    {
        public IEnumerable<(string Input, string Expected)> PartAData => new[]
        {
            ("BFFFBBFRRR", "567"),
            ("FFFBBBFRRR", "119"),
            ("BBFFBBFRLL", "820"),
        };

        public IEnumerable<(string Input, string Expected)> PartBData => new[]
        {
            (@"BFFFBBFRRR
BFFFBBFRLR", "566"),
        };
    }

    public class FunctionalTest : DayTest<Functional, TestData> { }
}
