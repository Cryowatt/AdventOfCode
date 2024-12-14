use advent::*;
use regex::Regex;

advent_day!(Day14, parse, Vec<Robot>, part1, part2);

pub fn parse(input: &str) -> InputType {
    let pattern = Regex::new(r"p=(?<PX>\d+),(?<PY>\d+) v=(?<VX>-?\d+),(?<VY>-?\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let capture = pattern.captures(line).unwrap();
            Robot {
                position: IPoint::new(
                    capture.name("PX").unwrap().as_str().parse().unwrap(),
                    capture.name("PY").unwrap().as_str().parse().unwrap(),
                ),
                velocity: IPoint::new(
                    capture.name("VX").unwrap().as_str().parse().unwrap(),
                    capture.name("VY").unwrap().as_str().parse().unwrap(),
                ),
            }
        })
        .collect()
}

#[derive(Debug)]
pub struct Robot {
    pub position: IPoint,
    pub velocity: IPoint,
}

enum Quadrant {
    NE,
    NW,
    SW,
    SE,
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
/// assert_eq!(12, part1_with_bounds::<11, 7>(&input));
/// ```
pub fn part1(input: &InputType) -> u32 {
    part1_with_bounds::<101, 103>(input)
}

pub fn part1_with_bounds<const WIDTH: i32, const HEIGHT: i32>(input: &InputType) -> u32 {
    let mut ne_count = 0;
    let mut nw_count = 0;
    let mut se_count = 0;
    let mut sw_count = 0;
    let x_center: i32 = WIDTH / 2;
    let y_center: i32 = HEIGHT / 2;

    input
        .iter()
        .filter_map(|robot| {
            let position = robot.position + robot.velocity * 100;
            let wrapped_position = IPoint::new(
                ((position.x % WIDTH) + WIDTH) % WIDTH,
                ((position.y % HEIGHT) + HEIGHT) % HEIGHT,
            );
            match (
                wrapped_position.x.cmp(&x_center),
                wrapped_position.y.cmp(&y_center),
            ) {
                (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => Some(Quadrant::NW),
                (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => Some(Quadrant::SW),
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => Some(Quadrant::NE),
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => Some(Quadrant::SE),
                _ => None,
            }
        })
        .for_each(|quadrant| match quadrant {
            Quadrant::NE => ne_count += 1,
            Quadrant::NW => nw_count += 1,
            Quadrant::SW => sw_count += 1,
            Quadrant::SE => se_count += 1,
        });

    ne_count * nw_count * se_count * sw_count
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
