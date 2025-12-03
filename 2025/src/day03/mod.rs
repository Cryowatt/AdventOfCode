use advent::*;

advent_day!(Day03, parse, Vec<Vec<u8>>, part1, part2);

pub fn parse(input: &str) -> InputType<'_> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

/// ```rust
/// use advent_of_code_2025::day03::*;
/// let input = parse(
/// r"");
/// assert_eq!(todo!(), part1(&input));
/// ```
pub fn part1(input: &InputType) -> u64 {
    todo!()
}

/// ```rust
/// use advent_of_code_2025::day03::*;
/// let input = parse(
/// r"");
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &InputType) -> u64 {
    todo!()
}
