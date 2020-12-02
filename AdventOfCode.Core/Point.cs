using System;

namespace AdventOfCode
{
    public struct Point
    {
        public int X;
        public int Y;

        public Point(int x, int y)
        {
            this.X = x;
            this.Y = y;
        }

        public static readonly Point Origin = new Point();

        public int ManhattanDistance => Math.Abs(X) + Math.Abs(Y);

        public static Point Right(int magnitude) => new Point(magnitude, 0);
        public static Point Up(int magnitude) => new Point(0, magnitude);
        public static Point Left(int magnitude) => new Point(-magnitude, 0);
        public static Point Down(int magnitude) => new Point(0, -magnitude);

        public static Point operator +(Point a, Point b) => new Point(a.X + b.X, a.Y + b.Y);

        public static Point operator -(Point a, Point b) => new Point(a.X - b.X, a.Y - b.Y);

        public static bool operator ==(Point a, Point b) => a.X == b.X && a.Y == b.Y;
        public static bool operator !=(Point a, Point b) => a.X != b.X || a.Y != b.Y;

        public override int GetHashCode() => HashCode.Combine(this.X, this.Y);

        public override bool Equals(object obj)
        {
            if (obj == null || !(obj is Point p))
            {
                return false;
            }

            return this == p;
        }

        public override string ToString() => $"({X}, {Y})";
    }
}