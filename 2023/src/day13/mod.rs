use std::{cmp::Ordering, sync::OnceLock};

use advent::*;
use log::debug;

advent_day!(Day13, parse, Vec<Vec<Vec<u8>>>, part1, part2);
advent_bench!(parse, cursed_regex, part2_regex);

pub fn parse(input: &str) -> Vec<Vec<Vec<u8>>> {
    let mut pattern_list = vec![];
    let mut pattern = vec![];
    for line in input.lines() {
        if line.len() == 0 {
            pattern_list.push(pattern);
            pattern = vec![];
        } else {
            pattern.push(line.as_bytes().to_vec())
        }
    }

    pattern_list.push(pattern);
    pattern_list
}

/// ```rust
/// use advent_of_code_2023::day13::*;
/// let input = parse(
/// "#.##..##.
/// ..#.##.#.
/// \x23#......#
/// \x23#......#
/// ..#.##.#.
/// ..##..##.
/// \x23.#.##.#.
///
/// \x23...##..#
/// \x23....#..#
/// ..##..###
/// \x23####.##.
/// \x23####.##.
/// ..##..###
/// \x23....#..#");
/// assert_eq!(405, part1(&input));
/// ```
pub fn part1(input: &Vec<Vec<Vec<u8>>>) -> u32 {
    fn vertical_mirror(pattern: &Vec<Vec<u8>>) -> Option<u32> {
        let width = pattern[0].len();

        let confirm_mirror = |index| {
            pattern.iter().all(|row| {
                (0..index)
                    .rev()
                    .zip(index..width)
                    .all(|(low_index, high_index)| row[low_index] == row[high_index])
            })
        };

        for i in 1..width {
            if confirm_mirror(i) {
                return Some(i as u32);
            }
        }
        None
    }

    fn horizontal_mirror(pattern: &Vec<Vec<u8>>) -> Option<u32> {
        let height = pattern.len();

        let confirm_mirror = |index| {
            (0..index)
                .rev()
                .zip(index..height)
                .all(|(low_index, high_index)| pattern[low_index] == pattern[high_index])
        };

        for i in 1..height {
            if confirm_mirror(i) {
                return Some(i as u32);
            }
        }
        None
    }

    input
        .iter()
        .map(|pattern| {
            vertical_mirror(pattern).unwrap_or_else(|| horizontal_mirror(pattern).unwrap() * 100)
        })
        .sum()
}

/// ```rust
/// use advent_of_code_2023::day13::*;
/// let input = parse(
/// r"two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen");
/// //assert_eq!(281, part2(&input));
/// ```
pub fn part2(input: &Vec<Vec<Vec<u8>>>) -> u32 {
    0
}
