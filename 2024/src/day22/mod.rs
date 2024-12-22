use advent::*;
use num::traits::WrappingMul;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_day!(Day22, parse, Vec<u32>, part1, part2);

pub fn parse(input: &str) -> InputType {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

/// ```rust
/// use advent_of_code_2024::day22::*;
/// let mut secret = generate(123);
/// assert_eq!(15887950, secret);
/// secret = generate(secret);
/// assert_eq!(16495136, secret);
/// secret = generate(secret);
/// assert_eq!(527345, secret);
/// secret = generate(secret);
/// assert_eq!(704524, secret);
/// secret = generate(secret);
/// assert_eq!(1553684, secret);
/// secret = generate(secret);
/// assert_eq!(12683156, secret);
/// secret = generate(secret);
/// assert_eq!(11100544, secret);
/// secret = generate(secret);
/// assert_eq!(12249484, secret);
/// secret = generate(secret);
/// assert_eq!(7753432, secret);
/// secret = generate(secret);
/// assert_eq!(5908254, secret);
/// ```
pub fn generate(mut secret: u32) -> u32 {
    secret = secret.wrapping_mul(64) ^ secret;
    secret = secret % 16777216;
    secret = (secret / 32) ^ secret;
    secret = secret % 16777216;
    secret = secret.wrapping_mul(2048) ^ secret;
    secret % 16777216
}

/// ```rust
/// use advent_of_code_2024::day22::*;
/// let input = parse(
/// r"1
/// 10
/// 100
/// 2024");
/// assert_eq!(37327623, part1(&input));
/// ```
pub fn part1(input: &InputType) -> u64 {
    input
        .par_iter()
        .map(|init| {
            let mut secret = *init;
            for _ in 0..2000 {
                secret = generate(secret);
            }
            secret as u64
        })
        .sum()
}

/// ```rust
/// use advent_of_code_2024::day22::*;
/// let input = parse(
/// r"1
/// 10
/// 100
/// 2024");
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &InputType) -> usize {
    0
}
