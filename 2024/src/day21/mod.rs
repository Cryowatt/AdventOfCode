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
            DoorCodeKey::D9 => &[RobotKey::Right, RobotKey::Up, RobotKey::Up, RobotKey::A],
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
            RobotKey::Right => &[RobotKey::Right, RobotKey::Down, RobotKey::A],
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
            RobotKey::A => &[RobotKey::Right, RobotKey::Up, RobotKey::A],
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
/// //assert_eq!(154115708116294, part2(&input));
/// ```
pub fn part2(input: &InputType) -> u64 {
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

            sequence_length * code_value
        })
        .sum::<u64>()
}
