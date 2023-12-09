use std::{
    collections::{HashSet, VecDeque},
    ops::Sub,
};

use advent::*;

advent_day!(Day4, parse, Vec<Card>, part1, part2);
advent_bench!(raw, cursed_regex, part1_cursed);

pub fn parse(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(":").unwrap();
            let (winning_numbers, played_numbers) = numbers.split_once('|').unwrap();
            Card {
                winning: winning_numbers
                    .split_whitespace()
                    .map(|number| number.parse::<u32>().unwrap())
                    .collect(),

                played: played_numbers
                    .split_whitespace()
                    .map(|number| number.parse::<u32>().unwrap())
                    .collect(),
            }
        })
        .collect()
}

pub fn raw(input: &str) -> &str {
    input
}

pub struct Card {
    winning: HashSet<u32>,
    played: HashSet<u32>,
}

/// ```rust
/// use advent_of_code_2023::day4::*;
/// let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
/// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
/// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
/// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
/// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
/// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
/// assert_eq!(13, part1_cursed(input));
/// ```
pub fn part1_cursed(input: &str) -> u32 {
    let line_match = onig::Regex::new(r"(\b(?<win>\d+)\s)(?=.*\|.*(?<match>\b\k<win>\b))").unwrap();
    input
        .lines()
        .map(|line| {
            let wins = line_match.captures_iter(line).count();
            match wins {
                0 => 0,
                _ => 2u32.pow(wins.sub(1) as u32),
            }
        })
        .sum()
}

/// ```rust
/// use advent_of_code_2023::day4::*;
/// let input = parse(
/// r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
/// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
/// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
/// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
/// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
/// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
/// assert_eq!(13, part1(&input));
/// ```
pub fn part1(input: &Vec<Card>) -> u32 {
    input
        .iter()
        .map(|card| {
            let wins = card.played.intersection(&card.winning).count();
            match wins {
                0 => 0,
                _ => 2u32.pow(wins.sub(1) as u32),
            }
        })
        .sum()
}

/// ```rust
/// use advent_of_code_2023::day4::*;
/// let input = parse(
/// r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
/// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
/// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
/// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
/// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
/// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
/// assert_eq!(30, part2(&input));
/// ```
pub fn part2(input: &Vec<Card>) -> u32 {
    let mut copies_state: VecDeque<u32> = VecDeque::<u32>::new();
    input
        .iter()
        .map(|card| card.played.intersection(&card.winning).count())
        .fold(0, |acc, wins| {
            let win_copies = copies_state.pop_front().unwrap_or_default() + 1;

            for i in 0..wins {
                match copies_state.get(i as usize) {
                    Some(copies) => copies_state[i as usize] = win_copies + copies,
                    None => copies_state.push_back(win_copies),
                };
            }
            acc + win_copies
        })
}
