using AdventOfCode;
using System;
using System.Collections.Generic;
using System.Text;

namespace AdventOfCode.Calendar.Day01
{
    public class BruteForce : SolutionBase<int[]>
    {
        public BruteForce() : base(Parsers.Lines().Int().ToArray())
        {
        }

        public override object PartA()
        {
            for (int i = 0; i < this.input.Length; i++)
            {
                var a = this.input[i];
                for (int j = i + 1; j < this.input.Length; j++)
                {
                    var b = this.input[j];
                    if (a + b == 2020)
                    {
                        return a * b;
                    }
                }
            }

            throw new InvalidOperationException();
        }

        public override object PartB()
        {
            for (int i = 0; i < this.input.Length; i++)
            {
                var a = this.input[i];
                for (int j = i + 1; j < this.input.Length; j++)
                {
                    var b = this.input[j];
                    for (int k = i + j + 1; k < this.input.Length; k++)
                    {
                        var c = this.input[k];
                        if (a + b + c == 2020)
                        {
                            return a * b * c;
                        }
                    }
                }
            }

            throw new InvalidOperationException();
        }
    }
}
