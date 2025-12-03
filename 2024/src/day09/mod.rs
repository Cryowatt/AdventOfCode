use std::iter;

use advent::*;

advent_day!(Day09, parse, Vec<u8>, part1, part2);

pub fn parse(input: &str) -> InputType<'_> {
    input.as_bytes().iter().map(|b| *b - b'0').collect()
}

/// ```rust
/// use advent_of_code_2024::day09::*;
/// let input = parse(
/// r"12345");
/// assert_eq!(60, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2024::day09::*;
/// let input = parse(
/// r"2333133121414131402");
/// assert_eq!(1928, part1(&input));
/// ```
pub fn part1(input: &InputType) -> isize {
    let mut head = input
        .iter()
        .enumerate()
        .flat_map(|(index, &value)| {
            iter::repeat_n(
                if index % 2 == 0 {
                    index as isize / 2
                } else {
                    -(index as isize / 2) - 1
                },
                value as usize,
            )
        })
        .peekable();
    let mut tail = input
        .iter()
        .enumerate()
        .filter(|&(index, _)| index % 2 == 0)
        .rev()
        .flat_map(|(index, &count)| iter::repeat_n((index / 2) as isize, count as usize))
        .peekable();

    let mut index = 0;
    let mut checksum = 0;

    while let Some(left) = head.next_if(|left| left < tail.peek().unwrap()) {
        if left < 0 {
            if let Some(right) = tail.next_if(|&right| left.abs() < right) {
                checksum += index * right;
                index += 1;
            }
        } else {
            checksum += index * left;
            index += 1;
        }
    }

    let last = *tail.peek().unwrap();

    while let Some(right) = tail.next_if(|&next| next == last) {
        checksum += index * right;
        index += 1;
    }

    checksum
}

/// ```rust
/// use advent_of_code_2024::day09::*;
/// let input = parse(
/// r"2333133121414131402");
/// assert_eq!(2858, part2(&input));
/// ```
pub fn part2(input: &InputType) -> u64 {
    let mut file_system: Vec<_> = input
        .iter()
        .enumerate()
        .map(|(index, &size)| {
            if index % 2 == 0 {
                FileAllocation::File(index as u32 / 2, size)
            } else {
                FileAllocation::Empty(size)
            }
        })
        .collect();

    let mut compressed: Vec<_> = Vec::new();

    while let Some(file) = file_system.pop() {
        match file {
            FileAllocation::Empty(_) => compressed.push(file),
            FileAllocation::File(_, size) => {
                match file_system
                    .iter()
                    .enumerate()
                    .filter(|&(_, allocation)| match allocation {
                        FileAllocation::Empty(space) => *space >= size,
                        _ => false,
                    })
                    .next()
                {
                    Some((index, FileAllocation::Empty(space))) => {
                        let free = *space - size;
                        if free == 0 {
                            file_system[index] = file;
                        } else {
                            file_system[index] = file;
                            file_system.insert(index + 1, FileAllocation::Empty(free));
                        }
                        compressed.push(FileAllocation::Empty(size));
                    }
                    _ => compressed.push(file),
                }
            }
        }
    }

    let mut checksum: u64 = 0;
    let mut block_id: u32 = 0;

    for entry in compressed.iter().rev() {
        match entry {
            FileAllocation::File(file_id, size) => {
                for id in block_id..(block_id + *size as u32) {
                    checksum += *file_id as u64 * id as u64;
                }
                block_id += *size as u32;
            }
            FileAllocation::Empty(space) => block_id += *space as u32,
        }
    }

    checksum
}

#[derive(Debug, Copy, Clone)]
enum FileAllocation {
    File(u32, u8),
    Empty(u8),
}
