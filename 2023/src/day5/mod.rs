use std::{
    cmp::Ordering,
    cmp::PartialOrd,
    collections::btree_map,
    ops::{Range, RangeInclusive},
};

use crate::{advent_bench, advent_day};

advent_day!(Day5, part1, part2);

/// ```rust
/// use advent_of_code::day5::*;
/// let input = r"seeds: 79 14 55 13
///
/// seed-to-soil map:
/// 50 98 2
/// 52 50 48
///
/// soil-to-fertilizer map:
/// 0 15 37
/// 37 52 2
/// 39 0 15
///
/// fertilizer-to-water map:
/// 49 53 8
/// 0 11 42
/// 42 0 7
/// 57 7 4
///
/// water-to-light map:
/// 88 18 7
/// 18 25 70
///
/// light-to-temperature map:
/// 45 77 23
/// 81 45 19
/// 68 64 13
///
/// temperature-to-humidity map:
/// 0 69 1
/// 1 0 69
///
/// humidity-to-location map:
/// 60 56 37
/// 56 93 4";
/// assert_eq!(35, part1(input));
/// ```
pub fn part1(input: &str) -> u32 {
    struct RangeMap {
        range: RangeInclusive<u32>,
        offset: u32,
    }

    fn map_regex(map_name: &str) -> String {
        let group_name = map_name.replace("-", "_");
        format!(
            r"{} map:(?P<{}>(?:\n(?:\d+ \d+ \d+))*)",
            map_name, group_name
        )
    }

    let data_parser = regex::Regex::new(
        format!(
            r"seeds:(?P<seeds>(?: \d+)+)\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}",
            map_regex("seed-to-soil"),
            map_regex("soil-to-fertilizer"),
            map_regex("fertilizer-to-water"),
            map_regex("water-to-light"),
            map_regex("light-to-temperature"),
            map_regex("temperature-to-humidity"),
            map_regex("humidity-to-location")
        )
        .as_str(),
    )
    .unwrap();
    let data = data_parser.captures(input).unwrap();
    let seeds = data
        .name("seeds")
        .map(|seeds| {
            seeds
                .as_str()
                .split_whitespace()
                .map(|id| id.parse::<u32>().expect("Seed is not u32"))
        })
        .expect("Failed to parse seeds");

    let parse_map = |name: &str| -> Vec<RangeMap> {
        let mut map: Vec<RangeMap> = data
            .name(name)
            .map(|section| {
                section.as_str().trim().lines().map(|line| {
                    let tokens: Vec<u32> = line
                        .split_whitespace()
                        .map(|token| token.parse::<u32>().expect("Failed to parse as u32"))
                        .collect();
                    let range = tokens[1]..=(tokens[1] + (tokens[2] - 1));
                    let offset = tokens[0].wrapping_sub(tokens[1]);
                    RangeMap { range, offset }
                    // tokens.
                })
            })
            .unwrap()
            .collect();
        map.sort_by_key(|item| *item.range.start());
        map
    };

    let seed_to_soil: Vec<RangeMap> = parse_map("seed_to_soil");
    let soil_to_fertilizer: Vec<RangeMap> = parse_map("soil_to_fertilizer");
    let fertilizer_to_water: Vec<RangeMap> = parse_map("fertilizer_to_water");
    let water_to_light: Vec<RangeMap> = parse_map("water_to_light");
    let light_to_temperature: Vec<RangeMap> = parse_map("light_to_temperature");
    let temperature_to_humidity: Vec<RangeMap> = parse_map("temperature_to_humidity");
    let humidity_to_location: Vec<RangeMap> = parse_map("humidity_to_location");
    let maps = [
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ];

    seeds
        .map(|seed| {
            maps.iter().fold(seed, |loc, map| {
                map.binary_search_by(|probe| {
                    if probe.range.contains(&loc) {
                        Ordering::Equal
                    } else {
                        probe.range.start().cmp(&loc)
                    }
                })
                .map_or(loc, |index| loc.wrapping_add(map[index].offset))
            })
        })
        .min()
        .unwrap()
}

/// ```rust
/// use advent_of_code::day5::*;
/// let input = r"seeds: 79 14 55 13
///
/// seed-to-soil map:
/// 50 98 2
/// 52 50 48
///
/// soil-to-fertilizer map:
/// 0 15 37
/// 37 52 2
/// 39 0 15
///
/// fertilizer-to-water map:
/// 49 53 8
/// 0 11 42
/// 42 0 7
/// 57 7 4
///
/// water-to-light map:
/// 88 18 7
/// 18 25 70
///
/// light-to-temperature map:
/// 45 77 23
/// 81 45 19
/// 68 64 13
///
/// temperature-to-humidity map:
/// 0 69 1
/// 1 0 69
///
/// humidity-to-location map:
/// 60 56 37
/// 56 93 4";
/// //assert_eq!(13, part1(input));
/// ```
pub fn part2(input: &str) -> u32 {
    0
    // unimplemented!()
}
