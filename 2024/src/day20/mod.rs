use std::collections::BinaryHeap;

use advent::*;
use array2d::Array2D;
use dashmap::DashMap;
use onig::Regex;
use rayon::prelude::*;

advent_day!(Day20, parse, Array2D<Tile>, part1, part2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Start,
    End,
    Empty,
}

pub fn parse(input: &str) -> InputType {
    Array2D::from_rows(
        &input
            .lines()
            .map(|line| {
                line.as_bytes()
                    .iter()
                    .map(|tile| match tile {
                        b'.' => Tile::Empty,
                        b'#' => Tile::Wall,
                        b'S' => Tile::Start,
                        b'E' => Tile::End,
                        _ => panic!("Illegal character"),
                    })
                    .collect()
            })
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// ```rust
/// use advent_of_code_2024::day20::*;
/// let input = parse(
/// r"###############
///# #...#...#.....#
///# #.#.#.#.#.###.#
///# #S#...#.#.#...#
///# #######.#.#.###
///# #######.#.#...#
///# #######.#.###.#
///# ###..E#...#...#
///# ###.#######.###
///# #...###...#...#
///# #.#####.#.###.#
///# #.#...#.#.#...#
///# #.#.#.#.#.#.###
///# #...#...#...###
///# ###############");
/// // There are 14 cheats that save 2 picoseconds.
/// assert_eq!(14, find_cheat(&input, 2));
/// // There are 14 cheats that save 4 picoseconds.
/// assert_eq!(14, find_cheat(&input, 4));
/// //There are 2 cheats that save 6 picoseconds.
/// assert_eq!(2, find_cheat(&input, 6));
/// //There are 4 cheats that save 8 picoseconds.
/// assert_eq!(4, find_cheat(&input, 8));
/// //There are 2 cheats that save 10 picoseconds.
/// assert_eq!(2, find_cheat(&input, 10));
/// //There are 3 cheats that save 12 picoseconds.
/// assert_eq!(3, find_cheat(&input, 12));
/// //There is one cheat that saves 20 picoseconds.
/// assert_eq!(1, find_cheat(&input, 20));
/// //There is one cheat that saves 36 picoseconds.
/// assert_eq!(1, find_cheat(&input, 36));
/// //There is one cheat that saves 38 picoseconds.
/// assert_eq!(1, find_cheat(&input, 38));
/// //There is one cheat that saves 40 picoseconds.
/// assert_eq!(1, find_cheat(&input, 40));
/// //There is one cheat that saves 64 picoseconds.
/// assert_eq!(1, find_cheat(&input, 64));
/// ```
pub fn part1(input: &InputType) -> usize {
    find_cheat(input, 100)
}

pub fn find_cheat(input: &InputType, length: u32) -> usize {
    let map = input;
    let bounds = UPoint::new(input.row_len() as u32, input.column_len() as u32);
    let start = input
        .enumerate_row_major()
        .find_map(|((row, column), &tile)| {
            (tile == Tile::Start).then(|| UPoint::new(column as u32, row as u32))
        })
        .unwrap();
    let end = input
        .enumerate_row_major()
        .find_map(|((row, column), &tile)| {
            (tile == Tile::Start).then(|| UPoint::new(column as u32, row as u32))
        })
        .unwrap();

    let end_distance = &mut Array2D::filled_with(None, input.row_len(), input.column_len());
    let pending = &mut BinaryHeap::new();

    let maybe_queue = |point: Option<UPoint>,
                       distance,
                       end_distance: &mut Array2D<Option<u32>>,
                       pending: &mut BinaryHeap<MinScore<Point<u32>>>| {
        if let Some(point) = point {
            let next_distance = end_distance
                .get_mut(point.y as usize, point.x as usize)
                .unwrap();
            if next_distance.is_none() && map[(point.y as usize, point.x as usize)] != Tile::Wall {
                pending.push(MinScore(distance, point));
                next_distance.replace(distance);
            }
        }
    };

    maybe_queue(Some(end), 0, end_distance, pending);

    while let Some(MinScore(distance, point)) = pending.pop() {
        let distance = distance + 1;
        maybe_queue(point.north_checked(), distance, end_distance, pending);
        maybe_queue(point.west_checked(), distance, end_distance, pending);
        maybe_queue(
            point.south_checked(&bounds),
            distance,
            end_distance,
            pending,
        );
        maybe_queue(point.east_checked(&bounds), distance, end_distance, pending);
    }

    for line in end_distance.as_rows() {
        for tile in line {
            print!(
                "{}",
                match tile {
                    Some(x) => (x % 10).to_string(),
                    None => ".".to_owned(),
                }
            )
        }
        println!();
    }

    0
}

fn maybe_queue(
    next: Option<Point<u32>>,
    map: &InputType,
    end_distance: &mut Array2D<Option<u32>>,
    pending: &mut BinaryHeap<MinScore<Point<u32>>>,
    distance: u32,
) {
}

/// ```rust
/// use advent_of_code_2024::day20::*;
/// let input = parse(
/// r"###############
///# #...#...#.....#
///# #.#.#.#.#.###.#
///# #S#...#.#.#...#
///# #######.#.#.###
///# #######.#.#...#
///# #######.#.###.#
///# ###..E#...#...#
///# ###.#######.###
///# #...###...#...#
///# #.#####.#.###.#
///# #.#...#.#.#...#
///# #.#.#.#.#.#.###
///# #...#...#...###
///# ###############");
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &InputType) -> u64 {
    0
}
