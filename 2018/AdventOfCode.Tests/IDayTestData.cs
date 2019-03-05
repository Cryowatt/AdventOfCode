using System.Collections.Generic;

namespace AdventOfCode.Tests
{
    public interface IDayTestData
    {
        int DayNumber { get; }

        IEnumerable<(string Input, string Expected)> PartATests { get; }

        IEnumerable<(string Input, string Expected)> PartBTests { get; }
    }
}