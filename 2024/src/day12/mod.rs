use advent::*;
use num::{traits::bounds, Integer};
use rayon::prelude::*;
use std::collections::HashMap;

advent_day!(Day12, parse, Vec<Vec<u8>>, part1, part2);

pub fn parse(input: &str) -> InputType {
    input
        .lines()
        .map(|line| line.as_bytes().iter().map(|b| *b).collect())
        .collect()
}

/// ```rust
/// use advent_of_code_2024::day12::*;
/// let input = parse(
/// r"RRRRIICCFF
/// RRRRIICCCF
/// VVRRRCCFFF
/// VVRCCCJFFF
/// VVVVCJJCFE
/// VVIVCCJJEE
/// VVIIICJJEE
/// MIIIIIJJEE
/// MIIISIJEEE
/// MMMISSJEEE");
/// assert_eq!(1930, part1(&input));
/// ```
pub fn part1(input: &InputType) -> u32 {
    let bounds = UPoint::new(input.first().unwrap().len() as u32, input.len() as u32);
    let mut region_area = [0u32; 26];
    let mut region_perimeter = [0u32; 26];

    let points = (0..bounds.y)
        .flat_map(|y| (0..bounds.x).map(move |x| UPoint::new(x, y)))
        .for_each(|position| {
            let (region, perimeter) = measure(position, input, &bounds);
            region_area[region as usize] += 1;
            region_perimeter[region as usize] += perimeter;
        });

    fn measure(position: UPoint, map: &InputType, bounds: &UPoint) -> (u8, u32) {
        let current_region = map[position.y as usize][position.x as usize];
        let mut perimeter = 0;

        if let Some(adjacent) = position.north_checked() {
            if map[adjacent.y as usize][adjacent.x as usize] != current_region {
                perimeter += 1;
            }
        } else {
            perimeter += 1;
        }
        if let Some(adjacent) = position.west_checked() {
            if map[adjacent.y as usize][adjacent.x as usize] != current_region {
                perimeter += 1;
            }
        } else {
            perimeter += 1;
        }
        if let Some(adjacent) = position.south_checked(bounds) {
            if map[adjacent.y as usize][adjacent.x as usize] != current_region {
                perimeter += 1;
            }
        } else {
            perimeter += 1;
        }
        if let Some(adjacent) = position.east_checked(bounds) {
            if map[adjacent.y as usize][adjacent.x as usize] != current_region {
                perimeter += 1;
            }
        } else {
            perimeter += 1;
        }

        (current_region - b'A', perimeter)
    }

    (0..26)
        .map(|region| {
            let price = region_area[region] * region_perimeter[region];
            println!(
                "A region of {} plants with price {} * {} = {}.",
                (region as u8 + b'A') as char,
                region_area[region],
                region_perimeter[region],
                price
            );
            price
        })
        .sum()
}

/// ```rust
/// use advent_of_code_2024::day12::*;
/// let input = parse(
/// r"RRRRIICCFF
/// RRRRIICCCF
/// VVRRRCCFFF
/// VVRCCCJFFF
/// VVVVCJJCFE
/// VVIVCCJJEE
/// VVIIICJJEE
/// MIIIIIJJEE
/// MIIISIJEEE
/// MMMISSJEEE");
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &InputType) -> u64 {
    0
}
