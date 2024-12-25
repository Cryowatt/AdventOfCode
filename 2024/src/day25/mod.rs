use advent::*;

advent_day!(Day25, parse, Vec<Lockset>, part1, part2);

#[derive(Debug)]
pub enum Lockset {
    Key([u8; 5]),
    Lock([u8; 5]),
}

pub fn parse(input: &str) -> InputType {
    input
        .split("\n\n")
        .map(|block| {
            let decode = block.lines().fold([0u8; 5], |mut decode, line| {
                let chars = line.as_bytes();
                for i in 0..5 {
                    if chars[i] == b'#' {
                        decode[i] += 1;
                    }
                }
                decode
            });

            if block.starts_with(".....") {
                Lockset::Key(decode)
            } else {
                Lockset::Lock(decode)
            }
        })
        .collect()
}

/// ```rust
/// use advent_of_code_2024::day25::*;
/// let input = parse(
/// r"#####
///# .####
///# .####
///# .####
///# .#.#.
///# .#...
///# .....
///#
///# #####
///# ##.##
///# .#.##
///# ...##
///# ...#.
///# ...#.
///# .....
///#
///# .....
///# #....
///# #....
///# #...#
///# #.#.#
///# #.###
///# #####
///#
///# .....
///# .....
///# #.#..
///# ###..
///# ###.#
///# ###.#
///# #####
///#
///# .....
///# .....
///# .....
///# #....
///# #.#..
///# #.#.#
///# #####");
/// assert_eq!(3, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2024::day25::*;
/// let input = parse(
///r"#####
///# #####
///# #####
///# #####
///# #####
///# #####
///# .....
///#
///# .....
///# .....
///# .....
///# .....
///# .....
///# ..#..
///# #####");
/// assert_eq!(0, part1(&input));
/// ```
pub fn part1(input: &InputType) -> u32 {
    let mut locks: Vec<&[u8; 5]> = vec![];
    let mut keys: Vec<&[u8; 5]> = vec![];
    let mut matching_pairs = 0;

    for lockset in input {
        match lockset {
            Lockset::Key(code) => {
                keys.push(code);
                'locks: for pins in locks.iter() {
                    for i in 0..5 {
                        if code[i] + pins[i] > 7 {
                            continue 'locks;
                        }
                    }

                    matching_pairs += 1;
                }
            }
            Lockset::Lock(pins) => {
                locks.push(pins);
                'keys: for code in keys.iter() {
                    for i in 0..5 {
                        if code[i] + pins[i] > 7 {
                            continue 'keys;
                        }
                    }

                    matching_pairs += 1;
                }
            }
        }
    }

    matching_pairs
}

/// ```rust
/// use advent_of_code_2024::day25::*;
/// let input = parse(
/// r"#####
///# .####
///# .####
///# .####
///# .#.#.
///# .#...
///# .....
///#
///# #####
///# ##.##
///# .#.##
///# ...##
///# ...#.
///# ...#.
///# .....
///#
///# .....
///# #....
///# #....
///# #...#
///# #.#.#
///# #.###
///# #####
///#
///# .....
///# .....
///# #.#..
///# ###..
///# ###.#
///# ###.#
///# #####
///#
///# .....
///# .....
///# .....
///# #....
///# #.#..
///# #.#.#
///# #####");
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &InputType) -> i32 {
    0
}
