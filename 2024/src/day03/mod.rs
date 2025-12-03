use regex::Regex;

use advent::*;

advent_day!(Day03, parse, str, part1, part2);

pub fn parse(input: &str) -> &InputType<'_> {
    input
}

/// ```rust
/// use advent_of_code_2024::day03::*;
/// let input = parse(
/// r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
/// assert_eq!(161, part1(&input));
/// ```
pub fn part1(input: &InputType) -> i32 {
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
/// r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
/// assert_eq!(48, part2(&input));
/// ```
pub fn part2(input: &InputType) -> i32 {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do)\(\)|(don't)\(\)").unwrap();

    pattern
        .captures_iter(input)
        .scan(true, |enabled, capture| {
            if let Some(_) = capture.get(3) {
                // Do
                *enabled = true;
                Some(0)
            } else if let Some(_) = capture.get(4) {
                // Don't
                *enabled = false;
                Some(0)
            } else {
                if *enabled {
                    Some(
                        capture.get(1).unwrap().as_str().parse::<i32>().unwrap()
                            * capture.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                    )
                } else {
                    Some(0)
                }
            }
        })
        .sum()
}
