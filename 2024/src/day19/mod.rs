use advent::*;
use dashmap::DashMap;
use onig::Regex;
use rayon::prelude::*;

advent_day!(Day19, parse, (Vec<&'a str>, Vec<&'a str>), part1, part2);

pub fn parse(input: &str) -> InputType {
    let towels = input.lines().next().unwrap().split(", ").collect();
    let designs = input.lines().skip(2).collect();
    (towels, designs)
}

/// ```rust
/// use advent_of_code_2024::day19::*;
/// let input = parse(
/// r"r, wr, b, g, bwu, rb, gb, br
///
/// brwrr
/// bggr
/// gbbr
/// rrbgbr
/// ubwu
/// bwurrg
/// brgr
/// bbrgwb");
/// assert_eq!(6, part1(&input));
/// ```
pub fn part1(input: &InputType) -> usize {
    let (towels, designs) = input;
    let mut pattern = String::from("^(");
    pattern.push_str(towels.join("|").as_str());
    pattern.push_str(")+$");
    let matcher = Regex::new(&pattern).unwrap();
    designs
        .par_iter()
        .filter(|design| matcher.is_match(design))
        .count()
}

/// ```rust
/// use advent_of_code_2024::day19::*;
/// let input = parse(
/// r"r, wr, b, g, bwu, rb, gb, br
///
/// brwrr
/// bggr
/// gbbr
/// rrbgbr
/// ubwu
/// bwurrg
/// brgr
/// bbrgwb");
/// assert_eq!(16, part2(&input));
/// ```
pub fn part2(input: &InputType) -> u64 {
    let (towels, designs) = input;
    let memo = &DashMap::new();

    fn combos<'a>(design: &'a str, towels: &Vec<&str>, memo: &DashMap<&'a str, u64>) -> u64 {
        if design.is_empty() {
            1
        } else if let Some(count) = memo.get(design) {
            *count
        } else {
            let count = towels
                .iter()
                .filter(|&&towel| design.starts_with(towel))
                .map(|&towel| combos(&design[towel.len()..], towels, memo))
                .sum();
            memo.insert(design, count);
            count
        }
    }

    designs
        .par_iter()
        .map(|design| combos(design, towels, memo))
        .sum()
}
