use std::{
    collections::HashMap,
    iter,
    sync::atomic::{AtomicU32, Ordering},
};

use advent::*;

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
    let end_nodes = input.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter_map(move |(x, &cell)| {
            if cell == 0 {
                Some(UPoint::new(x as u32, y as u32))
            } else {
                None
            }
        })
    });
    let start_nodes: HashMap<UPoint, AtomicU32> =
        HashMap::from_iter(input.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &cell)| {
                if cell == 9 {
                    Some((UPoint::new(x as u32, y as u32), AtomicU32::new(0)))
                } else {
                    None
                }
            })
        }));

    end_nodes.for_each(|node| explore(input, &bounds, node, &start_nodes));

    fn explore(
        map: &Vec<Vec<u8>>,
        bounds: &UPoint,
        end: UPoint,
        start_nodes: &HashMap<Point<u32>, AtomicU32>,
    ) {
        let mut map = map.clone();
        let mut alive = vec![(end, 0u8)];

        while let Some((pos, altitude)) = alive.pop() {
            if altitude == 9 {
                start_nodes[&pos].fetch_add(1, Ordering::Relaxed);
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
    }

    start_nodes
        .iter()
        .map(|(_, score)| score.load(Ordering::Relaxed))
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
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &InputType) -> i32 {
    0
}
