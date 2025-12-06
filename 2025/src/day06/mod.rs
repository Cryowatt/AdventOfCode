use advent::*;
use onig::Regex;

advent_day!(Day06, 6, Vec<(Vec<String>, Operator)>);

#[derive(Debug, Clone, Copy)]
enum Operator {
    Addition,
    Multiplication,
}

impl DayParser<Day> for Day {
    fn parse(input: &'_ str) -> Day {
        let mut lines = input.lines();
        let mut operand_rows = vec![];

        let mut line = lines.next().unwrap();
        while let Some(next) = lines.next() {
            operand_rows.push(line.to_string());
            line = next;
        }

        let operator_pattern = Regex::new(r"(([+*])\s+)(?:\s|$)").unwrap();
        Self(
            operator_pattern
                .captures_iter(line)
                .map(|op_cap| {
                    let operator = match op_cap.at(2).unwrap() {
                        "+" => Operator::Addition,
                        "*" => Operator::Multiplication,
                        _ => panic!(),
                    };
                    let text_width = op_cap.at(0).unwrap().len();
                    let column_width = op_cap.at(1).unwrap().len();
                    let operands = operand_rows
                        .iter_mut()
                        .map(|row| {
                            let remainder = row.split_off(text_width);
                            let operand = row.get(0..column_width).unwrap().to_string();
                            *row = remainder;
                            operand
                        })
                        .collect::<Vec<_>>();

                    (operands, operator)
                })
                .collect::<Vec<_>>(),
        )
    }
}

impl AdventDay for Day {
    /// ```rust
    /// use advent::*;
    /// use advent_of_code_2025::day06::*;
    /// let day = Day::parse(
    /// "123 328  51 64 \
    /// \n 45 64  387 23 \
    /// \n  6 98  215 314\
    /// \n*   +   *   +  ");
    /// assert_eq!("4277556", day.part1());
    /// ```
    fn part1(&self) -> String {
        self.input()
            .iter()
            .map(|(operand_rows, operator)| {
                let operands = operand_rows
                    .iter()
                    .map(|row| row.trim().parse::<u64>().unwrap());
                match operator {
                    Operator::Addition => operands.sum::<u64>(),
                    Operator::Multiplication => operands.product(),
                }
            })
            .sum::<u64>()
            .to_string()
    }

    // Low: 3315119450

    /// ```rust
    /// use advent::*;
    /// use advent_of_code_2025::day06::*;
    /// let day = Day::parse(
    /// "123 328  51 64 \
    /// \n 45 64  387 23 \
    /// \n  6 98  215 314\
    /// \n*   +   *   +  ");
    /// assert_eq!("3263827", day.part2());
    /// ```
    fn part2(&self) -> String {
        self.input()
            .iter()
            .map(|(operand_rows, operator)| {
                let operands = (0..operand_rows[0].len()).map(|i| {
                    operand_rows
                        .iter()
                        .map(|row| row.get(i..=i).unwrap().parse::<u64>())
                        .fold(0, |acc, digit| match digit {
                            Ok(digit) => (acc * 10) + digit,
                            Err(_) => acc,
                        })
                });

                match operator {
                    Operator::Addition => operands.sum::<u64>(),
                    Operator::Multiplication => operands.product(),
                }
            })
            .sum::<u64>()
            .to_string()
    }
}
