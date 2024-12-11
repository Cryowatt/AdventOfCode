use advent::*;
use num::Integer;
use rayon::prelude::*;

advent_day!(Day11, parse, Vec<u32>, part1, part2);

pub fn parse(input: &str) -> InputType {
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
pub fn part1(input: &InputType) -> usize {
    input
        .iter()
        .map(|value| *value as u64)
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .blink()
        .count()
}

trait PlutonianIterator<I> {
    fn blink(self) -> Plutonian<I>;
}

impl<I: Iterator<Item = u64>> PlutonianIterator<I> for I {
    fn blink(self) -> Plutonian<I> {
        Plutonian::new(self)
    }
}

pub struct Plutonian<I> {
    iter: I,
    buffer: Option<u64>,
}

impl<I> Plutonian<I> {
    fn new(iter: I) -> Plutonian<I> {
        Plutonian { iter, buffer: None }
    }
}

impl<'a, I: Iterator<Item = u64>> Iterator for Plutonian<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.buffer.take() {
            Some(value)
        } else {
            self.iter.next().and_then(|value| {
                if value == 0 {
                    // If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
                    Some(1)
                } else {
                    let digits = value.ilog10() + 1;
                    let (half, rem) = digits.div_rem(&2);

                    if rem == 0 {
                        // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
                        let left = value / 10u64.pow(half);
                        self.buffer = Some(value - left * 10u64.pow(half));
                        Some(left)
                    } else {
                        // If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
                        Some(value * 2024)
                    }
                }
            })
        }
    }
}

/// ```rust
/// use advent_of_code_2024::day11::*;
/// let input = parse(
/// r"125 17");
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &InputType) -> u32 {
    0
}
