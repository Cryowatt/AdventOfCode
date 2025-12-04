use advent::*;

advent_day!(Day03, 3, Vec<Vec<u8>>);

impl DayParser<Day> for Day {
    fn parse(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect()
                })
                .collect(),
        )
    }
}

impl AdventDay for Day {
    /// ```rust
    /// use advent::*;
    /// use advent_of_code_2025::day03::*;
    /// let day = Day::parse(
    /// r"987654321111111
    /// 811111111111119
    /// 234234234234278
    /// 818181911112111");
    /// assert_eq!("357", day.part1());
    /// ```
    fn part1(&self) -> String {
        self.input()
            .iter()
            .map(|line| {
                let (index, &first_digit) = line[..line.len() - 1]
                    .iter()
                    .enumerate()
                    .rev()
                    .max_by_key(|&(_, value)| *value)
                    .unwrap();
                let &second_digit = line[index + 1..].iter().max().unwrap();
                first_digit as u64 * 10 + second_digit as u64
            })
            .sum::<u64>()
            .to_string()
    }

    /// ```rust
    /// use advent::*;
    /// use advent_of_code_2025::day03::*;
    /// let day = Day::parse(
    /// r"987654321111111
    /// 811111111111119
    /// 234234234234278
    /// 818181911112111");
    /// assert_eq!("3121910778619", day.part2());
    /// ```
    fn part2(&self) -> String {
        self.input()
            .iter()
            .map(|line| {
                let mut joltage = 0u64;
                let mut index = 0;

                for i in 0..12 {
                    let (offset, &digit) = line[index..line.len() - (11 - i)]
                        .iter()
                        .enumerate()
                        .rev()
                        .max_by_key(|&(_, value)| *value)
                        .unwrap();
                    index += offset + 1;
                    joltage = joltage * 10 + digit as u64
                }
                joltage
            })
            .sum::<u64>()
            .to_string()
    }
}

// // pub fn parse(input: &str) -> InputType<'_> {
// //     input
// //         .lines()
// //         .map(|line| {
// //             line.chars()
// //                 .map(|c| c.to_digit(10).unwrap() as u8)
// //                 .collect()
// //         })
// //         .collect()
// // }

// /// ```rust
// /// use advent_of_code_2025::day03::*;
// /// let input = parse(
// /// r"987654321111111
// /// 811111111111119
// /// 234234234234278
// /// 818181911112111");
// /// assert_eq!(357, part1(&input));
// /// ```
// pub fn part1(input: &InputType) -> u64 {
//     input
//         .iter()
//         .map(|line| {
//             let (index, &first_digit) = line[..line.len() - 1]
//                 .iter()
//                 .enumerate()
//                 .rev()
//                 .max_by_key(|&(_, value)| *value)
//                 .unwrap();
//             let &second_digit = line[index + 1..].iter().max().unwrap();
//             first_digit as u64 * 10 + second_digit as u64
//         })
//         .sum()
// }

// /// ```rust
// /// use advent_of_code_2025::day03::*;
// /// let input = parse(
// /// r"987654321111111
// /// 811111111111119
// /// 234234234234278
// /// 818181911112111");
// /// assert_eq!(3121910778619, part2(&input));
// /// ```
// pub fn part2(input: &InputType) -> u64 {
//     input
//         .iter()
//         .map(|line| {
//             let mut joltage = 0u64;
//             let mut index = 0;

//             for i in 0..12 {
//                 let (offset, &digit) = line[index..line.len() - (11 - i)]
//                     .iter()
//                     .enumerate()
//                     .rev()
//                     .max_by_key(|&(_, value)| *value)
//                     .unwrap();
//                 index += offset + 1;
//                 joltage = joltage * 10 + digit as u64
//             }
//             joltage
//         })
//         .sum()
// }

// advent_bench!(parse, queue_based, part2_queued);

// /// ```rust
// /// use advent_of_code_2025::day03::*;
// /// let input = parse(
// /// r"987654321111111
// /// 811111111111119
// /// 234234234234278
// /// 818181911112111");
// /// assert_eq!(part2_queued(&input), part2(&input));
// /// ```
// pub fn part2_queued(input: &InputType) -> u64 {
//     input
//         .iter()
//         .map(|line| {
//             let mut queue = line.clone();

//             let mut index = 1;
//             while queue.len() > 12 && index < queue.len() {
//                 if queue[index] > queue[index - 1] {
//                     queue.remove(index - 1);
//                     index = (index - 1).max(1);
//                 } else {
//                     index += 1;
//                 }
//             }

//             println!("{:?}", queue);
//             queue[0..12].iter().fold(0, |acc, v| (acc * 10) + *v as u64)
//         })
//         .sum()
// }

// advent_bench!(parse, stack_based, part2_stack);

// /// ```rust
// /// use advent_of_code_2025::day03::*;
// /// let input = parse(
// /// r"987654321111111
// /// 811111111111119
// /// 234234234234278
// /// 818181911112111");
// /// assert_eq!(part2_stack(&input), part2(&input));
// /// ```
// pub fn part2_stack(input: &InputType) -> u64 {
//     input
//         .iter()
//         .map(|line| {
//             let mut stack: Vec<u8> = Vec::with_capacity(15);

//             for (index, &item) in line.iter().enumerate() {
//                 while stack.len() + (line.len() - index) > 12
//                     && stack.pop_if(|v| *v < item).is_some()
//                 {}
//                 stack.push(item);
//             }

//             stack[0..12].iter().fold(0, |acc, v| (acc * 10) + *v as u64)
//         })
//         .sum()
// }

// advent_bench!(parse, linkedlist, part2_linkedlist);

// /// ```rust
// /// use advent_of_code_2025::day03::*;
// /// let input = parse(
// /// r"987654321111111
// /// 811111111111119
// /// 234234234234278
// /// 818181911112111");
// /// assert_eq!(part2_linkedlist(&input), part2(&input));
// /// ```
// pub fn part2_linkedlist(input: &InputType) -> u64 {
//     input
//         .iter()
//         .map(|line| {
//             let mut stack = LinkedList::<u8>::new();

//             for (index, &item) in line.iter().enumerate() {
//                 while stack.len() + (line.len() - index) > 12
//                     && stack.back().map_or(false, |i| *i < item)
//                 {
//                     stack.pop_back();
//                 }
//                 stack.push_back(item);
//             }

//             stack
//                 .iter()
//                 .take(12)
//                 .fold(0, |acc, v| (acc * 10) + *v as u64)
//         })
//         .sum()
// }
