using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;

namespace AdventOfCode2017
{
    public static class Toolbox
    {
        public static IEnumerable<string> EnumerateLines(this string input)
        {
            return input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries);
        }

        public static IEnumerable<string> EnumerateCells(this string input)
        {
            // Test data tends to be space-delimited and real data is tab'd
            return input.Split(new[] { '\t', ' ', ',' }, StringSplitOptions.RemoveEmptyEntries);
        }

        public static IEnumerable<int> AsInt(this IEnumerable<string> input)
        {
            return input.Select(int.Parse);
        }

        public static float HexGridDistance(this Vector3 vector) =>
            Math.Max(Math.Abs(vector.X), Math.Max(Math.Abs(vector.Y), Math.Abs(vector.Z)));

        public static float ManhattanDistance(Vector3 from, Vector3 to) =>
            Math.Abs(from.X - to.X) + Math.Abs(from.Y - to.Y) + Math.Abs(from.Z - to.Z);

        public static float ManhattanDistance(this Vector3 vector) =>
            ManhattanDistance(vector, Vector3.Zero);
    }
}
