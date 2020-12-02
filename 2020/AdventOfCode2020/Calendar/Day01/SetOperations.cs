using AdventOfCode;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace AdventOfCode.Calendar.Day01
{
    public class SetOperations : SolutionBase<IEnumerable<int>>
    {
        public SetOperations() : base(Parsers.Lines().Int())
        {
        }

        public override object PartA()
        {
            var high = new HashSet<int>();
            var low = new HashSet<int>();

            foreach (var item in this.input)
            {
                if (item > 1010)
                {
                    high.Add(2020 - item);
                }
                else
                {
                    low.Add(item);
                }
            }

            low.IntersectWith(high);

            var a = low.First();

            return (2020 - a) * a;
        }

        public override object PartB()
        {
            throw new NotImplementedException();
        }
    }
}
