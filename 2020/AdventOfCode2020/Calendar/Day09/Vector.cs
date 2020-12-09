using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Numerics;
using System.Text.RegularExpressions;

namespace AdventOfCode.Calendar.Day09
{
    public class Vector : SolutionBase<ReadOnlyMemory<int>>
    {
        public Vector() : base(Parsers.Lines().Int().Memory(vectorPadding: true)) { }

        public override object PartA() => FirstExploit(this.input.Slice(0, 25), this.input.Slice(25));

        public int FirstExploit(ReadOnlyMemory<int> preamble, ReadOnlyMemory<int> data)
        {
            int vectorLength = Vector<int>.Count;
            int checkChunkCount = Math.DivRem(preamble.Length, vectorLength, out int remainder);
            checkChunkCount += (remainder > 0) ? 1 : 0;
            var checkBuffer = new Memory<int>(new int[vectorLength * checkChunkCount]);
            preamble.CopyTo(checkBuffer);

            for (int i = 0; i < data.Length; i++)
            {
                var number = new Vector<int>(data.Span[i]);
                for(int c = 0; c < checkChunkCount; c++)
                {
                    var check = new Vector<int>(checkBuffer.Span.Slice(c * vectorLength, vectorLength));
                    Vector.
                }
            }

            return 0;
        }

        public override object PartB() => throw new NotImplementedException();
    }
}
