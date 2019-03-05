using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using System.Linq;
using System.Linq.Expressions;
using System.Numerics;
using System.Reactive.Linq;
using System.Reactive.Subjects;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace AdventOfCode2017
{
    public class Day19 : IAdventDay
    {
        [DebuggerDisplay("({X}, {Y})")]
        public struct Point
        {
            public Point(int x, int y)
            {
                this.X = x;
                this.Y = y;
            }

            public int X { get; }
            public int Y { get; }

            public Point Reflect() => new Point(this.Y, this.X);
            public static Point operator +(Point a, Point b) => new Point(a.X + b.X, a.Y + b.Y);
            public static Point operator -(Point a) => new Point(-a.X, -a.Y);
        }

        public string RunPart1(string input)
        {
            char[][] map = input.EnumerateLines().Select(o => o.ToCharArray()).ToArray();
            return EnumerateMap(map).Letters;
        }

        private (string Letters, int Steps) EnumerateMap(char[][] map)
        {
            int steps = 0;
            var builder = new StringBuilder();
            int entryPoint = Array.IndexOf(map[0], '|');
            var position = new Point(entryPoint, 0);
            var direction = new Point(0, 1);

            while (true)
            {
                char c = map[position.Y][position.X];

                //endgame
                if (c == ' ')
                {
                    break;
                }

                switch (c)
                {
                    case '#':
                    case '-':
                    case '|':
                        break;
                    case '+':
                        direction = direction.Reflect();
                        var newPosition = position + direction;

                        if (map[newPosition.Y][newPosition.X] == ' ')
                        {
                            direction = -direction;
                        }
                        break;
                    default:
                        builder.Append(c);
                        break;
                }

                map[position.Y][position.X] = '#';
                position += direction;
                steps++;
            }

            return (builder.ToString(), steps);
        }

        public string RunPart2(string input)
        {
            char[][] map = input.EnumerateLines().Select(o => o.ToCharArray()).ToArray();
            return EnumerateMap(map).Steps.ToString();
        }
    }
}