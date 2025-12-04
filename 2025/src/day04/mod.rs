use advent::*;

advent_day!(Day04, 4, Vec<Vec<u8>>);

impl DayParser<Day> for Day {
    fn parse(input: &'_ str) -> Day {
        Self(
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '@' => 1,
                            _ => 0,
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
    /// use advent_of_code_2025::day04::*;
    /// let day = Day::parse(
    /// r"..@@.@@@@.
    /// @@@.@.@.@@
    /// @@@@@.@.@@
    /// @.@@@@..@.
    /// @@.@@@@.@@
    /// .@@@@@@@.@
    /// .@.@.@.@@@
    /// @.@@@.@@@@
    /// .@@@@@@@@.
    /// @.@.@@@.@.");
    /// assert_eq!("13", day.part1());
    /// ```
    fn part1(&self) -> String {
        fn add_vec<T: Copy + std::ops::AddAssign<T>>(left: &mut Vec<T>, right: &Vec<T>) {
            for i in 0..left.len() {
                left[i] += right[i]
            }
        }

        let bounds = (self.input().len() - 1, self.input()[0].len() - 1);
        let input = self.input();

        (0..=bounds.1)
            .map(|y| {
                let mut total = input[y as usize].clone();
                if y > 0 {
                    add_vec(&mut total, &input[y - 1])
                }
                if y < bounds.1 {
                    add_vec(&mut total, &input[y + 1])
                }
                total
            })
            .enumerate()
            .map(|(y, row)| {
                (0..=bounds.0)
                    .filter(|&x| {
                        if input[y][x] == 1 {
                            let mut rolls = row[x];
                            if x > 0 {
                                rolls += row[x - 1]
                            }
                            if x < bounds.0 {
                                rolls += row[x + 1]
                            }
                            rolls < 5
                        } else {
                            false
                        }
                    })
                    .count()
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        fn add_vec<T: Copy + std::ops::AddAssign<T>>(left: &mut Vec<T>, right: &Vec<T>) {
            for i in 0..left.len() {
                left[i] += right[i]
            }
        }

        let bounds = (self.input().len() - 1, self.input()[0].len() - 1);
        let input = self.input();

        (0..=bounds.1)
            .map(|y| {
                let mut total = input[y as usize].clone();
                if y > 0 {
                    add_vec(&mut total, &input[y - 1])
                }
                if y < bounds.1 {
                    add_vec(&mut total, &input[y + 1])
                }
                total
            })
            .enumerate()
            .map(|(y, row)| {
                (0..=bounds.0)
                    .filter(|&x| {
                        if input[y][x] == 1 {
                            let mut rolls = row[x];
                            if x > 0 {
                                rolls += row[x - 1]
                            }
                            if x < bounds.0 {
                                rolls += row[x + 1]
                            }
                            rolls < 5
                        } else {
                            false
                        }
                    })
                    .count()
            })
            .sum::<usize>()
            .to_string()
    }
}
