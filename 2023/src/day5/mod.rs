use std::{cmp::Ordering, ops::RangeInclusive};

use crate::advent_day;

advent_day!(Day5, parse, SeedMaps, part1, part2);

pub fn parse(input: &str) -> SeedMaps {
    fn map_regex(map_name: &str) -> String {
        let group_name = map_name.replace("-", "_");
        format!(
            r"{} map:(?P<{}>(?:\s+(?:\d+ \d+ \d+))*)",
            map_name, group_name
        )
    }

    let data_parser = regex::Regex::new(
        format!(
            r"seeds:(?P<seeds>(?: \d+)+)\s+{}\s+{}\s+{}\s+{}\s+{}\s+{}\s+{}",
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
    let seeds: Vec<u32> = data
        .name("seeds")
        .map(|seeds| {
            seeds
                .as_str()
                .split_whitespace()
                .map(|id| id.parse::<u32>().expect("Seed is not u32"))
        })
        .expect("Failed to parse seeds")
        .collect();

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
    SeedMaps {
        seeds,
        seed_to_soil: parse_map("seed_to_soil"),
        soil_to_fertilizer: parse_map("soil_to_fertilizer"),
        fertilizer_to_water: parse_map("fertilizer_to_water"),
        water_to_light: parse_map("water_to_light"),
        light_to_temperature: parse_map("light_to_temperature"),
        temperature_to_humidity: parse_map("temperature_to_humidity"),
        humidity_to_location: parse_map("humidity_to_location"),
    }
}

struct RangeMap {
    range: RangeInclusive<u32>,
    offset: u32,
}

impl RangeMap {
    fn contains(&self, location: &u32) -> bool {
        self.range.contains(location)
    }

    fn offset(&self, location: u32) -> u32 {
        location.wrapping_add(self.offset)
    }
}

pub struct SeedMaps {
    seeds: Vec<u32>,
    seed_to_soil: Vec<RangeMap>,
    soil_to_fertilizer: Vec<RangeMap>,
    fertilizer_to_water: Vec<RangeMap>,
    water_to_light: Vec<RangeMap>,
    light_to_temperature: Vec<RangeMap>,
    temperature_to_humidity: Vec<RangeMap>,
    humidity_to_location: Vec<RangeMap>,
}

/// ```rust
/// use advent_of_code::day5::*;
/// let input = parse(
/// r"seeds: 79 14 55 13
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
/// 56 93 4");
/// assert_eq!(35, part1(&input));
/// ```
pub fn part1(input: &SeedMaps) -> u32 {
    let maps = [
        &input.seed_to_soil,
        &input.soil_to_fertilizer,
        &input.fertilizer_to_water,
        &input.water_to_light,
        &input.light_to_temperature,
        &input.temperature_to_humidity,
        &input.humidity_to_location,
    ];

    input
        .seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |loc, map| {
                map.binary_search_by(|probe| {
                    if probe.range.contains(&loc) {
                        Ordering::Equal
                    } else {
                        probe.range.start().cmp(&loc)
                    }
                })
                .map_or(loc, |index| map[index].offset(loc))
            })
        })
        .min()
        .unwrap()
}

/// ```rust
/// use advent_of_code::day5::*;
/// let input = parse(
/// r"seeds: 79 14 55 13
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
/// 56 93 4");
/// assert_eq!(46, part2(&input));
/// ```
pub fn part2(input: &SeedMaps) -> u32 {
    let mut seed_ranges: Vec<RangeInclusive<u32>> = input
        .seeds
        .chunks_exact(2)
        .map(|chunk| chunk[0]..=(chunk[0] + (chunk[1] - 1)))
        .collect();

    let location_map = |locations: Vec<RangeInclusive<u32>>,
                        map: &Vec<RangeMap>|
     -> Vec<RangeInclusive<u32>> {
        locations
            .iter()
            .flat_map(|location_range| {
                let start_index = map
                    .binary_search_by(|probe| {
                        if probe.contains(location_range.start()) {
                            Ordering::Equal
                        } else {
                            probe.range.start().cmp(location_range.start())
                        }
                    })
                    .map_or_else(|index| index, |index| index);

                let mut new_ranges = Vec::new();
                let mut remaining_range = location_range.clone();
                let mut maps = map.get(start_index..).unwrap().iter();

                while !remaining_range.is_empty() {
                    match maps.next() {
                        Some(map_range) => {
                            if map_range.range.contains(remaining_range.start())
                                && map_range.range.contains(remaining_range.end())
                            {
                                // Fully contained, push all+offset
                                new_ranges.push(
                                    map_range.offset(*remaining_range.start())
                                        ..=map_range.offset(*remaining_range.end()),
                                );
                                break;
                                // remaining_range = *seed_range.end()..=*seed_range.end();
                            } else if map_range.range.contains(remaining_range.start()) {
                                // Seed head overlaps map tail, push head+offset
                                new_ranges.push(
                                    map_range.offset(*remaining_range.start())
                                        ..=map_range.offset(*map_range.range.end()),
                                );
                                // Remainder tail
                                remaining_range =
                                    (map_range.range.end() + 1)..=*remaining_range.end();
                            } else if map_range.range.contains(remaining_range.end()) {
                                // Seed tail overlaps map head, push head and tail+offset
                                new_ranges
                                    .push(*remaining_range.start()..=(map_range.range.start() - 1));
                                new_ranges.push(
                                    map_range.offset(*map_range.range.start())
                                        ..=map_range.offset(*remaining_range.end()),
                                );
                                break;
                            } else if remaining_range.contains(map_range.range.start())
                                && remaining_range.contains(map_range.range.end())
                            {
                                // Seed overlaps map, push head, body+offset
                                new_ranges
                                    .push(*remaining_range.start()..=(map_range.range.start() - 1));
                                new_ranges.push(
                                    map_range.offset(*map_range.range.start())
                                        ..=map_range.offset(*map_range.range.end()),
                                );
                                // Remainder tail
                                remaining_range =
                                    (map_range.range.end() + 1)..=*remaining_range.end();
                            }
                        }
                        None => {
                            new_ranges.push(remaining_range);
                            break;
                        }
                    }
                }

                new_ranges
            })
            .collect()
    };

    let soil_ranges = location_map(seed_ranges, &input.seed_to_soil);
    let fertilizer_ranges = location_map(soil_ranges, &input.soil_to_fertilizer);
    let water_ranges = location_map(fertilizer_ranges, &input.fertilizer_to_water);
    let light_ranges = location_map(water_ranges, &input.water_to_light);
    let temperature_ranges = location_map(light_ranges, &input.light_to_temperature);
    let humidity_ranges = location_map(temperature_ranges, &input.temperature_to_humidity);
    let location_ranges = location_map(humidity_ranges, &input.humidity_to_location);
    location_ranges
        .iter()
        .map(|location| *location.start())
        .min()
        .unwrap()
}
