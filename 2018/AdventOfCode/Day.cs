using System.Collections.Generic;

namespace AdventOfCode
{
    public abstract class Day<T> : IDay
    {
        public abstract T ParseInput(IEnumerable<string> input);

        public string RunPartA(IEnumerable<string> input) => RunPartA(ParseInput(input));

        public string RunPartB(IEnumerable<string> input) => RunPartB(ParseInput(input));

        public abstract string RunPartA(T input);

        public abstract string RunPartB(T input);
    }
}
