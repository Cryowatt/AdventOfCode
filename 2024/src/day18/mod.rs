use std::{collections::BinaryHeap, fmt::format};

use advent::*;
use array2d::Array2D;
use regex::Regex;

advent_day!(Day18, parse, Vec<UPoint>, part1, part2);

pub fn parse(input: &str) -> InputType {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            UPoint::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

/// ```rust
/// use advent_of_code_2024::day18::*;
/// let input = parse(
/// r"5,4
/// 4,2
/// 4,5
/// 3,0
/// 2,1
/// 6,3
/// 2,4
/// 1,5
/// 0,6
/// 3,3
/// 2,6
/// 5,1
/// 1,2
/// 5,5
/// 2,5
/// 6,5
/// 1,4
/// 0,4
/// 6,4
/// 1,1
/// 6,1
/// 1,0
/// 0,5
/// 1,6
/// 2,0");
/// assert_eq!(22, memory_space::<7, 7, 12>(&input));
/// ```
pub fn part1(input: &InputType) -> u32 {
    memory_space::<71, 71, 1024>(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Byte {
    Safe,
    Path,
    Corrupt,
}

#[derive(Debug, Clone, Copy)]
struct MinScore<T>(u32, T);

impl<T> Ord for MinScore<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl<T> PartialOrd for MinScore<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Eq for MinScore<T> {}

impl<T> PartialEq for MinScore<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub fn memory_space<const WIDTH: u32, const HEIGHT: u32, const BYTES: u32>(
    input: &InputType,
) -> u32 {
    let bounds = UPoint::new(WIDTH, HEIGHT);
    let end = UPoint::new(WIDTH - 1, HEIGHT - 1);
    let mut memory = Array2D::filled_with(Byte::Safe, HEIGHT as usize, WIDTH as usize);
    for address in input.iter().take(BYTES as usize) {
        memory[(address.y as usize, address.x as usize)] = Byte::Corrupt;
    }

    let mut pending = BinaryHeap::new();
    memory[(0, 0)] = Byte::Path;
    pending.push(MinScore(0, UPoint::new(0, 0)));

    let score = {
        loop {
            if let Some(MinScore(score, position)) = pending.pop() {
                if let Some(next) = position.north_checked() {
                    if next == end {
                        break score + 1;
                    } else if memory[(next.y as usize, next.x as usize)] == Byte::Safe {
                        memory[(next.y as usize, next.x as usize)] = Byte::Path;
                        pending.push(MinScore(score + 1, next));
                    }
                }
                if let Some(next) = position.west_checked() {
                    if next == end {
                        break score + 1;
                    } else if memory[(next.y as usize, next.x as usize)] == Byte::Safe {
                        memory[(next.y as usize, next.x as usize)] = Byte::Path;
                        pending.push(MinScore(score + 1, next));
                    }
                }
                if let Some(next) = position.south_checked(&bounds) {
                    if next == end {
                        break score + 1;
                    } else if memory[(next.y as usize, next.x as usize)] == Byte::Safe {
                        memory[(next.y as usize, next.x as usize)] = Byte::Path;
                        pending.push(MinScore(score + 1, next));
                    }
                }
                if let Some(next) = position.east_checked(&bounds) {
                    if next == end {
                        break score + 1;
                    } else if memory[(next.y as usize, next.x as usize)] == Byte::Safe {
                        memory[(next.y as usize, next.x as usize)] = Byte::Path;
                        pending.push(MinScore(score + 1, next));
                    }
                }
            } else {
                panic!("Couldn't find end");
            }
        }
    };

    score
}

/// ```rust
/// use advent_of_code_2024::day18::*;
/// let input = parse(
/// r"5,4
/// 4,2
/// 4,5
/// 3,0
/// 2,1
/// 6,3
/// 2,4
/// 1,5
/// 0,6
/// 3,3
/// 2,6
/// 5,1
/// 1,2
/// 5,5
/// 2,5
/// 6,5
/// 1,4
/// 0,4
/// 6,4
/// 1,1
/// 6,1
/// 1,0
/// 0,5
/// 1,6
/// 2,0");
/// assert_eq!(advent::UPoint::new(6,1), blocking_byte::<7, 7>(&input));
/// ```
pub fn part2(input: &InputType) -> String {
    let blocking = blocking_byte::<71, 71>(input);
    format!("{},{}", blocking.x, blocking.y)
}

pub fn blocking_byte<const WIDTH: u32, const HEIGHT: u32>(input: &InputType) -> UPoint {
    let mut min = 0;
    let mut max = input.len() - 1;
    let mut pass = 0;

    while min != max {
        pass += 1;
        let mid = (min + max) / 2;
        if mid == min {
            break;
        }
        if let Some(_) = is_solvable::<WIDTH, HEIGHT>(input, mid) {
            min = mid;
        } else {
            max = mid;
        }
        if pass > 16 {
            break;
        }
    }

    input[min]
}

pub fn is_solvable<const WIDTH: u32, const HEIGHT: u32>(
    input: &InputType,
    bytes: usize,
) -> Option<u32> {
    let bounds = UPoint::new(WIDTH, HEIGHT);
    let end = UPoint::new(WIDTH - 1, HEIGHT - 1);
    let mut memory = Array2D::filled_with(Byte::Safe, HEIGHT as usize, WIDTH as usize);
    for address in input.iter().take(bytes) {
        memory[(address.y as usize, address.x as usize)] = Byte::Corrupt;
    }

    let mut pending = BinaryHeap::new();
    memory[(0, 0)] = Byte::Path;
    pending.push(MinScore(0, UPoint::new(0, 0)));

    let score = {
        loop {
            if let Some(MinScore(score, position)) = pending.pop() {
                if let Some(next) = position.north_checked() {
                    if next == end {
                        break Some(score + 1);
                    } else if memory[(next.y as usize, next.x as usize)] == Byte::Safe {
                        memory[(next.y as usize, next.x as usize)] = Byte::Path;
                        pending.push(MinScore(score + 1, next));
                    }
                }
                if let Some(next) = position.west_checked() {
                    if next == end {
                        break Some(score + 1);
                    } else if memory[(next.y as usize, next.x as usize)] == Byte::Safe {
                        memory[(next.y as usize, next.x as usize)] = Byte::Path;
                        pending.push(MinScore(score + 1, next));
                    }
                }
                if let Some(next) = position.south_checked(&bounds) {
                    if next == end {
                        break Some(score + 1);
                    } else if memory[(next.y as usize, next.x as usize)] == Byte::Safe {
                        memory[(next.y as usize, next.x as usize)] = Byte::Path;
                        pending.push(MinScore(score + 1, next));
                    }
                }
                if let Some(next) = position.east_checked(&bounds) {
                    if next == end {
                        break Some(score + 1);
                    } else if memory[(next.y as usize, next.x as usize)] == Byte::Safe {
                        memory[(next.y as usize, next.x as usize)] = Byte::Path;
                        pending.push(MinScore(score + 1, next));
                    }
                }
            } else {
                break None;
            }
        }
    };

    score
}
