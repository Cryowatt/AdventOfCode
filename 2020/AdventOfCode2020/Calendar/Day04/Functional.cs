using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode.Calendar.Day04
{
    public class Functional : SolutionBase<IEnumerable<Dictionary<string, string>>>
    {
        private HashSet<string> RequiredFields = new HashSet<string>
        {
            "byr",
            "iyr",
            "eyr",
            "hgt",
            "hcl",
            "ecl",
            "pid",
            //"cid", // Hackerman
        };

        private static bool ValidateInt(string str, int min, int max)
        {
            var v = int.Parse(str);
            return min <= v && v <= max;
        }

        private static Regex heightPattern = new Regex(@"(?<Scalar>\d+?)(?<Unit>cm|in)");

        private static bool ValidateHeight(string str)
        {
            var match = heightPattern.Match(str);

            if (!match.Success)
            {
                return false;
            }

            var scalar = int.Parse(match.Groups["Scalar"].Value);
            var unit = match.Groups["Unit"].Value;

            return unit switch
            {
                "cm" => 150 <= scalar && scalar <= 193,
                "in" => 59 <= scalar && scalar <= 76,
                _ => throw new FormatException(),
            };
        }

        private static bool ValidatePattern(string str, string pattern) => Regex.IsMatch(str, pattern);

        public static IReadOnlyDictionary<string, Func<string, bool>> FieldValidators = new Dictionary<string, Func<string, bool>>
        {
            // byr (Birth Year) - four digits; at least 1920 and at most 2002.
            ["byr"] = o => ValidateInt(o, 1920, 2002),

            // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
            ["iyr"] = o => ValidateInt(o, 2010, 2020),

            // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
            ["eyr"] = o => ValidateInt(o, 2020, 2030),

            // hgt (Height) - a number followed by either cm or in:
            // If cm, the number must be at least 150 and at most 193.
            // If in, the number must be at least 59 and at most 76.
            ["hgt"] = ValidateHeight,

            // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            ["hcl"] = o => ValidatePattern(o, "^#[0-9a-f]{6}$"),

            // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            ["ecl"] = o => ValidatePattern(o, "^(amb|blu|brn|gry|grn|hzl|oth)$"),

            // pid (Passport ID) - a nine-digit number, including leading zeroes.
            ["pid"] = o => ValidatePattern(o, @"^[0-9]{9}$"),

            // cid (Country ID) - ignored, missing or not.
            // ["cid"] = o => true,
        };

        private static IEnumerable<Dictionary<string, string>> Parser(string input)
        {
            using (var reader = new StringReader(input))
            {
                string line;

                do
                {
                    var passport = new Dictionary<string, string>();
                    while (!string.IsNullOrEmpty(line = reader.ReadLine()))
                    {
                        foreach (var fields in line.Split(' '))
                        {
                            var tokens = fields.Split(':');
                            if (!passport.TryAdd(tokens[0], tokens[1]))
                            {
                                throw new FormatException("Duplicate field found");
                            }
                        }
                    }

                    yield return passport;
                }
                while (line != null);
            }
        }

        public Functional() : base(Parser) { }

        public override object PartA() =>
            this.input.Count(passport => RequiredFields.All(key => passport.ContainsKey(key)));

        public override object PartB() =>
            this.input.Count(passport => RequiredFields.All(key => passport.TryGetValue(key, out string v) && FieldValidators[key](v)));
    }
}
