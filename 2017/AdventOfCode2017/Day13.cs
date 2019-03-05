using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Numerics;

namespace AdventOfCode2017
{
    public class Day13 : AdventDay<IReadOnlyDictionary<int, int>>
    {
        protected override IReadOnlyDictionary<int, int> TransformInput(string input) =>
            (from line in input.EnumerateLines()
             let tokens = line.Split(':', ' ', StringSplitOptions.RemoveEmptyEntries)
             let index = int.Parse(tokens[0])
             let range = int.Parse(tokens[1])
             select (Index: index, Range: range)).ToImmutableDictionary(o => o.Index, o => o.Range);

        public override string RunPart1(IReadOnlyDictionary<int, int> input) =>
            (from position in Enumerable.Range(0, input.Keys.Max() + 1)
             let range = input.GetValueOrDefault(position)
             let severity = (range > 0 && position % ((range - 1) * 2) == 0) ? range * position : 0
             select severity).Sum().ToString();

        public override string RunPart2(IReadOnlyDictionary<int, int> input)
        {
            if (Vector.IsHardwareAccelerated)
            {
                Console.WriteLine("WARP SPEED!");
            }
            else
            {
                Console.WriteLine("This is gonna be slow...");
            }

            var segments =
                (from buffer in input.Buffer(Vector<byte>.Count)
                 let fillerCount = Vector<byte>.Count - buffer.Count
                 let filler = Enumerable.Repeat((byte)0, fillerCount)
                 select (
                    Ranges: new Vector<byte>(buffer.Select(o => (byte)((o.Value - 1) * 2)).Concat(Enumerable.Repeat((byte)1, fillerCount)).ToArray()),
                    Offset: new Vector<byte>(buffer.Select(o => (byte)o.Key).Concat(filler).ToArray()),
                    State: new Vector<byte>(buffer.Select(o => (byte)(o.Key % ((o.Value - 1) * 2))).Concat(filler).ToArray()),
                    Mask: new Vector<byte>(buffer.Select(o => byte.MaxValue).Concat(filler).ToArray())));

            var state = segments.Select(o => o.State).ToArray();
            var offset = segments.Select(o => o.Offset).ToArray();
            var ranges = segments.Select(o => o.Ranges).ToArray();
            var mask = segments.Select(o => o.Mask).ToArray();

            for (int delay = 1; delay < int.MaxValue; delay++)
            {
                for (int segment = 0; segment < state.Length; segment++)
                {
                    var stateIncrement = (state[segment] + Vector<byte>.One);
                    state[segment] = Vector.ConditionalSelect(Vector.LessThan<byte>(stateIncrement, ranges[segment]), stateIncrement, Vector<byte>.Zero);
                }

                bool canEscape = true;

                for (int segment = 0; segment < state.Length; segment++)
                {
                    if (Vector.EqualsAny(state[segment] & mask[segment], ~mask[segment]))
                    {
                        canEscape = false;
                        break;
                    }
                }

                if (canEscape)
                {
                    return delay.ToString();
                }
            }

            throw new InvalidOperationException();
        }
    }
}