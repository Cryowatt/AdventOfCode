use advent::*;

advent_day!(Day07, parse, Vec<(i64, Vec<i64>)>, part1, part2);

pub fn parse(input: &str) -> InputType {
    input
        .lines()
        .map(|line| {
            let (sum, terms) = line.split_once(':').unwrap();
            (
                sum.parse().unwrap(),
                terms
                    .split_whitespace()
                    .map(|term| term.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

/// ```rust
/// use advent_of_code_2024::day07::*;
/// let input = parse(
/// r"190: 10 19
/// 3267: 81 40 27
/// 83: 17 5
/// 156: 15 6
/// 7290: 6 8 6 15
/// 161011: 16 10 13
/// 192: 17 8 14
/// 21037: 9 7 18 13
/// 292: 11 6 16 20");
/// assert_eq!(3749, part1(&input));
/// ```
pub fn part1(input: &InputType) -> i64 {
    input
        .iter()
        .filter_map(|row| is_solvable(row.0, &row.1).then_some(row.0))
        .sum()
}

fn is_solvable(target: i64, terms: &Vec<i64>) -> bool {
    fn solve(accumulator: i64, terms: &[i64], target: i64) -> bool {
        if let Some((term, tail_terms)) = terms.split_first() {
            let product = accumulator * term;
            let sum = accumulator + term;
            (product <= target && solve(product, tail_terms, target))
                || (sum <= target && solve(sum, tail_terms, target))
        } else {
            target == accumulator
        }
    }

    let (term, tail_terms) = terms.split_first().unwrap();
    solve(*term, tail_terms, target)
}

/// ```rust
/// use advent_of_code_2024::day07::*;
/// let input = parse(
/// r"190: 10 19
/// 3267: 81 40 27
/// 83: 17 5
/// 156: 15 6
/// 7290: 6 8 6 15
/// 161011: 16 10 13
/// 192: 17 8 14
/// 21037: 9 7 18 13
/// 292: 11 6 16 20");
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &InputType) -> i32 {
    0
}
