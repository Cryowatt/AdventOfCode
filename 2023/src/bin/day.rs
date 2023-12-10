use std::env;

use advent::*;
use advent_of_code_2023::*;

fn main() {
    env_logger::init();

    let day_id = env::args()
        .nth(1)
        .expect("No arguments specified")
        .parse::<u8>()
        .expect("Day ID must be a number");
    match day_id {
        1 => {
            run_day!(1, day1::Day1);
        }
        2 => {
            run_day!(2, day2::Day2);
        }
        3 => {
            run_day!(3, day3::Day3);
        }
        4 => {
            run_day!(4, day4::Day4);
        }
        5 => {
            run_day!(5, day5::Day5);
        }
        6 => {
            run_day!(6, day6::Day6);
        }
        7 => {
            run_day!(7, day7::Day7);
        }
        8 => {
            run_day!(8, day8::Day8);
        }
        9 => {
            run_day!(9, day9::Day9);
        }
        19 => {
            run_day!(10, day10::Day10);
        }
        _ => unimplemented!(),
    }
}
