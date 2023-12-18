use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

use advent::*;
use bitflags::Flags;
use crossterm::style::Stylize;
use regex::Regex;

advent_day!(Day18, parse, DigPlan, part1, part2);

pub struct DigPlan {
    intructions: Vec<DigInstruction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DigInstruction {
    direction: Direction,
    length: u8,
    colour: u32,
}

pub fn parse(input: &str) -> DigPlan {
    let regex = Regex::new(r"([UDLR]) (\d+) \(#([0-9a-f]{6})\)").unwrap();
    DigPlan {
        intructions: input
            .lines()
            .map(|line| {
                let capture = regex.captures(line).unwrap();
                DigInstruction {
                    direction: match capture.get(1).unwrap().as_str() {
                        "U" => Direction::North,
                        "D" => Direction::South,
                        "R" => Direction::East,
                        "L" => Direction::West,
                        _ => unreachable!(),
                    },
                    length: capture.get(2).unwrap().as_str().parse::<u8>().unwrap(),
                    colour: u32::from_str_radix(capture.get(3).unwrap().as_str(), 16).unwrap(),
                }
            })
            .collect(),
    }
}

/// ```rust
/// use advent_of_code_2023::day18::*;
/// let input = parse(
/// r"R 6 (#70c710)
/// D 5 (#0dc571)
/// L 2 (#5713f0)
/// D 2 (#d2c081)
/// R 2 (#59c680)
/// D 2 (#411b91)
/// L 5 (#8ceee2)
/// U 2 (#caa173)
/// L 1 (#1b58a2)
/// U 2 (#caa171)
/// R 2 (#7807d2)
/// U 3 (#a77fa3)
/// L 2 (#015232)
/// U 2 (#7a21e3)");
/// assert_eq!(62, part1(&input));
/// ```
pub fn part1(input: &DigPlan) -> u32 {
    0
}

/// ```rust
/// use advent_of_code_2023::day18::*;
/// let input = parse(
/// r"R 6 (#70c710)
/// D 5 (#0dc571)
/// L 2 (#5713f0)
/// D 2 (#d2c081)
/// R 2 (#59c680)
/// D 2 (#411b91)
/// L 5 (#8ceee2)
/// U 2 (#caa173)
/// L 1 (#1b58a2)
/// U 2 (#caa171)
/// R 2 (#7807d2)
/// U 3 (#a77fa3)
/// L 2 (#015232)
/// U 2 (#7a21e3)");
/// //assert_eq!(94, part2(&input));
/// ```
pub fn part2(input: &DigPlan) -> u32 {
    0
}
