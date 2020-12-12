using AdventOfCode.Test;
using System.Collections.Generic;
using Xunit;

namespace AdventOfCode.Calendar.Day10
{
    public class TestData : ITestData
    {
        public IEnumerable<(string Input, string Expected)> PartAData => new[]
        {
            (@"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3", "220"),
        };

        public IEnumerable<(string Input, string Expected)> PartBData => new[]
        {
            (@"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3", "19208"),
        };

        public class FunctionalTest : DayTest<Functional, TestData> { }
    }
}