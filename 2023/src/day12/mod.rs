use std::collections::HashMap;

use advent::*;

advent_day!(Day12, parse, Vec<SpringRecord>, part1, part2);

pub fn parse(input: &str) -> Vec<SpringRecord> {
    input
        .lines()
        .map(|line| {
            let (row, pattern) = line.split_once(" ").unwrap();
            SpringRecord {
                row: row.as_bytes(),
                pattern: pattern
                    .split(",")
                    .map(|number| number.parse::<usize>().unwrap())
                    .collect(),
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpringRecord<'a> {
    row: &'a [u8],
    pattern: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Memo<'a> {
    row: &'a [u8],
    pattern: &'a [usize],
}

const BROKEN: u8 = b'#';
const OPERATIONAL: u8 = b'.';

fn trim_head_operational<'a>(row: &'a [u8]) -> &'a [u8] {
    for i in 0..row.len() {
        if row[i] != OPERATIONAL {
            return &row[i..];
        }
    }

    &[]
}

fn trim_tail_operational(row: &[u8]) -> Option<&[u8]> {
    for i in 0..row.len() {
        if row[i] == BROKEN {
            return Some(&row[i..]);
        }
    }

    return None;
}

fn trim_broken<'a>(row: &'a [u8], run_length: usize) -> Option<&'a [u8]> {
    for i in 0..run_length {
        if row[i] == OPERATIONAL {
            return None;
        }
    }

    return Some(&row[run_length..]);
}

fn matches_count<'a>(
    row: &'a [u8],
    pattern: &'a [usize],
    min_pattern_length: usize,
    memo: &mut Option<&mut HashMap<Memo<'a>, u64>>,
) -> u64 {
    // Memoization
    if let Some(matches) = memo
        .as_ref()
        .map_or(None, move |memo| memo.get(&Memo { row, pattern }))
    {
        return *matches;
    }

    let mut matches = 0;

    // Remove leading dots
    let row = trim_head_operational(row);

    // Remaining row isn't long enough for a pattern match
    if row.len() < min_pattern_length {
        return 0;
    }

    for i in 0..=(row.len() - min_pattern_length) {
        let row = &row[i..];

        // Match a run of broken/unknown cells
        if let Some(remaining) = trim_broken(row, pattern[0]) {
            if pattern.len() > 1 {
                // String of broken must end with a non-broken cell if there are additional patterns to match
                if remaining[0] != BROKEN {
                    matches += matches_count(
                        &remaining[1..],
                        &pattern[1..],
                        min_pattern_length - (pattern[0] + 1),
                        memo,
                    );
                }
            } else {
                // None means there's no more broken springs remaining, this is a match
                if None == trim_tail_operational(remaining) {
                    matches += 1
                }
            }
        }

        // Start cell is broken, there are no additional varations to check
        if row[0] == BROKEN {
            break;
        }
    }
    memo.as_mut()
        .map(|m| m.insert(Memo { row, pattern }, matches));
    matches
}

/// ```rust
/// use advent_of_code_2023::day12::*;
/// let input = parse(
/// r"???.### 1,1,3");
/// assert_eq!(1, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day12::*;
/// let input = parse(
/// r".??..??...?##. 1,1,3");
/// assert_eq!(4, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day12::*;
/// let input = parse(
/// r"?#?#?#?#?#?#?#? 1,3,1,6");
/// assert_eq!(1, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day12::*;
/// let input = parse(
/// r"????.#...#... 4,1,1");
/// assert_eq!(1, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day12::*;
/// let input = parse(
/// r"????.######..#####. 1,6,5");
/// assert_eq!(4, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day12::*;
/// let input = parse(
/// r"?###???????? 3,2,1");
/// assert_eq!(10, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day12::*;
/// let input = parse(
/// r"???.### 1,1,3
/// .??..??...?##. 1,1,3
/// ?#?#?#?#?#?#?#? 1,3,1,6
/// ????.#...#... 4,1,1
/// ????.######..#####. 1,6,5
/// ?###???????? 3,2,1");
/// assert_eq!(21, part1(&input));
/// ```
pub fn part1(input: &Vec<SpringRecord>) -> u64 {
    input
        .iter()
        .map(|record| {
            let min_pattern_length =
                record.pattern.iter().sum::<usize>() + record.pattern.len() - 1;

            matches_count(
                record.row,
                record.pattern.as_slice(),
                min_pattern_length,
                &mut None,
            )
        })
        .sum()
}

/// ```rust
/// use advent_of_code_2023::day12::*;
/// let input = parse(
/// r"???.### 1,1,3");
/// assert_eq!(1, part2(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day12::*;
/// let input = parse(
/// r".??..??...?##. 1,1,3");
/// assert_eq!(16384, part2(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day12::*;
/// let input = parse(
/// r"?#?#?#?#?#?#?#? 1,3,1,6");
/// assert_eq!(1, part2(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day12::*;
/// let input = parse(
/// r"????.#...#... 4,1,1");
/// assert_eq!(16, part2(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day12::*;
/// let input = parse(
/// r"????.######..#####. 1,6,5");
/// assert_eq!(2500, part2(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day12::*;
/// let input = parse(
/// r"?###???????? 3,2,1");
/// assert_eq!(506250, part2(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day12::*;
/// let input = parse(
/// r"???.### 1,1,3
/// .??..??...?##. 1,1,3
/// ?#?#?#?#?#?#?#? 1,3,1,6
/// ????.#...#... 4,1,1
/// ????.######..#####. 1,6,5
/// ?###???????? 3,2,1");
/// assert_eq!(525152, part2(&input));
/// ```
pub fn part2(input: &Vec<SpringRecord>) -> u64 {
    input
        .iter()
        .map(|record| {
            let unfolded_row = [record.row; 5].join(&b'?');
            let unfolded_pattern = record.pattern.repeat(5);
            let min_pattern_length =
                unfolded_pattern.iter().sum::<usize>() + unfolded_pattern.len() - 1;

            let mut memo = HashMap::<Memo, u64>::new();
            matches_count(
                unfolded_row.as_slice(),
                unfolded_pattern.as_slice(),
                min_pattern_length,
                &mut Some(&mut memo),
            )
        })
        .sum()
}
