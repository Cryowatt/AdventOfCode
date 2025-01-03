use advent::*;

advent_day!(Day01, parse, str, part1, part2);

pub fn parse(input: &str) -> &str {
    input
}

/// ```rust
/// use advent_of_code_2015::day01::*;
/// assert_eq!(0, part1(&(parse("(())"))));
/// ```
/// ```rust
/// use advent_of_code_2015::day01::*;
/// assert_eq!(0, part1(&parse("()()")));
/// ```
/// ```rust
/// use advent_of_code_2015::day01::*;
/// assert_eq!(3, part1(&parse("(((")));
/// ```
/// ```rust
/// use advent_of_code_2015::day01::*;
/// assert_eq!(3, part1(&parse("(()(()(")));
/// ```
/// ```rust
/// use advent_of_code_2015::day01::*;
/// assert_eq!(3, part1(&parse("))(((((")));
/// ```
/// ```rust
/// use advent_of_code_2015::day01::*;
/// assert_eq!(-1, part1(&parse("())")));
/// ```
/// ```rust
/// use advent_of_code_2015::day01::*;
/// assert_eq!(-1, part1(&parse("))(")));
/// ```
/// ```rust
/// use advent_of_code_2015::day01::*;
/// assert_eq!(-3, part1(&parse(")))")));
/// ```
/// ```rust
/// use advent_of_code_2015::day01::*;
/// assert_eq!(-3, part1(&parse(")())())")));
/// ```
/// ```rust
/// use advent_of_code_2015::day01::*;
/// ```
pub fn part1(input: &str) -> i32 {
    input.bytes().fold(0, |floor, c| match c {
        b'(' => floor + 1,
        b')' => floor - 1,
        _ => unreachable!(),
    })
}

/// ```rust
/// use advent_of_code_2015::day01::*;
/// assert_eq!(1, part2(&(parse(")"))));
/// ```
/// ```rust
/// use advent_of_code_2015::day01::*;
/// assert_eq!(5, part2(&parse("()())")));
/// ```
pub fn part2(input: &str) -> u32 {
    input
        .bytes()
        .scan(0, |floor, c| {
            match c {
                b'(' => *floor += 1,
                b')' => *floor -= 1,
                _ => unreachable!(),
            };
            Some(*floor)
        })
        .take_while(|v| *v >= 0)
        .count() as u32
        + 1
}
