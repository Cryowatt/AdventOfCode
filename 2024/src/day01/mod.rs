use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use advent::*;

advent_day!(Day01, parse, Vec<(i32, i32)>, part1, part2);

pub fn parse(input: &str) -> InputType<'_> {
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
pub fn part1(input: &InputType) -> i32 {
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
/// assert_eq!(31, part2(&input));
/// ```
pub fn part2(input: &InputType) -> i32 {
    let mut counts = HashMap::new();
    for key in input.iter().map(|item| item.1) {
        if let Err(mut entry) = counts.try_insert(key, 1) {
            *entry.entry.get_mut() += 1;
        }
    }

    input
        .iter()
        .map(|item| counts.get(&item.0).unwrap_or(&0) * item.0)
        .sum()
}
