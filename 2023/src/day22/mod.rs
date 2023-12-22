use advent::*;
use bitflags::*;
use regex::Regex;

advent_day!(Day22, parse, Vec<Line>, part1, part2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DigInstruction {
    direction: Direction,
    length: u32,
    colour: u32,
}

enum Axis {
    X,
    Y,
    Z,
}

pub struct Line {
    x: u32,
    y: u32,
    z: u32,
    axis: Axis,
    length: u32,
}

impl Line {
    pub fn new(start: (u32, u32, u32), end: (u32, u32, u32)) -> Self {
        let (axis, length) = if start.0 < end.0 {
            (Axis::X, end.0 - start.0)
        } else if start.1 < end.1 {
            (Axis::Y, end.1 - start.1)
        } else if start.2 < end.2 {
            (Axis::Z, end.2 - start.2)
        } else {
            unreachable!()
        };

        Self {
            x: start.0,
            y: start.0,
            z: start.0,
            axis,
            length,
        }
    }
}

pub fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("~").unwrap();
            let mut start_coords = start.split(",").map(|p| p.parse::<u32>().unwrap());
            let mut end_coords = end.split(",").map(|p| p.parse::<u32>().unwrap());
            Line::new(
                (
                    start_coords.next().unwrap(),
                    start_coords.next().unwrap(),
                    start_coords.next().unwrap(),
                ),
                (
                    end_coords.next().unwrap(),
                    end_coords.next().unwrap(),
                    end_coords.next().unwrap(),
                ),
            )
        })
        .collect()
}

/// ```rust
/// use advent_of_code_2023::day22::*;
/// let input = parse(
/// r"1,0,1~1,2,1
/// 0,0,2~2,0,2
/// 0,2,3~2,2,3
/// 0,0,4~0,2,4
/// 2,0,5~2,2,5
/// 0,1,6~2,1,6
/// 1,1,8~1,1,9");
/// assert_eq!(62, part1(&input));
/// ```
pub fn part1(blocks: &Vec<Line>) -> u64 {
    0
}

/// ```rust
/// use advent_of_code_2023::day22::*;
/// let input = parse(
/// r"1,0,1~1,2,1
/// 0,0,2~2,0,2
/// 0,2,3~2,2,3
/// 0,0,4~0,2,4
/// 2,0,5~2,2,5
/// 0,1,6~2,1,6
/// 1,1,8~1,1,9");
/// assert_eq!(952408144115, part2(&input));
/// ```
pub fn part2(blocks: &Vec<Line>) -> u64 {
    0
}
