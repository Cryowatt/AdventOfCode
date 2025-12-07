use advent::*;
use num::Integer;

advent_day!(Day07, 7, Vec<Vec<Cell>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Beam,
    Empty,
    Splitter,
}

impl DayParser<Day> for Day {
    fn parse(input: &'_ str) -> Day {
        Self(
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => Cell::Empty,
                            '^' => Cell::Splitter,
                            _ => Cell::Beam,
                        })
                        .collect()
                })
                .collect(),
        )
    }
}

impl AdventDay for Day {
    /// ```rust
    /// use advent::*;
    /// use advent_of_code_2025::day07::*;
    /// let day = Day::parse(
    /// r#".......S.......
    /// ...............
    /// .......^.......
    /// ...............
    /// ......^.^......
    /// ...............
    /// .....^.^.^.....
    /// ...............
    /// ....^.^...^....
    /// ...............
    /// ...^.^...^.^...
    /// ...............
    /// ..^...^.....^..
    /// ...............
    /// .^.^.^.^.^...^.
    /// ..............."#);
    /// assert_eq!("21", day.part1());
    /// ```
    fn part1(&self) -> String {
        let mut state = self.input().first().unwrap().clone();
        let mut split_count = 0;
        for row in self.input().iter().skip(2).step_by(2) {
            for i in 0..state.len() {
                if row[i] == Cell::Splitter && state[i] == Cell::Beam {
                    split_count.inc();
                    state[i] = Cell::Empty;
                    state[i - 1] = Cell::Beam;
                    state[i + 1] = Cell::Beam;
                }
            }
        }
        split_count.to_string()
    }

    /// ```rust
    /// use advent::*;
    /// use advent_of_code_2025::day07::*;
    /// let day = Day::parse(
    /// r#".......S.......
    /// ...............
    /// .......^.......
    /// ...............
    /// ......^.^......
    /// ...............
    /// .....^.^.^.....
    /// ...............
    /// ....^.^...^....
    /// ...............
    /// ...^.^...^.^...
    /// ...............
    /// ..^...^.....^..
    /// ...............
    /// .^.^.^.^.^...^.
    /// ..............."#);
    /// assert_eq!("40", day.part2());
    /// ```
    fn part2(&self) -> String {
        let mut state = self
            .input()
            .first()
            .unwrap()
            .iter()
            .map(|cell| match cell {
                Cell::Beam => 1,
                _ => 0,
            })
            .collect::<Vec<_>>();
        for row in self.input().iter().skip(2).step_by(2) {
            for i in 0..state.len() {
                let multiverses = state[i];
                if row[i] == Cell::Splitter && state[i] > 0 {
                    // Maths
                    state[i] = 0;
                    state[i - 1] += multiverses;
                    state[i + 1] += multiverses;
                }
            }
        }
        state.iter().sum::<u64>().to_string()
    }
}
