use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

use advent::*;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
}

impl From<DoorCodeKey> for u32 {
    fn from(value: DoorCodeKey) -> Self {
        match value {
            DoorCodeKey::D0 => 0,
            DoorCodeKey::D1 => 1,
            DoorCodeKey::D2 => 2,
            DoorCodeKey::D3 => 3,
            DoorCodeKey::D4 => 4,
            DoorCodeKey::D5 => 5,
            DoorCodeKey::D6 => 6,
            DoorCodeKey::D7 => 7,
            DoorCodeKey::D8 => 8,
            DoorCodeKey::D9 => 9,
            DoorCodeKey::A => panic!("A is non-value"),
        }
    }
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

fn door_path(from: &DoorCodeKey, to: &DoorCodeKey) -> &'static [RobotKey] {
    match from {
        DoorCodeKey::D7 => match to {
            DoorCodeKey::A => &[
                RobotKey::Right,
                RobotKey::Right,
                RobotKey::Down,
                RobotKey::Down,
                RobotKey::Down,
                RobotKey::A,
            ],
            DoorCodeKey::D0 => &[
                RobotKey::Right,
                RobotKey::Down,
                RobotKey::Down,
                RobotKey::Down,
                RobotKey::A,
            ],
            DoorCodeKey::D3 => &[
                RobotKey::Down,
                RobotKey::Down,
                RobotKey::Right,
                RobotKey::Right,
                RobotKey::A,
            ],
            DoorCodeKey::D2 => &[RobotKey::Down, RobotKey::Down, RobotKey::Right, RobotKey::A],
            DoorCodeKey::D1 => &[RobotKey::Down, RobotKey::Down, RobotKey::A],
            DoorCodeKey::D6 => &[
                RobotKey::Down,
                RobotKey::Right,
                RobotKey::Right,
                RobotKey::A,
            ],
            DoorCodeKey::D5 => &[RobotKey::Down, RobotKey::Right, RobotKey::A],
            DoorCodeKey::D4 => &[RobotKey::Down, RobotKey::A],
            DoorCodeKey::D9 => &[RobotKey::Right, RobotKey::Right, RobotKey::A],
            DoorCodeKey::D8 => &[RobotKey::Right, RobotKey::A],
            DoorCodeKey::D7 => &[RobotKey::A],
        },
        DoorCodeKey::D8 => match to {
            DoorCodeKey::A => &[
                RobotKey::Down,
                RobotKey::Down,
                RobotKey::Down,
                RobotKey::Right,
                RobotKey::A,
            ],
            DoorCodeKey::D0 => &[RobotKey::Down, RobotKey::Down, RobotKey::Down, RobotKey::A],
            DoorCodeKey::D3 => &[RobotKey::Down, RobotKey::Down, RobotKey::Right, RobotKey::A],
            DoorCodeKey::D2 => &[RobotKey::Down, RobotKey::Down, RobotKey::A],
            DoorCodeKey::D1 => &[RobotKey::Left, RobotKey::Down, RobotKey::Down, RobotKey::A],
            DoorCodeKey::D6 => &[RobotKey::Down, RobotKey::Right, RobotKey::A],
            DoorCodeKey::D5 => &[RobotKey::Down, RobotKey::A],
            DoorCodeKey::D4 => &[RobotKey::Left, RobotKey::Down, RobotKey::A],
            DoorCodeKey::D9 => &[RobotKey::Right, RobotKey::A],
            DoorCodeKey::D8 => &[RobotKey::A],
            DoorCodeKey::D7 => &[RobotKey::Left, RobotKey::A],
        },
        DoorCodeKey::D9 => match to {
            DoorCodeKey::A => &[RobotKey::Down, RobotKey::Down, RobotKey::Down, RobotKey::A],
            DoorCodeKey::D0 => &[
                RobotKey::Left,
                RobotKey::Down,
                RobotKey::Down,
                RobotKey::Down,
                RobotKey::A,
            ],
            DoorCodeKey::D3 => &[RobotKey::Down, RobotKey::Down, RobotKey::A],
            DoorCodeKey::D2 => &[RobotKey::Left, RobotKey::Down, RobotKey::Down, RobotKey::A],
            DoorCodeKey::D1 => &[
                RobotKey::Left,
                RobotKey::Left,
                RobotKey::Down,
                RobotKey::Down,
                RobotKey::A,
            ],
            DoorCodeKey::D6 => &[RobotKey::Down, RobotKey::A],
            DoorCodeKey::D5 => &[RobotKey::Left, RobotKey::Down, RobotKey::A],
            DoorCodeKey::D4 => &[RobotKey::Left, RobotKey::Left, RobotKey::Down, RobotKey::A],
            DoorCodeKey::D9 => &[RobotKey::A],
            DoorCodeKey::D8 => &[RobotKey::Left, RobotKey::A],
            DoorCodeKey::D7 => &[RobotKey::Left, RobotKey::Left, RobotKey::A],
        },
        DoorCodeKey::D4 => match to {
            DoorCodeKey::A => &[
                RobotKey::Right,
                RobotKey::Right,
                RobotKey::Down,
                RobotKey::Down,
                RobotKey::A,
            ],
            DoorCodeKey::D0 => &[RobotKey::Right, RobotKey::Down, RobotKey::Down, RobotKey::A],
            DoorCodeKey::D3 => &[
                RobotKey::Down,
                RobotKey::Right,
                RobotKey::Right,
                RobotKey::A,
            ],
            DoorCodeKey::D2 => &[RobotKey::Down, RobotKey::Right, RobotKey::A],
            DoorCodeKey::D1 => &[RobotKey::Down, RobotKey::A],
            DoorCodeKey::D6 => &[RobotKey::Right, RobotKey::Right, RobotKey::A],
            DoorCodeKey::D5 => &[RobotKey::Right, RobotKey::A],
            DoorCodeKey::D4 => &[RobotKey::A],
            DoorCodeKey::D9 => &[RobotKey::Right, RobotKey::Right, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D8 => &[RobotKey::Right, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D7 => &[RobotKey::Up, RobotKey::A],
        },
        DoorCodeKey::D5 => match to {
            DoorCodeKey::A => &[RobotKey::Down, RobotKey::Down, RobotKey::Right, RobotKey::A],
            DoorCodeKey::D0 => &[RobotKey::Down, RobotKey::Down, RobotKey::A],
            DoorCodeKey::D3 => &[RobotKey::Down, RobotKey::Right, RobotKey::A],
            DoorCodeKey::D2 => &[RobotKey::Down, RobotKey::A],
            DoorCodeKey::D1 => &[RobotKey::Left, RobotKey::Down, RobotKey::A],
            DoorCodeKey::D6 => &[RobotKey::Right, RobotKey::A],
            DoorCodeKey::D5 => &[RobotKey::A],
            DoorCodeKey::D4 => &[RobotKey::Left, RobotKey::A],
            DoorCodeKey::D9 => &[RobotKey::Right, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D8 => &[RobotKey::Up, RobotKey::A],
            DoorCodeKey::D7 => &[RobotKey::Left, RobotKey::Up, RobotKey::A],
        },
        DoorCodeKey::D6 => match to {
            DoorCodeKey::A => &[RobotKey::Down, RobotKey::Down, RobotKey::A],
            DoorCodeKey::D0 => &[RobotKey::Left, RobotKey::Down, RobotKey::Down, RobotKey::A],
            DoorCodeKey::D3 => &[RobotKey::Down, RobotKey::A],
            DoorCodeKey::D2 => &[RobotKey::Left, RobotKey::Down, RobotKey::A],
            DoorCodeKey::D1 => &[RobotKey::Left, RobotKey::Left, RobotKey::Down, RobotKey::A],
            DoorCodeKey::D6 => &[RobotKey::A],
            DoorCodeKey::D5 => &[RobotKey::Left, RobotKey::A],
            DoorCodeKey::D4 => &[RobotKey::Left, RobotKey::Left, RobotKey::A],
            DoorCodeKey::D9 => &[RobotKey::Up, RobotKey::A],
            DoorCodeKey::D8 => &[RobotKey::Left, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D7 => &[RobotKey::Left, RobotKey::Left, RobotKey::Up, RobotKey::A],
        },
        DoorCodeKey::D1 => match to {
            DoorCodeKey::A => &[
                RobotKey::Right,
                RobotKey::Right,
                RobotKey::Down,
                RobotKey::A,
            ],
            DoorCodeKey::D0 => &[RobotKey::Right, RobotKey::Down, RobotKey::A],
            DoorCodeKey::D3 => &[RobotKey::Right, RobotKey::Right, RobotKey::A],
            DoorCodeKey::D2 => &[RobotKey::Right, RobotKey::A],
            DoorCodeKey::D1 => &[RobotKey::A],
            DoorCodeKey::D6 => &[RobotKey::Right, RobotKey::Right, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D5 => &[RobotKey::Right, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D4 => &[RobotKey::Up, RobotKey::A],
            DoorCodeKey::D9 => &[
                RobotKey::Right,
                RobotKey::Right,
                RobotKey::Up,
                RobotKey::Up,
                RobotKey::A,
            ],
            DoorCodeKey::D8 => &[RobotKey::Right, RobotKey::Up, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D7 => &[RobotKey::Up, RobotKey::Up, RobotKey::A],
        },
        DoorCodeKey::D2 => match to {
            DoorCodeKey::A => &[RobotKey::Down, RobotKey::Right, RobotKey::A],
            DoorCodeKey::D0 => &[RobotKey::Down, RobotKey::A],
            DoorCodeKey::D3 => &[RobotKey::Right, RobotKey::A],
            DoorCodeKey::D2 => &[RobotKey::A],
            DoorCodeKey::D1 => &[RobotKey::Left, RobotKey::A],
            DoorCodeKey::D6 => &[RobotKey::Right, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D5 => &[RobotKey::Up, RobotKey::A],
            DoorCodeKey::D4 => &[RobotKey::Left, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D9 => &[RobotKey::Up, RobotKey::Up, RobotKey::Right, RobotKey::A],
            DoorCodeKey::D8 => &[RobotKey::Up, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D7 => &[RobotKey::Left, RobotKey::Up, RobotKey::Up, RobotKey::A],
        },
        DoorCodeKey::D3 => match to {
            DoorCodeKey::A => &[RobotKey::Down, RobotKey::A],
            DoorCodeKey::D0 => &[RobotKey::Left, RobotKey::Down, RobotKey::A],
            DoorCodeKey::D3 => &[RobotKey::A],
            DoorCodeKey::D2 => &[RobotKey::Left, RobotKey::A],
            DoorCodeKey::D1 => &[RobotKey::Left, RobotKey::Left, RobotKey::A],
            DoorCodeKey::D6 => &[RobotKey::Up, RobotKey::A],
            DoorCodeKey::D5 => &[RobotKey::Left, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D4 => &[RobotKey::Left, RobotKey::Left, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D9 => &[RobotKey::Up, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D8 => &[RobotKey::Left, RobotKey::Up, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D7 => &[
                RobotKey::Left,
                RobotKey::Left,
                RobotKey::Up,
                RobotKey::Up,
                RobotKey::A,
            ],
        },
        DoorCodeKey::D0 => match to {
            DoorCodeKey::A => &[RobotKey::Right, RobotKey::A],
            DoorCodeKey::D0 => &[RobotKey::A],
            DoorCodeKey::D3 => &[RobotKey::Right, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D2 => &[RobotKey::Up, RobotKey::A],
            DoorCodeKey::D1 => &[RobotKey::Up, RobotKey::Left, RobotKey::A],
            DoorCodeKey::D6 => &[RobotKey::Right, RobotKey::Up, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D5 => &[RobotKey::Up, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D4 => &[RobotKey::Up, RobotKey::Up, RobotKey::Left, RobotKey::A],
            DoorCodeKey::D9 => &[
                RobotKey::Right,
                RobotKey::Up,
                RobotKey::Up,
                RobotKey::Up,
                RobotKey::A,
            ],
            DoorCodeKey::D8 => &[RobotKey::Up, RobotKey::Up, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D7 => &[
                RobotKey::Up,
                RobotKey::Up,
                RobotKey::Up,
                RobotKey::Left,
                RobotKey::A,
            ],
        },
        DoorCodeKey::A => match to {
            DoorCodeKey::A => &[RobotKey::A],
            DoorCodeKey::D0 => &[RobotKey::Left, RobotKey::A],
            DoorCodeKey::D3 => &[RobotKey::Up, RobotKey::A],
            DoorCodeKey::D2 => &[RobotKey::Left, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D1 => &[RobotKey::Up, RobotKey::Left, RobotKey::Left, RobotKey::A],
            DoorCodeKey::D6 => &[RobotKey::Up, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D5 => &[RobotKey::Left, RobotKey::Up, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D4 => &[
                RobotKey::Up,
                RobotKey::Up,
                RobotKey::Left,
                RobotKey::Left,
                RobotKey::A,
            ],
            DoorCodeKey::D9 => &[RobotKey::Up, RobotKey::Up, RobotKey::Up, RobotKey::A],
            DoorCodeKey::D8 => &[
                RobotKey::Left,
                RobotKey::Up,
                RobotKey::Up,
                RobotKey::Up,
                RobotKey::A,
            ],
            DoorCodeKey::D7 => &[
                RobotKey::Up,
                RobotKey::Up,
                RobotKey::Up,
                RobotKey::Left,
                RobotKey::Left,
                RobotKey::A,
            ],
        },
    }
}

fn robot_path(from: &RobotKey, to: &RobotKey) -> &'static [RobotKey] {
    match from {
        RobotKey::Up => match to {
            RobotKey::Right => &[RobotKey::Down, RobotKey::Right, RobotKey::A],
            RobotKey::Down => &[RobotKey::Down, RobotKey::A],
            RobotKey::Left => &[RobotKey::Down, RobotKey::Left, RobotKey::A],
            RobotKey::A => &[RobotKey::Right, RobotKey::A],
            RobotKey::Up => &[RobotKey::A],
        },
        RobotKey::A => match to {
            RobotKey::Right => &[RobotKey::Down, RobotKey::A],
            RobotKey::Down => &[RobotKey::Left, RobotKey::Down, RobotKey::A],
            RobotKey::Left => &[RobotKey::Down, RobotKey::Left, RobotKey::Left, RobotKey::A],
            RobotKey::A => &[RobotKey::A],
            RobotKey::Up => &[RobotKey::Left, RobotKey::A],
        },
        RobotKey::Left => match to {
            RobotKey::Right => &[RobotKey::Right, RobotKey::Right, RobotKey::A],
            RobotKey::Down => &[RobotKey::Right, RobotKey::A],
            RobotKey::Left => &[RobotKey::A],
            RobotKey::A => &[RobotKey::Right, RobotKey::Right, RobotKey::Up, RobotKey::A],
            RobotKey::Up => &[RobotKey::Right, RobotKey::Up, RobotKey::A],
        },
        RobotKey::Down => match to {
            RobotKey::Right => &[RobotKey::Right, RobotKey::A],
            RobotKey::Down => &[RobotKey::A],
            RobotKey::Left => &[RobotKey::Left, RobotKey::A],
            RobotKey::A => &[RobotKey::Up, RobotKey::Right, RobotKey::A],
            RobotKey::Up => &[RobotKey::Up, RobotKey::A],
        },
        RobotKey::Right => match to {
            RobotKey::Right => &[RobotKey::A],
            RobotKey::Down => &[RobotKey::Left, RobotKey::A],
            RobotKey::Left => &[RobotKey::Left, RobotKey::Left, RobotKey::A],
            RobotKey::A => &[RobotKey::Up, RobotKey::A],
            RobotKey::Up => &[RobotKey::Left, RobotKey::Up, RobotKey::A],
        },
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
pub fn part1(input: &InputType) -> u32 {
    input
        .iter()
        .map(|code| {
            let code_value = code.iter().take(3).fold(0u32, |a, &key| {
                (a * 10) + <DoorCodeKey as Into<u32>>::into(key)
            });
            let sequence_length: u32 = code
                .iter()
                .scan(DoorCodeKey::A, |from, to| {
                    let from_copy = *from;
                    *from = *to;
                    Some(door_path(&from_copy, to))
                })
                .flatten()
                .scan(RobotKey::A, |from, to| {
                    let from_copy = *from;
                    *from = *to;
                    Some(robot_path(&from_copy, to))
                })
                .flatten()
                .scan(RobotKey::A, |from, to| {
                    let from_copy = *from;
                    *from = *to;
                    Some(robot_path(&from_copy, to))
                })
                .flatten()
                .count() as u32;

            sequence_length * code_value
        })
        .sum()
}

/// ```rust
/// use advent_of_code_2024::day21::*;
/// let input = parse(
/// r"029A
/// 980A
/// 179A
/// 456A
/// 379A");
/// assert_eq!(154115708116294, part2(&input));
/// ```
pub fn part2(input: &InputType) -> u64 {
    println!(
        "{:?}",
        door_path(&DoorCodeKey::A, &DoorCodeKey::D0)
            .iter()
            .scan(RobotKey::A, |from, to| {
                let from_copy = *from;
                *from = *to;
                Some(robot_path(&from_copy, to))
            })
            .flatten()
            .collect::<Vec<_>>()
    );
    println!(
        "{:?}",
        door_path(&DoorCodeKey::D0, &DoorCodeKey::D2)
            .iter()
            .scan(RobotKey::A, |from, to| {
                let from_copy = *from;
                *from = *to;
                Some(robot_path(&from_copy, to))
            })
            .flatten()
            .collect::<Vec<_>>()
    );
    println!(
        "{:?}",
        door_path(&DoorCodeKey::D2, &DoorCodeKey::D9)
            .iter()
            .scan(RobotKey::A, |from, to| {
                let from_copy = *from;
                *from = *to;
                Some(robot_path(&from_copy, to))
            })
            .flatten()
            .collect::<Vec<_>>()
    );
    println!(
        "{:?}",
        door_path(&DoorCodeKey::D9, &DoorCodeKey::A)
            .iter()
            .scan(RobotKey::A, |from, to| {
                let from_copy = *from;
                *from = *to;
                Some(robot_path(&from_copy, to))
            })
            .flatten()
            .collect::<Vec<_>>()
    );

    // optimize();
    const ALL_KEYS: [RobotKey; 5] = [
        RobotKey::A,
        RobotKey::Left,
        RobotKey::Right,
        RobotKey::Up,
        RobotKey::Down,
    ];

    let mut key_seq_length = HashMap::new();
    for from_code in ALL_KEYS {
        for to_code in ALL_KEYS {
            key_seq_length.insert(
                (from_code, to_code),
                robot_path(&from_code, &to_code).len() as u64,
            );
        }
    }

    for _ in 0..24 {
        let source_table = key_seq_length.clone();
        key_seq_length.clear();
        for from_code in ALL_KEYS {
            for to_code in ALL_KEYS {
                key_seq_length.insert(
                    (from_code, to_code),
                    robot_path(&from_code, &to_code)
                        .iter()
                        .scan(RobotKey::A, |from, to| {
                            let from_copy = *from;
                            *from = *to;
                            Some(source_table[&(from_copy, *to)])
                        })
                        .sum::<u64>(),
                );
            }
        }

        // println!("FK :: {:?}", key_seq_length);
    }

    input
        .iter()
        .map(|code| {
            let code_value = code.iter().take(3).fold(0u64, |a, &key| {
                (a * 10) + <DoorCodeKey as Into<u32>>::into(key) as u64
            });

            let sequence_length = code
                .iter()
                .scan(DoorCodeKey::A, |from, to| {
                    let from_copy = *from;
                    *from = *to;
                    Some(door_path(&from_copy, to))
                })
                .flatten()
                .scan(RobotKey::A, |from, to| {
                    let from_copy = *from;
                    *from = *to;
                    Some(key_seq_length[&(from_copy, *to)])
                })
                .sum::<u64>();
            println!(
                "{} * {} = {}",
                sequence_length,
                code_value,
                sequence_length * code_value
            );
            sequence_length * code_value
        })
        .sum::<u64>()
}

fn optimize() {
    const ALL_KEYS: [RobotKey; 5] = [
        RobotKey::A,
        RobotKey::Left,
        RobotKey::Right,
        RobotKey::Up,
        RobotKey::Down,
    ];

    let mut key_positions = HashMap::new();
    key_positions.insert(RobotKey::Up, IPoint::new(1, 0));
    key_positions.insert(RobotKey::A, IPoint::new(2, 0));
    key_positions.insert(RobotKey::Left, IPoint::new(0, 1));
    key_positions.insert(RobotKey::Down, IPoint::new(1, 1));
    key_positions.insert(RobotKey::Right, IPoint::new(2, 1));

    const ALL_CODES: [DoorCodeKey; 11] = [
        DoorCodeKey::A,
        DoorCodeKey::D0,
        DoorCodeKey::D1,
        DoorCodeKey::D2,
        DoorCodeKey::D3,
        DoorCodeKey::D4,
        DoorCodeKey::D5,
        DoorCodeKey::D6,
        DoorCodeKey::D7,
        DoorCodeKey::D8,
        DoorCodeKey::D9,
    ];

    let mut code_positions = HashMap::new();
    code_positions.insert(DoorCodeKey::D7, IPoint::new(0, 0));
    code_positions.insert(DoorCodeKey::D8, IPoint::new(1, 0));
    code_positions.insert(DoorCodeKey::D9, IPoint::new(2, 0));
    code_positions.insert(DoorCodeKey::D4, IPoint::new(0, 1));
    code_positions.insert(DoorCodeKey::D5, IPoint::new(1, 1));
    code_positions.insert(DoorCodeKey::D6, IPoint::new(2, 1));
    code_positions.insert(DoorCodeKey::D1, IPoint::new(0, 2));
    code_positions.insert(DoorCodeKey::D2, IPoint::new(1, 2));
    code_positions.insert(DoorCodeKey::D3, IPoint::new(2, 2));
    code_positions.insert(DoorCodeKey::D0, IPoint::new(1, 3));
    code_positions.insert(DoorCodeKey::A, IPoint::new(2, 3));

    let mut first_order_keys = HashMap::new();
    let mut min_key_route = HashMap::new();
    let mut min_2nd_key_route = HashMap::new();
    let mut min_3rd_key_route = HashMap::new();

    for from in ALL_KEYS {
        for to in ALL_KEYS {
            let start = key_positions[&from];
            let offset = key_positions[&to] - key_positions[&from];
            let sequence = &mut vec![];
            let paths = enumerate_paths(start, offset, IPoint::origin(), sequence);

            // for seq in paths.iter() {
            //     println!("{from} {to} {:?}", seq, from = from, to = to);
            // }
            min_key_route.insert((from, to), paths[0].len());
            first_order_keys.insert((from, to), paths);
        }
    }

    println!("First Order");

    for from in ALL_KEYS {
        for to in ALL_KEYS {
            let paths = first_order_keys.get(&(from, to)).unwrap();
            let best = paths.iter().min_by_key(|path| {
                // for path in paths {
                path.iter()
                    .scan(RobotKey::A, |from, to| {
                        let length = min_key_route[&(*from, *to)];
                        *from = *to;
                        Some(length)
                    })
                    .sum::<usize>()
            });

            min_2nd_key_route.insert((from, to), best.unwrap());

            println!("Best path {:?}->{:?} {:?}", from, to, best.unwrap());
        }
    }

    println!("Second Order");

    for from in ALL_KEYS {
        for to in ALL_KEYS {
            let paths = first_order_keys.get(&(from, to)).unwrap();
            let best = paths.iter().min_by_key(|path| {
                // for path in paths {
                path.iter()
                    .scan(RobotKey::A, |from, to| {
                        let length = min_2nd_key_route[&(*from, *to)];
                        *from = *to;
                        Some(length)
                    })
                    .flatten()
                    .scan(RobotKey::A, |from, to| {
                        let length = min_key_route[&(*from, *to)];
                        *from = *to;
                        Some(length)
                    })
                    .sum::<usize>()
            });

            min_3rd_key_route.insert((from, to), best.unwrap());

            println!("Best path {:?}->{:?} {:?}", from, to, best.unwrap());
        }
    }

    println!("Second Order");

    for from in ALL_KEYS {
        for to in ALL_KEYS {
            let paths = first_order_keys.get(&(from, to)).unwrap();
            let best = paths.iter().min_by_key(|path| {
                // for path in paths {
                path.iter()
                    .scan(RobotKey::A, |from, to| {
                        let length = min_2nd_key_route[&(*from, *to)];
                        *from = *to;
                        Some(length)
                    })
                    .flatten()
                    .scan(RobotKey::A, |from, to| {
                        let length = min_key_route[&(*from, *to)];
                        *from = *to;
                        Some(length)
                    })
                    .sum::<usize>()
            });

            println!("Best path {:?}->{:?} {:?}", from, to, best.unwrap());
        }
    }

    println!("Best door");

    for from in ALL_CODES {
        for to in ALL_CODES {
            let start = code_positions[&from];
            let offset = code_positions[&to] - code_positions[&from];
            let sequence = &mut vec![];
            let paths = enumerate_paths(start, offset, IPoint::new(0, 3), sequence);

            let seq = paths.iter().min_by_key(|path| {
                path.iter()
                    .scan(RobotKey::A, |from, to| {
                        let length = min_2nd_key_route[&(*from, *to)];
                        *from = *to;
                        Some(length)
                    })
                    .flatten()
                    .scan(RobotKey::A, |from, to| {
                        let length = min_key_route[&(*from, *to)];
                        *from = *to;
                        Some(length)
                    })
                    .sum::<usize>()
            });

            println!("{from:?} {to:?} {:?}", seq, from = from, to = to);

            // min_key_route.insert((from, to), paths[0].len());
            // first_order_keys.insert((from, to), paths);
        }
    }

    fn enumerate_paths(
        start: IPoint,
        offset: IPoint,
        illegal: IPoint,
        sequence: &mut Vec<RobotKey>,
    ) -> Vec<Vec<RobotKey>> {
        let mut valid_paths = vec![];
        if start == illegal {
            // Illegal
            valid_paths
        } else if offset.x == 0 && offset.y == 0 {
            sequence.push(RobotKey::A);
            valid_paths.push(sequence.clone());
            sequence.pop();
            valid_paths
        } else {
            if offset.x > 0 {
                sequence.push(RobotKey::Right);
                let paths = enumerate_paths(start.right(), offset.left(), illegal, sequence);
                valid_paths.extend(paths);
                sequence.pop();
            } else if offset.x < 0 {
                sequence.push(RobotKey::Left);
                let paths = enumerate_paths(start.left(), offset.right(), illegal, sequence);
                valid_paths.extend(paths);
                sequence.pop();
            }

            if offset.y > 0 {
                sequence.push(RobotKey::Down);
                let paths = enumerate_paths(start.down(), offset.up(), illegal, sequence);
                valid_paths.extend(paths);
                sequence.pop();
            } else if offset.y < 0 {
                sequence.push(RobotKey::Up);
                let paths = enumerate_paths(start.up(), offset.down(), illegal, sequence);
                valid_paths.extend(paths);
                sequence.pop();
            }

            valid_paths
        }
    }
}
