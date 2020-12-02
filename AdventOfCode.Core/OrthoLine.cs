using System;

namespace AdventOfCode
{
    public struct OrthoLine
    {
        public int Start;
        public int End;
        public int Offset;
        public bool IsHorizontal;

        public OrthoLine(int start, int end, int offset, bool isHorizontal)
        {
            this.Start = start;
            this.End = end;
            this.Offset = offset;
            this.IsHorizontal = isHorizontal;
        }

        public Point StartPoint => this.IsHorizontal ? new Point(this.Start, this.Offset) : new Point(this.Offset, this.Start);

        public Point EndPoint => this.IsHorizontal ? new Point(this.End, this.Offset) : new Point(this.Offset, this.End);

        public static OrthoLine From(Point start, Point end)
        {
            if (start.X == end.X)
            {
                return new OrthoLine(start.Y, end.Y, start.X, false);
            }
            else if (start.Y == end.Y)
            {
                return new OrthoLine(start.X, end.X, start.Y, true);
            }
            else
            {
                throw new ArgumentException();
            }
        }

        public bool IsVertical => !IsHorizontal;

        public int Length => Math.Abs(this.Start - this.End);

        private bool IsBetween(int start, int end, int value)
        {
            return (start <= value && value <= end) || (start >= value && value >= end);
        }

        public bool Intersects(OrthoLine line, out Point intersection)
        {
            if (((this.IsHorizontal && line.IsHorizontal) || (this.IsVertical && line.IsVertical)))
            {
                if (this.Offset == line.Offset)
                {
                    // Lines are coincidental horizontally
                    return this.IntersectsCoincidental(line, out intersection);
                }
                else
                {
                    intersection = Point.Origin;
                    return false;
                }
            }
            else if (IsBetween(this.Start, this.End, line.Offset) && IsBetween(line.Start, line.End, this.Offset))
            {
                // Lines intersect 
                if (this.IsHorizontal)
                {
                    intersection = new Point(line.Offset, this.Offset);
                }
                else
                {
                    intersection = new Point(this.Offset, line.Offset);
                }

                return true;
            }
            else
            {
                // Parallel lines can't intersect
                intersection = Point.Origin;
                return false;
            }
        }

        private bool IntersectsCoincidental(OrthoLine line, out Point intersection)
        {
            if ((this.Start <= line.Start && line.Start <= this.End) || (this.Start >= line.Start && line.Start >= this.End))
            {
                intersection = this.IsHorizontal ? new Point(line.Start, this.Offset) : new Point(this.Offset, line.Start);
                return true;
            }
            else if ((this.Start <= line.End && line.End <= this.End) || (this.Start >= line.End && line.End >= this.End))
            {
                intersection = this.IsHorizontal ? new Point(line.End, this.Offset) : new Point(this.Offset, line.End);
                return true;
            }
            else if ((line.Start <= this.Start && this.Start <= line.End) || (line.Start >= this.Start && this.Start >= line.End))
            {
                intersection = this.IsHorizontal ? new Point(this.Start, this.Offset) : new Point(this.Offset, this.Start);
                return true;
            }
            else if ((line.Start <= this.End && this.End <= line.End) || (line.Start >= this.End && this.End >= line.End))
            {
                intersection = this.IsHorizontal ? new Point(this.End, this.Offset) : new Point(this.Offset, this.End);
                return true;
            }
            else
            {
                intersection = Point.Origin;
                return false;
            }
        }
    }
}
