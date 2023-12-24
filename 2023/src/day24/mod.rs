use advent::*;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};

advent_day!(Day24, parse, Vec<HeisenbergCompensator>, part1, part2);

pub fn parse(input: &str) -> Vec<HeisenbergCompensator> {
    input
        .lines()
        .map(|line| {
            let (position, velocity) = line.split_once("@").unwrap();
            HeisenbergCompensator {
                position: position.parse().unwrap(),
                velocity: velocity.parse().unwrap(),
            }
        })
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct HeisenbergCompensator {
    position: Point3D<i64>,
    velocity: Point3D<i64>,
}

/// ```rust
/// use advent_of_code_2023::day24::*;
/// let input = parse(
/// r"19, 13, 30 @ -2,  1, -2
/// 18, 19, 22 @ -1, -1, -2
/// 20, 25, 34 @ -2, -2, -4
/// 12, 31, 28 @ -1, -2, -1
/// 20, 19, 15 @  1, -5, -3");
/// assert_eq!(2, snow_collisions(&input, 7f64, 27f64));
/// ```
/// 13038 TOO LOW!
pub fn part1(storm: &Vec<HeisenbergCompensator>) -> u32 {
    snow_collisions(storm, 200000000000000f64, 400000000000000f64)
}

pub fn snow_collisions(storm: &Vec<HeisenbergCompensator>, min: f64, max: f64) -> u32 {
    storm
        .into_iter()
        .enumerate()
        .map(|(index, hail_a)| {
            let a1 = hail_a.velocity.x as f64;
            let b1 = hail_a.velocity.y as f64;
            let x1 = hail_a.position.x as f64;
            let y1 = hail_a.position.y as f64;

            storm
                .into_iter()
                .skip(index + 1)
                .filter(move |hail_b| {
                    let a2 = hail_b.velocity.x as f64;
                    let b2 = hail_b.velocity.y as f64;
                    let x2 = hail_b.position.x as f64;
                    let y2 = hail_b.position.y as f64;

                    let x = (a1 * b2 * x2 - a2 * (a1 * (y2 - y1) + b1 * x1)) / (a1 * b2 - a2 * b1);
                    let y = y1 + (b1 / a1) * (x - x1);
                    let t1 = (x - x1) / a1;
                    let t2 = (x - x2) / a2;

                    x <= max && y <= max && x >= min && y >= min && t1 >= 0.0f64 && t2 >= 0.0f64
                })
                .count() as u32
        })
        .sum()
}

/// ```rust
/// use advent_of_code_2023::day24::*;
/// let input = parse(
/// r"19, 13, 30 @ -2,  1, -2
/// 18, 19, 22 @ -1, -1, -2
/// 20, 25, 34 @ -2, -2, -4
/// 12, 31, 28 @ -1, -2, -1
/// 20, 19, 15 @  1, -5, -3");
/// //assert_eq!(?, part1(&input));
/// ```
pub fn part2(hail: &Vec<HeisenbergCompensator>) -> u32 {
    0
}
