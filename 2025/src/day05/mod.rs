use std::collections::BTreeSet;
use std::ops::Bound::Included;

use advent::*;
use itertools::Itertools;

advent_day!(Day05, 5, (Vec<(u64, u64)>, Vec<u64>));

impl DayParser<Day> for Day {
    fn parse(input: &'_ str) -> Day {
        let (ranges, ids) = input.split_once("\n\n").unwrap();
        let ranges = ranges
            .lines()
            .map(|range| {
                let (start, end) = range.split_once('-').unwrap();
                (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
            })
            .collect();
        let ids = ids
            .lines()
            .map(|line| line.parse::<u64>().unwrap())
            .collect();
        Self((ranges, ids))
    }
}

impl AdventDay for Day {
    /// ```rust
    /// use advent::*;
    /// use advent_of_code_2025::day05::*;
    /// let day = Day::parse(
    /// r"3-5
    /// 10-14
    /// 16-20
    /// 12-18
    ///
    /// 1
    /// 5
    /// 8
    /// 11
    /// 17
    /// 32");
    /// assert_eq!("3", day.part1());
    /// ```
    fn part1(&self) -> String {
        let (ranges, ids) = self.input();
        let ids: BTreeSet<u64> = BTreeSet::from_iter(ids.iter().copied());
        reduce_ranges(ranges)
            .iter()
            .map(|range| ids.range((Included(range.0), Included(range.1))).count())
            .sum::<usize>()
            .to_string()
    }

    /// ```rust
    /// use advent::*;
    /// use advent_of_code_2025::day05::*;
    /// let day = Day::parse(
    /// r"3-5
    /// 10-14
    /// 16-20
    /// 12-18
    ///
    /// 1
    /// 5
    /// 8
    /// 11
    /// 17
    /// 32");
    /// assert_eq!("14", day.part2());
    /// ```
    fn part2(&self) -> String {
        reduce_ranges(&self.input().0)
            .iter()
            .map(|range| range.1 - range.0 + 1)
            .sum::<u64>()
            .to_string()
    }
}

fn reduce_ranges(ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut ranges = ranges.iter().sorted_by_key(|item| item.0);
    let mut current_range = *ranges.next().unwrap();
    let mut reduced = vec![];
    while let Some(&(start, end)) = ranges.next() {
        if current_range.0 <= start && end <= current_range.1 {
            // Next range is contained by current
        } else if current_range.0 <= start && start <= current_range.1 {
            // Next range appends current
            current_range.1 = end;
        } else {
            // Next range is new range
            reduced.push(current_range);
            current_range = (start, end);
        }
    }
    reduced.push(current_range);

    reduced
}
