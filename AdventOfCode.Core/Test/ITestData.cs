using System.Collections.Generic;

namespace AdventOfCode.Test
{
    public interface ITestData
    {
        IEnumerable<(string Input, string Expected)> PartAData { get; }

        IEnumerable<(string Input, string Expected)> PartBData { get; }
    }
}
