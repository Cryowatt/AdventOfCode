#![feature(array_windows)]
// #![feature(ascii_char)]
// #![feature(async_closure)]
// #![feature(bound_map)]
#![feature(coroutines)]
#![feature(exact_div)]
// #![feature(extend_one)]
#![feature(int_roundings)]
// #![feature(isqrt)]
#![feature(iter_from_coroutine)]
#![feature(iter_map_windows)]
// #![feature(lazy_cell)]
#![feature(map_try_insert)]
// #![feature(new_uninit)]
#![feature(portable_simd)]
#![feature(stmt_expr_attributes)]
#![feature(test)]

use advent::AdventDay;
#[cfg(feature = "day01")]
use advent::DayId;

#[cfg(feature = "day01")]
pub mod day01;
#[cfg(feature = "day02")]
pub mod day02;
#[cfg(feature = "day03")]
pub mod day03;
#[cfg(feature = "day04")]
pub mod day04;
#[cfg(feature = "day05")]
pub mod day05;
#[cfg(feature = "day06")]
pub mod day06;
#[cfg(feature = "day07")]
pub mod day07;
#[cfg(feature = "day08")]
pub mod day08;
#[cfg(feature = "day09")]
pub mod day09;
#[cfg(feature = "day10")]
pub mod day10;
#[cfg(feature = "day11")]
pub mod day11;
#[cfg(feature = "day12")]
pub mod day12;

pub fn new_day(day_id: u8) -> Option<Box<dyn AdventDay>> {
    match day_id {
        #[cfg(feature = "day01")]
        day01::Day::DAY_ID => Some(Box::new(day01::Day::default())),
        #[cfg(feature = "day02")]
        day02::Day::DAY_ID => Some(Box::new(day02::Day::default())),
        #[cfg(feature = "day03")]
        day03::Day::DAY_ID => Some(Box::new(day03::Day::default())),
        #[cfg(feature = "day04")]
        day04::Day::DAY_ID => Some(Box::new(day04::Day::default())),
        #[cfg(feature = "day05")]
        day05::Day::DAY_ID => Some(Box::new(day05::Day::default())),
        #[cfg(feature = "day06")]
        day06::Day::DAY_ID => Some(Box::new(day06::Day::default())),
        #[cfg(feature = "day07")]
        day07::Day::DAY_ID => Some(Box::new(day07::Day::default())),
        #[cfg(feature = "day08")]
        day08::Day::DAY_ID => Some(Box::new(day08::Day::default())),
        #[cfg(feature = "day09")]
        day09::Day::DAY_ID => Some(Box::new(day09::Day::default())),
        #[cfg(feature = "day10")]
        day10::Day::DAY_ID => Some(Box::new(day10::Day::default())),
        #[cfg(feature = "day11")]
        day11::Day::DAY_ID => Some(Box::new(day11::Day::default())),
        #[cfg(feature = "day12")]
        day12::Day::DAY_ID => Some(Box::new(day12::Day::default())),
        _ => None,
    }
}
