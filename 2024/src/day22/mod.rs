use std::collections::HashMap;

use advent::*;
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
/// 2
/// 3
/// 2024");
/// assert_eq!(23, part2(&input));
/// ```
pub fn part2(input: &InputType) -> i32 {
    const SEQUENCE_MAX: usize = 19 * 19 * 19 * 19;
    fn push_sequence(price_change: i32, sequence: i32) -> i32 {
        ((sequence * 19) + (price_change + 10)) % SEQUENCE_MAX as i32
    }

    let sequences = input
        .par_iter()
        .map(|init| {
            let mut sequence_score = HashMap::new();
            // let mut sequence_score = [0u32; SEQUENCE_MAX];
            let mut sequence = 0;
            let mut secret = *init;
            let mut last_price = (secret % 10) as i32;

            secret = generate(secret);
            let price = secret as i32 % 10;
            sequence = push_sequence(price - last_price, sequence);
            last_price = price;
            secret = generate(secret);
            let price = secret as i32 % 10;
            sequence = push_sequence(price - last_price, sequence);
            last_price = price;
            secret = generate(secret);
            let price = secret as i32 % 10;
            sequence = push_sequence(price - last_price, sequence);
            last_price = price;

            for _ in 3..2000 {
                secret = generate(secret);
                let price = secret as i32 % 10;
                sequence = push_sequence(price - last_price, sequence);
                let _ = sequence_score.try_insert(sequence, price);
                last_price = price;
            }
            sequence_score
        })
        .reduce(
            || HashMap::new(),
            |mut all_scores, scores| {
                for (sequence, bananas) in scores {
                    match all_scores.try_insert(sequence, bananas) {
                        Ok(_) => {}
                        Err(mut err) => *(err.entry.get_mut()) += bananas,
                    }
                }
                all_scores
            },
        );

    *sequences.values().max().unwrap()
}
