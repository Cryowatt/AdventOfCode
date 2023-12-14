use std::{
    cmp::Ordering,
    io::stdout,
    ops::{Range, RangeBounds, RangeFull, RangeTo},
    os::windows::io::AsRawHandle,
};

use advent::*;

advent_day!(Day14, parse, DishMap, part1, part2);

pub fn parse(input: &str) -> DishMap {
    // println!("{}", input);
    let mut rounds = vec![];
    let mut squares = vec![];
    let mut width = 0;
    let mut height = 0;

    for (y, line) in input.lines().enumerate() {
        height = y;
        let line = line.as_bytes();
        // let mut row_lookup = vec![];
        // row_lookup.reserve_exact(line.len());
        // let mut row_slide = None;
        width = line.len();

        for x in 0..line.len() {
            match line[x] {
                b'#' => {
                    squares.push(UPoint::new(x as u32, y as u32));
                }
                b'O' => {
                    rounds.push(UPoint::new(x as u32, y as u32));
                }
                _ => {}
            }
            // if line[x] == b'#' {
            //     // cell contains an immovable block
            //     slide_lookup[y][x] = None;
            //     if let Some(row_slide) = row_slide {
            //         sl
            //     }
            //     row_slide = None;
            // } else {
            //     let horizontal_slide = if x > 0 {
            //         slide_lookup[y][x - 1].unwrap_or_default()
            //     } else {
            //         let slide = vec![];
            //         slides.push(slide);
            //         slide
            //     };
            //     let vertical_slide = if y > 0 {
            //         slide_lookup[y - 1][x].unwrap_or_default()
            //     } else {
            //         vec![]
            //     };
            // }
            //
        }

        // slide_lookup.push(row_lookup);
    }
    // .map(|(y, line)| for y in 0..line.len() {})
    // input.lines().enumerate().map(|(y, line)| for y in 0..line.len() {})
    DishMap {
        width,
        height: height + 1,
        rounds,
        squares,
    }
}

pub struct DishMap {
    width: usize,
    height: usize,
    rounds: Vec<UPoint>,
    squares: Vec<UPoint>,
}

/// ```rust
/// use advent_of_code_2023::day14::*;
/// let input = parse(
/// "O....#....
/// O.OO#....#
/// .....##...
/// OO.#O....O
/// .O.....O#.
/// O.#..O.#.#
/// ..O..#O..O
/// .......O..
///##....###..
///##OO..#....");
/// assert_eq!(136, part1(&input));
/// ```
pub fn part1(input: &DishMap) -> u32 {
    // #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Segment {
        range: Range<usize>,
        rocks: u32,
    }

    let mut vertical_segments = vec![];
    vertical_segments.reserve_exact(input.height);

    // Add columns
    for _col in 0..input.width {
        vertical_segments.push(vec![Segment {
            range: 0..input.height,
            rocks: 0,
        }]);
    }

    for square in input.squares.iter() {
        let column = vertical_segments.get_mut(square.x as usize).unwrap();
        let segment_index = column
            .binary_search_by(|segment| {
                // print!(
                //     "B {} in {}..{} ",
                //     square.y, segment.range.start, segment.range.end
                // );
                if segment.range.contains(&(square.y as usize)) {
                    // println!("EQUAL");
                    Ordering::Equal
                } else {
                    segment.range.start.cmp(&(square.y as usize))
                }
            })
            .unwrap();

        // Split segment
        // match segment_index {
        //     Ok(index) => {
        let high_range = {
            // Split low half of segment
            let segment = column.get_mut(segment_index).unwrap();
            let low_range = segment.range.start..square.y as usize;
            let high_range = (square.y as usize + 1)..segment.range.end;
            // println!(
            //     "SPLIT {}..{} => {}..{} {}..{}",
            //     segment.range.start,
            //     segment.range.end,
            //     low_range.start,
            //     low_range.end,
            //     high_range.start,
            //     high_range.end,
            // );
            segment.range = low_range;
            high_range
        };

        // Add high half of segment
        column.insert(
            segment_index + 1,
            Segment {
                range: high_range,
                rocks: 0,
            },
        );
        //     }
        //     Err(_) => unreachable!(),
        // }
        // segment.spl.split_at(mid)
        // println!("{}, {}", square.x, square.y);
    }

    // for (x, column) in vertical_segments.iter().enumerate() {
    //     for segment in column {
    //         println!("{x} {}..{}", segment.range.start, segment.range.end);
    //     }
    // }

    let mut weight = 0;

    for round in input.rounds.iter() {
        let column = vertical_segments.get_mut(round.x as usize).unwrap();
        let segment_index = column
            .binary_search_by(|segment| {
                // print!(
                //     "B {} in {}..{} ",
                //     round.y, segment.range.start, segment.range.end
                // );
                if segment.range.contains(&(round.y as usize)) {
                    // println!("EQUAL");
                    Ordering::Equal
                } else {
                    // round.y.cmp(&(segment.range.start as u32))
                    segment.range.start.cmp(&(round.y as usize))
                }
            })
            .unwrap();
        let segment = column.get_mut(segment_index).unwrap();
        weight += (input.height - segment.range.start) as u32 - segment.rocks;
        segment.rocks += 1;
    }

    // vertical_segments.0
    weight
}

/// ```rust
/// use advent_of_code_2023::day14::*;
/// let input = parse(
/// "O....#....
/// O.OO#....#
/// .....##...
/// OO.#O....O
/// .O.....O#.
/// O.#..O.#.#
/// ..O..#O..O
/// .......O..
///##....###..
///##OO..#....");
/// //assert_eq!(400, part2(&input));
/// ```
pub fn part2(input: &DishMap) -> u32 {
    0
}
