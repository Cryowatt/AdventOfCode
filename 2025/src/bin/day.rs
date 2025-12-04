use std::env;

use advent::*;
use advent_of_code_2025::*;

fn main() {
    env_logger::init();

    let day_id = env::args()
        .nth(1)
        .expect("No arguments specified")
        .parse::<u8>()
        .expect("Day ID must be a number");
    println!(
        "{}",
        match day_id {
            #[cfg(feature = "day01")]
            day01::Day::DAY_ID => day01::Day::default().all(),
            #[cfg(feature = "day02")]
            day02::Day::DAY_ID => day02::Day::default().all(),
            #[cfg(feature = "day03")]
            day03::Day::DAY_ID => day03::Day::default().all(),
            #[cfg(feature = "day04")]
            day04::Day::DAY_ID => day04::Day::default().all(),
            #[cfg(feature = "day05")]
            day05::Day::DAY_ID => day05::Day::default().all(),
            #[cfg(feature = "day06")]
            day06::Day::DAY_ID => day06::Day::default().all(),
            #[cfg(feature = "day07")]
            day07::Day::DAY_ID => day07::Day::default().all(),
            #[cfg(feature = "day08")]
            day08::Day::DAY_ID => day08::Day::default().all(),
            #[cfg(feature = "day09")]
            day09::Day::DAY_ID => day09::Day::default().all(),
            #[cfg(feature = "day10")]
            day10::Day::DAY_ID => day10::Day::default().all(),
            #[cfg(feature = "day11")]
            day11::Day::DAY_ID => day11::Day::default().all(),
            #[cfg(feature = "day12")]
            day12::Day::DAY_ID => day12::Day::default().all(),
            _ => unimplemented!(),
        }
    )
}
