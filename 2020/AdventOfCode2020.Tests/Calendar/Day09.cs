using AdventOfCode.Test;
using System.Collections.Generic;
using Xunit;

namespace AdventOfCode.Calendar.Day09
{
    public class TestData : ITestData
    {
        string Preamble = @"1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
";

        public IEnumerable<(string Input, string Expected)> PartAData => new[]
        {
            (Preamble + "50", "50"),
            (@"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576", "127")
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

        public class VectorTest : DayTest<Vector, TestData> { }
        public class FunctionalTest : DayTest<Functional, TestData> { }
    }
}