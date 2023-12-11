use advent::*;

advent_day!(Day10, parse, PipeMap, part1, part2);

pub fn parse(input: &str) -> StarMap {
    let stars = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes().enumerate().filter_map(move |(x, char)| {
                if char == b'#' {
                    Some(UPoint::new(x as u32, y as u32))
                } else {
                    None
                }
            })
        })
        .collect();
    StarMap { stars }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StarMap {
    stars: Vec<UPoint>,
}

impl StarMap {}

/// ```rust
/// use advent_of_code_2023::day10::*;
/// let input = parse(
/// r"...#......
/// .......#..
/// #.........
/// ..........
/// ......#...
/// .#........
/// .........#
/// ..........
/// .......#..
/// #...#.....");
/// assert_eq!(374, part1(&input));
/// ```
pub fn part1(map: &StarMap) -> u32 {
    0
}

/// ```rust
/// use advent_of_code_2023::day10::*;
/// let input = parse(
/// r"...#......
/// .......#..
/// #.........
/// ..........
/// ......#...
/// .#........
/// .........#
/// ..........
/// .......#..
/// #...#.....");
/// //assert_eq!(4, part2(&input));
/// ```
pub fn part2(map: &StarMap) -> u32 {
    0
}
