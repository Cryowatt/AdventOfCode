use std::env;

use advent_of_code::*;

fn main() {
    env_logger::init();
    let day_id = env::args().next().expect("No arguments specified").parse::<u8>().expect("Day ID must be a number");
    match day_id {
        1 => { day!(1, advent_of_code::Day1); },
        _ => unimplemented!(),
    }
}
