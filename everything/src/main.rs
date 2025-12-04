// use advent_of_code_2015::* as a2015;
// use advent_of_code_2023::* as a2023;
// use advent_of_code_2024::* as a2024;
// use advent_of_code_2025::* as a2025;

use std::collections::HashMap;

use advent::AdventDay;
// use tokio::{join, spawn};
use rayon::prelude::*;

// fn runner<D: Day>() -> (String, String) {
//     let mut part1_answer: Option<String> = None;
//     let mut part2_answer: Option<String> = None;
//     rayon::scope(|s| {
//         s.spawn( |_| part1_answer = Some(D::part1()));
//         s.spawn( |_| part2_answer = Some(D::part2()));
//     });

//     (part1_answer.unwrap(), part2_answer.unwrap())
// }

// macro_rules! advent_day {
//     ($year:ty, $results:ident) => {
//         $tasks.insert("1982-01-11", (spawn(run())));
//     };
// }

// #[tokio::main]

fn main() {
    println!("AdventOfCode");

    let years = (2015..=2025).collect::<Vec<_>>();
    // let days = (1..=31).collect::<Vec<_>>();
    let all_days = years
        .iter()
        .flat_map(|year| (1..=31).map(|day| (*year, day)))
        .map(|(year, day)| match year {
            2025 => advent_of_code_2025::new_day(day),
            _ => None,
        })
        .collect::<Vec<_>>();

    let all_parts = all_days
        .iter()
        .filter_map(|day| day.as_ref().map(|day| (1, day)))
        .chain(
            all_days
                .iter()
                .filter_map(|day| day.as_ref().map(|day| (2, day))),
        )
        .collect::<Vec<_>>();

    rayon::scope(|s| {
        all_parts.iter().for_each(|&(part, day)| {
            s.spawn(|_| {
                let _ = day.part1();
            })
        });
    })
    // days.into_par_iter().for_each(|day| {});

    // let advent = [
    //     || (advent_of_code_2025::day01::part1,advent_of_code_2025::day01::part2),
    //     // || (advent_of_code_2025::day02::part1,advent_of_code_2025::day02::part2),
    //     // || (advent_of_code_2025::day03::part1,advent_of_code_2025::day03::part2),
    // ];

    // let mut results:HashMap<String, (String, String)> = HashMap::new();
    // // let mut tasks = vec![];
    // rayon::scope(|s| {
    //     s.spawn(|s|  runner::<advent_of_code_2025::day01::Day01>());
    //     advent_day!(advent_of_code_2025::day01::Day01, results);
    // });

    // // advent_day!(tasks);
    // for (day, result) in results {
    //     println!("Day {}: {}", day, result.await.unwrap());
    // }
    // // tasks.push(spawn(async || advent_of_code_2025::day03::Day03::part2()));
    // // let threadpool = rayon::ThreadPoolBuilder::new().build().unwrap();
    // // threadpool.spawn(|| advent_of_code_2025::day03::Day03::part1());
    // // threadpool.spawn(|| advent_of_code_2025::day03::Day03::part2());
}
