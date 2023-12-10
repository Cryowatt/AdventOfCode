use bitflags::bitflags;

use advent::*;

advent_day!(Day10, parse, PipeMap, part1, part2);

pub fn parse(input: &str) -> PipeMap {
    PipeMap {
        rows: input
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|cell| match cell {
                        b'|' => PipeEnds::North.union(PipeEnds::South),
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
    }
}

bitflags! {
    pub struct PipeEnds: u8 {
        const North = 0b00000001;
        const South = 0b00000010;
        const West =  0b00000100;
        const East =  0b00001000;
        const Start = 0b00010000;
    }
}

struct PipeMap {
    rows: Vec<Vec<PipeEnds>>,
}

/// ```rust
/// use advent_of_code_2023::day10::*;
/// let input = parse(
/// r".....
/// .F-7.
/// .|.|.
/// .L-J.
/// .....");
/// assert_eq!(4, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day10::*;
/// let input = parse(
/// r"7-F7-
/// .FJ|7
/// SJLL7
/// |F--J
/// LJ.LJ");
/// assert_eq!(8, part1(&input));
/// ```
pub fn part1(input: &PipeMap) -> u32 {
    unimplemented!();
}

/// ```rust
/// use advent_of_code_2023::day10::*;
/// let input = parse(
/// r".....
/// .F-7.
/// .|.|.
/// .L-J.
/// .....");
/// // assert_eq!(4, part2(&input));
/// ```
pub fn part2(input: &PipeMap) -> u32 {
    unimplemented!();
}
