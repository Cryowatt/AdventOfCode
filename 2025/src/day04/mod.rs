use advent::*;
use num::Integer;

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
    /// assert_eq!("43", day.part2());
    /// ```
    fn part2(&self) -> String {
        fn add_vec<T: Copy + std::ops::AddAssign<T>>(left: &mut Vec<T>, right: &Vec<T>) {
            for i in 0..left.len() {
                left[i] += right[i]
            }
        }

        let bounds = UPoint::new(self.input().len() as u32, self.input()[0].len() as u32);
        let input = self.input();

        let verticals = (0..bounds.y).map(|y| {
            let mut total = input[y as usize].clone();
            if y > 0 {
                add_vec(&mut total, &input[y as usize - 1])
            }
            if y < bounds.y - 1 {
                add_vec(&mut total, &input[y as usize + 1])
            }
            total
        });
        let mut grid = verticals
            .enumerate()
            .map(|(y, row)| {
                (0..bounds.x as usize)
                    .map(|x| {
                        if input[y][x] == 1 {
                            let mut rolls = row[x];
                            if x > 0 {
                                rolls += row[x - 1]
                            }
                            if x < bounds.x as usize - 1 {
                                rolls += row[x + 1]
                            }
                            rolls
                        } else {
                            0
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut stack = (0..bounds.y)
            .flat_map(|y| (0..bounds.x).map(move |x| UPoint::new(x, y)))
            .collect::<Vec<_>>();
        fn dec_rolls(p: Point<u32>, grid: &mut Vec<Vec<u8>>, stack: &mut Vec<UPoint>) {
            if grid[p.y as usize][p.x as usize] > 0 {
                grid[p.y as usize][p.x as usize] -= 1;
                stack.push(p);
            }
        }

        let mut removed_count = 0;
        while let Some(point) = stack.pop() {
            let rolls = grid[point.y as usize][point.x as usize];
            if 0 < rolls && rolls < 5 {
                removed_count.inc();
                grid[point.y as usize][point.x as usize] = 0;
                if let Some(p) = point.north_checked() {
                    dec_rolls(p, &mut grid, &mut stack);
                    if let Some(p) = p.west_checked() {
                        dec_rolls(p, &mut grid, &mut stack);
                    }
                    if let Some(p) = p.east_checked(&bounds) {
                        dec_rolls(p, &mut grid, &mut stack);
                    }
                }
                if let Some(p) = point.west_checked() {
                    dec_rolls(p, &mut grid, &mut stack);
                }
                if let Some(p) = point.south_checked(&bounds) {
                    dec_rolls(p, &mut grid, &mut stack);
                    if let Some(p) = p.west_checked() {
                        dec_rolls(p, &mut grid, &mut stack);
                    }
                    if let Some(p) = p.east_checked(&bounds) {
                        dec_rolls(p, &mut grid, &mut stack);
                    }
                }
                if let Some(p) = point.east_checked(&bounds) {
                    dec_rolls(p, &mut grid, &mut stack);
                }
            }
        }

        removed_count.to_string()
    }
}
