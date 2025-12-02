use advent::*;
use num::Integer;

advent_day!(Day01, parse, Vec<i32>, part1, part2);

pub fn parse(input: &str) -> InputType<'_> {
    input
        .lines()
        .map(|line| {
            let tokens = line.split_at(1);
            let direction = match tokens.0 {
                "L" => -1,
                "R" => 1,
                _ => panic!(),
            };
            direction * tokens.1.parse::<i32>().unwrap()
        })
        .collect()
}

/// ```rust
/// use advent_of_code_2025::day01::*;
/// let input = parse(
/// r"L68
/// L30
/// R48
/// L5
/// R60
/// L55
/// L1
/// L99
/// R14
/// L82");
/// assert_eq!(3, part1(&input));
/// ```
pub fn part1(input: &InputType) -> usize {
    input
        .iter()
        .copied()
        .scan(50, |state, rotation| {
            *state = (*state + rotation) % 100;
            Some(*state)
        })
        .filter(|&state| state == 0)
        .count()
}

/// ```rust
/// use advent_of_code_2025::day01::*;
/// let input = parse(
/// r"L68
/// L30
/// R48
/// L5
/// R60
/// L55
/// L1
/// L99
/// R14
/// L82");
/// assert_eq!(6, part2(&input));
/// ```
/// ```rust
/// use advent_of_code_2025::day01::*;
/// let input = parse(
/// r"L50");
/// assert_eq!(1, part2(&input));
/// ```
/// ```rust
/// use advent_of_code_2025::day01::*;
/// let input = parse(
/// r"L150");
/// assert_eq!(2, part2(&input));
/// ```
pub fn part2(input: &InputType) -> i32 {
    input
        .iter()
        .copied()
        .scan(50, |state, rotation| {
            let rotation_sign = rotation.signum();
            let new_state = *state + rotation;
            // Match zero crossings
            let zc = match (state.signum(), new_state.signum()) {
                (0, _) => 0,
                (1, 1) => 0,
                (-1, -1) => 0,
                _ => 1,
            };
            // Count rotations and normalize position
            let (q, r) = new_state.div_rem(&(100 * rotation_sign));
            *state = r;
            Some(q + zc)
        })
        .sum()
}
