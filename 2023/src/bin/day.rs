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
        5 => {
            run_day!(5, day05::Day05);
        }
        6 => {
            run_day!(6, day06::Day06);
        }
        7 => {
            run_day!(7, day07::Day07);
        }
        8 => {
            run_day!(8, day08::Day08);
        }
        9 => {
            run_day!(9, day09::Day09);
        }
        10 => {
            run_day!(10, day10::Day10);
        }
        11 => {
            run_day!(11, day11::Day11);
        }
        12 => {
            run_day!(12, day12::Day12);
        }
        13 => {
            run_day!(13, day13::Day13);
        }
        14 => {
            run_day!(14, day14::Day14);
        }
        15 => {
            run_day!(15, day15::Day15);
        }
        16 => {
            run_day!(16, day16::Day16);
        }
        17 => {
            run_day!(17, day17::Day17);
        }
        18 => {
            run_day!(18, day18::Day18);
        }
        19 => {
            run_day!(19, day19::Day19);
        }
        20 => {
            run_day!(20, day20::Day20);
        }
        21 => {
            run_day!(21, day21::Day21);
        }
        _ => unimplemented!(),
    }
}
