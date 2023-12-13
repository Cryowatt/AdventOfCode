use advent::*;
use md5::{
    digest::{generic_array::GenericArray, typenum::U16},
    Digest, Md5,
};

advent_day!(Day04, parse, &str, part1, part2);

pub fn parse(input: &str) -> &str {
    input
}

fn md5_find_digits<F: Fn(GenericArray<u8, U16>) -> bool>(
    prefixed_hash: &Md5,
    magnitude: u32,
    predicate: &F,
) -> Option<u32> {
    let digits_hash = (0..=9)
        .map(|digit| {
            let mut hash = prefixed_hash.clone();
            hash.update(&[b'0' + digit]);
            hash
        })
        .enumerate();

    if magnitude >= 1 {
        for (digit, hash) in digits_hash {
            if let Some(index) = md5_find_digits(&hash.clone(), magnitude - 1, predicate) {
                return Some((digit as u32 * 10u32.pow(magnitude)) + index);
            }
        }
    } else {
        for (digit, hash) in digits_hash {
            let result = hash.finalize();
            if predicate(result) {
                return Some(digit as u32);
            }
        }
    }

    None
}

fn md5_find<F: Fn(GenericArray<u8, U16>) -> bool>(input: &str, predicate: F) -> Option<u32> {
    let prefixed_hash = Md5::new_with_prefix(input);
    md5_find_digits(&prefixed_hash, 5, &predicate)
        // .or_else(|| md5_find_digits(&prefixed_hash, 1, &predicate))
        // .or_else(|| md5_find_digits(&prefixed_hash, 2, &predicate))
        // .or_else(|| md5_find_digits(&prefixed_hash, 3, &predicate))
        // .or_else(|| md5_find_digits(&prefixed_hash, 4, &predicate))
        // .or_else(|| md5_find_digits(&prefixed_hash, 5, &predicate))
        .or_else(|| md5_find_digits(&prefixed_hash, 6, &predicate))
}

/// ```rust
/// use advent_of_code_2015::day04::*;
/// assert_eq!(609043, part1(&(parse("abcdef"))));
/// ```
pub fn part1(input: &str) -> u32 {
    md5_find(input, |result| {
        result[0] == 0 && result[1] == 0 && result[2] & 0xf0 == 0
    })
    .unwrap()
}

/// ```rust
/// use advent_of_code_2015::day04::*;
/// assert_eq!(6742839, part2(&(parse("abcdef"))));
/// ```
pub fn part2(input: &str) -> u32 {
    md5_find(input, |result| {
        result[0] == 0 && result[1] == 0 && result[2] == 0
    })
    .unwrap()
}
