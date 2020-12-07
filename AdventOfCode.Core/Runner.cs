using AdventOfCode;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace AdventOfCode
{
    public class Runner
    {
        private object ConsoleSync = new object();

        public async Task Run(string[] args)
        {
            Console.ResetColor();
            Console.Clear();
            var namespaceParser = new Regex(@"Day(?<DayId>\d+)");
            var discoveredDays =
                from exportedType in this.GetType().Assembly.GetExportedTypes()
                where exportedType.GetInterfaces().Any(@interface => @interface == typeof(IDay)) && !exportedType.IsAbstract
                let dayId = int.Parse(namespaceParser.Match(exportedType.Namespace).Groups["DayId"].Value)
                group exportedType by dayId;

            await Task.WhenAll(discoveredDays.Select(o =>
                RunDay(o.Key, o)));

            // 2nd run to eliminate first-run JIT slowdowns issues.
            var timer = Stopwatch.StartNew();
            await Task.WhenAll(discoveredDays.Select(o =>
                RunDay(o.Key, o)));
            timer.Stop();

            lock (ConsoleSync)
            {
                Console.SetCursorPosition(0, (discoveredDays.Count() * 2) + 1);
                Console.WriteLine($"Total execution time: {timer.Elapsed}");
            }
        }

        private async Task RunDay(int dayNumber, IEnumerable<Type> implementationTypes)
        {
            string dayId = $"Day{dayNumber:00}";
            //lock (ConsoleSync)
            //{
            //    Console.SetCursorPosition(0, dayNumber);
            //    Console.Write(dayId);
            //}

            var input = File.ReadAllText($"Calendar/{dayId}/Input.txt");

            var implementations = implementationTypes.OrderBy(o => o.Name).Select(o =>
              {
                  var day = (IDay)Activator.CreateInstance(o);
                  day.Parse(input);
                  return day;
              });

            await Task.WhenAll(
                RunPart(dayId + "A", implementations, o => o.PartA(), (dayNumber - 1) * 2),
                RunPart(dayId + "B", implementations, o => o.PartB(), ((dayNumber - 1) * 2) + 1));
            //foreach (var implementation in implementations)
            //{
            //    var day = (IDay)Activator.CreateInstance(implementation);
            //    day.Parse(input);

            //}
        }

        private async Task RunPart(string dayId, IEnumerable<IDay> implementations, Func<IDay, object> part, int top)
        {
            lock (ConsoleSync)
            {
                Console.SetCursorPosition(0, top);
                Console.Write(dayId);
                Console.Write(' ');
            }

            var pending = implementations.Select(o => RunPart(() => part(o), o.GetType().Name)).ToHashSet();

            var completed = new SortedSet<(string Result, TimeSpan Duration, string Name)>(
                Comparer<(string Result, TimeSpan Duration, string Name)>.Create(
                    (x, y) => x.Name.CompareTo(y.Name)));

            while (pending.Any())
            {
                var complete = await Task.WhenAny(pending);
                pending.Remove(complete);
                completed.Add(await complete);

                lock (ConsoleSync)
                {
                    int partId = 0;
                    (string Result, TimeSpan Duration, string Name) fastest;

                    if (completed.Any(o => o.Result != null))
                    {
                        fastest = completed.Where(o => o.Result != null).MinBy(o => o.Duration).First();
                    }
                    else
                    {
                        fastest = default;
                    }

                    foreach (var implementation in completed)
                    {
                        Console.SetCursorPosition(7 + (partId * 9), top);
                        if (implementation.Result == null)
                        {
                            //Failure
                            Console.BackgroundColor = ConsoleColor.DarkRed;
                        }
                        else if (implementation == fastest)
                        {
                            Console.BackgroundColor = ConsoleColor.DarkYellow;
                            Console.ForegroundColor = ConsoleColor.Green;
                        }
                        else
                        {
                            Console.ForegroundColor = ConsoleColor.Green;
                        }

                        Console.Write(ClampLength(implementation.Name, 8));
                        Console.ResetColor();
                        Console.Write(' ');

                        partId++;
                    }

                    if (!completed.All(o => o.Result == fastest.Result || o.Result == null))
                    {
                        Console.ForegroundColor = ConsoleColor.Red;
                        //Console.SetCursorPosition(4, top);
                        Console.Write($"ERROR!!");
                        Console.ResetColor();
                    }
                    else if (fastest != default)
                    {
                        //Console.SetCursorPosition(6, top);
                        Console.Write($"[{fastest.Duration.TotalMilliseconds:0.0000}] {fastest.Result}    ");
                    }
                }
            }
        }

        private string ClampLength(string text, int length)
        {
            var offset = length - text.Length;

            if(offset > 0)  
            {
                return text + new string(' ', offset);
            }
            else if(offset < 0)
            {
                return text.Substring(0, length);
            }
            else
            {
                return text;
            }
        }

        private Task<(string Result, TimeSpan Duration, string Name)> RunPart(
            Func<object> part,
            string name) =>
            Task.Factory.StartNew(() =>
        {
            var timer = Stopwatch.StartNew();
            string result;

            try
            {
                result = part().ToString();
            }
            catch(Exception e)
            {
                result = null;
            }
            timer.Stop();
            return (result, timer.Elapsed, name);
        });
    }
}
