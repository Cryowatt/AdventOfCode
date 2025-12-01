use advent::*;

advent_day!(Day01, parse, Vec<i32>, part1, part2);

pub fn parse(input: &str) -> InputType {
    input
        .lines()
        .map(|line| {
            let tokens = line.split_at(1);
            let direction = (match tokens.0 {
                "L" => -1,
                "R" => 1,
                _ => panic!(),
            });
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
            // match *state {
            //     0 => Some(1),
            //     _ => Some(0),
            // }
        })
        .filter(|&state| state == 0)
        .count()
}

// /// ```rust
// /// use advent_of_code_2025::day01::*;
// /// let input = parse(
// /// r"3   4
// /// 4   3
// /// 2   5
// /// 1   3
// /// 3   9
// /// 3   3");
// /// assert_eq!(31, part2(&input));
// /// ```
pub fn part2(input: &InputType) -> i32 {
    i32::MIN
}
