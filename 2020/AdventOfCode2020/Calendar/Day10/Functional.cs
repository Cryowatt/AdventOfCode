using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode.Calendar.Day10
{
    public unsafe class Functional : SolutionBase<IEnumerable<int>>
    {
        public Functional() : base(Parsers.Lines().Int()) { }

        public override object PartA()
        {
            var adapters = new SortedSet<int>(this.input);
            var joltage = 0;
            var differenceCount = new int[4];

            foreach (var adapter in adapters)
            {
                differenceCount[adapter - joltage]++;
                joltage = adapter;
            }

            differenceCount[3]++;

            return differenceCount[1] * differenceCount[3];
        }

        public override object PartB()
        {
            var combinations = new Dictionary<int, long>();
            int maxJoltage = this.input.Max();
            int deviceJoltage = this.input.Max() + 3;
            combinations[maxJoltage] = 1;

            for (int j = maxJoltage - 1; j >= 0; j--)
            {
                var validAdapters = this.input.Where(o => o > j && o - j <= 3);
                combinations[j] = validAdapters.Sum(o => combinations[o]);
            }

            return combinations[0];
        }
    }
}