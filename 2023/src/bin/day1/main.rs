#![feature(test)]

use std::{cell::OnceCell, sync::OnceLock};

use log::debug;
use onig::Regex;

fn main() {
    env_logger::init();
    let input = include_str!("input.txt");

    println!("== Day 1 ==");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

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

static NUMBER_MATCH: OnceLock<Regex> = OnceLock::new();

fn part2(input: &str) -> u32 {
    let mut total = 0;

    let number_match = NUMBER_MATCH.get_or_init(||
        Regex::new(r"\d|on(?=e)|tw(?=o)|thre(?=e)|four|fiv(?=e)|six|seve(?=n)|eigh(?=t)|nin(?=e)")
            .unwrap());
    for line in input.lines() {
        debug!("{}", line);
        let mut digits = number_match.captures_iter(line).map(|c| {
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
mod test {
    #[ctor::ctor]
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn part1_example_test() {
        let input = r"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!(142, super::part1(input));
    }

    #[test]
    fn part2_example_test() {
        let input = r"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!(281, super::part2(input));
    }

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
        assert_eq!(281 + 83, super::part2(input));
    }
}

#[cfg(test)]
mod bench {
    extern crate test;

    #[bench]
    fn part1_bench(b: &mut test::Bencher) {
        let input = include_str!("input.txt");
        b.iter(|| test::black_box(super::part1(input)));
    }

    #[bench]
    fn part2_bench(b: &mut test::Bencher) {
        let input = include_str!("input.txt");
        b.iter(|| test::black_box(super::part2(input)));
    }
}
