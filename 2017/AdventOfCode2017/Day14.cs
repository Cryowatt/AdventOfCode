using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Numerics;
using System.Text;

namespace AdventOfCode2017
{
    public class Day14 : IAdventDay
    {
        private static readonly IReadOnlyDictionary<char, int> bitCount =
            Enumerable.Range(0, 16).ToImmutableDictionary(o => Convert.ToString(o, 16)[0], o => Convert.ToString(o, 2).Count(c => c == '1'));

        private static readonly IReadOnlyList<string> binaryString =
            Enumerable.Range(0, 256).Select(
                o =>
                {
                    var binary = Convert.ToString(o, 2);
                    return new string('0', 8 - binary.Length) + binary;
                }).ToImmutableList();

        private static Day10 KnotHash = new Day10();

        public string RunPart1(string input) =>
            (from row in Enumerable.Range(0, 128)
             let hash = KnotHash.RunPart2($"{input}-{row}")
             from c in hash
             select bitCount[c]).Sum().ToString();


        public string RunPart2(string input)
        {
            char[][] grid =
                (from rowNumber in Enumerable.Range(0, 128)
                 let hash = KnotHash.KnotHash(Encoding.ASCII.GetBytes($"{input}-{rowNumber}"))
                 let row =
                    (from c in hash
                     select binaryString[c])
                 select string.Concat(row).ToCharArray()).ToArray();

            int groupCount = 0;

            for (int y = 0; y < 128; y++)
            {
                for (int x = 0; x < 128; x++)
                {
                    if (grid[y][x] == '1')
                    {
                        ClearGroup(grid, x, y);
                        groupCount++;
                    }
                }
            }

            return groupCount.ToString();
        }

        private void ClearGroup(char[][] grid, int x, int y)
        {
            if (0 <= y && y < 128 && 0 <= x && x < 128 && grid[y][x] == '1')
            {
                grid[y][x] = ' ';
                ClearGroup(grid, x + 1, y);
                ClearGroup(grid, x - 1, y);
                ClearGroup(grid, x, y + 1);
                ClearGroup(grid, x, y - 1);
            }
        }
    }
}