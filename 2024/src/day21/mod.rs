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

// v<<<AA>AA>^A
// v<A<AAA>>^AAvA^AAvA^<A
// v<<AA<AA>>^A
// v<A<AA>>^AAv<<A>>^AAvAA<^A

/// Each call gets one step closer to the destination
fn door_code_step(from: DoorCodeKey, to: &DoorCodeKey) -> IPoint {
    // Vertical wins: to(A)
    // VERTICAL WIN D1 D0
    // VERTICAL WIN D1 A
    // VERTICAL WIN D2 A
    // VERTICAL WIN D4 D0
    // VERTICAL WIN D4 D2
    // VERTICAL WIN D4 D3
    // VERTICAL WIN D4 A
    // VERTICAL WIN D5 D3
    // VERTICAL WIN D5 A
    // VERTICAL WIN D7 D0
    // VERTICAL WIN D7 D2
    // VERTICAL WIN D7 D3
    // VERTICAL WIN D7 D5
    // VERTICAL WIN D7 D6
    // VERTICAL WIN D7 A
    // VERTICAL WIN D8 D3
    // VERTICAL WIN D8 D6
    // VERTICAL WIN D8 A
    // 789
    // 456
    // 123
    // .0A
    match from {
        DoorCodeKey::D0 => match to {
            DoorCodeKey::D7 => IPoint::new(-1, -3),
            DoorCodeKey::D8 => IPoint::new(0, -3),
            DoorCodeKey::D9 => IPoint::new(1, -3),
            DoorCodeKey::D4 => IPoint::new(-1, -2),
            DoorCodeKey::D5 => IPoint::new(0, -2),
            DoorCodeKey::D6 => IPoint::new(1, -2),
            DoorCodeKey::D1 => IPoint::new(-1, -1),
            DoorCodeKey::D2 => IPoint::new(0, -1),
            DoorCodeKey::D3 => IPoint::new(1, -1),
            DoorCodeKey::D0 => IPoint::new(0, 0),
            DoorCodeKey::A => IPoint::new(1, 0),
        },
        DoorCodeKey::D1 => match to {
            DoorCodeKey::D7 => IPoint::new(0, -2),
            DoorCodeKey::D8 => IPoint::new(1, -2),
            DoorCodeKey::D9 => IPoint::new(2, -2),
            DoorCodeKey::D4 => IPoint::new(0, -1),
            DoorCodeKey::D5 => IPoint::new(1, -1),
            DoorCodeKey::D6 => IPoint::new(2, -1),
            DoorCodeKey::D1 => IPoint::new(0, 0),
            DoorCodeKey::D2 => IPoint::new(1, 0),
            DoorCodeKey::D3 => IPoint::new(2, 0),
            DoorCodeKey::D0 => IPoint::new(1, 1),
            DoorCodeKey::A => IPoint::new(2, 1),
        },
        DoorCodeKey::D2 => match to {
            DoorCodeKey::D7 => IPoint::new(-1, -2),
            DoorCodeKey::D8 => IPoint::new(0, -2),
            DoorCodeKey::D9 => IPoint::new(1, -2),
            DoorCodeKey::D4 => IPoint::new(-1, -1),
            DoorCodeKey::D5 => IPoint::new(0, -1),
            DoorCodeKey::D6 => IPoint::new(1, -1),
            DoorCodeKey::D1 => IPoint::new(-1, 0),
            DoorCodeKey::D2 => IPoint::new(0, 0),
            DoorCodeKey::D3 => IPoint::new(1, 0),
            DoorCodeKey::D0 => IPoint::new(0, 1),
            DoorCodeKey::A => IPoint::new(1, 1),
        },
        DoorCodeKey::D3 => match to {
            DoorCodeKey::D7 => IPoint::new(-2, -2),
            DoorCodeKey::D8 => IPoint::new(-1, -2),
            DoorCodeKey::D9 => IPoint::new(0, -2),
            DoorCodeKey::D4 => IPoint::new(-2, -1),
            DoorCodeKey::D5 => IPoint::new(-1, -1),
            DoorCodeKey::D6 => IPoint::new(0, -1),
            DoorCodeKey::D1 => IPoint::new(-2, 0),
            DoorCodeKey::D2 => IPoint::new(-1, 0),
            DoorCodeKey::D3 => IPoint::new(0, 0),
            DoorCodeKey::D0 => IPoint::new(-1, 1),
            DoorCodeKey::A => IPoint::new(0, 1),
        },
        DoorCodeKey::D4 => match to {
            DoorCodeKey::D7 => IPoint::new(0, -1),
            DoorCodeKey::D8 => IPoint::new(1, -1),
            DoorCodeKey::D9 => IPoint::new(2, -1),
            DoorCodeKey::D4 => IPoint::new(0, 0),
            DoorCodeKey::D5 => IPoint::new(1, 0),
            DoorCodeKey::D6 => IPoint::new(2, 0),
            DoorCodeKey::D1 => IPoint::new(0, 1),
            DoorCodeKey::D2 => IPoint::new(1, 1),
            DoorCodeKey::D3 => IPoint::new(2, 1),
            DoorCodeKey::D0 => IPoint::new(1, 2),
            DoorCodeKey::A => IPoint::new(2, 2),
        },
        DoorCodeKey::D5 => match to {
            DoorCodeKey::D7 => IPoint::new(-1, -1),
            DoorCodeKey::D8 => IPoint::new(0, -1),
            DoorCodeKey::D9 => IPoint::new(1, -1),
            DoorCodeKey::D4 => IPoint::new(-1, 0),
            DoorCodeKey::D5 => IPoint::new(0, 0),
            DoorCodeKey::D6 => IPoint::new(1, 0),
            DoorCodeKey::D1 => IPoint::new(-1, 1),
            DoorCodeKey::D2 => IPoint::new(0, 1),
            DoorCodeKey::D3 => IPoint::new(1, 1),
            DoorCodeKey::D0 => IPoint::new(0, 2),
            DoorCodeKey::A => IPoint::new(1, 2),
        },
        DoorCodeKey::D6 => match to {
            DoorCodeKey::D7 => IPoint::new(-2, -1),
            DoorCodeKey::D8 => IPoint::new(-1, -1),
            DoorCodeKey::D9 => IPoint::new(0, -1),
            DoorCodeKey::D4 => IPoint::new(-2, 0),
            DoorCodeKey::D5 => IPoint::new(-1, 0),
            DoorCodeKey::D6 => IPoint::new(0, 0),
            DoorCodeKey::D1 => IPoint::new(-2, 1),
            DoorCodeKey::D2 => IPoint::new(-1, 1),
            DoorCodeKey::D3 => IPoint::new(0, 1),
            DoorCodeKey::D0 => IPoint::new(-1, 2),
            DoorCodeKey::A => IPoint::new(0, 2),
        },
        DoorCodeKey::D7 => match to {
            DoorCodeKey::D7 => IPoint::new(0, 0),
            DoorCodeKey::D8 => IPoint::new(1, 0),
            DoorCodeKey::D9 => IPoint::new(2, 0),
            DoorCodeKey::D4 => IPoint::new(0, 1),
            DoorCodeKey::D5 => IPoint::new(1, 1),
            DoorCodeKey::D6 => IPoint::new(2, 1),
            DoorCodeKey::D1 => IPoint::new(0, 2),
            DoorCodeKey::D2 => IPoint::new(1, 2),
            DoorCodeKey::D3 => IPoint::new(2, 2),
            DoorCodeKey::D0 => IPoint::new(1, 3),
            DoorCodeKey::A => IPoint::new(2, 3),
        },
        DoorCodeKey::D8 => match to {
            DoorCodeKey::D7 => IPoint::new(-1, 0),
            DoorCodeKey::D8 => IPoint::new(0, 0),
            DoorCodeKey::D9 => IPoint::new(1, 0),
            DoorCodeKey::D4 => IPoint::new(-1, 1),
            DoorCodeKey::D5 => IPoint::new(0, 1),
            DoorCodeKey::D6 => IPoint::new(1, 1),
            DoorCodeKey::D1 => IPoint::new(-1, 2),
            DoorCodeKey::D2 => IPoint::new(0, 2),
            DoorCodeKey::D3 => IPoint::new(1, 2),
            DoorCodeKey::D0 => IPoint::new(0, 3),
            DoorCodeKey::A => IPoint::new(1, 3),
        },
        DoorCodeKey::D9 => match to {
            DoorCodeKey::D7 => IPoint::new(-2, 0),
            DoorCodeKey::D8 => IPoint::new(-1, 0),
            DoorCodeKey::D9 => IPoint::new(0, 0),
            DoorCodeKey::D4 => IPoint::new(-2, 1),
            DoorCodeKey::D5 => IPoint::new(-1, 1),
            DoorCodeKey::D6 => IPoint::new(0, 1),
            DoorCodeKey::D1 => IPoint::new(-2, 2),
            DoorCodeKey::D2 => IPoint::new(-1, 2),
            DoorCodeKey::D3 => IPoint::new(0, 2),
            DoorCodeKey::D0 => IPoint::new(-1, 3),
            DoorCodeKey::A => IPoint::new(0, 3),
        },
        DoorCodeKey::A => match to {
            DoorCodeKey::D7 => IPoint::new(-2, -3),
            DoorCodeKey::D8 => IPoint::new(-1, -3),
            DoorCodeKey::D9 => IPoint::new(0, -3),
            DoorCodeKey::D4 => IPoint::new(-2, -2),
            DoorCodeKey::D5 => IPoint::new(-1, -2),
            DoorCodeKey::D6 => IPoint::new(0, -2),
            DoorCodeKey::D1 => IPoint::new(-2, -1),
            DoorCodeKey::D2 => IPoint::new(-1, -1),
            DoorCodeKey::D3 => IPoint::new(0, -1),
            DoorCodeKey::D0 => IPoint::new(-1, 0),
            DoorCodeKey::A => IPoint::new(0, 0),
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
            RobotKey::A => (RobotKey::A, RobotKey::Up),
            _ => (RobotKey::Down, RobotKey::Left),
        },
        RobotKey::Down => match to {
            RobotKey::Down => (RobotKey::Down, RobotKey::A),
            RobotKey::Up => (RobotKey::Up, RobotKey::Up),
            RobotKey::Left => (RobotKey::Left, RobotKey::Left),
            _ => (RobotKey::Right, RobotKey::Right),
        },
        RobotKey::A => match to {
            RobotKey::A => (RobotKey::A, RobotKey::A),
            RobotKey::Up | RobotKey::Down => (RobotKey::Up, RobotKey::Left),
            _ => (RobotKey::Right, RobotKey::Down),
        },
    }
}

