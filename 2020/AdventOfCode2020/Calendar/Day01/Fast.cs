using System;

namespace AdventOfCode.Calendar.Day01
{
    public class Fast : SolutionBase<int[]>
    {
        public Fast() : base(Parsers.Lines().Int().ToArray()) { }

        public override object PartA()
        {
            var result = new int[1010];
            foreach (var item in this.input)
            {
                var index = (item > 1010) ? 2020 - item : item;

                if (result[index] > 0)
                {
                    return result[index] * item;
                }

                result[index] = item;
            }

            throw new InvalidOperationException();
        }

        public override object PartB()
        {
            var target = new int[2020];
            foreach (var item in this.input)
            {
                var index = 2020 - item;
                target[index] = item;
            }

            for (int i = 0; i < this.input.Length; i++)
            {
                var a = this.input[i];
                for (int j = i + 1; j < this.input.Length; j++)
                {
                    var b = this.input[j];
                    var index = a + b;
                    if (index > 2019)
                    {
                        continue;
                    }
                    var c = target[index];
                    if (c > 0)
                    {
                        return a * b * c;
                    }
                }
            }

            throw new InvalidOperationException();
        }
    }
}
