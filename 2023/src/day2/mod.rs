use std::sync::OnceLock;

use advent::*;

advent_day!(Day2, parse, Vec<&str>, part1, part2);

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

/// ```rust
/// use advent_of_code_2023::day2::*;
/// let input = parse(
/// r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
/// assert_eq!(8, part1(&input));
/// ```
pub fn part1(input: &Vec<&str>) -> u32 {
    static LINE_PARSER: OnceLock<regex::Regex> = OnceLock::new();
    static COLOUR_PARSER: OnceLock<regex::Regex> = OnceLock::new();

    let line_parser = LINE_PARSER.get_or_init(|| regex::Regex::new(r"Game (\d+): (.*)").unwrap());
    let colour_parser =
        COLOUR_PARSER.get_or_init(|| regex::Regex::new(r"(\d\d+) (red|green|blue)").unwrap());

    input
        .iter()
        .filter_map(|line| {
            let (_, [id, rounds]) = line_parser.captures(line).unwrap().extract();

            let is_valid = colour_parser.captures_iter(rounds).all(|pull| {
                let (_, [count, colour]) = pull.extract();
                match colour {
                    "red" => count.parse::<u8>().unwrap() <= 12,
                    "green" => count.parse::<u8>().unwrap() <= 13,
                    "blue" => count.parse::<u8>().unwrap() <= 14,
                    _ => unreachable!(),
                }
            });

            match is_valid {
                true => Some(id.parse::<u32>().unwrap()),
                false => None,
            }
        })
        .sum()
}

/// ```rust
/// use advent_of_code_2023::day2::*;
/// let input = parse(
/// r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
/// assert_eq!(2286, part2(&input));
/// ```
pub fn part2(input: &Vec<&str>) -> u32 {
    static COLOUR_PARSER: OnceLock<regex::Regex> = OnceLock::new();

    let colour_parser =
        COLOUR_PARSER.get_or_init(|| regex::Regex::new(r"(\d+) (red|green|blue)").unwrap());

    struct DiceCount {
        red: u32,
        green: u32,
        blue: u32,
    }

    impl DiceCount {
        const ZERO: DiceCount = DiceCount {
            red: 0,
            green: 0,
            blue: 0,
        };
        fn power(&self) -> u32 {
            self.red * self.green * self.blue
        }

        fn max_red(&self, red: u32) -> DiceCount {
            DiceCount {
                red: self.red.max(red),
                green: self.green,
                blue: self.blue,
            }
        }
        fn max_green(&self, green: u32) -> DiceCount {
            DiceCount {
                red: self.red,
                green: self.green.max(green),
                blue: self.blue,
            }
        }
        fn max_blue(&self, blue: u32) -> DiceCount {
            DiceCount {
                red: self.red,
                green: self.green,
                blue: self.blue.max(blue),
            }
        }
    }

    input
        .iter()
        .map(|line| {
            let min_dice_set =
                colour_parser
                    .captures_iter(line)
                    .fold(DiceCount::ZERO, |acc, capture| {
                        let (_, [count, colour]) = capture.extract();
                        match colour {
                            "red" => acc.max_red(count.parse::<u32>().unwrap()),
                            "green" => acc.max_green(count.parse::<u32>().unwrap()),
                            "blue" => acc.max_blue(count.parse::<u32>().unwrap()),
                            _ => unreachable!(),
                        }
                    });

            min_dice_set.power()
        })
        .sum()
}
