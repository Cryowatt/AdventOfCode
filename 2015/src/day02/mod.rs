use advent::*;

advent_day!(Day02, parse, Vec<(u32, u32, u32)>, part1, part2);

pub fn parse(input: &str) -> Vec<(u32, u32, u32)> {
    input
        .lines()
        .map(|line| {
            let dimensions: Vec<&str> = line.split('x').collect();
            (
                dimensions[0].parse().unwrap(),
                dimensions[1].parse().unwrap(),
                dimensions[2].parse().unwrap(),
            )
        })
        .collect()
}

/// ```rust
/// use advent_of_code_2015::day02::*;
/// assert_eq!(58, part1(&(parse("2x3x4"))));
/// ```
/// ```rust
/// use advent_of_code_2015::day02::*;
/// assert_eq!(43, part1(&parse("1x1x10")));
/// ```
pub fn part1(input: &Vec<(u32, u32, u32)>) -> u32 {
    input
        .iter()
        .map(|(length, width, height)| {
            let face = width * height;
            let side = length * height;
            let footprint = length * width;
            face * 2 + side * 2 + footprint * 2 + face.min(side.min(footprint))
        })
        .sum()
}

/// ```rust
/// use advent_of_code_2015::day02::*;
/// assert_eq!(34, part2(&(parse("2x3x4"))));
/// ```
/// ```rust
/// use advent_of_code_2015::day02::*;
/// assert_eq!(14, part2(&parse("1x1x10")));
/// ```
pub fn part2(input: &Vec<(u32, u32, u32)>) -> u32 {
    input
        .iter()
        .map(|(length, width, height)| {
            let volume = length * width * height;
            let face = (2 * width) + (2 * height);
            let side = (2 * length) + (2 * height);
            let footprint = (2 * length) + (2 * width);
            face.min(side.min(footprint)) + volume
        })
        .sum()
}
