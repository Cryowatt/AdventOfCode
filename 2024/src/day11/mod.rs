use std::collections::HashMap;

use advent::*;
use num::Integer;

advent_day!(Day11, parse, Vec<u32>, part1, part2);

pub fn parse(input: &str) -> InputType<'_> {
    input
        .split_whitespace()
        .map(|token| token.parse().unwrap())
        .collect()
}

/// ```rust
/// use advent_of_code_2024::day11::*;
/// let input = parse(
/// r"125 17");
/// assert_eq!(55312, part1(&input));
/// ```
pub fn part1(input: &InputType) -> u64 {
    let mut memo = HashMap::new();
    input
        .iter()
        .map(|value| blink(*value as u64, 25, &mut memo))
        .sum()
}

/// ```rust
/// use advent_of_code_2024::day11::*;
/// let input = parse(
/// r"125 17");
/// assert_eq!(65601038650482, part2(&input));
/// ```
pub fn part2(input: &InputType) -> u64 {
    let mut memo = HashMap::new();
    input
        .iter()
        .map(|value| blink(*value as u64, 75, &mut memo))
        .sum()
}

fn blink(value: u64, blinks: u32, memo: &mut HashMap<(u64, u32), u64>) -> u64 {
    if let Some(count) = memo.get_mut(&(value, blinks)) {
        *count
    } else {
        let count = if blinks == 0 {
            1
        } else if value == 0 {
            // If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
            blink(1, blinks - 1, memo)
        } else {
            let digits = value.ilog10() + 1;
            let (half, rem) = digits.div_rem(&2);

            if rem == 0 {
                // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
                let left = value / 10u64.pow(half);
                let right = value - left * 10u64.pow(half);
                blink(left, blinks - 1, memo) + blink(right, blinks - 1, memo)
            } else {
                // If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
                blink(value * 2024, blinks - 1, memo)
            }
        };

        memo.insert((value, blinks), count);
        count
    }
}
