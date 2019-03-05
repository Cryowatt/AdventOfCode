using System.Linq;
using System.Reactive.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode2017
{
    public class Day15 : AdventDay<(long A, long B)>
    {
        protected override (long A, long B) TransformInput(string input)
        {
            var match = Regex.Matches(input, @"Generator (A|B) starts with (?<Start>\d+)");
            return (int.Parse(match[0].Groups["Start"].Value), int.Parse(match[1].Groups["Start"].Value));
        }

        private const int FactorA = 16807;
        private const int FactorB = 48271;
        private const long Mask = 0xffff;

        public override string RunPart1((long A, long B) input)
        {
            long a = input.A;
            long b = input.B;
            int count = 0;

            for (int i = 0; i < 40_000_000; i++)
            {
                a = (a * FactorA) % int.MaxValue;
                b = (b * FactorB) % int.MaxValue;

                if ((a & Mask) == (b & Mask))
                {
                    count++;
                }
            }

            return count.ToString();
        }

        //public override string RunPart1((long A, long B) input) =>
        //    EnumerableEx.Generate(input.A, state => true, state => (state * FactorA) % int.MaxValue, state => state).Take(40_000_000).Zip(
        //        EnumerableEx.Generate(input.B, state => true, state => (state * FactorB) % int.MaxValue, state => state).Take(40_000_000),
        //        (a, b) => (a & Mask) == (b & Mask)).Count(o => o).ToString();

        public override string RunPart2((long A, long B) input) =>
            EnumerableEx.Generate(input.A, state => true, state => (state * FactorA) % int.MaxValue, state => state).Where(o => o % 4 == 0).Take(5_000_000).Zip(
                EnumerableEx.Generate(input.B, state => true, state => (state * FactorB) % int.MaxValue, state => state).Where(o => o % 8 == 0).Take(5_000_000),
                (a, b) => (a & Mask) == (b & Mask)).Count(o => o).ToString();
    }
}