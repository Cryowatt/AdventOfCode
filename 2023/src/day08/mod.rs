use std::collections::HashMap;

use advent::*;
use num::Integer;

advent_day!(Day08, parse, WastelandMap, part1, part2);
advent_bench!(parse, cursed_hashmap, part1_hashmap, part2_hashmap);

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

    let mut base26_lookup = [None; 17576];

    for entry in map.iter() {
        fn decode_node(node: &str) -> u16 {
            node.bytes()
                .fold(0u16, |acc, digit| (acc * 26) + (digit - b'A') as u16)
        }

        let node_index = decode_node(entry.0);
        base26_lookup[node_index as usize] =
            Some([decode_node(entry.1[0]), decode_node(entry.1[1])]);
    }

    WastelandMap {
        instructions,
        map,
        base26_lookup,
    }
}

pub struct WastelandMap<'a> {
    instructions: Vec<u8>,
    map: HashMap<&'a str, [&'a str; 2]>,
    base26_lookup: [Option<[u16; 2]>; 17576],
}

/// ```rust
/// use advent_of_code_2023::day08::*;
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
/// use advent_of_code_2023::day08::*;
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
        .scan(0u16, |node, instruction| {
            let next = input.base26_lookup[*node as usize].unwrap()[(*instruction) as usize];

            if next == 17575 {
                None
            } else {
                *node = next;
                Some(input.base26_lookup[*node as usize].unwrap()[(*instruction) as usize])
            }
        })
        .count() as u32
        + 1
}

/// ```rust
/// use advent_of_code_2023::day08::*;
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
/// use advent_of_code_2023::day08::*;
/// let input = parse(
/// r"LLR
///
/// AAA = (BBB, BBB)
/// BBB = (AAA, ZZZ)
/// ZZZ = (ZZZ, ZZZ)");
/// assert_eq!(6, part1(&input));
/// ```
pub fn part1_hashmap(input: &WastelandMap) -> u32 {
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
/// use advent_of_code_2023::day08::*;
/// let input = parse(
/// r"LR
///
/// AAA = (AAB, XXX)
/// AAB = (XXX, AAZ)
/// AAZ = (AAB, XXX)
/// BBA = (BBB, XXX)
/// BBB = (BBC, BBC)
/// BBC = (BBZ, BBZ)
/// BBZ = (BBB, BBB)
/// XXX = (XXX, XXX)");
/// assert_eq!(6, part2(&input));
/// ```
pub fn part2(input: &WastelandMap) -> u64 {
    (0..input.base26_lookup.len())
        .step_by(26)
        .filter_map(|entry| input.base26_lookup[entry].map(|_| entry as u16))
        .map(|start_node| {
            input
                .instructions
                .iter()
                .cycle()
                .scan(start_node, |node, instruction| {
                    let current = *node;
                    *node = input.base26_lookup[(*node) as usize].unwrap()[*instruction as usize];
                    Some(current)
                })
                .take_while(|step| step % 26 != 25)
                .count() as u64
        })
        .fold(1u64, |lcm: u64, cycle| lcm.lcm(&cycle))
}

/// ```rust
/// use advent_of_code_2023::day08::*;
/// let input = parse(
/// r"LR
///
/// AAA = (AAB, XXX)
/// AAB = (XXX, AAZ)
/// AAZ = (AAB, XXX)
/// BBA = (BBB, XXX)
/// BBB = (BBC, BBC)
/// BBC = (BBZ, BBZ)
/// BBZ = (BBB, BBB)
/// XXX = (XXX, XXX)");
/// assert_eq!(6, part2(&input));
/// ```
pub fn part2_hashmap(input: &WastelandMap) -> u64 {
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
