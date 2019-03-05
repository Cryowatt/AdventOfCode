using System;
using System.Linq;

namespace AdventOfCode2017
{
    public class Day2 : IAdventDay
    {
        public string RunPart1(string input) => (
            from line in input.EnumerateLines()
            let elements = line.EnumerateCells().AsInt()
            let max = elements.Max()
            let min = elements.Min()
            select max - min
        ).Sum().ToString();

        public string RunPart2(string input) => (
            from line in input.EnumerateLines()
            let elements = line.EnumerateCells().AsInt()
            from a in elements
            from b in elements
            where a != b && a % b == 0
            select a / b
        ).Sum().ToString();
    }
}
