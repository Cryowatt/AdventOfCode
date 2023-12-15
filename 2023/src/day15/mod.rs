use std::{
    cmp::Ordering,
    collections::{hash_map::DefaultHasher, HashMap},
    hash::Hasher,
    ops::Range,
    str::Bytes,
};

use advent::*;

advent_day!(Day15, parse, Initialization, part1, part2);

pub fn parse(input: &str) -> Initialization {
    Initialization {
        hash_test: input.lines().flat_map(|line| line.split(",")).collect(),
        instructions: input
            .lines()
            .flat_map(|line| {
                line.split(",").map(|item| {
                        let (label, operation) = match item.chars().last().unwrap() {
                            '-' => (&item[..item.len() - 1], Operation::Remove),
                            _ => (
                                &item[..item.len() - 2],
                                Operation::Add(
                                    item.chars().last().unwrap().to_digit(10).unwrap() as u8
                                ),
                            ),
                        };
                        Instruction {
                            label,
                            hash: hash(label.bytes()),
                            operation,
                        }
                    })
            })
            .collect(),
    }
}

pub struct Initialization<'a> {
    hash_test: Vec<&'a str>,
    instructions: Vec<Instruction<'a>>,
}

struct Instruction<'a> {
    label: &'a str,
    hash: u8,
    operation: Operation,
}

enum Operation {
    Remove,
    Add(u8),
}

/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("rn=1");
/// assert_eq!(30, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("cm-");
/// assert_eq!(253, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("qp=3");
/// assert_eq!(97, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("cm=2");
/// assert_eq!(47, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("qp-");
/// assert_eq!(14, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("pc=4");
/// assert_eq!(180, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("ot=9");
/// assert_eq!(9, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("ab=5");
/// assert_eq!(197, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("pc-");
/// assert_eq!(48, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("pc=6");
/// assert_eq!(214, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("ot=7");
/// assert_eq!(231, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
/// assert_eq!(1320, part1(&input));
/// ```
pub fn part1(input: &Initialization) -> u32 {
    input
        .hash_test
        .iter()
        .map(|item| hash(item.bytes()) as u32)
        .sum()
}

fn hash(bytes: Bytes) -> u8 {
    bytes.fold(0u8, |hash, b| hash.wrapping_add(b).wrapping_mul(17))
}

/// ```rust
/// use advent_of_code_2023::day15::*;
/// let input = parse("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
/// assert_eq!(145, part2(&input));
/// ```
pub fn part2(input: &Initialization) -> u32 {
    let mut boxes: [Vec<(&str, u8)>; 256] = [
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];

    fn find_index(hash_box: &Vec<(&str, u8)>, label: &str) -> Option<usize> {
        for i in 0..hash_box.len() {
            if hash_box[i].0 == label {
                return Some(i);
            }
        }

        None
    }

    for instruction in input.instructions.iter() {
        match instruction.operation {
            Operation::Remove => {
                let hash_box = &mut boxes[instruction.hash as usize];
                if let Some(index) = find_index(&hash_box, instruction.label) {
                    hash_box.remove(index);
                }
            }
            Operation::Add(lens) => {
                let hash_box = &mut boxes[instruction.hash as usize];
                match find_index(&hash_box, instruction.label) {
                    Some(index) => hash_box[index] = (instruction.label, lens),
                    None => hash_box.push((instruction.label, lens)),
                }
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(box_id, hash_box)| {
            hash_box
                .iter()
                .enumerate()
                .map(|(lens_index, (_label, lens))| (lens_index + 1) as u32 * *lens as u32)
                .sum::<u32>()
                * (box_id as u32 + 1)
        })
        .sum::<u32>()
}
