using AdventOfCode.Test;
using System.Collections.Generic;
using Xunit;

namespace AdventOfCode.Calendar.Day08
{
    public class TestData : ITestData
    {
        public IEnumerable<(string Input, string Expected)> PartAData => new[]
        {
            (@"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6", "5"),
        };

        public IEnumerable<(string Input, string Expected)> PartBData => new[]
        {
            (@"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6", "8"),
        };

    public class FunctionalTest : DayTest<Functional, TestData> { }
    }
}