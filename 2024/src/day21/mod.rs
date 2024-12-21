use std::{
    array,
    fmt::{Display, Write},
    str::FromStr,
};

use advent::*;
use array2d::Array2D;

advent_day!(Day21, parse, Vec<[DoorCodeKey; 4]>, part1, part2);

pub fn parse(input: &str) -> InputType {
    input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&c| c.into())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RobotKey {
    Up,
    Left,
    Right,
    Down,
    A,
}

impl Display for RobotKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            RobotKey::Up => '^',
            RobotKey::Left => '<',
            RobotKey::Right => '>',
            RobotKey::Down => 'v',
            RobotKey::A => 'A',
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DoorCodeKey {
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    A,
    Blank,
}

impl From<u8> for DoorCodeKey {
    fn from(value: u8) -> Self {
        match value {
            b'0' => DoorCodeKey::D0,
            b'1' => DoorCodeKey::D1,
            b'2' => DoorCodeKey::D2,
            b'3' => DoorCodeKey::D3,
            b'4' => DoorCodeKey::D4,
            b'5' => DoorCodeKey::D5,
            b'6' => DoorCodeKey::D6,
            b'7' => DoorCodeKey::D7,
            b'8' => DoorCodeKey::D8,
            b'9' => DoorCodeKey::D9,
            b'A' => DoorCodeKey::A,
            _ => panic!("Illegal door code"),
        }
    }
}

/// Each call gets one step closer to the destination
fn door_code_step(from: DoorCodeKey, to: DoorCodeKey) -> (DoorCodeKey, RobotKey) {
    // 789
    // 456
    // 123
    // .0A
    match from {
        DoorCodeKey::D0 => match to {
            DoorCodeKey::D0 => (DoorCodeKey::D0, RobotKey::A),
            DoorCodeKey::A => (DoorCodeKey::A, RobotKey::Right),
            DoorCodeKey::Blank => (DoorCodeKey::Blank, RobotKey::Left),
            _ => (DoorCodeKey::D2, RobotKey::Up),
        },
        DoorCodeKey::D1 => match to {
            DoorCodeKey::D1 => (DoorCodeKey::D1, RobotKey::A),
            DoorCodeKey::D2 | DoorCodeKey::D3 | DoorCodeKey::D0 | DoorCodeKey::A => {
                (DoorCodeKey::D2, RobotKey::Right)
            }
            DoorCodeKey::Blank => (DoorCodeKey::Blank, RobotKey::Down),
            _ => (DoorCodeKey::D4, RobotKey::Up),
        },
        DoorCodeKey::D2 => match to {
            DoorCodeKey::D2 => (DoorCodeKey::D2, RobotKey::A),
            DoorCodeKey::D1 => (DoorCodeKey::D1, RobotKey::Left),
            DoorCodeKey::D3 => (DoorCodeKey::D1, RobotKey::Right),
            DoorCodeKey::D0 | DoorCodeKey::A | DoorCodeKey::Blank => {
                (DoorCodeKey::D0, RobotKey::Down)
            }
            _ => (DoorCodeKey::D5, RobotKey::Up),
        },
        DoorCodeKey::D3 => match to {
            DoorCodeKey::D3 => (DoorCodeKey::D3, RobotKey::A),
            DoorCodeKey::D1 | DoorCodeKey::D2 => (DoorCodeKey::D2, RobotKey::Left),
            DoorCodeKey::D0 | DoorCodeKey::A | DoorCodeKey::Blank => {
                (DoorCodeKey::A, RobotKey::Down)
            }
            _ => (DoorCodeKey::D6, RobotKey::Up),
        },
        DoorCodeKey::D4 => match to {
            DoorCodeKey::D4 => (DoorCodeKey::D4, RobotKey::A),
            DoorCodeKey::D5 | DoorCodeKey::D6 => (DoorCodeKey::D5, RobotKey::Right),
            DoorCodeKey::D7 | DoorCodeKey::D8 | DoorCodeKey::D9 => (DoorCodeKey::D7, RobotKey::Up),
            _ => (DoorCodeKey::D1, RobotKey::Down),
        },
        DoorCodeKey::D5 => match to {
            DoorCodeKey::D5 => (DoorCodeKey::D5, RobotKey::A),
            DoorCodeKey::D4 => (DoorCodeKey::D4, RobotKey::Left),
            DoorCodeKey::D6 => (DoorCodeKey::D6, RobotKey::Right),
            DoorCodeKey::D7 | DoorCodeKey::D8 | DoorCodeKey::D9 => (DoorCodeKey::D8, RobotKey::Up),
            _ => (DoorCodeKey::D2, RobotKey::Down),
        },
        DoorCodeKey::D6 => match to {
            DoorCodeKey::D6 => (DoorCodeKey::D6, RobotKey::A),
            DoorCodeKey::D4 | DoorCodeKey::D5 => (DoorCodeKey::D5, RobotKey::Left),
            DoorCodeKey::D7 | DoorCodeKey::D8 | DoorCodeKey::D9 => (DoorCodeKey::D9, RobotKey::Up),
            _ => (DoorCodeKey::D3, RobotKey::Down),
        },
        DoorCodeKey::D7 => match to {
            DoorCodeKey::D7 => (DoorCodeKey::D7, RobotKey::A),
            DoorCodeKey::D8 | DoorCodeKey::D9 => (DoorCodeKey::D8, RobotKey::Right),
            _ => (DoorCodeKey::D4, RobotKey::Down),
        },
        DoorCodeKey::D8 => match to {
            DoorCodeKey::D8 => (DoorCodeKey::D8, RobotKey::A),
            DoorCodeKey::D7 => (DoorCodeKey::D7, RobotKey::Left),
            DoorCodeKey::D9 => (DoorCodeKey::D9, RobotKey::Right),
            _ => (DoorCodeKey::D5, RobotKey::Down),
        },
        DoorCodeKey::D9 => match to {
            DoorCodeKey::D9 => (DoorCodeKey::D9, RobotKey::A),
            DoorCodeKey::D7 | DoorCodeKey::D8 => (DoorCodeKey::D8, RobotKey::Left),
            _ => (DoorCodeKey::D6, RobotKey::Down),
        },
        DoorCodeKey::A => match to {
            DoorCodeKey::A => (DoorCodeKey::A, RobotKey::A),
            DoorCodeKey::D0 | DoorCodeKey::Blank => (DoorCodeKey::D0, RobotKey::Left),
            _ => (DoorCodeKey::D3, RobotKey::Up),
        },
        DoorCodeKey::Blank => panic!("Robot panic on gap"),
    }
}

