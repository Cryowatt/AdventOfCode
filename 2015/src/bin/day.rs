use std::env;

use advent::*;
use advent_of_code_2015::*;

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
        // 2 => {
        //     run_day!(2, advent_of_code::Day2);
        // }
        // 3 => {
        //     run_day!(3, advent_of_code::Day3);
        // }
        // 4 => {
        //     run_day!(4, advent_of_code::Day4);
        // }
        // 5 => {
        //     run_day!(5, advent_of_code::Day5);
        // }
        // 6 => {
        //     run_day!(6, advent_of_code::Day6);
        // }
        _ => unimplemented!(),
    }
}