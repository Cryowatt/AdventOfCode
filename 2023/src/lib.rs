#![feature(test)]

mod day1;
mod day2;

pub use day1::*;
pub use day2::*;

#[macro_export]
macro_rules! run_day {
    ($id:literal, $day:path) => {        
        println!("== Day {} ==", $id);
        println!("Part 1: {}", <$day>::part1(<$day>::INPUT));
        println!("Part 2: {}", <$day>::part2(<$day>::INPUT));
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
