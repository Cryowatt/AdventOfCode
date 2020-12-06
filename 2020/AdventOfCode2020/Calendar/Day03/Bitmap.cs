using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace AdventOfCode.Calendar.Day03
{
    public unsafe class Bitmap : SolutionBase<ReadOnlyMemory<int>>
    {
        // Hardcoding because lazy
        private const int Width = 31;

        private static int ParseLine(string line)
        {
            int bitmap = 0;

            for (int i = 0; i < Width; i++)
            {
                bitmap += line[i] == '#' ? 1 << i : 0;
            }

            return bitmap;
        }

        public Bitmap() : base(Parsers.Lines(ParseLine).Memory(vectorPadding: true)) { }

        public override void Parse(string input)
        {
            base.Parse(input);
        }

        public override object PartA() => CountTrees(new Point(3, 1));
        //{
        //    var vwidth = Vector<int>.Count;
        //    var map = this.input.Span;
        //    Vector<int> rows;
        //    Vector<int> positionRows;
        //    var zero = Vector<int>.Zero;
        //    var one = Vector<int>.One;
        //    Vector<int> vectorSum = zero;
        //    int i;
        //    Span<int> positions = new Span<int>(new int[vwidth]);

        //    for (i = 0; i < (this.input.Length - vwidth); i += vwidth)
        //    {
        //        rows = new Vector<int>(map.Slice(i, vwidth));
        //        for (int r = 0; r < vwidth; r++)
        //        {
        //            positions[r] = 1 << (((i + r) * 3) % Width);
        //        }
        //        positionRows = new Vector<int>(positions);
        //        vectorSum -= Vector.GreaterThan(Vector.BitwiseAnd(rows, positionRows), zero);
        //    }

        //    var trees = Vector.Dot(vectorSum, one);

        //    for (; i < this.input.Length; i++)
        //    {
        //        trees += (map[i] & (1 << ((i * 3) % Width))) > 0 ? 1 : 0;
        //    }

        //    return trees;
        //}

        private int CountTrees(Point velocity)
        {
            var vwidth = Vector<int>.Count;
            var map = this.input.Span;
            Vector<int> rows;
            Vector<int> positionRows;
            var zero = Vector<int>.Zero;
            var one = Vector<int>.One;
            Vector<int> vectorSum = zero;
            int i;
            int* ppositions = stackalloc int[vwidth];
            Span<int> positions = new Span<int>(ppositions, vwidth);

            for (i = 0; i < this.input.Length; i += vwidth)
            {
                rows = new Vector<int>(map.Slice(i, vwidth));
                int* p = ppositions;
                for (int r = 0; r < vwidth; r += velocity.Y)
                {
                    *p = 1 << (((i + r) * velocity.X / velocity.Y) % Width);
                    p += velocity.Y;
                }
                positionRows = new Vector<int>(positions);
                vectorSum -= Vector.GreaterThan(Vector.BitwiseAnd(rows, positionRows), zero);
            }

            var trees = Vector.Dot(vectorSum, one);

            return trees;
        }

        public override object PartB() =>
            (long)CountTrees(new Point(1, 1)) *
            CountTrees(new Point(3, 1)) *
            CountTrees(new Point(5, 1)) *
            CountTrees(new Point(7, 1)) *
            CountTrees(new Point(1, 2));
    }
}
