use std::{cmp::Ordering, ops::Div, sync::OnceLock};

use log::debug;

use crate::{advent_bench, advent_day};

advent_day!(Day6, parse, Vec<BoatRace>, part1, part2);

pub struct BoatRace {
    time: u32,
    distance: u32,
}

pub fn parse(input: &str) -> Vec<BoatRace> {
    let mut rows = input.lines().map(|line| {
        line.split_whitespace()
            .skip(1)
            .map(|value| value.parse::<u32>().unwrap())
    });
    let time = rows.next().unwrap();
    let distance = rows.next().unwrap();

    time.zip(distance)
        .map(|(time, distance)| BoatRace { time, distance })
        .collect()
}

/// ```rust
/// use advent_of_code::day6::*;
/// let input = parse(
/// r"Time:      7  15   30
/// Distance:  9  40  200");
/// assert_eq!(288, part1(&input));
/// ```
pub fn part1(input: &Vec<BoatRace>) -> u32 {
    input
        .iter()
        .map(|race| {
            let half_time = race.time / 2;
            let remainder = race.time % 2;
            let wins = (1..=half_time)
                .filter(|hold_time| race.distance < hold_time * (race.time - hold_time))
                .count() as u32;
            (wins * 2) - (1 - remainder)
        })
        .product()
}

/// ```rust
/// use advent_of_code::day6::*;
/// let input = parse(
/// r"Time:      7  15   30
/// Distance:  9  40  200");
/// //assert_eq!(420, part2(&input));
/// ```
pub fn part2(input: &Vec<BoatRace>) -> u32 {
    0
}
