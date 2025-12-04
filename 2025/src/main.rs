#![feature(test)]

use advent::*;
use advent_of_code_2025::*;

fn main() {
    println!("AdventOfCode 2025");

    #[cfg(feature = "day01")]
    println!("{}", day01::Day::default().all());
    #[cfg(feature = "day02")]
    println!("{}", day02::Day::default().all());
    #[cfg(feature = "day03")]
    println!("{}", day03::Day::default().all());
    #[cfg(feature = "day04")]
    println!("{}", day04::Day::default().all());
    #[cfg(feature = "day05")]
    println!("{}", day05::Day::default().all());
    #[cfg(feature = "day06")]
    println!("{}", day06::Day::default().all());
    #[cfg(feature = "day07")]
    println!("{}", day07::Day::default().all());
    #[cfg(feature = "day08")]
    println!("{}", day08::Day::default().all());
    #[cfg(feature = "day09")]
    println!("{}", day09::Day::default().all());
    #[cfg(feature = "day10")]
    println!("{}", day10::Day::default().all());
    #[cfg(feature = "day11")]
    println!("{}", day11::Day::default().all());
    #[cfg(feature = "day12")]
    println!("{}", day12::Day::default().all());
}
