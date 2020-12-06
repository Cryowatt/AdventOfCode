using AdventOfCode.Test;
using System.Collections.Generic;

namespace AdventOfCode.Calendar.Day01
{
    public class TestData : ITestData
    {
        public IEnumerable<(string Input, string Expected)> PartAData => new[]
        {
            (@"1721
            979
            366
            299
            675
            1456", "514579")
        };

        public IEnumerable<(string Input, string Expected)> PartBData => new[]
        {
            (@"1721
            979
            366
            299
            675
            1456", "241861950")
        };
    }

    public class BruteForceTest : DayTest<BruteForce, TestData> { }
    public class FastTest : DayTest<Fast, TestData> { }
}
