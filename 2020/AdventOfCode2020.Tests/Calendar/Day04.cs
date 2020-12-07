using AdventOfCode.Test;
using System.Collections.Generic;
using Xunit;

namespace AdventOfCode.Calendar.Day04
{
    public class TestData : ITestData
    {
        public IEnumerable<(string Input, string Expected)> PartAData => new[]
        {
            (@"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in", "2"),
            (@"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm", "1"),
            (@"iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929", "0"),
            (@"hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm", "1"),
            (@"hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in", "0"),
        };

        public IEnumerable<(string Input, string Expected)> PartBData => new[]
        {
            (@"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007", "0"),
            (@"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719", "4"),
        };
    }

    public class FunctionalTest : DayTest<Functional, TestData>
    {
        // cid (Country ID) - ignored, missing or not.
        // ["cid"] = true,

        [Theory]
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        [InlineData("byr", "1919", false)]
        [InlineData("byr", "1920", true)]
        [InlineData("byr", "2002", true)]
        [InlineData("byr", "2003", false)]
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        [InlineData("iyr", "2009", false)]
        [InlineData("iyr", "2010", true)]
        [InlineData("iyr", "2020", true)]
        [InlineData("iyr", "2021", false)]
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        [InlineData("eyr", "2019", false)]
        [InlineData("eyr", "2020", true)]
        [InlineData("eyr", "2030", true)]
        [InlineData("eyr", "2031", false)]
        // hgt (Height) - a number followed by either cm or in:
        // If cm, the number must be at least 150 and at most 193.
        // If in, the number must be at least 59 and at most 76.
        [InlineData("hgt", "123", false)]
        [InlineData("hgt", "149cm", false)]
        [InlineData("hgt", "150cm", true)]
        [InlineData("hgt", "193cm", true)]
        [InlineData("hgt", "194cm", false)]
        [InlineData("hgt", "58in", false)]
        [InlineData("hgt", "59in", true)]
        [InlineData("hgt", "76in", true)]
        [InlineData("hgt", "7in", false)]
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        [InlineData("hcl", "#123456", true)]
        [InlineData("hcl", "#789abc", true)]
        [InlineData("hcl", "#dedbef", true)]
        [InlineData("hcl", "#123", false)]
        [InlineData("hcl", "#g12345", false)]
        [InlineData("hcl", "123456", false)]
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        [InlineData("ecl", "amb", true)]
        [InlineData("ecl", "blu", true)]
        [InlineData("ecl", "brn", true)]
        [InlineData("ecl", "gry", true)]
        [InlineData("ecl", "grn", true)]
        [InlineData("ecl", "hzl", true)]
        [InlineData("ecl", "oth", true)]
        [InlineData("ecl", "omg", false)]
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        [InlineData("pid", "000000000", true)]
        [InlineData("pid", "00000000", false)]
        public void ValidatorTest(string field, string input, bool expected)
        {
            Assert.Equal(expected, Functional.FieldValidators[field](input));
        }
    }

}
