use advent::*;

advent_day!(Day24, parse, HeisenbergCompensator, part1, part2);

pub fn parse(input: &str) -> HeisenbergCompensator {
    input.lines().map(|line| {let (position, velocity) = line.split_once("@").unwrap();
    
    let position = {
        let position_values = position.split_terminator(", ");

        Point3D::new(position_values.next().unwrap().parse().unwrap(), position_values.next().unwrap().parse().unwrap(), position_values.next().unwrap().parse().unwrap())
    };
})
    HeisenbergCompensator {

    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeisenbergCompensator {}

/// ```rust
/// use advent_of_code_2023::Day24::*;
/// let input = parse(
/// r"19, 13, 30 @ -2,  1, -2
/// 18, 19, 22 @ -1, -1, -2
/// 20, 25, 34 @ -2, -2, -4
/// 12, 31, 28 @ -1, -2, -1
/// 20, 19, 15 @  1, -5, -3");
/// assert_eq!(2, part1(&input));
/// ```
pub fn part1(map: &HeisenbergCompensator) -> u32 {
    0
}

/// ```rust
/// use advent_of_code_2023::Day24::*;
/// let input = parse(
/// r"19, 13, 30 @ -2,  1, -2
/// 18, 19, 22 @ -1, -1, -2
/// 20, 25, 34 @ -2, -2, -4
/// 12, 31, 28 @ -1, -2, -1
/// 20, 19, 15 @  1, -5, -3");
/// //assert_eq!(?, part1(&input));
/// ```
pub fn part2(map: &HeisenbergCompensator) -> u32 {
    0
}
