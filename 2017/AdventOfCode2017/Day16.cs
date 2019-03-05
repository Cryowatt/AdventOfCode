using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Reactive.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode2017
{
    public class Day16 : AdventDay<IEnumerable<Func<IImmutableList<int>, IImmutableList<int>>>>
    {
        public int ChainLength = 16;
        public int Dances = 1_000_000_000;

        //x8/10,s15,x5/0,pd/a,x14/7,pg/l,s4
        protected override IEnumerable<Func<IImmutableList<int>, IImmutableList<int>>> TransformInput(string input) =>
            input.EnumerateCells().Select<string, Func<IImmutableList<int>, IImmutableList<int>>>(o =>
            {
                var match = Regex.Match(o, @"(?<Op>x)(?<A>\d+)/(?<B>\d+)|(?<Op>p)(?<A>[a-p])/(?<B>[a-p])|(?<Op>s)(?<A>\d+)");
                var op = match.Groups["Op"].Value;
                switch (op.Single())
                {
                    case 's':
                        return s => Spin(s, match);
                    case 'x':
                        return s => Exchange(s, match);
                    case 'p':
                        return s => Partner(s, match);
                    default: throw new InvalidOperationException();
                }
            });


        private static IImmutableList<int> Spin(IImmutableList<int> list, Match match)
        {
            int length = int.Parse(match.Groups["A"].Value);
            return list.Repeat().Skip(list.Count - length).Take(list.Count).ToImmutableList();
        }

        private static IImmutableList<int> Exchange(IImmutableList<int> list, Match match)
        {
            int ixa = int.Parse(match.Groups["A"].Value);
            int ixb = int.Parse(match.Groups["B"].Value);
            int a = list[ixa];
            int b = list[ixb];
            return list.SetItem(ixa, b).SetItem(ixb, a);
        }

        private static IImmutableList<int> Partner(IImmutableList<int> list, Match match)
        {
            int a = match.Groups["A"].Value.Single() - 'a';
            int b = match.Groups["B"].Value.Single() - 'a';
            int ixa = list.IndexOf(a);
            int ixb = list.IndexOf(b);
            return list.SetItem(ixa, b).SetItem(ixb, a);
        }

        private static string GetDancerPositionString(IImmutableList<int> positions) =>
            string.Concat(positions.Select(o => (char)('a' + o)));

        private static ulong GetDancerPositionState(IImmutableList<int> positions)
        {
            ulong state = 0;

            for (int i = 0; i < positions.Count; i++)
            {
                state |= (ulong)positions[i] << (i * 4);
            }

            return state;
        }

        public override string RunPart1(IEnumerable<Func<IImmutableList<int>, IImmutableList<int>>> input) =>
            GetDancerPositionString(
                input.Scan(
                    (IImmutableList<int>)Enumerable.Range(0, this.ChainLength).ToImmutableList(),
                    (a, v) => v(a))
                .Last());

        public override string RunPart2(IEnumerable<Func<IImmutableList<int>, IImmutableList<int>>> input)
        {
            IImmutableList<int> initialState = Enumerable.Range(0, this.ChainLength).ToImmutableList();
            var start = GetDancerPositionState(initialState);

            var loop = EnumerableEx.Generate(
                initialState,
                state => GetDancerPositionState(state) != start || state == initialState,
                state => input.Scan(state, (a, v) => v(a)).Last(),
                state => state).ToList();

            return GetDancerPositionString(loop[this.Dances % loop.Count]);
        }
    }
}