fn robot_key_step(from: RobotKey, to: RobotKey) -> (RobotKey, RobotKey) {
    // .^A
    // <v>

    match from {
        RobotKey::Up => todo!(),
        RobotKey::Left => todo!(),
        RobotKey::Right => todo!(),
        RobotKey::Down => todo!(),
        RobotKey::A => todo!(),
    }
}

/// ```rust
/// use advent_of_code_2024::day21::*;
/// let input = parse(
/// r"029A
/// 980A
/// 179A
/// 456A
/// 379A");
/// assert_eq!(126384, part1(&input));
/// ```
pub fn part1(input: &InputType) -> usize {
    for door_code_sequence in input {
        let mut door_code_position = DoorCodeKey::A;
        let mut door_bot_position = RobotKey::A;

        for &door_code in door_code_sequence {
            print!("{:?} ", door_code);
            let mut watchdog = 0;
            loop {
                let (next_position, robot_key) = door_code_step(door_code_position, door_code);
                door_code_position = next_position;

                print!("{}", robot_key);
                if robot_key == RobotKey::A {
                    // Key pressed, is done
                    break;
                }
                watchdog += 1;
                if watchdog > 10 {
                    panic!("Watchdog failure");
                }
            }
            println!();
        }
        println!();
    }
    69
}

/// ```rust
/// use advent_of_code_2024::day21::*;
/// let input = parse(
/// r"029A
/// 980A
/// 179A
/// 456A
/// 379A");
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &InputType) -> usize {
    0
}
