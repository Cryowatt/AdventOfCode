using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode.Calendar.Day05
{
    public class Functional : SolutionBase<IEnumerable<string>>
    {
        public Functional() : base(Parsers.Lines()) { }

        private int Decode(string pass)
        {
            int seatId = 0;
            for (int i = 0; i < 10; i++)
            {
                seatId <<= 1;

                if (pass[i] == 'B' || pass[i] == 'R')
                {
                    seatId++;
                }
            }

            return seatId;
        }

        public override object PartA() =>
            (from pass in this.input
             select Decode(pass)).Max();


        public override object PartB()
        {
            var knownSeats =
                (from pass in this.input
                 let seatId = Decode(pass)
                 orderby seatId
                 select seatId).ToArray();

            for(int i = 0; i < knownSeats.Length; i++)
            {
                var seat = knownSeats[i];
                var nextSeat = knownSeats[i + 1];

                if (seat + 2 == nextSeat)
                {
                    return seat + 1;
                }
            }

            return "";
        }
    }
}
