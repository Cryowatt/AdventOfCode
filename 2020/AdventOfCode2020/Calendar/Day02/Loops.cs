using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode.Calendar.Day02
{
    public class Loops : SolutionBase<IEnumerable<(int Min, int Max, char Letter, string Password)>>
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

        public Loops() : base(ParseLine) { }

        public override object PartA()
        {
            var validCount = 0;
            foreach(var line in this.input)
            {
                var check = 0;

                foreach(var c in line.Password)
                {
                    if(c == line.Letter)
                    {
                        check++;
                        if(check > line.Max)
                        {
                            break;
                        }
                    }
                }

                if(line.Min <= check && check <= line.Max)
                {
                    validCount++;
                }
            }

            return validCount.ToString();
        }

        public override object PartB()
        {
            var validCount = 0;
            foreach (var line in this.input)
            {
                if(line.Password[line.Min - 1] == line.Letter ^ line.Password[line.Max - 1] == line.Letter)
                {
                    validCount++;
                }
            }

            return validCount.ToString();
        }
    }
}
