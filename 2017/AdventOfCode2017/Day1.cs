using System.Threading.Tasks;
using System.Linq;

namespace AdventOfCode2017
{
    public class Day1 : IAdventDay
    {
        public string RunPart1(string input) => (input.Zip(input.Append(input.First()).Skip(1), (a, b) => (a == b) ? a - '0' : 0).Sum()).ToString();

        public string RunPart2(string input) => (input.Skip(input.Length / 2).Zip(input.Take(input.Length / 2), (a, b) => (a == b) ? a - '0' : 0).Sum() * 2).ToString();
    }
}
