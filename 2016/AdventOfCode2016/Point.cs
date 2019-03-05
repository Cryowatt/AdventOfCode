using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AdventOfCode
{
    public struct Point
    {
        public int X;
        public int Y;

        public static Point Zero { get { return new Point(); } }

        public Point Offset(Direction orientation, int offset)
        {
            if (IsHorizontal(orientation))
            {
                return new Point
                {
                    X = this.X + ((orientation == Direction.East) ? offset : -offset),
                    Y = this.Y
                };
            }
            else
            {
                return new Point
                {
                    X = this.X,
                    Y = this.Y + ((orientation == Direction.North) ? offset : -offset),
                };
            }
        }

        public IEnumerable<Point> Interpolate(Direction orientation, int offset)
        {
            for (int i = 1; i <= offset; i++)
            {
                yield return this.Offset(orientation, i);
            }
        }

        public IEnumerable<Point> Adjacent()
        {
            yield return this.Offset(Direction.North, 1);
            yield return this.Offset(Direction.East, 1);
            yield return this.Offset(Direction.South, 1);
            yield return this.Offset(Direction.West, 1);
        }

        private bool IsHorizontal(Direction orientation)
        {
            return (orientation == Direction.East || orientation == Direction.West);
        }

        public override string ToString()
        {
            return $"{{{this.X}, {this.Y}}}";
        }

        public static bool operator ==(Point left, Point right)
        {
            return left.X == right.X && left.Y == right.Y;
        }

        public static bool operator !=(Point left, Point right)
        {
            return left.X != right.X && left.Y != right.Y;
        }

        public override bool Equals(object obj)
        {
            if(obj is Point){
                return this == (Point)obj;
            }else
            {
                return false;
            }
        }

        public override int GetHashCode()
        {
            return this.X ^ (this.Y << 8);
        }
    }
}
