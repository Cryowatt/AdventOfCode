using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace AdventOfCode.CSharp
{
    public class Day1 : Day<IEnumerable<int>>
    {
        public override IEnumerable<int> ParseInput(IEnumerable<string> input)
            => input.Select(o => int.Parse(o));

        public override string RunPartA(IEnumerable<int> input)
            => input.Sum().ToString();

        public override string RunPartB(IEnumerable<int> input)
        {
            int frequency = 0;
            var seenFrequencies = new HashSet<int> { frequency };

            while (true)
            {
                foreach (var f in input)
                {
                    frequency += f;
                    if (!seenFrequencies.Add(frequency))
                    {
                        return frequency.ToString();
                    }
                }
            }
        }
    }
}
