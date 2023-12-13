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
            run_day!(1, day01::Day01);
        }
        2 => {
            run_day!(2, day02::Day02);
        }
        3 => {
            run_day!(3, day03::Day03);
        }
        4 => {
            run_day!(4, day04::Day04);
        }
        // 5 => {
        //     run_day!(5, day05::Day05);
        // }
        // 6 => {
        //     run_day!(6, day06::Day06);
        // }
        _ => unimplemented!(),
    }
}
