use std::env;

use advent_of_code::*;

fn main() {
    env_logger::init();
    
    let day_id = env::args().nth(1).expect("No arguments specified").parse::<u8>().expect("Day ID must be a number");
    match day_id {
        1 => { run_day!(1, advent_of_code::Day1); },
        2 => { run_day!(2, advent_of_code::Day2); },
        3 => { run_day!(3, advent_of_code::Day3); },
        _ => unimplemented!(),
    }
}
