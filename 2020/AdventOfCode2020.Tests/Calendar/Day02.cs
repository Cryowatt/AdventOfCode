using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AdventOfCode.Calendar.Day02
{
    public static class BaseTests
    {
        public static IEnumerable<(string Input, string Expected)> PartATests => new[]
        {
            ("1-3 a: abcde", "1"),
            ("1-3 b: cdefg", "0"),
            ("2-9 c: ccccccccc", "1")
        };

        public static IEnumerable<(string Input, string Expected)> PartBTests => new[]
        {
            ("1-3 a: abcde", "1"),
            ("1-3 b: cdefg", "0"),
            ("2-9 c: ccccccccc", "0")
        };
    }

    public class RegularExpressionsTests : DayTest<RegularExpressionsTests, RegularExpressions>
    {
        public override IEnumerable<(string Input, string Expected)> PartATests => BaseTests.PartATests;

        public override IEnumerable<(string Input, string Expected)> PartBTests => BaseTests.PartBTests;
    }
}
