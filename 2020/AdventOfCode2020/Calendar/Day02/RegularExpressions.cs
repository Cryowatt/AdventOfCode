using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace AdventOfCode.Calendar.Day02
{
    public class RegularExpressions : SolutionBase<IEnumerable<(int Min, int Max, char Letter, string Password)>>
    {
        private static Regex parser = new(@"(?<Min>\d+?)\-(?<Max>\d+?) (?<Letter>[a-z]): (?<Password>[a-z]+)", RegexOptions.Compiled);

        private static IEnumerable<(int Min, int Max, char Letter, string Password)> ParseLine(string line)
        {
            var matches = parser.Matches(line);

            foreach (var match in matches.OfType<Match>())
            {
                yield return new(
                    int.Parse(match.Groups["Min"].Value),
                    int.Parse(match.Groups["Max"].Value),
                    match.Groups["Letter"].Value.Single(),
                    match.Groups["Password"].Value
                );
            }
        }

        public RegularExpressions() : base(ParseLine) { }

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
