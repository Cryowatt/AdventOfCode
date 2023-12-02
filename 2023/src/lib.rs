#![feature(test)]

mod day1;
pub use day1::*;

#[macro_export]
macro_rules! day {
    ($id:literal, $day:path) => {        
        println!("== Day {} ==", $id);
        println!("Part 1: {}", <$day>::part1(<$day>::INPUT));
        println!("Part 2: {}", <$day>::part2(<$day>::INPUT));
    };
}