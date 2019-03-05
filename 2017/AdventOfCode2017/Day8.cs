using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode2017
{
    public class Day8 : IAdventDay
    {
        private static Dictionary<string, Func<int, int, bool>> OperatorMap = new Dictionary<string, Func<int, int, bool>>
        {
            { "==",  (left, right) => left == right},
            { "!=",  (left, right) => left != right},
            { "<",  (left, right) => left < right},
            { ">",  (left, right) => left > right},
            { "<=",  (left, right) => left <= right},
            { ">=",  (left, right) => left >= right},
        };

        //gug dec 188 if zpw >= 8
        public string RunPart1(string input) =>
            (from line in input.EnumerateLines()
             let match = Regex.Match(line, @"(?<Reg>\w+) (?<Op>dec|inc) (?<Val>-?\d+) if (?<CReg>\w+) (?<COp>[<>!=]+) (?<CVal>-?\d+)")
             let isIncremenet = match.Groups["Op"].Value == "inc"
             let Reg = match.Groups["Reg"].Value
             let Val = int.Parse(match.Groups["Val"].Value) * (isIncremenet ? 1 : -1)
             let CReg = match.Groups["CReg"].Value
             let COp = OperatorMap[match.Groups["COp"].Value]
             let CVal = int.Parse(match.Groups["CVal"].Value)
             let instruction = (Reg, Val, CReg, COp, CVal)
             select instruction).Aggregate(
                ImmutableDictionary<string, int>.Empty,
                (r, i) => (i.COp(r.GetValueOrDefault(i.CReg), i.CVal)) ?
                r.SetItem(i.Reg, r.GetValueOrDefault(i.Reg) + i.Val) : r)
            .Max(o => o.Value).ToString();

        public string RunPart2(string input) =>
            (from line in input.EnumerateLines()
             let match = Regex.Match(line, @"(?<Reg>\w+) (?<Op>dec|inc) (?<Val>-?\d+) if (?<CReg>\w+) (?<COp>[<>!=]+) (?<CVal>-?\d+)")
             let isIncremenet = match.Groups["Op"].Value == "inc"
             let Reg = match.Groups["Reg"].Value
             let Val = int.Parse(match.Groups["Val"].Value) * (isIncremenet ? 1 : -1)
             let CReg = match.Groups["CReg"].Value
             let COp = OperatorMap[match.Groups["COp"].Value]
             let CVal = int.Parse(match.Groups["CVal"].Value)
             let instruction = (Reg, Val, CReg, COp, CVal)
             select instruction).Scan(
                ImmutableDictionary<string, int>.Empty,
                (r, i) => (i.COp(r.GetValueOrDefault(i.CReg), i.CVal)) ?
                r.SetItem(i.Reg, r.GetValueOrDefault(i.Reg) + i.Val) : r)
            .SelectMany(o=>o)
            .Max(o => o.Value).ToString();
    }
}