use advent::*;

advent_day!(Day8, parse, WastelandMap, part1, part2);

pub fn parse(input: &str) -> WastelandMap {
    unimplemented!()
}

pub struct WastelandMap {}

/// ```rust
/// use advent_of_code_2023::day8::*;
/// let input = parse(
/// r"RL
///
/// AAA = (BBB, CCC)
/// BBB = (DDD, EEE)
/// CCC = (ZZZ, GGG)
/// DDD = (DDD, DDD)
/// EEE = (EEE, EEE)
/// GGG = (GGG, GGG)
/// ZZZ = (ZZZ, ZZZ)");
/// assert_eq!(6, part1(&input));
/// ```
pub fn part1(input: &WastelandMap) -> u32 {
    unimplemented!()
}

/// ```rust
/// use advent_of_code_2023::day8::*;
/// let input = parse(
/// r"RL
///
/// AAA = (BBB, CCC)
/// BBB = (DDD, EEE)
/// CCC = (ZZZ, GGG)
/// DDD = (DDD, DDD)
/// EEE = (EEE, EEE)
/// GGG = (GGG, GGG)
/// ZZZ = (ZZZ, ZZZ)");
/// //assert_eq!(?, part2(&input));
/// ```
pub fn part2(input: &WastelandMap) -> u32 {
    unimplemented!()
}
