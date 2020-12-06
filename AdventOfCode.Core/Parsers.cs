using System;
using System.Collections.Generic;
using System.Diagnostics.Contracts;
using System.Linq;
using System.Numerics;
using System.Text;

namespace AdventOfCode
{
    public static class Parsers
    {
        public static Func<string, IEnumerable<string>> Lines() =>
            input => input.Split(new[] { "\r", "\n", "\r\n" }, StringSplitOptions.RemoveEmptyEntries);

        public static Func<string, IEnumerable<T>> Lines<T>(
            Func<string, T> elementParser) =>
                input => input.Split(new[] { "\r", "\n", "\r\n" }, StringSplitOptions.RemoveEmptyEntries).Select(elementParser);

        public static Func<string, IEnumerable<string>> Array() =>
            input => input.Split(",");

        public static Func<string, IEnumerable<T>> Array<T>(
            Func<string, T> elementParser) =>
                input => input.Split(",").Select(elementParser);

        public static Func<string, IEnumerable<Point>> Direction(this Func<string, IEnumerable<string>> parser) =>
            input => parser(input).Select(item =>
            {
                int magnitude = int.Parse(item.Substring(1));

                return item[0] switch
                {
                    'R' => Point.Right(magnitude),
                    'U' => Point.Up(magnitude),
                    'L' => Point.Left(magnitude),
                    'D' => Point.Down(magnitude),
                    _ => throw new FormatException()
                };
            });

        public static Func<string, IEnumerable<OrthoLine>> ContinuousLine(this Func<string, IEnumerable<Point>> parser) =>
            input => parser(input).Scan(
                new OrthoLine(0, 0, 0, true),
                (line, direction) => OrthoLine.From(line.EndPoint, line.EndPoint + direction));

        public static Func<string, IList<TElement>> ToList<TElement>(this Func<string, IEnumerable<TElement>> parser) =>
            input => parser(input).ToList();

        public static Func<string, TElement[]> ToArray<TElement>(this Func<string, IEnumerable<TElement>> parser) =>
            input => parser(input).ToArray();

        public static Func<string, IEnumerable<int>> Int(this Func<string, IEnumerable<string>> parser) =>
            input => parser(input).Select(int.Parse);

        public static Func<string, ReadOnlyMemory<T>> Memory<T>(this Func<string, IEnumerable<T>> parser, bool vectorPadding = false)
             where T : struct
        {
            if (vectorPadding)
            {
                return input =>
                {
                    var array = parser(input).ToArray();
                    var padding = Vector<T>.Count - (array.Length % Vector<T>.Count);
                    var paddedArray = new T[array.Length + padding];
                    array.CopyTo(paddedArray, 0);
                    return new ReadOnlyMemory<T>(paddedArray);
                };
            }
            else
            {
                return input => new ReadOnlyMemory<T>(parser(input).ToArray());
            }
        }


        public static Func<string, ReadOnlyMemory<char>> Raw() =>
            input => new ReadOnlyMemory<char>(input.ToCharArray());
    }
}
