use std::iter;

use advent::*;

advent_day!(Day10, parse, Vec<Vec<u8>>, part1, part2);

pub fn parse(input: &str) -> InputType {
    input
        .lines()
        .map(|line| line.as_bytes().iter().cloned().collect())
        .collect()
}

/// ```rust
/// use advent_of_code_2024::day10::*;
/// let input = parse(
/// r"89010123
/// 78121874
/// 87430965
/// 96549874
/// 45678903
/// 32019012
/// 01329801
/// 10456732");
/// assert_eq!(36, part1(&input));
/// ```
pub fn part1(input: &InputType) -> i32 {
    0
}

/// ```rust
/// use advent_of_code_2024::day10::*;
/// let input = parse(
/// r"89010123
/// 78121874
/// 87430965
/// 96549874
/// 45678903
/// 32019012
/// 01329801
/// 10456732");
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &InputType) -> i32 {
    0
}
