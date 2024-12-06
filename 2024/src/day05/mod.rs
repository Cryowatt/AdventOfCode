use rayon::prelude::*;
use std::collections::HashMap;

use advent::*;

advent_day!(Day05, parse, (Vec<(u8, u8)>, Vec<Vec<u8>>), part1, part2);

pub fn parse(input: &str) -> InputType {
    let rules = input
        .lines()
        .map_while(|line| {
            line.split_once('|').map(|(first, second)| {
                (first.parse::<u8>().unwrap(), second.parse::<u8>().unwrap())
            })
        })
        .collect();

    let page_updates = input
        .lines()
        .skip_while(|line| !line.contains(','))
        .map(|line| {
            line.split(',')
                .map(|value| value.parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    (rules, page_updates)
}

/// ```rust
/// use advent_of_code_2024::day05::*;
/// let input = parse(
/// r"47|53
/// 97|13
/// 97|61
/// 97|47
/// 75|29
/// 61|13
/// 75|53
/// 29|13
/// 97|29
/// 53|29
/// 61|53
/// 97|53
/// 61|29
/// 47|13
/// 75|47
/// 97|75
/// 47|61
/// 75|61
/// 47|29
/// 75|13
/// 53|13
///
/// 75,47,61,53,29
/// 97,61,53,29,13
/// 75,29,13
/// 75,97,47,61,53
/// 61,13,29
/// 97,13,75,29,47");
/// assert_eq!(143, part1(&input));
/// ```
pub fn part1(input: &InputType) -> u32 {
    let (order, page_updates) = input;
    page_updates
        .par_iter()
        .filter_map(|pages| {
            let ranks = rank(order, pages);
            if pages.is_sorted_by_key(|key| ranks.get(key).cloned().unwrap_or_default()) {
                Some(pages[pages.len() / 2] as u32)
            } else {
                None
            }
        })
        .sum()
}

/// ```rust
/// use advent_of_code_2024::day05::*;
/// let input = parse(
/// r"47|53
/// 97|13
/// 97|61
/// 97|47
/// 75|29
/// 61|13
/// 75|53
/// 29|13
/// 97|29
/// 53|29
/// 61|53
/// 97|53
/// 61|29
/// 47|13
/// 75|47
/// 97|75
/// 47|61
/// 75|61
/// 47|29
/// 75|13
/// 53|13
///
/// 75,47,61,53,29
/// 97,61,53,29,13
/// 75,29,13
/// 75,97,47,61,53
/// 61,13,29
/// 97,13,75,29,47");
/// assert_eq!(123, part2(&input));
/// ```
pub fn part2(input: &InputType) -> u32 {
    let (order, page_updates) = input;
    page_updates
        .par_iter()
        .filter_map(|pages| {
            let ranks = &rank(order, pages);
            if !pages.is_sorted_by_key(|key| ranks.get(key).map_or(0, |v| *v)) {
                let mut pages = pages.clone();
                pages.sort_by_key(|key| ranks.get(key).map_or(0, |v| *v));
                Some(pages[pages.len() / 2] as u32)
            } else {
                None
            }
        })
        .sum()
}

fn rank(rules: &Vec<(u8, u8)>, pages: &Vec<u8>) -> HashMap<u8, u8> {
    rules
        .iter()
        .filter(|&rule| pages.contains(&rule.0) && pages.contains(&rule.1))
        .fold(HashMap::new(), |mut map, rule| {
            let entry = map.entry(rule.1).or_default();
            *entry += 1;
            map
        })
}
