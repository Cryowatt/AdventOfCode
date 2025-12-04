use advent::*;
use num::Integer;

advent_day!(Day01, 1, Vec<i32>);

impl DayParser<Day> for Day {
    fn parse(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| {
                    let tokens = line.split_at(1);
                    let direction = match tokens.0 {
                        "L" => -1,
                        "R" => 1,
                        _ => panic!(),
                    };
                    direction * tokens.1.parse::<i32>().unwrap()
                })
                .collect(),
        )
    }
}

impl AdventDay for Day {
    /// ```rust
    /// use advent::*;
    /// use advent_of_code_2025::day01::*;
    /// let day = Day::parse(
    /// r"L68
    /// L30
    /// R48
    /// L5
    /// R60
    /// L55
    /// L1
    /// L99
    /// R14
    /// L82");
    /// assert_eq!("3", day.part1());
    /// ```
    fn part1(&self) -> String {
        self.input()
            .iter()
            .copied()
            .scan(50, |state, rotation| {
                *state = (*state + rotation) % 100;
                Some(*state)
            })
            .filter(|&state| state == 0)
            .count()
            .to_string()
    }

    /// ```rust
    /// use advent::*;
    /// use advent_of_code_2025::day01::*;
    /// let day = Day::parse(
    /// r"L68
    /// L30
    /// R48
    /// L5
    /// R60
    /// L55
    /// L1
    /// L99
    /// R14
    /// L82");
    /// assert_eq!("6", day.part2());
    /// ```
    /// ```rust
    /// use advent::*;
    /// use advent_of_code_2025::day01::*;
    /// let day = Day::parse(
    /// r"L50");
    /// assert_eq!("1", day.part2());
    /// ```
    /// ```rust
    /// use advent::*;
    /// use advent_of_code_2025::day01::*;
    /// let day = Day::parse(
    /// r"L150");
    /// assert_eq!("2", day.part2());
    /// ```
    fn part2(&self) -> String {
        self.input()
            .iter()
            .copied()
            .scan(50, |state, rotation| {
                let rotation_sign = rotation.signum();
                let new_state = *state + rotation;
                // Match zero crossings
                let zc = match (state.signum(), new_state.signum()) {
                    (0, _) => 0,
                    (1, 1) => 0,
                    (-1, -1) => 0,
                    _ => 1,
                };
                // Count rotations and normalize position
                let (q, r) = new_state.div_rem(&(100 * rotation_sign));
                *state = r;
                Some(q + zc)
            })
            .sum::<i32>()
            .to_string()
    }
}
