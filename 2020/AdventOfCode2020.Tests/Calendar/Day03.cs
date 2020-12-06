using AdventOfCode.Test;
using System.Collections.Generic;

namespace AdventOfCode.Calendar.Day03
{
    public class TestData : ITestData
    {
        public IEnumerable<(string Input, string Expected)> PartAData => new[]
        {
            (@"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#", "7")
        };

        public IEnumerable<(string Input, string Expected)> PartBData => new[]
        {
            (@"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#", "7")
        };
    }

    public class StringOpsTest : DayTest<StringOps, TestData> { }
    public class BitmapsTest : ParityTest<Bitmap, StringOps> { }
}
