using AdventOfCode.CSharp;
using AdventOfCode.FSharp;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;

namespace AdventOfCode.Runner
{
    class Program
    {
        static void Main(string[] args)
        {
            RunAll();
        }

        static void RunAll()
        {

            for (int i = 1; i <= 25; i++)
            {
                Benchmark(i, 'A', (day, input) => day?.RunPartA(input));
                Benchmark(i, 'B', (day, input) => day?.RunPartB(input));
            }
        }

        private static void Benchmark(int dayNumber, char part, Func<IDay, IEnumerable<string>, string> action)
        {
            IDayFactory csharp = new CSharpDayFactory();
            IDayFactory fsharp = new FSharpDayFactory();
            var cday = csharp.GetDay(dayNumber);
            var fday = fsharp.GetDay(dayNumber);

            if (cday == null && fday == null)
            {
                Console.WriteLine($"Day{dayNumber} not completed.");
                return;
            }

            var input = File.ReadLines($"Content/Day{dayNumber}.txt").Memoize();
            var timer = Stopwatch.StartNew();
            var canswer = action(cday, input);
            var csharpTime = timer.Elapsed.TotalSeconds;
            timer = Stopwatch.StartNew();
            var fanswer = action(fday, input);
            var fsharpTime = timer.Elapsed.TotalSeconds;

            Console.Write($"Day {dayNumber}:{part}\t[{csharpTime}] [{fsharpTime - csharpTime}]\t[{canswer}]");
            if (canswer != fanswer)
            {
                Console.Write($"\tANSWER MISMATCH! {canswer} != {fanswer}");
            }
            Console.WriteLine();
        }
    }
}
