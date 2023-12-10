use advent::*;

advent_day!(Day9, parse, Vec<Vec<i32>>, part1, part2);

pub fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

/// ```rust
/// use advent_of_code_2023::day9::*;
/// let input = parse(
/// r"0 3 6 9 12 15");
/// assert_eq!(18, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day9::*;
/// let input = parse(
/// r"1 3 6 10 15 21");
/// assert_eq!(28, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day9::*;
/// let input = parse(
/// r"10 13 16 21 30 45");
/// assert_eq!(68, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day9::*;
/// let input = parse(
/// r"0 3 6 9 12 15
/// 1 3 6 10 15 21
/// 10 13 16 21 30 45");
/// assert_eq!(114, part1(&input));
/// ```
pub fn part1(input: &Vec<Vec<i32>>) -> i32 {
    // Pretty sure this can be solved with geometric series shit, but I'm not a fucking math major
    input
        .iter()
        .map(|sequence| {
            let mut next = 0;
            let mut working_set = sequence.clone();
            let mut depth = 1;

            loop {
                let mut zeroed = true;

                for i in 0..(working_set.len() - depth) {
                    let diff = working_set[i + 1] - working_set[i];
                    working_set[i] = diff;
                    if diff != 0 {
                        zeroed = false;
                    }
                }

                next += working_set[working_set.len() - depth];

                if zeroed {
                    break;
                }

                depth += 1;
            }

            next
        })
        .sum()
}

/// ```rust
/// use advent_of_code_2023::day9::*;
/// let input = parse(
/// r"0 3 6 9 12 15");
/// assert_eq!(-3, part2(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day9::*;
/// let input = parse(
/// r"1 3 6 10 15 21");
/// assert_eq!(0, part2(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day9::*;
/// let input = parse(
/// r"10 13 16 21 30 45");
/// assert_eq!(5, part2(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day9::*;
/// let input = parse(
/// r"0 3 6 9 12 15
/// 1 3 6 10 15 21
/// 10 13 16 21 30 45");
/// assert_eq!(2, part2(&input));
/// ```
pub fn part2(input: &Vec<Vec<i32>>) -> i32 {
    input
        .iter()
        .map(|sequence| {
            let mut next = 0;
            let mut working_set = sequence.clone();
            let mut depth = 1;

            loop {
                let mut zeroed = true;

                for i in (depth..(working_set.len() - 1)).rev() {
                    let diff = working_set[i - 1] - working_set[i];
                    working_set[i] = diff;
                    if diff != 0 {
                        zeroed = false;
                    }
                }

                next += working_set[depth - 1];

                if zeroed {
                    break;
                }

                depth += 1;
            }

            next
        })
        .sum()
}
