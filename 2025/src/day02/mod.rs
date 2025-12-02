use advent::*;
use num::Integer;
use onig::Regex;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_day!(Day02, parse, Vec<(u64, u64)>, part1, part2);

pub fn parse(input: &str) -> InputType<'_> {
    input
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect()
}

/// ```rust
/// use advent_of_code_2025::day02::*;
/// let input = parse(
/// r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124");
/// assert_eq!(1227775554, part1(&input));
/// ```
pub fn part1(input: &InputType) -> u64 {
    input
        .iter()
        .map(|&(start, end)| {
            let prefix_start = start.shr10(start.digits().div_ceil(2));
            let prefix_end = end.shr10(end.digits() / 2);
            (prefix_start..=prefix_end)
                .filter_map(|prefix| {
                    if prefix == 0 {
                        None
                    } else {
                        let id = prefix.shl10(prefix.digits()) + prefix;
                        if id >= start && id <= end {
                            Some(id)
                        } else {
                            None
                        }
                    }
                })
                .sum::<u64>()
        })
        .sum()
}

/// ```rust
/// use advent_of_code_2025::day02::*;
/// let input = parse(
/// r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124");
/// assert_eq!(4174379265, part2(&input));
/// ```
pub fn part2(input: &InputType) -> u64 {
    let pattern_shift = [1, 10, 100, 1000, 10000, 100000];
    input
        .iter()
        .map(|(start, end)| {
            (*start..=*end)
                .filter(|id| {
                    let digits = id.digits();
                    for pattern_length in 1..digits {
                        let (repeats, remainder) = digits.div_rem(&pattern_length);
                        if repeats == 0 {
                            break;
                        }

                        if remainder != 0 {
                            continue;
                        }

                        // let pattern_shift = 1u64.shl10(pattern_length);
                        let pattern = (1..repeats).fold(1, |pattern, _| {
                            (pattern * pattern_shift[pattern_length as usize]) + 1
                        });

                        if id.div_exact(pattern).is_some() {
                            return true;
                        }
                    }
                    false
                })
                .sum::<u64>()
        })
        .sum()
}
