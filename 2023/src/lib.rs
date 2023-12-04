#![feature(ascii_char)]
#![feature(int_roundings)]
#![feature(test)]

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

pub use day1::Day1;
pub use day2::Day2;
pub use day3::Day3;
pub use day4::Day4;

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
        println!("Day {}# [{:<9}] [{:<9}]", $id, <$day>::part1(<$day>::INPUT), <$day>::part2(<$day>::INPUT));
    };
}

#[macro_export]
macro_rules! advent_day {
    ($day:ident, $part1_func:expr, $part2_func:expr) => {
        pub struct $day;

        impl $day {
            pub const INPUT: &str = include_str!("input.txt");

            pub fn part1(input: &str) -> u32 {
                $part1_func(input)
            }

            pub fn part2(input: &str) -> u32 {
                $part2_func(input)
            }
        }

        #[cfg(test)]
        mod bench {
            use super::$day as Day;

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
    };
}

#[macro_export]
macro_rules! advent_bench {
    ($module:ident::$part1_func:ident) => {
        #[cfg(test)]
        mod $module {
            extern crate test;

            #[bench]
            fn bench(b: &mut test::Bencher) {
                pub const INPUT: &str = include_str!("input.txt");
                b.iter(|| test::black_box(super::$part1_func(INPUT)));
            }
        }
    };
}
