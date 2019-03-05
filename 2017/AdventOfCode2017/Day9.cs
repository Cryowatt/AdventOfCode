using System.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode2017
{
    public class Day9 : IAdventDay
    {
        //gug dec 188 if zpw >= 8
        public string RunPart1(string input) =>
            Regex.Replace(input, @"(<((!.)|[^>])*>)|,", string.Empty)
                .Scan(
                    (Depth: 0, IsOpenGroup: false),
                    (a, c) => ((c == '{') ? a.Depth + 1 : a.Depth - 1, c == '{'))
                .Where(o => o.IsOpenGroup).Sum(o => o.Depth).ToString();

        public string RunPart2(string input) =>
            (from Match match in Regex.Matches(input, @"<((?:(?:!.)|[^>])*)>")
             let dropIgnored = Regex.Replace(match.Groups[1].Value, "(!.)", string.Empty)
             select dropIgnored.Length).Sum().ToString();
    }
}