use advent::*;

advent_day!(Day02, parse, Vec<Vec<i32>>, part1, part2);

pub fn parse(input: &str) -> InputType<'_> {
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
pub fn part1(input: &InputType) -> usize {
    input.iter().filter(|&row| is_safe(row.iter())).count()
}

fn is_safe<'a, T: Iterator<Item = &'a i32>>(input: T) -> bool {
    input
        .map_windows(|[&x, &y]| match x - y {
            -3..=-1 => -1,
            1..=3 => 1,
            _ => 0,
        })
        .scan((0i32, 0i32), |state, x| {
            state.0 += 1;
            state.1 += x;
            Some((state.0, state.1))
        })
        .all(|(count, score)| score.abs() == count)
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
/// assert_eq!(4, part2(&input));
/// ```
pub fn part2(input: &InputType) -> usize {
    input
        .iter()
        .filter(|&row| {
            is_safe(row.iter())
                || (0..row.len()).any(|i| is_safe(row.iter().take(i).chain(row.iter().skip(i + 1))))
        })
        .count()
}
