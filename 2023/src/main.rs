#![feature(test)]
mod day1;

fn main() {
    println!("Running all advent days");

    println!("== Day 1 ==");
    let input = include_str!("bin/day1/input.txt");
    println!("Part 1: {}", day1::part1(input));
    println!("Part 2: {}", day1::part2(input));
}
