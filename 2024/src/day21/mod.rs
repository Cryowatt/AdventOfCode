use std::{
    array,
    fmt::{Display, Write},
    iter,
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

/// Each call gets one step closer to the destination
fn door_code_step(from: DoorCodeKey, to: &DoorCodeKey) -> (DoorCodeKey, RobotKey) {
    // 789
    // 456
    // 123
    // .0A
    match from {
        DoorCodeKey::D0 => match to {
            DoorCodeKey::D0 => (DoorCodeKey::D0, RobotKey::A),
            DoorCodeKey::A => (DoorCodeKey::A, RobotKey::Right),
            _ => (DoorCodeKey::D2, RobotKey::Up),
        },
        DoorCodeKey::D1 => match to {
            DoorCodeKey::D1 => (DoorCodeKey::D1, RobotKey::A),
            DoorCodeKey::D2 | DoorCodeKey::D3 | DoorCodeKey::D0 | DoorCodeKey::A => {
                (DoorCodeKey::D2, RobotKey::Right)
            }
            _ => (DoorCodeKey::D4, RobotKey::Up),
        },
        DoorCodeKey::D2 => match to {
            DoorCodeKey::D2 => (DoorCodeKey::D2, RobotKey::A),
            DoorCodeKey::D1 => (DoorCodeKey::D1, RobotKey::Left),
            (DoorCodeKey::D5 | DoorCodeKey::D8) => (DoorCodeKey::D5, RobotKey::Up),
            DoorCodeKey::D0 => (DoorCodeKey::D0, RobotKey::Down),
            _ => (DoorCodeKey::D3, RobotKey::Right),
        },
        DoorCodeKey::D3 => match to {
            DoorCodeKey::D3 => (DoorCodeKey::D3, RobotKey::A),
            DoorCodeKey::D1 | DoorCodeKey::D2 => (DoorCodeKey::D2, RobotKey::Left),
            DoorCodeKey::D0 | DoorCodeKey::A => (DoorCodeKey::A, RobotKey::Down),
            _ => (DoorCodeKey::D6, RobotKey::Up),
        },
        DoorCodeKey::D4 => match to {
            DoorCodeKey::D4 => (DoorCodeKey::D4, RobotKey::A),
            DoorCodeKey::D7 | DoorCodeKey::D8 | DoorCodeKey::D9 => (DoorCodeKey::D7, RobotKey::Up),
            DoorCodeKey::D1 => (DoorCodeKey::D1, RobotKey::Down),
            _ => (DoorCodeKey::D5, RobotKey::Right),
        },
        DoorCodeKey::D5 => match to {
            DoorCodeKey::D5 => (DoorCodeKey::D5, RobotKey::A),
            DoorCodeKey::D4 => (DoorCodeKey::D4, RobotKey::Left),
            DoorCodeKey::D3 | DoorCodeKey::D6 | DoorCodeKey::A => {
                (DoorCodeKey::D6, RobotKey::Right)
            }
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
            DoorCodeKey::D4 | DoorCodeKey::D1 => (DoorCodeKey::D4, RobotKey::Down),
            _ => (DoorCodeKey::D8, RobotKey::Right),
        },
        DoorCodeKey::D8 => match to {
            DoorCodeKey::D8 => (DoorCodeKey::D8, RobotKey::A),
            DoorCodeKey::D7 => (DoorCodeKey::D7, RobotKey::Left),
            DoorCodeKey::D9 | DoorCodeKey::D6 | DoorCodeKey::D3 | DoorCodeKey::A => {
                (DoorCodeKey::D9, RobotKey::Right)
            }
            _ => (DoorCodeKey::D5, RobotKey::Down),
        },
        DoorCodeKey::D9 => match to {
            DoorCodeKey::D9 => (DoorCodeKey::D9, RobotKey::A),
            DoorCodeKey::D7 | DoorCodeKey::D8 => (DoorCodeKey::D8, RobotKey::Left),
            _ => (DoorCodeKey::D6, RobotKey::Down),
        },
        DoorCodeKey::A => match to {
            DoorCodeKey::A => (DoorCodeKey::A, RobotKey::A),
            DoorCodeKey::D0 => (DoorCodeKey::D0, RobotKey::Left),
            _ => (DoorCodeKey::D3, RobotKey::Up),
        },
    }
}

fn robot_key_step(from: RobotKey, to: RobotKey) -> (RobotKey, RobotKey) {
    // [.][^][A]
    // [<][v][>]
    match from {
        RobotKey::Up => match to {
            RobotKey::Up => (RobotKey::Up, RobotKey::A),
            RobotKey::A => (RobotKey::A, RobotKey::Right),
            _ => (RobotKey::Down, RobotKey::Down),
        },
        RobotKey::Left => match to {
            RobotKey::Left => (RobotKey::Left, RobotKey::A),
            _ => (RobotKey::Down, RobotKey::Right),
        },
        RobotKey::Right => match to {
            RobotKey::Right => (RobotKey::Right, RobotKey::A),
            RobotKey::Left | RobotKey::Down => (RobotKey::Down, RobotKey::Left),
            RobotKey::Up | RobotKey::A => (RobotKey::A, RobotKey::Up),
        },
        RobotKey::Down => match to {
            RobotKey::Down => (RobotKey::Down, RobotKey::A),
            RobotKey::Up => (RobotKey::Up, RobotKey::Up),
            RobotKey::Left => (RobotKey::Left, RobotKey::Left),
            _ => (RobotKey::Right, RobotKey::Right),
        },
        RobotKey::A => match to {
            RobotKey::A => (RobotKey::A, RobotKey::A),
            RobotKey::Up => (RobotKey::Up, RobotKey::Left),
            _ => (RobotKey::Right, RobotKey::Down),
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
    fn door_bot<'a>(code: &'a [DoorCodeKey; 4]) -> impl Iterator<Item = RobotKey> + use<'a> {
        iter::from_coroutine(
            #[coroutine]
            move || {
                let mut door_key = DoorCodeKey::A;
                for key in code {
                    loop {
                        let (next_position, robot_key) = door_code_step(door_key, &key);
                        door_key = next_position;
                        yield robot_key;
                        if robot_key == RobotKey::A {
                            break;
                        }
                    }
                }
            },
        )
        // let code_value = code.iter().take(3).fold(0u32, |a, &key| {
        //     (a * 10) + <DoorCodeKey as Into<u32>>::into(key)
        // });

        // //     println!("{} * {}", sequence_length, code_value);
        // //     sequence_length * code_value
        // code_value
    }
    // door_check();
    // robot_check();

    // iter::from_fn(f)
    // let fk = iter::from_coroutine(
    //     #[coroutine]
    //     || {
    //         yield 1;
    //         yield 2;
    //         yield 3;
    //     },
    // );
    let door_bot_key = &mut RobotKey::A;
    let bot_bot_key = &mut RobotKey::A;
    input
        .iter()
        .take(1)
        .map(|code| {
            let sequence_length = door_bot(code).inspect(|k| print!("{}", k)).count();

            let code_value = code.iter().take(3).fold(0u32, |a, &key| {
                (a * 10) + <DoorCodeKey as Into<u32>>::into(key)
            });

            println!("  {} * {}", sequence_length, code_value);
            sequence_length as u32 * code_value
        })
        // .map(|code| {
        // let sequence_length = iter::from_coroutine(
        //     #[coroutine]
        //     || {
        //         for &key in code {
        //             loop {
        //                 let (next_position, robot_key) = door_code_step(*door_key, key);
        //                 *door_key = next_position;
        //                 yield robot_key;
        //                 if robot_key == RobotKey::A {
        //                     break;
        //                 }
        //             }
        //         }
        //     },
        // )
        // .flat_map(|key| {
        //     // print!(" [{}::{}]", *door_bot_key, key);
        //     iter::from_coroutine(
        //         #[coroutine]
        //         || loop {
        //             let (next_position, robot_key) = robot_key_step(*door_bot_key, key);
        //             print!("[[step {} {}]]", door_bot_key, next_position);
        //             *door_bot_key = next_position;
        //             yield robot_key;
        //             if robot_key == RobotKey::A {
        //                 break;
        //             }
        //         },
        //     )
        // })
        // .inspect(|key| print!("{}", key))
        // .flat_map(|key| {
        //     iter::from_coroutine(
        //         #[coroutine]
        //         move || loop {
        //             let (next_position, robot_key) = robot_key_step(bot_bot_key, key);
        //             bot_bot_key = next_position;
        //             yield robot_key;
        //             if robot_key == RobotKey::A {
        //                 break;
        //             }
        //         },
        //     )
        // })
        // let sequence_length = code
        //     .iter()
        //     .flat_map(|&key| {
        //         let mut current_key = DoorCodeKey::A;
        //         iter::from_coroutine(
        //             #[coroutine]
        //             move || loop {
        //                 let (next_position, robot_key) = door_code_step(current_key, key);
        //                 current_key = next_position;
        //                 yield robot_key;
        //                 if robot_key == RobotKey::A {
        //                     break;
        //                 }
        //             },
        //         )
        //     })
        //     // .flat_map(|key| {
        //     //     iter::from_coroutine(
        //     //         #[coroutine]
        //     //         move || {
        //     //             let mut current_key = RobotKey::A;
        //     //             loop {
        //     //                 let (next_position, robot_key) = robot_key_step(current_key, key);
        //     //                 current_key = next_position;
        //     //                 yield robot_key;
        //     //                 if robot_key == RobotKey::A {
        //     //                     break;
        //     //                 }
        //     //             }
        //     //         },
        //     //     )
        //     // })
        //     .count() as u32;
        //     //<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
        //     //v<Av<<Av<<AAAv<<AAv<Av<<Av<<AAAv<<AAv<<AAv<<AAAA
        //     let code_value = code.iter().take(3).fold(0u32, |a, &key| {
        //         (a * 10) + <DoorCodeKey as Into<u32>>::into(key)
        //     });
        //     println!("{} * {}", sequence_length, code_value);
        //     sequence_length * code_value
        //     // for &key in code {}
        //     // code.map(|key| key.into())
        // })
        .sum()
    // for door_code_sequence in input {
    //     let mut door_code_position = DoorCodeKey::A;
    //     let mut door_bot_position = RobotKey::A;

    //     for &door_code in door_code_sequence {
    //         print!("{:?} ", door_code);
    //         let mut watchdog = 0;
    //         loop {
    //             let (next_position, robot_key) = door_code_step(door_code_position, door_code);
    //             door_code_position = next_position;

    //             print!("{}", robot_key);
    //             if robot_key == RobotKey::A {
    //                 // Key pressed, is done
    //                 break;
    //             }
    //             watchdog += 1;
    //             if watchdog > 10 {
    //                 panic!("Watchdog failure");
    //             }
    //         }
    //         println!();
    //     }
    //     println!();
    // }
    // 69
}

fn robot_check() {
    let all_robot_keys = [
        RobotKey::Up,
        RobotKey::Down,
        RobotKey::Left,
        RobotKey::Right,
        RobotKey::A,
    ];

    for from in all_robot_keys {
        for to in all_robot_keys {
            print!("{:?} -> {:?}: ", from, to);
            let mut current = from;
            let mut watchdog = 0;
            let mut last_robot_key = None;
            let mut key_changes = 0;

            loop {
                let (next_position, robot_key) = robot_key_step(current, to);
                print!("{}", robot_key);
                current = next_position;
                if let Some(last_key) = last_robot_key {
                    if last_key != robot_key {
                        key_changes += 1;
                    }
                }

                last_robot_key = Some(robot_key);

                if robot_key == RobotKey::A {
                    break;
                }
                watchdog += 1;
                if watchdog > 10 {
                    panic!("Watchdog failure");
                }
            }

            if key_changes > 2 {
                println!(" {}!!!!!!!!!!!!!!!", key_changes);
            } else {
                println!(" {}", key_changes);
            }
        }
    }
}

fn door_check() {
    let all_codes = [
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
        DoorCodeKey::A,
        // DoorCodeKey::Blank,
    ];

    for from in all_codes {
        for to in all_codes {
            print!("{:?} -> {:?}: ", from, to);
            let mut current = from;
            let mut watchdog = 0;
            let mut last_robot_key = None;
            let mut key_changes = 0;

            loop {
                let (next_position, robot_key) = door_code_step(current, &to);
                print!("{}", robot_key);
                current = next_position;
                if let Some(last_key) = last_robot_key {
                    if last_key != robot_key {
                        key_changes += 1;
                    }
                }

                last_robot_key = Some(robot_key);

                if robot_key == RobotKey::A {
                    break;
                }
                watchdog += 1;
                if watchdog > 10 {
                    panic!("Watchdog failure");
                }
            }

            if key_changes > 2 {
                println!(" {}!!!!!!!!!!!!!!!", key_changes);
            } else {
                println!(" {}", key_changes);
            }
        }
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
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &InputType) -> usize {
    0
}
