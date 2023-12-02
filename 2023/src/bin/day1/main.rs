use advent_of_code::*;

fn main() {
    env_logger::init();
    let input = include_str!("input.txt");

    println!("== Day 1 ==");
    println!("Part 1: {}", day1::part1(input));
    println!("Part 2: {}", day1::part2(input));
}
