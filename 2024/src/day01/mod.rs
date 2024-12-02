use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    sync::OnceLock,
};

use advent::*;
use log::debug;

advent_day!(Day01, parse, Vec<(i32, i32)>, part1, part2);

pub fn parse(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|line| {
            let mut tokens = line.split_whitespace();
            (
                tokens.next().unwrap().parse::<i32>().unwrap(),
                tokens.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect()
}

/// ```rust
/// use advent_of_code_2024::day01::*;
/// let input = parse(
/// r"3   4
/// 4   3
/// 2   5
/// 1   3
/// 3   9
/// 3   3");
/// assert_eq!(11, part1(&input));
/// ```
pub fn part1(input: &Vec<(i32, i32)>) -> i32 {
    let mut left_heap = BinaryHeap::with_capacity(1000);
    let mut right_heap = BinaryHeap::with_capacity(1000);

    for item in input {
        left_heap.push(Reverse(item.0));
        right_heap.push(Reverse(item.1));
    }

    let items = left_heap.len();

    (0..items)
        .map(|_| (left_heap.pop().unwrap().0 - right_heap.pop().unwrap().0).abs())
        .sum()
    // for _ in 0..items {
    //     let a = left_heap.pop().unwrap();
    //     let b = right_heap.pop().unwrap();
    //     println!(
    //         "{:?} {:?}",
    //         left_heap.pop().unwrap(),
    //         right_heap.pop().unwrap()
    //     )
    // }

    //     let left_sorted = input.iter().map(|entry| entry.0);
    //    left_sorted.

    // let mut total = 0;

    // for line in input {
    //     let mut digits = line.chars().filter_map(|c| c.to_digit(10));
    //     let first = digits.next().unwrap();
    //     let last = digits.last().unwrap_or(first);

    //     debug!("{} => {}{} | {}", line, first, last, total);
    //     total += first * 10 + last;
    // }

    // total
}

/// ```rust
/// use advent_of_code_2024::day01::*;
/// let input = parse(
/// r"3   4
/// 4   3
/// 2   5
/// 1   3
/// 3   9
/// 3   3");
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &Vec<(i32, i32)>) -> i32 {
    0
    // const DIGITS: [(&str, u32); 18] = [
    //     ("1", 1),
    //     ("2", 2),
    //     ("3", 3),
    //     ("4", 4),
    //     ("5", 5),
    //     ("6", 6),
    //     ("7", 7),
    //     ("8", 8),
    //     ("9", 9),
    //     ("eight", 8),
    //     ("five", 5),
    //     ("four", 4),
    //     ("nine", 9),
    //     ("one", 1),
    //     ("seven", 7),
    //     ("six", 6),
    //     ("three", 3),
    //     ("two", 2),
    // ];

    // fn digit_search(segment: &str) -> Option<u32> {
    //     DIGITS
    //         .binary_search_by(|item| {
    //             if segment.starts_with(item.0) {
    //                 Ordering::Equal
    //             } else {
    //                 item.0.cmp(segment)
    //             }
    //         })
    //         .map_or(None, |index| Some(DIGITS.get(index).unwrap().1))
    // }

    // input
    //     .iter()
    //     .map(|line| {
    //         let first = (0..line.len())
    //             .filter_map(|i| digit_search(&line[i..]))
    //             .nth(0)
    //             .unwrap();

    //         let last = (0..line.len())
    //             .rev()
    //             .filter_map(|i| digit_search(&line[i..]))
    //             .nth(0)
    //             .unwrap();

    //         (first * 10) + last
    //     })
    //     .sum()
}

#[cfg(test)]
mod unittests {
    // #[test]
    // fn part2_overlap_case() {
    //     let input = r"two1nine
    //         eightwothree
    //         abcone2threexyz
    //         xtwone3four
    //         4nineeightseven2
    //         zoneight234
    //         7pqrstsixteen
    //         eighthree";
    //     assert_eq!(281 + 83, super::part2(&super::parse(input)));
    // }
}
