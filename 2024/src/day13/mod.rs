use advent::*;
use num::Integer;
use regex::Regex;

advent_day!(Day13, parse, Vec<ClawGame>, part1, part2);

pub fn parse(input: &str) -> InputType<'_> {
    let pattern = Regex::new(
        r"Button A: X\+(?<AX>\d+), Y\+(?<AY>\d+)
Button B: X\+(?<BX>\d+), Y\+(?<BY>\d+)
Prize: X=(?<X>\d+), Y=(?<Y>\d+)",
    )
    .unwrap();

    pattern
        .captures_iter(input)
        .map(|capture| ClawGame {
            ax: capture.name("AX").unwrap().as_str().parse().unwrap(),
            ay: capture.name("AY").unwrap().as_str().parse().unwrap(),
            bx: capture.name("BX").unwrap().as_str().parse().unwrap(),
            by: capture.name("BY").unwrap().as_str().parse().unwrap(),
            x: capture.name("X").unwrap().as_str().parse().unwrap(),
            y: capture.name("Y").unwrap().as_str().parse().unwrap(),
        })
        .collect()
}

#[derive(Debug)]
pub struct ClawGame {
    pub ax: i32,
    pub ay: i32,
    pub bx: i32,
    pub by: i32,
    pub x: i32,
    pub y: i32,
}

/// ```rust
/// use advent_of_code_2024::day13::*;
/// let input = parse(
/// r"Button A: X+94, Y+34
/// Button B: X+22, Y+67
/// Prize: X=8400, Y=5400
///
/// Button A: X+26, Y+66
/// Button B: X+67, Y+21
/// Prize: X=12748, Y=12176
///
/// Button A: X+17, Y+86
/// Button B: X+84, Y+37
/// Prize: X=7870, Y=6450
///
/// Button A: X+69, Y+23
/// Button B: X+27, Y+71
/// Prize: X=18641, Y=10279");
/// assert_eq!(480, part1(&input));
/// ```
pub fn part1(input: &InputType) -> i32 {
    input
        .iter()
        .filter_map(
            |&ClawGame {
                 ax,
                 ay,
                 bx,
                 by,
                 x,
                 y,
             }| {
                if let (a, 0) = (y * bx - by * x).div_rem(&(ay * bx - by * ax)) {
                    if let (b, 0) = (y * ax - ay * x).div_rem(&(by * ax - ay * bx)) {
                        Some(3 * a + b)
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
        )
        .sum()
}

/// ```rust
/// use advent_of_code_2024::day13::*;
/// let input = parse(
/// r"Button A: X+94, Y+34
/// Button B: X+22, Y+67
/// Prize: X=8400, Y=5400
///
/// Button A: X+26, Y+66
/// Button B: X+67, Y+21
/// Prize: X=12748, Y=12176
///
/// Button A: X+17, Y+86
/// Button B: X+84, Y+37
/// Prize: X=7870, Y=6450
///
/// Button A: X+69, Y+23
/// Button B: X+27, Y+71
/// Prize: X=18641, Y=10279");
/// assert_eq!(875318608908, part2(&input));
/// ```
pub fn part2(input: &InputType) -> i64 {
    input
        .iter()
        .filter_map(
            |&ClawGame {
                 ax,
                 ay,
                 bx,
                 by,
                 x,
                 y,
             }| {
                let a_qr = ((y as i64 + 10000000000000i64) * bx as i64
                    - by as i64 * (x as i64 + 10000000000000i64))
                    .div_rem(&((ay * bx - by * ax) as i64));
                if let (a, 0) = a_qr {
                    let b_qr = ((y as i64 + 10000000000000i64) * ax as i64
                        - ay as i64 * (x as i64 + 10000000000000i64))
                        .div_rem(&(by as i64 * ax as i64 - ay as i64 * bx as i64));
                    if let (b, 0) = b_qr {
                        Some(3 * a + b)
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
        )
        .sum()
}
