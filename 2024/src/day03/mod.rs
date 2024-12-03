use regex::Regex;

use advent::*;

advent_day!(Day03, parse, Vec<Vec<i32>>, part1, part2);

pub fn parse(input: &str) -> &str {
    input
}

/// ```rust
/// use advent_of_code_2024::day03::*;
/// let input = parse(
/// r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
/// assert_eq!(161, part1(&input));
/// ```
pub fn part1(input: &str) -> i32 {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    pattern
        .captures_iter(input)
        .map(|capture| {
            capture.get(1).unwrap().as_str().parse::<i32>().unwrap()
                * capture.get(2).unwrap().as_str().parse::<i32>().unwrap()
        })
        .sum()
}

/// ```rust
/// use advent_of_code_2024::day03::*;
/// let input = parse(
/// r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &str) -> i32 {
    0
}
