use advent::*;
use array2d::Array2D;
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

#[derive(Debug, Clone, Copy)]
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
            let wrapped_position =
                IPoint::new(position.x.rem_euclid(WIDTH), position.y.rem_euclid(HEIGHT));
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

/// No test cases
pub fn part2(input: &InputType) -> u32 {
    part2_with_bounds::<101, 103, 32>(input)
}

pub fn part2_with_bounds<const WIDTH: i32, const HEIGHT: i32, const TREE_RUN: i32>(
    input: &InputType,
) -> u32 {
    let mut robots = input.to_owned();

    fn update<const WIDTH: i32, const HEIGHT: i32>(robots: &mut Vec<Robot>) {
        robots.iter_mut().for_each(|robot| {
            robot.position.x = (robot.position.x + robot.velocity.x).rem_euclid(WIDTH);
            robot.position.y = (robot.position.y + robot.velocity.y).rem_euclid(HEIGHT);
        });
    }

    fn run_length<const WIDTH: i32, const HEIGHT: i32>(robots: &Vec<Robot>) -> u32 {
        let mut robo_grid = Array2D::filled_with(false, HEIGHT as usize, WIDTH as usize);
        let mut max_run = 0;
        for position in robots.iter().map(|robot| robot.position) {
            robo_grid[(position.y as usize, position.x as usize)] = true;
        }

        for y in 0..HEIGHT {
            let mut run_length = 0;
            for x in 0..WIDTH {
                if robo_grid[(y as usize, x as usize)] {
                    run_length += 1;
                } else {
                    max_run = max_run.max(run_length);
                    run_length = 0;
                }
            }
        }
        max_run
    }
    let mut frame = 0;

    while run_length::<WIDTH, HEIGHT>(&robots) < 31 {
        frame += 1;
        update::<WIDTH, HEIGHT>(&mut robots);
    }
    frame
}
