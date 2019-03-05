using Microsoft.Extensions.CommandLineUtils;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading.Tasks;

namespace AdventOfCode2017
{
    public static class Program
    {
        static int Main(string[] args)
        {
            CommandLineApplication application = new CommandLineApplication(throwOnUnexpectedArg: true);
            var daysToRun = application.Argument("day", "Which day to run", true);
            application.OnExecute(() => RunDays(daysToRun.Values.Select(int.Parse).ToArray()));
            var result = application.Execute(args);
            WaitForContinue();
            return result;
        }

        public static async Task<int> RunDays(int[] days)
        {
            var dayInterface = typeof(IAdventDay);
            IEnumerable<Type> dayTypes = 
                typeof(Program).Assembly.GetTypes()
                .Where(o => dayInterface.IsAssignableFrom(o) && o.IsClass && !o.IsAbstract).Memoize();

            if (days.Any())
            {
                dayTypes = days.Select(dayNumber => dayTypes.Single(type => type.Name == "Day" + dayNumber)).ToList();
            }

            // Just show the latest day by default
            foreach (var dayType in dayTypes)
            {
                IAdventDay day = (IAdventDay)Activator.CreateInstance(dayType);
                var input = await File.ReadAllTextAsync($"Input/{dayType.Name}.txt");

                try
                {
                    Console.WriteLine($"{dayType.Name} - Part 1");
                    var timer = Stopwatch.StartNew();
                    Console.WriteLine("{0}\n{1}", day.RunPart1(input), timer.Elapsed);
                    Console.WriteLine();
                }
                catch (NotImplementedException)
                {
                    Console.WriteLine("Not Completed!");
                }

                try
                {
                    Console.WriteLine($"{dayType.Name} - Part 2");
                    var timer = Stopwatch.StartNew();
                    Console.WriteLine("{0}\n{1}", day.RunPart2(input), timer.Elapsed);
                }
                catch (NotImplementedException)
                {
                    Console.WriteLine("Not Completed!");
                }

                Console.WriteLine();
            }

            return 0;
        }

        [Conditional("DEBUG")]
        public static void WaitForContinue()
        {
            Console.WriteLine();
            Console.WriteLine("Press a thing to get out of this shit.");
            Console.ReadLine();
        }
    }
}
