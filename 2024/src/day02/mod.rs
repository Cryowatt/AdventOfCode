use advent::*;

advent_day!(Day02, parse, Vec<Vec<i32>>, part1, part2);

pub fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

/// ```rust
/// use advent_of_code_2024::day02::*;
/// let input = parse(
/// r"7 6 4 2 1
/// 1 2 7 8 9
/// 9 7 6 2 1
/// 1 3 2 4 5
/// 8 6 4 4 1
/// 1 3 6 7 9");
/// assert_eq!(2, part1(&input));
/// ```
pub fn part1(input: &Vec<Vec<i32>>) -> usize {
    input
        .iter()
        .filter(|row| {
            let count = row.len() - 1;
            let score: i32 = row
                .windows(2)
                .map(|x| x[0] - x[1])
                .map(|pair| match pair {
                    -3..=-1 => -1,
                    1..=3 => 1,
                    _ => 0,
                })
                .sum();

            score.abs() == count as i32
        })
        .count()
}

/// ```rust
/// use advent_of_code_2024::day02::*;
/// let input = parse(
/// r"7 6 4 2 1
/// 1 2 7 8 9
/// 9 7 6 2 1
/// 1 3 2 4 5
/// 8 6 4 4 1
/// 1 3 6 7 9");
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &Vec<Vec<i32>>) -> i32 {
    0
}
