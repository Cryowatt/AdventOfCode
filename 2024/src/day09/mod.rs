use std::iter;

use advent::*;

advent_day!(Day09, parse, Vec<u8>, part1, part2);

pub fn parse(input: &str) -> InputType {
    input.as_bytes().iter().map(|b| *b - b'0').collect()
}

/// ```rust
/// use advent_of_code_2024::day09::*;
/// let input = parse(
/// r"2333133121414131402");
/// assert_eq!(1928, part1(&input));
/// ```
pub fn part1(input: &InputType) -> usize {
    // let mut left = 0;
    // let mut right = input.len() - 1;

    // let total_files
    // let mut checksum = 0;

    // let left: Vec<usize> = input
    //     .iter()
    //     .enumerate()
    //     .flat_map(|(index, &value)| iter::repeat_n(index / 2, value as usize))
    //     .collect();
    // let right = input.iter().enumerate();

    // println!("{:?}", left);
    // println!("{:?}", input);
    0
}

/// ```rust
/// use advent_of_code_2024::day09::*;
/// let input = parse(
/// r"2333133121414131402");
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &InputType) -> usize {
    0
}
