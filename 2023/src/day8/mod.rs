use std::collections::HashMap;

use advent::*;
use num::Integer;

advent_day!(Day8, parse, WastelandMap, part1, part2);

pub fn parse(input: &str) -> WastelandMap {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .unwrap()
        .bytes()
        .map(|instruction| match instruction {
            b'L' => 0,
            b'R' => 1,
            _ => unreachable!(),
        })
        .collect();

    let _blank_line = lines.next();

    let map: HashMap<&str, [&str; 2]> = lines
        .map(|line| {
            let (key, children) = line.split_once('=').unwrap();
            let (left, right) = children.split_once(',').unwrap();
            (
                key.trim_end(),
                [
                    left.trim().trim_start_matches('('),
                    right.trim().trim_end_matches(')'),
                ],
            )
        })
        .collect();

    WastelandMap { instructions, map }
}

pub struct WastelandMap<'a> {
    instructions: Vec<u8>,
    map: HashMap<&'a str, [&'a str; 2]>,
}

/// ```rust
/// use advent_of_code_2023::day8::*;
/// let input = parse(
/// r"RL
///
/// AAA = (BBB, CCC)
/// BBB = (DDD, EEE)
/// CCC = (ZZZ, GGG)
/// DDD = (DDD, DDD)
/// EEE = (EEE, EEE)
/// GGG = (GGG, GGG)
/// ZZZ = (ZZZ, ZZZ)");
/// assert_eq!(2, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day8::*;
/// let input = parse(
/// r"LLR
///
/// AAA = (BBB, BBB)
/// BBB = (AAA, ZZZ)
/// ZZZ = (ZZZ, ZZZ)");
/// assert_eq!(6, part1(&input));
/// ```
pub fn part1(input: &WastelandMap) -> u32 {
    input
        .instructions
        .iter()
        .cycle()
        .scan("AAA", |node, instruction| {
            let next = input.map[node][(*instruction) as usize];

            if next == "ZZZ" {
                None
            } else {
                *node = next;
                Some(input.map[node][(*instruction) as usize])
            }
        })
        .count() as u32
        + 1
}

/// ```rust
/// use advent_of_code_2023::day8::*;
/// let input = parse(
/// r"LR
///
/// 11A = (11B, XXX)
/// 11B = (XXX, 11Z)
/// 11Z = (11B, XXX)
/// 22A = (22B, XXX)
/// 22B = (22C, 22C)
/// 22C = (22Z, 22Z)
/// 22Z = (22B, 22B)
/// XXX = (XXX, XXX)");
/// assert_eq!(6, part2(&input));
/// ```
pub fn part2(input: &WastelandMap) -> u64 {
    input
        .map
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|start_node| {
            input
                .instructions
                .iter()
                .cycle()
                .scan(*start_node, |node, instruction| {
                    let current = *node;
                    *node = input.map[*node][*instruction as usize];
                    Some(current)
                })
                .take_while(|step| !step.ends_with("Z"))
                .count() as u64
        })
        .fold(1u64, |lcm: u64, cycle| lcm.lcm(&cycle))
}
