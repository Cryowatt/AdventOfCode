using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Collections.Specialized;
using System.Linq;
using System.Numerics;

namespace AdventOfCode2017
{
    public class Day21 : AdventDay<IDictionary<int, byte[]>>
    {
        public int Iterations { get; set; } = 5;

        private Vector2[][] symetry = new[]
        {
            new [] { new Vector2(0, 0), new Vector2(1, 0), new Vector2(1, 0), new Vector2(1, 1) }, // 0
            new [] { new Vector2(1, 0), new Vector2(1, 1), new Vector2(1, 0), new Vector2(1, 1) }, // 90

            new [] { new Vector2(0, 1), new Vector2(0, 0), new Vector2(1, 1), new Vector2(1, 0) }, // 0h
            new [] { new Vector2(1, 0), new Vector2(1, 1), new Vector2(0, 0), new Vector2(0, 1) }, // 0v
            new [] { new Vector2(0, 1), new Vector2(0, 0), new Vector2(1, 1), new Vector2(1, 0) }, // 90h
            new [] { new Vector2(1, 0), new Vector2(1, 1), new Vector2(0, 0), new Vector2(0, 1) }, // 90v
        };

        protected override IDictionary<int, byte[]> TransformInput(string input)
        {
            var seed =
                from line in input.EnumerateLines()
                let fragments = line.Replace("/", "").Split(new[] { '=', '>', ' ' }, StringSplitOptions.RemoveEmptyEntries)
                let source = fragments[0].Select(o => o == '#' ? (byte)1 : (byte)0).ToImmutableArray()
                //..Select((c, i) => c == '#' ? 1 << i : 0).Sum() + (fragments[0].Length == 4 ? 2 << 16 : 3 << 16)
                let target = fragments[1].Select(o => o == '#' ? (byte)1 : (byte)0).ToArray()
                select (source, target);

            var transformations = new Dictionary<int, byte[]>();

            foreach (var item in seed)
            {
                var source = item.source;

                //add item
                if (item.source.Length == 4)
                {
                    // rotate
                    var mut = source.ToBuilder();
                    mut[0] = source[2];
                    mut[1] = source[0];
                    mut[2] = source[4];
                    mut[3] = source[1];
                    item.source.ToBuilder();
                }
                else
                {
                }
            }

            return transformations;
        }

        public override string RunPart1(IDictionary<int, byte[]> input)
        {
            int maxGridWidth = 3;

            for (int i = 0; i < this.Iterations; i++)
            {
                var div3 = Math.DivRem(maxGridWidth, 3, out int remainder);
                maxGridWidth += remainder == 0 ? div3 : maxGridWidth >> 1;
            }

            var grid = Enumerable.Range(0, maxGridWidth).Select(o => new byte[maxGridWidth]).ToArray();
            grid[0][1] = grid[1][2] = grid[2][0] = grid[2][1] = grid[2][2] = 1;
            int gridWidth = 3;

            for (int i = 0; i < this.Iterations; i++)
            {
                var div3 = Math.DivRem(gridWidth, 3, out int remainder);
                int gridStep = remainder == 0 ? 3 : 2;

                for (int y = gridWidth - gridStep; y >= 0; y -= gridStep)
                {
                    for (int x = gridWidth - gridStep; x >= 0; x -= gridStep)
                    {
                        Enhance(input, grid, new Vector2(x, y), new Vector2(x / gridStep * (gridStep + 1), y / gridStep * (gridStep + 1)), gridStep);
                        Console.WriteLine($"{x} {y}");
                    }
                }

                gridWidth += remainder == 0 ? div3 : maxGridWidth >> 1;
            }

            return grid.SelectMany(o => o).Count(o => o == 1).ToString();
        }

        private void Enhance(IDictionary<int, byte[]> tranformations, byte[][] grid, Vector2 from, Vector2 to, int step)
        {
            int val = step << 16;
            val |= grid[(int)from.Y + 0][(int)from.X + 0] << 0;
            val |= grid[(int)from.Y + 0][(int)from.X + 1] << 1;
            val |= grid[(int)from.Y + 1][(int)from.X + 0] << 2;
            val |= grid[(int)from.Y + 1][(int)from.X + 1] << 3;

            var newPattern = tranformations[val];

            for (int i = 0; i < step; i++)
            {
                Array.Copy(newPattern, i, grid[(int)to.Y + i], (int)to.X, step);
            }
        }

        public override string RunPart2(IDictionary<int, byte[]> input) =>
            throw new NotImplementedException();
    }
}