using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Numerics;

namespace AdventOfCode2017
{
    public class Day11 : IAdventDay
    {
        private readonly IDictionary<string, Vector3> Directions =
            new Dictionary<string, Vector3>
            {
                {"n", Vector3.UnitY - Vector3.UnitZ}, //+x
                {"s", -Vector3.UnitY + Vector3.UnitZ}, //-X
                {"sw", Vector3.UnitZ - Vector3.UnitX}, //-y
                {"ne", -Vector3.UnitZ + Vector3.UnitX}, //+y
                {"se", Vector3.UnitX - Vector3.UnitY}, //-z
                {"nw", -Vector3.UnitX + Vector3.UnitY}, //+z
            }.ToImmutableDictionary();

        public string RunPart1(string input) =>
            input.EnumerateCells().Aggregate(Vector3.Zero, (a, o) => Directions[o] + a).HexGridDistance().ToString();

        public string RunPart2(string input) =>
            input.EnumerateCells().Scan(Vector3.Zero, (a, o) => Directions[o] + a).Select(o => o.HexGridDistance()).Max().ToString();
    }
}