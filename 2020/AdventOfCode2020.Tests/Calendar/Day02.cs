using AdventOfCode.Test;
using System.Collections.Generic;

namespace AdventOfCode.Calendar.Day02
{
    public class TestData : ITestData
    {
        public IEnumerable<(string Input, string Expected)> PartAData => new[]
        {
            ("1-3 a: abcde", "1"),
            ("1-3 b: cdefg", "0"),
            ("2-9 c: ccccccccc", "1")
        };

        public IEnumerable<(string Input, string Expected)> PartBData => new[]
        {
            ("1-3 a: abcde", "1"),
            ("1-3 b: cdefg", "0"),
            ("2-9 c: ccccccccc", "0")
        };
    }

    public class LoopsTests : DayTest<Loops, TestData> { }
    public class LinqTests : DayTest<Linq, TestData> { }
}
