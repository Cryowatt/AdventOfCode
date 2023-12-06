use advent::*;

advent_day!(Day6, parse, Vec<BoatRace>, part1, part2);

pub struct BoatRace {
    time: u32,
    meta_time_length: u32,
    distance: u32,
    meta_distance_length: u32,
}

pub fn parse(input: &str) -> Vec<BoatRace> {
    let mut rows = input.lines().map(|line| {
        line.split_whitespace()
            .skip(1)
            .map(|value| (value.parse::<u32>().unwrap(), value.len() as u32))
    });
    let time = rows.next().unwrap();
    let distance = rows.next().unwrap();

    time.zip(distance)
        .map(|(time, distance)| BoatRace {
            time: time.0,
            meta_time_length: time.1,
            distance: distance.0,
            meta_distance_length: distance.1,
        })
        .collect()
}

/// ```rust
/// use advent_of_code_2023::day6::*;
/// let input = parse(
/// r"Time:      7  15   30
/// Distance:  9  40  200");
/// assert_eq!(288, part1(&input));
/// ```
pub fn part1(input: &Vec<BoatRace>) -> u32 {
    input
        .iter()
        .map(|race| {
            let time = race.time as f64;
            let target = (race.distance + 1) as f64;
            let quad = (-time + (time.powi(2) - (-4.0 * -target)).sqrt()) / -2.0;
            ((time + 1.0) - quad.ceil() * 2.0) as u32
        })
        .product()
}

/// ```rust
/// use advent_of_code_2023::day6::*;
/// let input = parse(
/// r"Time:      7  15   30
/// Distance:  9  40  200");
/// assert_eq!(71503, part2(&input));
/// ```
pub fn part2(input: &Vec<BoatRace>) -> u32 {
    let (time, distance) = input.iter().fold((0, 0), |acc, race| {
        (
            acc.0 * (10u64.pow(race.meta_time_length as u32)) as u64 + race.time as u64,
            acc.1 * (10u64.pow(race.meta_distance_length as u32)) as u64 + race.distance as u64,
        )
    });

    let time = time as f64;
    let target = (distance + 1) as f64;
    let quad = (-time + (time.powi(2) - (-4.0 * -target)).sqrt()) / -2.0;
    ((time + 1.0) - quad.ceil() * 2.0) as u32
}
