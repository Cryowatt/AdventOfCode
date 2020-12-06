using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode.Calendar.Day02
{
    public class Linq : SolutionBase<IEnumerable<(int Min, int Max, char Letter, string Password)>>
    {
        private static Regex parser = new(@"(?<Min>\d+?)\-(?<Max>\d+?) (?<Letter>[a-z]): (?<Password>[a-z]+)", RegexOptions.Compiled);

        private static IEnumerable<(int Min, int Max, char Letter, string Password)> ParseLine(string line) =>
            (from match in parser.Matches(line).OfType<Match>()
             select (
                 int.Parse(match.Groups["Min"].Value),
                 int.Parse(match.Groups["Max"].Value),
                 match.Groups["Letter"].Value.Single(),
                 match.Groups["Password"].Value
             )).ToArray();

        public Linq() : base(ParseLine) { }

        public override object PartA() =>
            this.input.Count(line =>
            {
                var check = line.Password.Count(c => c == line.Letter);
                return line.Min <= check && check <= line.Max;
            });

        public override object PartB() =>
            this.input.Count(line => line.Password[line.Min - 1] == line.Letter ^ line.Password[line.Max - 1] == line.Letter);
    }
}
