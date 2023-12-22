use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use advent::*;

advent_day!(Day22, parse, Vec<Line>, part1, part2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DigInstruction {
    direction: Direction,
    length: u32,
    colour: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Line {
    x: u32,
    y: u32,
    z: u32,
    axis: Axis,
    length: u32,
}

impl Ord for Line {
    fn cmp(&self, other: &Self) -> Ordering {
        other.z.cmp(&self.z)
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Line {
    pub fn new(start: (u32, u32, u32), end: (u32, u32, u32)) -> Self {
        let (axis, length) = if start.0 < end.0 {
            (Axis::X, end.0 - start.0 + 1)
        } else if start.1 < end.1 {
            (Axis::Y, end.1 - start.1 + 1)
        } else {
            (Axis::Z, end.2 - start.2 + 1)
        };

        Self {
            x: start.0,
            y: start.1,
            z: start.2,
            axis,
            length,
        }
    }
}

pub fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("~").unwrap();
            let mut start_coords = start.split(",").map(|p| p.parse::<u32>().unwrap());
            let mut end_coords = end.split(",").map(|p| p.parse::<u32>().unwrap());
            Line::new(
                (
                    start_coords.next().unwrap(),
                    start_coords.next().unwrap(),
                    start_coords.next().unwrap(),
                ),
                (
                    end_coords.next().unwrap(),
                    end_coords.next().unwrap(),
                    end_coords.next().unwrap(),
                ),
            )
        })
        .collect()
}

/// ```rust
/// use advent_of_code_2023::day22::*;
/// let input = parse(
/// r"1,0,1~1,2,1
/// 0,0,2~2,0,2
/// 0,2,3~2,2,3
/// 0,0,4~0,2,4
/// 2,0,5~2,2,5
/// 0,1,6~2,1,6
/// 1,1,8~1,1,9");
/// assert_eq!(5, part1(&input));
/// ```
pub fn part1(blocks: &Vec<Line>) -> u64 {
    let mut blocks = BinaryHeap::from_iter(blocks.iter());
    let mut supporting = vec![HashSet::new()];
    let mut supported_by = vec![HashSet::new()];

    // Height map is an Y,X grid containg lowest Z point and the block id (zero for the base)
    let mut height_map = vec![];
    for _y in 0..10 {
        height_map.push(vec![
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
        ]);
    }

    // Drop blocks
    let mut block_id = 0;
    while let Some(block) = blocks.pop() {
        block_id += 1;
        supporting.push(HashSet::new());
        let mut block_supported_by = HashSet::new();

        match block.axis {
            Axis::X => {
                let max_height = (block.x + 0..block.x + block.length)
                    .map(|x: u32| height_map[block.y as usize][x as usize].0)
                    .max()
                    .unwrap();

                for x in block.x + 0..block.x + block.length {
                    let (z, supporting_block) = height_map[block.y as usize][x as usize];
                    if z == max_height {
                        supporting[supporting_block].insert(block_id);
                        block_supported_by.insert(supporting_block);
                    }
                    height_map[block.y as usize][x as usize] = (max_height + 1, block_id);
                }
            }
            Axis::Y => {
                let max_height = (block.y + 0..block.y + block.length)
                    .map(|y: u32| height_map[y as usize][block.x as usize].0)
                    .max()
                    .unwrap();

                for y in block.y + 0..block.y + block.length {
                    let (z, supporting_block) = height_map[y as usize][block.x as usize];
                    if z == max_height {
                        supporting[supporting_block].insert(block_id);
                        block_supported_by.insert(supporting_block);
                    }
                    height_map[y as usize][block.x as usize] = (max_height + 1, block_id);
                }
            }
            Axis::Z => {
                let (z, supporting_block) = height_map[block.y as usize][block.x as usize];
                height_map[block.y as usize][block.x as usize] = (z + block.length, block_id);
                supporting[supporting_block].insert(block_id);
                block_supported_by.insert(supporting_block);
            }
        }

        supported_by.push(block_supported_by);
    }

    supporting
        .iter()
        .enumerate()
        .filter_map(|(block_id, contacts)| {
            if contacts.iter().all(|id| supported_by[*id].len() > 1) {
                Some(block_id)
            } else {
                None
            }
        })
        .count() as u64
}

/// ```rust
/// use advent_of_code_2023::day22::*;
/// let input = parse(
/// r"1,0,1~1,2,1
/// 0,0,2~2,0,2
/// 0,2,3~2,2,3
/// 0,0,4~0,2,4
/// 2,0,5~2,2,5
/// 0,1,6~2,1,6
/// 1,1,8~1,1,9");
/// assert_eq!(7, part2(&input));
/// ```
pub fn part2(blocks: &Vec<Line>) -> u64 {
    let mut blocks = BinaryHeap::from_iter(blocks.iter());
    let mut supporting = vec![HashSet::new()];
    let mut supported_by = vec![HashSet::new()];

    // Height map is an Y,X grid containg lowest Z point and the block id (zero for the base)
    let mut height_map = vec![];
    for _y in 0..10 {
        height_map.push(vec![
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
        ]);
    }

    // Drop blocks
    let mut block_id = 0;
    while let Some(block) = blocks.pop() {
        block_id += 1;
        supporting.push(HashSet::new());
        let mut block_supported_by = HashSet::new();

        match block.axis {
            Axis::X => {
                let max_height = (block.x + 0..block.x + block.length)
                    .map(|x: u32| height_map[block.y as usize][x as usize].0)
                    .max()
                    .unwrap();

                for x in block.x + 0..block.x + block.length {
                    let (z, supporting_block) = height_map[block.y as usize][x as usize];
                    if z == max_height {
                        supporting[supporting_block].insert(block_id);
                        block_supported_by.insert(supporting_block);
                    }
                    height_map[block.y as usize][x as usize] = (max_height + 1, block_id);
                }
            }
            Axis::Y => {
                let max_height = (block.y + 0..block.y + block.length)
                    .map(|y: u32| height_map[y as usize][block.x as usize].0)
                    .max()
                    .unwrap();

                for y in block.y + 0..block.y + block.length {
                    let (z, supporting_block) = height_map[y as usize][block.x as usize];
                    if z == max_height {
                        supporting[supporting_block].insert(block_id);
                        block_supported_by.insert(supporting_block);
                    }
                    height_map[y as usize][block.x as usize] = (max_height + 1, block_id);
                }
            }
            Axis::Z => {
                let (z, supporting_block) = height_map[block.y as usize][block.x as usize];
                height_map[block.y as usize][block.x as usize] = (z + block.length, block_id);
                supporting[supporting_block].insert(block_id);
                block_supported_by.insert(supporting_block);
            }
        }

        supported_by.push(block_supported_by);
    }

    supporting
        .iter()
        .enumerate()
        .skip(1)
        .map(|(block_id, contacts)| {
            let mut unsupported = HashSet::new();
            unsupported.insert(block_id);
            let mut disturbed = Vec::from_iter(contacts.iter().map(|&id| id));

            while let Some(dependant_id) = disturbed.pop() {
                if unsupported.is_superset(&supported_by[dependant_id]) {
                    unsupported.insert(dependant_id);
                    disturbed.extend(supporting[dependant_id].iter());
                }
            }

            unsupported.len() as u64 - 1
        })
        .sum()
}
