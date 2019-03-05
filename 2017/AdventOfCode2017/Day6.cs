using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

namespace AdventOfCode2017
{
    public class Day6 : IAdventDay
    {
        public string RunPart1(string input) =>
            EnumerableEx.Generate(
                input.EnumerateCells().AsInt().ToImmutableArray(),
                o => true,
                s => Enumerable.Range(s.IndexOf(s.Max()) + 1, s.Max())
                    .Scan(
                        s.SetItem(s.IndexOf(s.Max()), 0),
                        (a, i) => a.SetItem(i % a.Length, a[i % a.Length] + 1))
                    .Last(), s => s)
                .Scan(
                    (History: ImmutableHashSet<string>.Empty, IsDupe: false),
                    (a, o) => (a.History.Add(string.Join('\t', o)), a.History.Contains(string.Join('\t', o))))
                .TakeWhile(o => !o.IsDupe).Count().ToString();

        public string RunPart2(string input) =>
            EnumerableEx.Generate(
                input.EnumerateCells().AsInt().ToImmutableArray(),
                o => true,
                s => Enumerable.Range(s.IndexOf(s.Max()) + 1, s.Max())
                    .Scan(
                        s.SetItem(s.IndexOf(s.Max()), 0),
                        (a, i) => a.SetItem(i % a.Length, a[i % a.Length] + 1))
                    .Last(), s => string.Join('\t', s))
                .Scan(
                    (History: ImmutableHashSet<string>.Empty, IsDupe: false, Bank: string.Empty),
                    (a, o) => (a.History.Add(o), a.History.Contains(o), o))
                .SkipWhile(o => !o.IsDupe)
                .Scan(
                    (History: ImmutableHashSet<string>.Empty, IsDupe: false, Bank: string.Empty),
                    (a, o) => (a.History.Add(o.Bank), a.History.Contains(o.Bank), o.Bank))
                .TakeWhile(o => !o.IsDupe).Count().ToString();
    }
}