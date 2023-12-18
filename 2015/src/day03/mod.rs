use std::collections::HashSet;

use advent::*;

advent_day!(Day03, parse, Vec<Direction>, part1, part2);

pub fn parse(input: &str) -> Vec<Direction> {
    input
        .chars()
        .map(|c| match c {
            '^' => Direction::North,
            'v' => Direction::South,
            '>' => Direction::East,
            '<' => Direction::West,
            _ => unreachable!(),
        })
        .collect()
}

/// ```rust
/// use advent_of_code_2015::day03::*;
/// assert_eq!(2, part1(&(parse(">"))));
/// ```
/// ```rust
/// use advent_of_code_2015::day03::*;
/// assert_eq!(4, part1(&parse("^>v<")));
/// ```
/// ```rust
/// use advent_of_code_2015::day03::*;
/// assert_eq!(2, part1(&parse("^v^v^v^v^v")));
/// ```
pub fn part1(input: &Vec<Direction>) -> u32 {
    let mut visited = HashSet::<IPoint>::new();
    let mut location = IPoint::origin();
    visited.insert(location);

    for direction in input {
        location = match direction {
            Direction::North => location.north(),
            Direction::South => location.south(),
            Direction::East => location.east(),
            Direction::West => location.west(),
        };
        visited.insert(location);
    }

    visited.len() as u32
}

/// ```rust
/// use advent_of_code_2015::day03::*;
/// assert_eq!(3, part2(&(parse("^v"))));
/// ```
/// ```rust
/// use advent_of_code_2015::day03::*;
/// assert_eq!(3, part2(&parse("^>v<")));
/// ```
/// ```rust
/// use advent_of_code_2015::day03::*;
/// assert_eq!(11, part2(&parse("^v^v^v^v^v")));
/// ```
pub fn part2(input: &Vec<Direction>) -> u32 {
    let mut visited = HashSet::<IPoint>::new();
    let mut location = IPoint::origin();
    let mut robo_location = IPoint::origin();
    visited.insert(location);

    for directions in input.chunks(2) {
        location = match directions[0] {
            Direction::North => location.north(),
            Direction::South => location.south(),
            Direction::East => location.east(),
            Direction::West => location.west(),
        };
        visited.insert(location);

        robo_location = match directions[1] {
            Direction::North => robo_location.north(),
            Direction::South => robo_location.south(),
            Direction::East => robo_location.east(),
            Direction::West => robo_location.west(),
        };
        visited.insert(robo_location);
    }

    visited.len() as u32
}
