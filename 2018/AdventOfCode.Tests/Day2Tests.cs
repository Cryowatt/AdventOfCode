using System.Collections.Generic;

namespace AdventOfCode.Tests
{
    public class Day2Tests : DayTest<Day2Tests>, IDayTestData
    {
        public int DayNumber => 2;

        public IEnumerable<(string Input, string Expected)> PartATests => new[]
        {
            ("abcdef,bababc,abbcde,abcccd,aabcdd,abcdee,ababab", "12")
        };

        public IEnumerable<(string Input, string Expected)> PartBTests => new[]
        {
            ("abcde,fghij,klmno,pqrst,fguij,axcye,wvxyz", "fgij"),
        };
    }
}