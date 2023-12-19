use std::{collections::HashMap, ops::RangeInclusive};

use advent::*;
use bitflags::*;
use rayon::vec;
use regex::Regex;

advent_day!(Day18, parse, DigPlan, part1, part2);

pub struct DigPlan {
    intructions: Vec<DigInstruction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DigInstruction {
    direction: Direction,
    length: u32,
    colour: u32,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Orientation: u8 {
        const North = 0b00000001;
        const South = 0b00000010;
    }
}

pub fn parse(input: &str) -> DigPlan {
    let regex = Regex::new(r"([UDLR]) (\d+) \(#([0-9a-f]{6})\)").unwrap();
    DigPlan {
        intructions: input
            .lines()
            .map(|line| {
                let capture = regex.captures(line).unwrap();
                DigInstruction {
                    direction: match capture.get(1).unwrap().as_str() {
                        "U" => Direction::North,
                        "D" => Direction::South,
                        "R" => Direction::East,
                        "L" => Direction::West,
                        _ => unreachable!(),
                    },
                    length: capture.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                    colour: u32::from_str_radix(capture.get(3).unwrap().as_str(), 16).unwrap(),
                }
            })
            .collect(),
    }
}

/// ```rust
/// use advent_of_code_2023::day18::*;
/// let input = parse(
/// r"R 6 (#70c710)
/// D 5 (#0dc571)
/// L 2 (#5713f0)
/// D 2 (#d2c081)
/// R 2 (#59c680)
/// D 2 (#411b91)
/// L 5 (#8ceee2)
/// U 2 (#caa173)
/// L 1 (#1b58a2)
/// U 2 (#caa171)
/// R 2 (#7807d2)
/// U 3 (#a77fa3)
/// L 2 (#015232)
/// U 2 (#7a21e3)");
/// assert_eq!(62, part1(&input));
/// ```
pub fn part1(plan: &DigPlan) -> u64 {
    // let mut instruction = plan.intructions.clone();
    // instruction.push(instruction.first().unwrap().clone());

    let mut points = vec![IPoint::origin()];
    let mut rotation_state = Rotation::Neutral;
    let mut last_direction = plan.intructions.last().unwrap().direction;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Rotation {
        Clockwise,
        Neutral,
        CounterNeutral,
        CounterClockwise,
    }

    points.extend(
        plan.intructions
            .windows(2)
            .scan(Point::origin(), |last, window| {
                let step = window[0];
                let next_step = window[1];

                let rotation = match (step.direction, next_step.direction) {
                    (Direction::North, Direction::East) => Rotation::Clockwise,
                    (Direction::North, Direction::West) => Rotation::CounterClockwise,
                    (Direction::South, Direction::East) => Rotation::CounterClockwise,
                    (Direction::South, Direction::West) => Rotation::Clockwise,
                    (Direction::East, Direction::North) => Rotation::CounterClockwise,
                    (Direction::East, Direction::South) => Rotation::Clockwise,
                    (Direction::West, Direction::North) => Rotation::Clockwise,
                    (Direction::West, Direction::South) => Rotation::CounterClockwise,

                    // Illegal to move the same direction twice
                    (Direction::North, Direction::North) => todo!(),
                    (Direction::South, Direction::South) => todo!(),
                    (Direction::East, Direction::East) => todo!(),
                    (Direction::West, Direction::West) => todo!(),

                    // Illegal 180
                    (Direction::North, Direction::South) => todo!(),
                    (Direction::South, Direction::North) => todo!(),
                    (Direction::East, Direction::West) => todo!(),
                    (Direction::West, Direction::East) => todo!(),
                };

                let rotation_correction = match (rotation_state, rotation) {
                    (Rotation::Clockwise, Rotation::Clockwise) => Rotation::Clockwise,
                    (Rotation::Clockwise, Rotation::CounterClockwise) => Rotation::CounterNeutral,
                    (Rotation::Neutral, Rotation::Clockwise) => Rotation::Clockwise,
                    (Rotation::Neutral, Rotation::CounterClockwise) => Rotation::CounterNeutral,
                    (Rotation::CounterNeutral, Rotation::Clockwise) => Rotation::Neutral,
                    (Rotation::CounterNeutral, Rotation::CounterClockwise) => {
                        Rotation::CounterClockwise
                    }
                    (Rotation::CounterClockwise, Rotation::Clockwise) => Rotation::Neutral,
                    (Rotation::CounterClockwise, Rotation::CounterClockwise) => {
                        Rotation::CounterClockwise
                    }

                    // Neutrals are illegal for right side
                    (Rotation::Clockwise, Rotation::Neutral) => todo!(),
                    (Rotation::Clockwise, Rotation::CounterNeutral) => todo!(),
                    (Rotation::Neutral, Rotation::Neutral) => todo!(),
                    (Rotation::Neutral, Rotation::CounterNeutral) => todo!(),
                    (Rotation::CounterNeutral, Rotation::Neutral) => todo!(),
                    (Rotation::CounterNeutral, Rotation::CounterNeutral) => todo!(),
                    (Rotation::CounterClockwise, Rotation::Neutral) => todo!(),
                    (Rotation::CounterClockwise, Rotation::CounterNeutral) => todo!(),
                };

                rotation_state = rotation_correction;

                let rotation_correction = match rotation_state {
                    Rotation::Clockwise => 1,
                    Rotation::CounterClockwise => -1,
                    _ => 0,
                };
                last_direction = step.direction;

                *last = match step.direction {
                    Direction::East => {
                        Point::new(last.x + step.length as i32 + rotation_correction, last.y)
                    }
                    Direction::West => {
                        Point::new(last.x - (step.length as i32 + rotation_correction), last.y)
                    }
                    Direction::South => {
                        Point::new(last.x, last.y + step.length as i32 + rotation_correction)
                    }
                    Direction::North => {
                        Point::new(last.x, last.y - (step.length as i32 + rotation_correction))
                    }
                };
                println!("{},{} [{rotation_correction}]", last.x, last.y);
                Some(*last)
            }),
    );
    points.push(IPoint::origin());

    // points
    //     .iter()
    //     .for_each(|p| println!("{},{} [{rotation_correction}]", p.x, p.y));

    let total = points
        .windows(2)
        .map(|pairs| pairs[0].x as i64 * pairs[1].y as i64 - pairs[1].x as i64 * pairs[0].y as i64)
        .sum::<i64>();
    total.abs() as u64 / 2
    // let first = plan.intructions.first().unwrap();
    // plan.intructions.append(other)
    // plan.intructions.iter().

    // let mut row_map = HashMap::<i32, Vec<(i32, Orientation)>>::new();
    // let mut position = IPoint::origin();
    // let mut perimeter = 0;
    // for step in plan.intructions.iter() {
    //     fn insert_horizontal(
    //         row_map: &mut HashMap<i32, Vec<(i32, Orientation)>>,
    //         y: i32,
    //         range: RangeInclusive<i32>,
    //     ) {
    //         match row_map.get_mut(&y) {
    //             Some(dug) => {
    //                 for x in range {
    //                     match dug.binary_search_by(|probe| probe.0.cmp(&x)) {
    //                         Ok(_) => unreachable!(),
    //                         Err(index) => dug.insert(index, (x, Orientation::empty())),
    //                     }
    //                 }
    //             }
    //             None => {
    //                 let _ = row_map.insert(y, range.map(|x| (x, Orientation::empty())).collect());
    //             }
    //         }
    //     }

    //     fn insert_vertical(
    //         row_map: &mut HashMap<i32, Vec<(i32, Orientation)>>,
    //         x: i32,
    //         range: RangeInclusive<i32>,
    //     ) {
    //         let start = *range.start();
    //         let end = *range.end();
    //         for y in range {
    //             let orientation = if y == start {
    //                 Orientation::South
    //             } else if y == end {
    //                 Orientation::North
    //             } else {
    //                 Orientation::North | Orientation::South
    //             };

    //             match row_map.get_mut(&y) {
    //                 Some(dug) => match dug.binary_search_by(|probe| probe.0.cmp(&x)) {
    //                     Ok(index) => {
    //                         unreachable!()
    //                     }
    //                     Err(index) => dug.insert(index, (x, orientation)),
    //                 },
    //                 None => {
    //                     let _ = row_map.insert(y, vec![(x, orientation)]);
    //                 }
    //             }
    //         }
    //     }

    //     perimeter += step.length as u32;
    //     position = match step.direction {
    //         Direction::East => {
    //             let new_position = Point::new(position.x + step.length as i32, position.y);
    //             insert_horizontal(
    //                 &mut row_map,
    //                 position.y,
    //                 position.x + 1..=new_position.x - 1,
    //             );
    //             new_position
    //         }
    //         Direction::West => {
    //             let new_position = Point::new(position.x - step.length as i32, position.y);
    //             insert_horizontal(
    //                 &mut row_map,
    //                 position.y,
    //                 new_position.x + 1..=position.x - 1,
    //             );
    //             new_position
    //         }
    //         Direction::South => {
    //             let new_position = Point::new(position.x, position.y + step.length as i32);
    //             insert_vertical(&mut row_map, position.x, position.y..=new_position.y);
    //             new_position
    //         }
    //         Direction::North => {
    //             let new_position = Point::new(position.x, position.y - step.length as i32);
    //             insert_vertical(&mut row_map, position.x, new_position.y..=position.y);
    //             new_position
    //         }
    //     };
    // }

    // row_map
    //     .iter()
    //     .map(|(y, dug)| {
    //         let mut loop_orientation = Orientation::empty();
    //         let mut is_inside = false;
    //         const NORTH_SOUTH: Orientation = Orientation::North.union(Orientation::South);

    //         let mut last_x = i32::MIN;
    //         let mut row_sum = 0;

    //         for cell in dug {
    //             let path_orientation = cell.1.intersection(NORTH_SOUTH);

    //             if !is_inside {
    //                 loop_orientation = path_orientation;
    //                 is_inside = true;
    //             } else {
    //                 row_sum += cell.0 - last_x - 1;
    //                 loop_orientation ^= path_orientation;
    //                 if loop_orientation.is_empty() {
    //                     is_inside = false;
    //                 }
    //             }
    //             last_x = cell.0;
    //         }

    //         row_sum
    //     })
    //     .sum::<i32>() as u32
    //     + perimeter as u32
    // 0
}

/// ```rust
/// use advent_of_code_2023::day18::*;
/// let input = parse(
/// r"R 6 (#70c710)
/// D 5 (#0dc571)
/// L 2 (#5713f0)
/// D 2 (#d2c081)
/// R 2 (#59c680)
/// D 2 (#411b91)
/// L 5 (#8ceee2)
/// U 2 (#caa173)
/// L 1 (#1b58a2)
/// U 2 (#caa171)
/// R 2 (#7807d2)
/// U 3 (#a77fa3)
/// L 2 (#015232)
/// U 2 (#7a21e3)");
/// assert_eq!(952408144115, part2(&input));
/// ```
pub fn part2(plan: &DigPlan) -> u64 {
    part1(&DigPlan {
        intructions: plan.intructions.iter().map(|instruction| 
                //instruction.colour
            DigInstruction{direction: match instruction.colour & 0b11 {
                0 => Direction::East,
                1 => Direction::South,
                2=>Direction::West,
                3=>Direction::North,
                _ => unreachable!()
            },
                colour: instruction.colour,
                length: instruction.colour >> 4
        }).collect()}    )
    }
