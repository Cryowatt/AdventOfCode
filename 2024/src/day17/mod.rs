use advent::*;
use regex::Regex;

advent_day!(Day17, parse, (Computer, Vec<u8>), part1, part2);

#[derive(Debug)]
pub struct Computer {
    a: u32,
    b: u32,
    c: u32,
}

pub fn parse(input: &str) -> InputType {
    let pattern = Regex::new(
        r"Register A: (?<A>\d+)
Register B: (?<B>\d+)
Register C: (?<C>\d+)

Program: (?<P>\d(,\d)+)",
    )
    .unwrap();

    let captures = pattern.captures(input).unwrap();
    (
        Computer {
            a: captures.name("A").unwrap().as_str().parse().unwrap(),
            b: captures.name("B").unwrap().as_str().parse().unwrap(),
            c: captures.name("C").unwrap().as_str().parse().unwrap(),
        },
        captures
            .name("P")
            .unwrap()
            .as_str()
            .split(',')
            .map(|b| b.parse().unwrap())
            .collect(),
    )
}

/// ```rust
/// use advent_of_code_2024::day17::*;
/// let input = parse(
/// r"Register A: 729
/// Register B: 0
/// Register C: 0
///
/// Program: 0,1,5,4,3,0");
/// assert_eq!("4,6,3,5,6,3,5,2,1,0".to_string(), part1(&input));
/// ```
pub fn part1(input: &InputType) -> String {
    "".to_string()
}

/// ```rust
/// use advent_of_code_2024::day17::*;
/// let input = parse(
/// r"Register A: 729
/// Register B: 0
/// Register C: 0
///
/// Program: 0,1,5,4,3,0");
/// assert_eq!("".to_string(), part2(&input));
/// ```
pub fn part2(input: &InputType) -> String {
    "".to_string()
}
