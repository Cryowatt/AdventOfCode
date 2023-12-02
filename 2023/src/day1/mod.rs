use std::sync::OnceLock;

use log::debug;
use onig::Regex;

use crate::advent_day;

advent_day!(Day1, part1, part2);

/// ```rust
/// let input = r"1abc2
/// pqr3stu8vwx
/// a1b2c3d4e5f
/// treb7uchet";
/// assert_eq!(142, advent_of_code::Day1::part1(input));
/// ```
fn part1(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));
        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);

        debug!("{} => {}{} | {}", line, first, last, total);
        total += first * 10 + last;
    }

    total
}

/// ```rust
/// let input = r"two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen";
/// assert_eq!(281, advent_of_code::Day1::part2(input));
/// ```
fn part2(input: &str) -> u32 {
    static NUMBER_PARSER: OnceLock<Regex> = OnceLock::new();
    let mut total = 0;

    let number_parser = NUMBER_PARSER.get_or_init(|| {
        Regex::new(
            r"\d|on(?=e)|tw(?=o)|thre(?=e)|four|fiv(?=e)|six|seve(?=n)|eigh(?=t)|nin(?=e)",
        )
        .unwrap()
    });
    for line in input.lines() {
        debug!("{}", line);
        let mut digits = number_parser.captures_iter(line).map(|c| {
            let number = c.at(0).unwrap();
            debug!("number: {}", number);

            match number {
                "on" => 1,
                "tw" => 2,
                "thre" => 3,
                "four" => 4,
                "fiv" => 5,
                "six" => 6,
                "seve" => 7,
                "eigh" => 8,
                "nin" => 9,
                _ => number.chars().next().unwrap().to_digit(10).unwrap(),
            }
        });

        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);

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
        assert_eq!(281 + 83, Day1::part2(input));
    }
}
