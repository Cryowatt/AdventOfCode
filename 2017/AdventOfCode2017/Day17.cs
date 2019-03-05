using System;
using System.Collections.Immutable;
using System.Linq;
using System.Reactive.Linq;

namespace AdventOfCode2017
{
    public class Day17 : AdventDay<int>
    {
        public int Part2Iterations = 50_000_000;

        protected override int TransformInput(string input) =>
            int.Parse(input);

        public override string RunPart1(int input) =>
            Enumerable.Range(1, 2017)
            .Scan((Position: 0, Buffer: ImmutableList.Create<int>(0)),
                (a, i) =>
                {
                    int newPosition = ((a.Position + input) % a.Buffer.Count) + 1;
                    return (newPosition, a.Buffer.Insert(newPosition, i));
                }).Select(o => o.Buffer[(o.Position + 1) % o.Buffer.Count]).Last().ToString();

        //(Position+3 % Index)+1
        public override string RunPart2(int input) =>
            Enumerable.Range(1, Part2Iterations)
            .Scan(0, (a, i) => ((a + input) % i) + 1)
            .Select((p, i) => (p, i: i + 1)).Last(o => o.p == 1).i.ToString();
    }
}