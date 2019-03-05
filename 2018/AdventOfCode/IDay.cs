using System.Collections.Generic;

namespace AdventOfCode
{
    public interface IDay
    {
        string RunPartA(IEnumerable<string> input);

        string RunPartB(IEnumerable<string> input);
    }
}
