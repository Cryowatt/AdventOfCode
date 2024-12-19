use advent::*;
use onig::Regex;

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
    println!("{}", pattern);
    let matcher = Regex::new(&pattern).unwrap();
    designs
        .iter()
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
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &InputType) -> u32 {
    0
}
