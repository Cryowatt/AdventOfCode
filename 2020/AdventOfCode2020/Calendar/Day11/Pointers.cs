using System;
using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode.Calendar.Day11
{
    public class Seat
    {
        public bool Occupied;
        public Seat[] Adjacent;
    }

    public unsafe struct SeatView
    {
        public int SeatOffset;
        public fixed int AdjacentOffsets[8];

        public bool ApplyRule(ReadOnlySpan<byte> seats, Span<byte> newSeats)
        {
            // If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
            // If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
            // Otherwise, the seat's state does not change.
            var currentSeat = seats[this.SeatOffset];

            if (currentSeat == 0)
            {
                int occupiedAdjacent = 0;

                for (int i = 0; i < 8; i++)
                {
                    occupiedAdjacent += seats[AdjacentOffsets[i]];
                }

                bool ruleTriggered = occupiedAdjacent == 0;
                newSeats[this.SeatOffset] = ruleTriggered ? 1 : 0;
                return ruleTriggered;
            }
            else
            {
                int occupiedAdjacent = 0;

                for (int i = 0; i < 8 || occupiedAdjacent < 4; i++)
                {
                    occupiedAdjacent += seats[AdjacentOffsets[i]];
                }

                bool ruleTriggered = occupiedAdjacent >= 4;
                newSeats[this.SeatOffset] = ruleTriggered ? 0 : 1;
                return ruleTriggered;
            }
        }
    }

    public unsafe class Pointers : SolutionBase<(int SeatCount, IEnumerable<SeatView> SeatingChart)>
    {
        public static (int SeatCount, IEnumerable<SeatView> SeatingChart) Parser(string[] lines)
        {
            int GetSafeOffset(int y, int x)
            {
                if (y < 0 || y > lines.Length || x < 0 || x > lines[0].Length)
                {
                    return -1;
                }

                return y * lines[0].Length + x;
            }

            List<SeatView> seatView = new List<SeatView>();
            for (int y = 0; y < lines.Length; y++)
            {
                var line = lines[y];

                for (int x = 0; x < line.Length; x++)
                {
                    if (line[x] == 'L')
                    {
                        var seat = new SeatView { SeatOffset = y * line.Length + x };
                        seat.AdjacentOffsets[0] = GetSafeOffset(y - 1, x - 1);
                        seat.AdjacentOffsets[1] = GetSafeOffset(y - 1, x);
                        seat.AdjacentOffsets[2] = GetSafeOffset(y - 1, x + 1);
                        seat.AdjacentOffsets[3] = GetSafeOffset(y, x - 1);
                        seat.AdjacentOffsets[4] = GetSafeOffset(y, x + 1);
                        seat.AdjacentOffsets[5] = GetSafeOffset(y + 1, x - 1);
                        seat.AdjacentOffsets[6] = GetSafeOffset(y + 1, x);
                        seat.AdjacentOffsets[7] = GetSafeOffset(y + 1, x + 1);
                        seatView.Add(seat);
                    }
                }
            }

            return (lines.Length * lines[0].Length, seatView);
        }

        public Pointers() : base(input => Parser(Parsers.Lines().ToArray()(input))) { }

        public override object PartA()
        {
            var seatStateSwap = new byte[2][];
            seatStateSwap[0] = new byte[this.input.SeatCount];
            seatStateSwap[1] = new byte[this.input.SeatCount];


        }

        public override object PartB()
        {
        }
    }
}