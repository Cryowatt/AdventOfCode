using System.Collections.Generic;

namespace AdventOfCode.Tests
{
    public class Day1Tests : DayTest<Day1Tests>, IDayTestData
    {
        public int DayNumber => 1;

        public IEnumerable<(string Input, string Expected)> PartATests => new[]
        {
            ("+1, +1, +1", "3"),
            ("+1, +1, -2", "0"),
            ("-1, -2, -3", "-6"),
        };

        public IEnumerable<(string Input, string Expected)> PartBTests => new[]
        {
            ("+1, -1", "0"),
            ("+3, +3, +4, -2, -4", "10"),
            ("-6, +3, +8, +5, -6", "5"),
            ("+7, +7, -2, -7, -4", "14"),
        };
    }
}