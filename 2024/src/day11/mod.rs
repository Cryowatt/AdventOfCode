use advent::*;
use bitflags::iter;
use rayon::prelude::*;

advent_day!(Day11, parse, Vec<u32>, part1, part2);

pub fn parse(input: &str) -> InputType {
    input
        .split_whitespace()
        .map(|token| token.parse().unwrap())
        .collect()
}

/// ```rust
/// use advent_of_code_2024::day11::*;
/// let input = parse(
/// r"125 17");
/// assert_eq!(55312, part1(&input));
/// ```
pub fn part1(input: &InputType) -> u32 {
    0
}

/// ```rust
/// use advent_of_code_2024::day11::*;
/// let input = parse(
/// r"125 17");
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &InputType) -> u32 {
    0
}
