using System.IO;
using Xunit;

namespace AdventOfCode2017.Test
{
    public abstract class DayTest<TDay> where TDay : IAdventDay, new()
    {
        protected TDay Day;

        public DayTest()
        {
            this.Day = new TDay();
        }

        protected string RunPart1(string input) => Day.RunPart1(input);

        protected string RunPart2(string input) => Day.RunPart2(input);

        [Trait("Performance", "Part1")]
        [Fact]
        public void Part1Perf()
        {
            var input = File.ReadAllText($"Input/{this.Day.GetType().Name}.txt");
            RunPart1(input);
        }

        [Trait("Performance", "Part2")]
        [Fact]
        public void Part2Perf()
        {
            var input = File.ReadAllText($"Input/{this.Day.GetType().Name}.txt");
            RunPart2(input);
        }
    }
}