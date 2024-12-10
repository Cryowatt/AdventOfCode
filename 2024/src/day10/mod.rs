use std::{
    collections::HashMap,
    iter,
    sync::atomic::{AtomicU32, Ordering},
};

use advent::*;
use rayon::prelude::*;

advent_day!(Day10, parse, Vec<Vec<u8>>, part1, part2);

pub fn parse(input: &str) -> InputType {
    input
        .lines()
        .map(|line| line.as_bytes().iter().map(|b| b - b'0').collect())
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
pub fn part1(input: &InputType) -> u32 {
    let bounds = UPoint::new(input.first().unwrap().len() as u32, input.len() as u32);

    fn explore(map: &Vec<Vec<u8>>, bounds: &UPoint, end: UPoint) -> u32 {
        let mut map = map.clone();
        let mut alive = vec![(end, 0u8)];
        let mut score = 0;

        while let Some((pos, altitude)) = alive.pop() {
            if altitude == 9 {
                score += 1;
            } else {
                let target_altitude = altitude + 1;

                if let Some(pos) = pos.north_checked() {
                    if map[pos.y as usize][pos.x as usize] == target_altitude {
                        map[pos.y as usize][pos.x as usize] = 255;
                        alive.push((pos, target_altitude));
                    }
                }
                if let Some(pos) = pos.west_checked() {
                    if map[pos.y as usize][pos.x as usize] == target_altitude {
                        map[pos.y as usize][pos.x as usize] = 255;
                        alive.push((pos, target_altitude));
                    }
                }
                if let Some(pos) = pos.south_checked(bounds) {
                    if map[pos.y as usize][pos.x as usize] == target_altitude {
                        map[pos.y as usize][pos.x as usize] = 255;
                        alive.push((pos, target_altitude));
                    }
                }
                if let Some(pos) = pos.east_checked(bounds) {
                    if map[pos.y as usize][pos.x as usize] == target_altitude {
                        map[pos.y as usize][pos.x as usize] = 255;
                        alive.push((pos, target_altitude));
                    }
                }
            }
        }

        score
    }

    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &cell)| {
                if cell == 0 {
                    Some(UPoint::new(x as u32, y as u32))
                } else {
                    None
                }
            })
        })
        .par_bridge()
        .map(|node| explore(input, &bounds, node))
        .sum()
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
/// assert_eq!(81, part2(&input));
/// ```
pub fn part2(input: &InputType) -> u32 {
    let bounds = UPoint::new(input.first().unwrap().len() as u32, input.len() as u32);

    fn explore(map: &Vec<Vec<u8>>, bounds: &UPoint, end: UPoint) -> u32 {
        let mut alive = vec![(end, 0u8)];
        let mut score = 0;

        while let Some((pos, altitude)) = alive.pop() {
            if altitude == 9 {
                score += 1;
            } else {
                let target_altitude = altitude + 1;

                if let Some(pos) = pos.north_checked() {
                    if map[pos.y as usize][pos.x as usize] == target_altitude {
                        alive.push((pos, target_altitude));
                    }
                }
                if let Some(pos) = pos.west_checked() {
                    if map[pos.y as usize][pos.x as usize] == target_altitude {
                        alive.push((pos, target_altitude));
                    }
                }
                if let Some(pos) = pos.south_checked(bounds) {
                    if map[pos.y as usize][pos.x as usize] == target_altitude {
                        alive.push((pos, target_altitude));
                    }
                }
                if let Some(pos) = pos.east_checked(bounds) {
                    if map[pos.y as usize][pos.x as usize] == target_altitude {
                        alive.push((pos, target_altitude));
                    }
                }
            }
        }

        score
    }

    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &cell)| {
                if cell == 0 {
                    Some(UPoint::new(x as u32, y as u32))
                } else {
                    None
                }
            })
        })
        .par_bridge()
        .map(|node| explore(input, &bounds, node))
        .sum()
}
