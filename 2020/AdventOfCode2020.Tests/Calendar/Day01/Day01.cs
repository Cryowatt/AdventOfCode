using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AdventOfCode.Calendar.Day01
{
    public static class BaseTests
    {
        public static IEnumerable<(string Input, string Expected)> PartATests => new[]
        {
            (@"1721
            979
            366
            299
            675
            1456", "514579")
        };

        public static IEnumerable<(string Input, string Expected)> PartBTests => new[]
        {
            (@"1721
            979
            366
            299
            675
            1456", "241861950")
        };
    }

    public class BruteForceTest : DayTest<BruteForceTest, BruteForce>
    {
        public override IEnumerable<(string Input, string Expected)> PartATests => BaseTests.PartATests;

        public override IEnumerable<(string Input, string Expected)> PartBTests => BaseTests.PartBTests;
    }

    public class SetOperationsTest : DayTest<SetOperationsTest, SetOperations>
    {
        public override IEnumerable<(string Input, string Expected)> PartATests => BaseTests.PartATests;

        public override IEnumerable<(string Input, string Expected)> PartBTests => BaseTests.PartBTests;
    }

    public class FastTest : DayTest<FastTest, Fast>
    {
        public override IEnumerable<(string Input, string Expected)> PartATests => BaseTests.PartATests;

        public override IEnumerable<(string Input, string Expected)> PartBTests => BaseTests.PartBTests;
    }
}
