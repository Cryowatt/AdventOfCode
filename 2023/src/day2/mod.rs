use std::sync::OnceLock;

pub struct Day2;

impl Day2 {
    pub const INPUT: &str = include_str!("input.txt");

    /// ```rust
    /// let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    /// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    /// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    /// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    /// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    /// assert_eq!(8, advent_of_code::Day2::part1(input));
    /// ```
    pub fn part1(input: &str) -> u32 {
        let _ = env_logger::builder().is_test(true).try_init();
        static LINE_PARSER: OnceLock<regex::Regex> = OnceLock::new();
        static COLOUR_PARSER: OnceLock<regex::Regex> = OnceLock::new();

        let line_parser = LINE_PARSER.get_or_init(|| {
            regex::Regex::new(
                r"Game (\d+): (.*)",
            )
            .unwrap()
        });
        let colour_parser = COLOUR_PARSER.get_or_init(|| {
            regex::Regex::new(
                r"(\d\d+) (red|green|blue)",
            )
            .unwrap()
        });

        input.lines().filter_map(|line| {
            let (_, [id, rounds]) = line_parser.captures(line).unwrap().extract();

            let is_valid = colour_parser.captures_iter(rounds).all(|pull| {
                let (_, [count, colour]) = pull.extract();
                match colour {
                    "red" => count.parse::<u8>().unwrap() <= 12,
                    "green" => count.parse::<u8>().unwrap() <= 13,
                    "blue" => count.parse::<u8>().unwrap() <= 14,
                    _ => unreachable!()
                }
            });

            match is_valid {
                true => Some(id.parse::<u32>().unwrap()),
                false => None
            }
        }).sum()
    }

    
    pub fn part2(input: &str) -> u32 {
        unimplemented!()
    }
}

#[cfg(test)]
mod unittests {
    use crate::Day1;

    #[ctor::ctor]
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn part1_example_test() {
        let input = r"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!(142, Day1::part1(input));
    }

    #[test]
    fn part2_example_test() {
        let input = r"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!(281, Day1::part2(input));
    }

    #[test]
    fn part2_overlap_case() {
        let input = r"two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen
    eighthree";
        assert_eq!(281 + 83, Day1::part2(input));
    }
}

#[cfg(test)]
mod bench {
    use crate::Day2 as Day;

    extern crate test;

    #[bench]
    fn part1_bench(b: &mut test::Bencher) {
        b.iter(|| test::black_box(Day::part1(Day::INPUT)));
    }

    #[bench]
    fn part2_bench(b: &mut test::Bencher) {
        b.iter(|| test::black_box(Day::part2(Day::INPUT)));
    }
}
