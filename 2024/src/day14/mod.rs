use advent::*;
use num::Integer;
use regex::Regex;

advent_day!(Day14, parse, Vec<Robot>, part1, part2);

pub fn parse(input: &str) -> InputType {
    let pattern = Regex::new(r"p=(?<PX>\d+),(?<PY>\d+) v=(?<VX>-?\d+),(?<VY>-?\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let capture = pattern.captures(line).unwrap();
            Robot {
                position: UPoint::new(
                    capture.name("PX").unwrap().as_str().parse::<u32>().unwrap(),
                    capture.name("PY").unwrap().as_str().parse::<u32>().unwrap(),
                ),
                velocity: IPoint::new(
                    capture.name("VX").unwrap().as_str().parse::<i32>().unwrap(),
                    capture.name("VY").unwrap().as_str().parse::<i32>().unwrap(),
                ),
            }
        })
        .collect()
}

#[derive(Debug)]
pub struct Robot {
    pub position: UPoint,
    pub velocity: IPoint,
}

/// ```rust
/// use advent_of_code_2024::day14::*;
/// let input = parse(
/// r"p=0,4 v=3,-3
/// p=6,3 v=-1,-3
/// p=10,3 v=-1,2
/// p=2,0 v=2,-1
/// p=0,0 v=1,3
/// p=3,0 v=-2,-2
/// p=7,6 v=-1,-3
/// p=3,0 v=-1,-2
/// p=9,3 v=2,3
/// p=7,3 v=-1,2
/// p=2,4 v=2,-3
/// p=9,5 v=-3,-3");
/// assert_eq!(12, part1(&input));
/// ```
pub fn part1(input: &InputType) -> u32 {
    0
}

/// ```rust
/// use advent_of_code_2024::day14::*;
/// let input = parse(
/// r"p=0,4 v=3,-3
/// p=6,3 v=-1,-3
/// p=10,3 v=-1,2
/// p=2,0 v=2,-1
/// p=0,0 v=1,3
/// p=3,0 v=-2,-2
/// p=7,6 v=-1,-3
/// p=3,0 v=-1,-2
/// p=9,3 v=2,3
/// p=7,3 v=-1,2
/// p=2,4 v=2,-3
/// p=9,5 v=-3,-3");
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &InputType) -> u32 {
    0
}
