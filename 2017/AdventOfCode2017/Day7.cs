using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode2017
{
    public class Day7 : IAdventDay
    {
        //ugmfrt (307) -> vwtpihe, sohtd, yluell, zrltc, bmfcc, dresa, httspr
        public string RunPart1(string input) =>
                (from puzzle in EnumerableEx.Return(
                    from line in input.EnumerateLines()
                    let match = Regex.Match(line, @"(?<Name>\w+) \((?<Mass>\d+)\)(?: -> (?<Children>\w+)(?:, (?<Children>\w+))*)?")
                    select (Name: match.Groups["Name"].Value, Mass: match.Groups["Mass"].Value, Children: match.Groups["Children"].Captures.Select(o => o.Value))).Memoize()
                 let allChildren = (
                     from disk in puzzle
                     from child in disk.Children
                     select child).ToHashSet()
                 from disk in puzzle
                 where !allChildren.Contains(disk.Name)
                 select disk.Name).Single();

        public string RunPart2(string input) =>
                (from puzzle in EnumerableEx.Return(
                    from line in input.EnumerateLines()
                    let match = Regex.Match(line, @"(?<Name>\w+) \((?<Mass>\d+)\)(?: -> (?<Children>\w+)(?:, (?<Children>\w+))*)?")
                    select (Name: match.Groups["Name"].Value, Mass: int.Parse(match.Groups["Mass"].Value), Children: match.Groups["Children"].Captures.Select(o => o.Value))).Memoize()
                 let nodes = puzzle.ToDictionary(o => o.Name)
                 let edges =
                    (from disk in puzzle
                     from child in disk.Children
                     select (Parent: disk, Child: nodes[child])).Memoize()
                 let parentMap = edges.ToDictionary(o => o.Child.Name, o => o.Parent)
                 let childMap = edges.ToLookup(o => o.Parent.Name, o => o.Child)
                 let tree =
                    (from disk in puzzle
                     let totalMass = disk.Mass + childMap[disk.Name].Expand(o => childMap[o.Name]).Sum(o => o.Mass)
                     let parent = parentMap.GetValueOrDefault(disk.Name)
                     select (Disk: disk, TotalMass: totalMass)).ToDictionary(o => o.Disk.Name)
                 from node in tree.Values
                 let children =
                    from child in childMap[node.Disk.Name]
                    select tree[child.Name]
                 from child in children
                 group (child, children) by child.TotalMass into childGroup
                 where childGroup.Count() == 1
                 let wrongChild = childGroup.Single()
                 let wrongValue = wrongChild.child.TotalMass
                 let correctValue = wrongChild.children.First(o => o.TotalMass != wrongValue).TotalMass
                 orderby correctValue ascending
                 select wrongChild.child.Disk.Mass - (wrongValue - correctValue)).First().ToString();
    }
}