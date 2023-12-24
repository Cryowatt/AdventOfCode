use advent::*;
use num::complex::ComplexFloat;
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
/// //assert_eq!(47, part2(&input));
/// ```
pub fn part2(storm: &Vec<HeisenbergCompensator>) -> u32 {
    // Collection of all points by t, then for each hail
    let mut time_points = vec![];
    for _t in 0..=storm.len() {
        time_points.push(vec![]);
    }

    for hail in storm {
        // let mut points = vec![];
        for t in 1..=storm.len() {
            time_points[t as usize].push(hail.position + (hail.velocity * t as i64));
        }
        // hail_lines.push(points);
    }

    // For all t1 points, find velocity to all t2 points, validate with t3 and remainder
    for (idx1, &t1) in time_points[1].iter().enumerate() {
        let x1 = t1.x as f64;
        let y1 = t1.y as f64;
        let z1 = t1.z as f64;

        for (idx2, &t2) in time_points[2]
            .iter()
            .enumerate()
            .filter(|(id, _)| *id != idx1)
        {
            let x2 = t2.x as f64;
            let y2 = t2.y as f64;
            let z2 = t2.z as f64;

            for (idx3, &t3) in time_points[3]
                .iter()
                .enumerate()
                .filter(|(id, _)| *id != idx1 && *id != idx2)
            {
                let x3 = t3.x as f64;
                let y3 = t3.y as f64;
                let z3 = t3.z as f64;

                // // x2(y3 – y2) + x1(y2 – y3) + x2(y2 – y1) – x3(y2 – y1) = 0
                let colinear_check =
                    x2 * (y3 - y2) + x1 * (y2 - y3) + x2 * (y2 - y1) - x3 * (y2 - y1);
                if colinear_check.abs() < 100f64 {
                    println!("COLINEAR!");
                } else {
                    println!("{colinear_check}");
                }
            }
            // let t3 = t2 + (t2 - t1);
            // let fk = time_points[3].iter().find(|&&p| p == t3);
            // println!("{:?} {:?} {:?} {:?} {:?}", t1, t2, t2 - t1, t3, fk);
        }
    }
    0
}