fn key_bot<'a, I>(key_sequence: I) -> impl Iterator<Item = RobotKey>
where
    I: Iterator<Item = RobotKey>,
{
    iter::from_coroutine(
        #[coroutine]
        move || {
            let mut current_key = RobotKey::A;
            for key in key_sequence {
                loop {
                    let (next_position, robot_key) = robot_key_step(current_key, key);
                    current_key = next_position;
                    yield robot_key;
                    if robot_key == RobotKey::A {
                        break;
                    }
                }
            }
        },
    )
}

// [7][8][9]
// [4][5][6]
// [1][2][3]
//    [0][A]
fn door_bot<'a>(code: &'a [DoorCodeKey; 4]) -> impl Iterator<Item = RobotKey> + use<'a> {
    iter::from_coroutine(
        #[coroutine]
        move || {
            let mut door_key = DoorCodeKey::A;
            for key in code {
                // loop {
                let vertical_first = vertical_oriendtation(door_key, key);

                let offset = door_code_step(door_key, key);
                let vertical_key = (offset.y > 0)
                    .then_some(RobotKey::Down)
                    .unwrap_or(RobotKey::Up);

                let horizontal_key = (offset.x > 0)
                    .then_some(RobotKey::Right)
                    .unwrap_or(RobotKey::Left);

                if vertical_first {
                    for _y in (offset.y.min(0))..offset.y.max(0) {
                        yield vertical_key
                    }

                    for _x in (offset.x.min(0))..offset.x.max(0) {
                        yield horizontal_key
                    }
                } else {
                    for _x in (offset.x.min(0))..offset.x.max(0) {
                        yield horizontal_key
                    }

                    for _y in (offset.y.min(0))..offset.y.max(0) {
                        yield vertical_key
                    }
                }

                yield RobotKey::A;

                door_key = *key;
            }
        },
    )
}

