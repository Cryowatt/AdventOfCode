using AdventOfCode.Test;
using System.Collections.Generic;
using Xunit;

namespace AdventOfCode.Calendar.Day07
{
    public class TestData : ITestData
    {
        public IEnumerable<(string Input, string Expected)> PartAData => new[]
        {
            (@"", ""),
        };

        public IEnumerable<(string Input, string Expected)> PartBData => new[]
        {
            (@"", ""),
    }

    public class FunctionalTest : DayTest<Functional, TestData> { }
}
