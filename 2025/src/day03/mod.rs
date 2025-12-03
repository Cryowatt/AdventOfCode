use advent::*;

advent_day!(Day03, parse, Vec<Vec<u8>>, part1, part2);

pub fn parse(input: &str) -> InputType<'_> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

/// ```rust
/// use advent_of_code_2025::day03::*;
/// let input = parse(
/// r"987654321111111
/// 811111111111119
/// 234234234234278
/// 818181911112111");
/// assert_eq!(357, part1(&input));
/// ```
pub fn part1(input: &InputType) -> u64 {
    input
        .iter()
        .map(|line| {
            let (index, &first_digit) = line[..line.len() - 1]
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|&(_, value)| *value)
                .unwrap();
            let &second_digit = line[index + 1..].iter().max().unwrap();
            first_digit as u64 * 10 + second_digit as u64
        })
        .sum()
}

/// ```rust
/// use advent_of_code_2025::day03::*;
/// let input = parse(
/// r"987654321111111
/// 811111111111119
/// 234234234234278
/// 818181911112111");
/// assert_eq!(3121910778619, part2(&input));
/// ```
pub fn part2(input: &InputType) -> u64 {
    input
        .iter()
        .map(|line| {
            let mut joltage = 0u64;
            let mut index = 0;

            for i in 0..12 {
                let (offset, &digit) = line[index..line.len() - (11 - i)]
                    .iter()
                    .enumerate()
                    .rev()
                    .max_by_key(|&(_, value)| *value)
                    .unwrap();
                index += offset + 1;
                joltage = joltage * 10 + digit as u64
            }
            joltage
        })
        .sum()
}
