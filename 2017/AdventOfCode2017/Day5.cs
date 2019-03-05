using System.Collections.Immutable;
using System.Linq;

namespace AdventOfCode2017
{
    public class Day5 : AdventDay<int[]>
    {
        protected override int[] TransformInput(string input)
        {
            return input.EnumerateLines().AsInt().ToArray();
        }
        public override string RunPart1(int[] input)
        {
            int count = 0;

            for (int ip = 0; 0 <= ip && ip < input.Length; count++)
            {
                int jump = input[ip];
                input[ip]++;
                ip += jump;
            }

            return count.ToString();
        }

        public override unsafe string RunPart2(int[] input)
        {
            fixed (int* _input = input)
            {
                int* pMax = _input + input.Length;
                int* pinput = _input;

                int count = 0;

                for (; _input <= pinput && pinput < pMax; count++)
                {
                    int jump = *pinput;

                    if (jump >= 3)
                    {
                        (*pinput) = jump - 1;
                    }
                    else
                    {
                        (*pinput) = jump + 1;
                    }

                    pinput += jump;
                }

                return count.ToString();
            }
        }
    }
}
