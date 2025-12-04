use itertools::Itertools;

use advent::*;

advent_day!(Day02, 2, Vec<(u64, u64)>);

impl DayParser<Day> for Day {
    fn parse(input: &str) -> Self {
        Self(
            input
                .split(',')
                .map(|range| {
                    let (start, end) = range.split_once('-').unwrap();
                    (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
                })
                .collect(),
        )
    }
}

const SPLITS: [u64; 10] = [
    1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
];

impl AdventDay for Day {
    /// ```rust
    /// use advent::*;
    /// use advent_of_code_2025::day02::*;
    /// let day = Day::parse(
    /// r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124");
    /// assert_eq!("1227775554", day.part1());
    /// ```
    fn part1(&self) -> String {
        let patterns: Vec<Vec<(u32, u64)>> = vec![
            vec![],                 // 0
            vec![],                 // 1
            vec![(1, 11)],          // 2
            vec![],                 // 3
            vec![(2, 01_01)],       // 4
            vec![],                 // 5
            vec![(3, 001_001)],     // 6
            vec![],                 // 7
            vec![(4, 0001_0001)],   // 8
            vec![],                 // 9
            vec![(5, 00001_00001)], // 10
        ];

        find_patterns(self.input(), patterns).to_string()
    }

    /// ```rust
    /// use advent::*;
    /// use advent_of_code_2025::day02::*;
    /// let day = Day::parse(
    /// r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124");
    /// assert_eq!("4174379265", day.part2());
    /// ```
    fn part2(&self) -> String {
        let patterns: Vec<Vec<(u32, u64)>> = vec![
            vec![],                                                       // 0
            vec![],                                                       // 1
            vec![(1, 11)],                                                // 2
            vec![(1, 111)],                                               // 3
            vec![(1, 1111), (2, 01_01)],                                  // 4
            vec![(1, 11111)],                                             // 5
            vec![(1, 111111), (2, 01_01_01), (3, 001_001)],               // 6
            vec![(1, 1111111)],                                           // 7
            vec![(1, 11111111), (2, 01_01_01_01), (4, 0001_0001)],        // 8
            vec![(1, 111111111), (3, 001_001_001)],                       // 9
            vec![(1, 1111111111), (2, 01_01_01_01_01), (5, 00001_00001)], // 10
        ];

        find_patterns(self.input(), patterns).to_string()
    }
}

fn find_patterns(input: &InputType, patterns: Vec<Vec<(u32, u64)>>) -> u64 {
    input
        .iter()
        .flat_map(|(start, end)| {
            // Normalize ranges to same decimal lengths
            let mut ranges = vec![*start];

            for i in start.digits()..end.digits() {
                ranges.push(SPLITS[i as usize]);
            }

            ranges.push(*end + 1);

            ranges
                .windows(2)
                .map(|x| (x[0], x[1] - 1))
                .collect::<Vec<(u64, u64)>>()
        })
        .map(|(start, end)| {
            // Use patterns table to build candidates from prefixes
            let digits = start.digits();
            patterns
                .get(digits as usize)
                .unwrap()
                .iter()
                .flat_map(|(prefix, pattern)| {
                    let step = 1u64.shl10(digits - prefix);
                    (start..(end + step))
                        .step_by(step as usize)
                        .filter_map(move |source| {
                            let prefix = source.shr10(digits - prefix);
                            let candidate = prefix * pattern;
                            if start <= candidate && candidate <= end {
                                Some(candidate)
                            } else {
                                None
                            }
                        })
                })
                .unique()
                .sum::<u64>()
        })
        .sum::<u64>()
}
