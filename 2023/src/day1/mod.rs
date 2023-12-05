use std::sync::OnceLock;

use log::debug;

use crate::advent_day;

advent_day!(Day1, parse, Vec<&str>, part1, part2);

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

/// ```rust
/// use advent_of_code::day1::*;
/// let input = parse(
/// r"1abc2
/// pqr3stu8vwx
/// a1b2c3d4e5f
/// treb7uchet");
/// assert_eq!(142, part1(&input));
/// ```
pub fn part1(input: &Vec<&str>) -> u32 {
    let mut total = 0;
    for line in input {
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));
        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);

        debug!("{} => {}{} | {}", line, first, last, total);
        total += first * 10 + last;
    }

    total
}

/// ```rust
/// use advent_of_code::day1::*;
/// let input = parse(
/// r"two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen");
/// assert_eq!(281, part2(&input));
/// ```
pub fn part2(input: &Vec<&str>) -> u32 {
    static NUMBER_PARSER: OnceLock<regex::Regex> = OnceLock::new();
    let mut total = 0;

    let number_parser = NUMBER_PARSER.get_or_init(|| {
        regex::Regex::new(r"\d|one|t(?:wo|hree)|f(?:our|ive)|s(?:ix|even)|eight|nine").unwrap()
    });

    fn parse_digit(digit: &str) -> u32 {
        match digit {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => digit.chars().next().unwrap().to_digit(10).unwrap(),
        }
    }

    for line in input {
        debug!("{}", line);
        let mut captures = number_parser.captures_iter(line);
        let first = parse_digit(captures.next().unwrap().get(0).unwrap().as_str());
        let last = captures
            .last()
            .map_or(first, |cap| parse_digit(cap.get(0).unwrap().as_str()));

        debug!("{}{}", first, last);
        total += (first * 10) + last;
    }

    total
}

#[cfg(test)]
mod unittests {
    use crate::Day1;

    #[test]
    fn part2_overlap_case() {
        let input = r"two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
            eighthree";
        assert_eq!(281 + 83, Day1::part2(&Day1::parse(input)));
    }
}