fn vertical_oriendtation(door_key: DoorCodeKey, key: &DoorCodeKey) -> bool {
    let vertical_first = match (door_key, *key) {
        (DoorCodeKey::A, _) => true,
        (DoorCodeKey::D0, _) => true,

        // (DoorCodeKey::D1, DoorCodeKey::D0) => true,
        // (DoorCodeKey::D1, DoorCodeKey::A) => true,
        (DoorCodeKey::D2, DoorCodeKey::A) => true,

        // (DoorCodeKey::D4, DoorCodeKey::D0) => true,
        (DoorCodeKey::D4, DoorCodeKey::D2) => true,
        (DoorCodeKey::D4, DoorCodeKey::D3) => true,
        // (DoorCodeKey::D4, DoorCodeKey::A) => true,
        (DoorCodeKey::D5, DoorCodeKey::D3) => true,
        (DoorCodeKey::D5, DoorCodeKey::A) => true,

        // (DoorCodeKey::D7, DoorCodeKey::D0) => true,
        (DoorCodeKey::D7, DoorCodeKey::D2) => true,
        (DoorCodeKey::D7, DoorCodeKey::D3) => true,
        (DoorCodeKey::D7, DoorCodeKey::D5) => true,
        (DoorCodeKey::D7, DoorCodeKey::D6) => true,
        // (DoorCodeKey::D7, DoorCodeKey::A) => true,
        (DoorCodeKey::D8, DoorCodeKey::D3) => true,
        (DoorCodeKey::D8, DoorCodeKey::D6) => true,
        (DoorCodeKey::D8, DoorCodeKey::A) => true,

        _ => false,
    };
    vertical_first
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
/// too high: 171460
/// too low: 162200
/// bad: 166248

pub fn part1(input: &InputType) -> u32 {
    door_check();
    robot_check();
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

    // [7][8][9]
    // [4][5][6]
    // [1][2][3]
    //    [0][A]

    //    [^][A]
    // [<][v][>]
    //       v<<A>>^A<vA<A>>^AAvA<^A>AA<vA<A>>^AvAA^AAv<<A>^A>AAA<vA<A>>^AvA^AAAA  68 * 29
    // -1:   v<<A>>^A<A>AvA<^AA>A<vAAA>^A
    //       v<<A>>^A<A>AvA<^AA>A<vAAA>^A
    // -2:   <A^A>^^AvvvA
    //       <A^A>^^AvvvA

    //       v<<A>>^AAAA<vA<A>>^AAAvA<^A>A<vA<A>>^AvA^AAAA<A>A<vA>A^AA  57 * 980
    //       v<<A>>^AA<vA<A>>^AAAAvA<^A>AAA<vA>A^AAAv<<A>^A>A<vA<A>>^AvA^AAAA  64 * 179
    //       v<<A>>^AAA<vA<A>>^AAAvAA^AAv<<A>^A>A<vA>A^AAv<<A>^A>A<vA<A>>^AvA^AAA  68 * 456
    //       v<<A>>^AA<vA<A>>^AAAAvA<^A>AAA<vA>A^AAAv<<A>^A>A<vA<A>>^AvA^AAAA  64 * 379

    // 029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
    //       <vA<AA>>^AvAA<^A>Av<<A>>^AvA^A<vA>^Av<<A>^A>AAvA^Av<<A>A>^AAAvA<^A>A  68 * 29

    // 980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
    //       v<<A>>^AAAvA^A<vA<AA>>^AvAA<^A>Av<<A>A>^AAAvA<^A>A<vA>^A<A>A  60 * 980

    // 179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
    //       <vA<AA>>^AAvA<^A>AvA^Av<<A>>^AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A  64 * 179

    // 456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
    //       <vA<AA>>^AAvA<^A>AAvA^A<vA>^A<A>A<vA>^A<A>Av<<A>A>^AAvA<^A>A  60 * 456

    // 379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
    //       v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A  64 * 379
    input
        .iter()
        // .take(1)
        .map(|code| {
            let sequence_length = key_bot(key_bot(door_bot(code)))
                //  (key_bot(door_bot(code)))
                //  ((door_bot(code)))
                .inspect(|k| print!("{}", k))
                .count();

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
    let mut vert = 0;
    let mut horz = 0;

    for from in all_codes {
        for to in all_codes {
            let position = match from {
                DoorCodeKey::D7 => IPoint::new(0, 0),
                DoorCodeKey::D8 => IPoint::new(1, 0),
                DoorCodeKey::D9 => IPoint::new(2, 0),
                DoorCodeKey::D4 => IPoint::new(0, 1),
                DoorCodeKey::D5 => IPoint::new(1, 1),
                DoorCodeKey::D6 => IPoint::new(2, 1),
                DoorCodeKey::D1 => IPoint::new(0, 2),
                DoorCodeKey::D2 => IPoint::new(1, 2),
                DoorCodeKey::D3 => IPoint::new(2, 2),
                DoorCodeKey::D0 => IPoint::new(1, 3),
                DoorCodeKey::A => IPoint::new(2, 3),
            };
            let offset = door_code_step(from, &to);
            let reverse_offset = door_code_step(to, &from);

            if offset.x != -reverse_offset.x || offset.y != -reverse_offset.y {
                println!("ERROR! {:?} => {:?}", from, to);
            }

            if vertical_oriendtation(from, &to) {
                let vertical = key_bot(key_bot(iter::from_coroutine(
                    #[coroutine]
                    move || {
                        let offset = door_code_step(from, &to);
                        let vertical_key = (offset.y > 0)
                            .then_some(RobotKey::Down)
                            .unwrap_or(RobotKey::Up);

                        let horizontal_key = (offset.x > 0)
                            .then_some(RobotKey::Right)
                            .unwrap_or(RobotKey::Left);

                        for y in (offset.y.min(0))..offset.y.max(0) {
                            if position + IPoint::new(0, y) == IPoint::new(0, 3) {
                                panic!("FK");
                            }
                            yield vertical_key
                        }

                        for x in (offset.x.min(0))..offset.x.max(0) {
                            if position + IPoint::new(x, offset.y) == IPoint::new(0, 3) {
                                panic!("FK");
                            }
                            yield horizontal_key
                        }

                        yield RobotKey::A;
                    },
                )))
                .count();
            } else {
                let horizontal = key_bot(key_bot(iter::from_coroutine(
                    #[coroutine]
                    move || {
                        let offset = door_code_step(from, &to);
                        let vertical_key = (offset.y > 0)
                            .then_some(RobotKey::Down)
                            .unwrap_or(RobotKey::Up);

                        let horizontal_key = (offset.x > 0)
                            .then_some(RobotKey::Right)
                            .unwrap_or(RobotKey::Left);

                        for x in (offset.x.min(0))..offset.x.max(0) {
                            if position + IPoint::new(x, 0) == IPoint::new(0, 3) {
                                panic!("FK");
                            }
                            yield horizontal_key
                        }

                        for y in (offset.y.min(0))..offset.y.max(0) {
                            if position + IPoint::new(offset.x, y) == IPoint::new(0, 3) {
                                panic!("FK");
                            }
                            yield vertical_key
                        }
                        yield RobotKey::A;
                    },
                )))
                .count();
            }

            // if vertical < horizontal {
            //     println!("VERTICAL WIN {:?} {:?}", from, to);
            //     vert += 1;
            // } else if vertical > horizontal {
            //     horz += 1;
            //     // println!("HORIZONTAL WIN {:?} {:?}", from, to);
            // }
        }
    }
    println!("{} {} ", vert, horz);
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
