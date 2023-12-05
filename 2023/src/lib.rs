#![feature(ascii_char)]
#![feature(extend_one)]
#![feature(int_roundings)]
#![feature(test)]

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

pub use day1::Day1;
pub use day2::Day2;
pub use day3::Day3;
pub use day4::Day4;
pub use day5::Day5;

#[derive(Copy, Clone)]
struct UPoint {
    x: u32,
    y: u32,
}

impl UPoint {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    fn left(&self) -> Option<Self> {
        self.x.checked_sub(1).map(|x| Self::new(x, self.y))
    }

    fn right(&self) -> Option<Self> {
        self.x.checked_add(1).map(|x| Self::new(x, self.y))
    }

    fn up(&self) -> Option<Self> {
        self.y.checked_sub(1).map(|y| Self::new(self.x, y))
    }

    fn down(&self) -> Option<Self> {
        self.y.checked_add(1).map(|y| Self::new(self.x, y))
    }
}

#[macro_export]
macro_rules! run_day {
    ($id:literal, $day:path) => {
        let input = <$day>::parse(<$day>::INPUT);
        println!(
            "Day {}# [{:<10}] [{:<10}]",
            $id,
            <$day>::part1(&input),
            <$day>::part2(&input)
        );
    };
}

#[macro_export]
macro_rules! advent_day {
    ($day:ident, $parser:expr, $input_type:ty, $part1_func:expr, $part2_func:expr) => {
        pub struct $day;

        impl $day {
            pub const INPUT: &'static str = include_str!("input.txt");

            pub fn parse(input: &str) -> $input_type {
                $parser(input)
            }

            pub fn part1(input: &$input_type) -> u32 {
                $part1_func(input)
            }

            pub fn part2(input: &$input_type) -> u32 {
                $part2_func(input)
            }
        }

        #[cfg(test)]
        mod bench {
            use super::$day as Day;

            extern crate test;

            #[bench]
            fn part1_bench(b: &mut test::Bencher) {
                let input = Day::parse(Day::INPUT);
                b.iter(|| test::black_box(Day::part1(&input)));
            }

            #[bench]
            fn part2_bench(b: &mut test::Bencher) {
                let input = Day::parse(Day::INPUT);
                b.iter(|| test::black_box(Day::part2(&input)));
            }
        }
    };
}

#[macro_export]
macro_rules! advent_bench {
    ($parser:ident, $input_type:ty, $module:ident::$part1_func:ident) => {
        #[cfg(test)]
        mod $module {
            extern crate test;

            #[bench]
            fn bench(b: &mut test::Bencher) {
                let input = super::$parser(include_str!("input.txt"));
                b.iter(|| test::black_box(super::$part1_func(input)));
            }
        }
    };
}
