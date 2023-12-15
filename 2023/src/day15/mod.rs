use std::{
    cmp::Ordering,
    collections::{hash_map::DefaultHasher, HashMap},
    hash::Hasher,
    ops::Range,
};

use advent::*;

advent_day!(Day15, parse, Vec<&str>, part1, part2);

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().flat_map(|line| line.split(",")).collect()
}

/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("rn=1");
/// assert_eq!(30, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("cm-");
/// assert_eq!(253, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("qp=3");
/// assert_eq!(97, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("cm=2");
/// assert_eq!(47, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("qp-");
/// assert_eq!(14, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("pc=4");
/// assert_eq!(180, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("ot=9");
/// assert_eq!(9, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("ab=5");
/// assert_eq!(197, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("pc-");
/// assert_eq!(48, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("pc=6");
/// assert_eq!(214, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("ot=7");
/// assert_eq!(231, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
/// assert_eq!(1320, part1(&input));
/// ```
pub fn part1(input: &Vec<&str>) -> u32 {
    input
        .iter()
        .map(|item| {
            item.bytes()
                .fold(0u8, |hash, b| hash.wrapping_add(b).wrapping_mul(17)) as u32
        })
        .sum()
}

/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse(
/// "O....#....
/// O.OO#....#
/// .....##...
/// OO.#O....O
/// .O.....O#.
/// O.#..O.#.#
/// ..O..#O..O
/// .......O..
///##....###..
///##OO..#....");
/// //assert_eq!(64, part2(&input));
/// ```
pub fn part2(input: &Vec<&str>) -> u32 {
    0
}
