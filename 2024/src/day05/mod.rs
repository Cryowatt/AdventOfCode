use advent::*;

advent_day!(Day05, parse, (Vec<(u8, u8)>, Vec<Vec<u8>>), part1, part2);

pub fn parse(input: &str) -> InputType {
    let rules = input
        .lines()
        .map_while(|line| {
            line.split_once('|')
                .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
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
    let (rules, page_updates) = input;
    page_updates
        .iter()
        .filter(|&pages| {
            rules.iter().all(|&(first, second)| {
                if let Some(first_index) = pages.iter().position(|&page| page == first) {
                    if let Some(second_index) = pages.iter().position(|&page| page == second) {
                        first_index < second_index
                    } else {
                        true
                    }
                } else {
                    true
                }
            })
        })
        .map(|valid_line| valid_line[valid_line.len() / 2] as u32)
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
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &InputType) -> usize {
    0
}
