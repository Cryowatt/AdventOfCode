using System;
using System.Linq;

namespace AdventOfCode2017
{
    public class Day4 : IAdventDay
    {
        public string RunPart1(string input) =>
            (from line in input.EnumerateLines()
             let passphase =
                 from word in line.EnumerateCells()
                 group word by word
             where passphase.All(o => o.Count() == 1)
             select line).Count().ToString();

        public string RunPart2(string input) =>
            (from line in input.EnumerateLines()
             let passphase =
                 from word in line.EnumerateCells()
                 group word by new string(word.OrderBy(o=>o).ToArray())
             where passphase.All(o => o.Count() == 1)
             select line).Count().ToString();
    }
}
