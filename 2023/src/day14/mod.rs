use std::{
    cmp::Ordering,
    collections::{hash_map::DefaultHasher, HashMap},
    hash::Hasher,
    ops::Range,
};

use advent::*;

advent_day!(Day14, parse, DishMap, part1, part2);

pub fn parse(input: &str) -> DishMap {
    let mut rounds = vec![];
    let mut squares = vec![];
    let mut width = 0;
    let mut height = 0;

    for (y, line) in input.lines().enumerate() {
        height = y;
        let line = line.as_bytes();
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
        }
    }

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

struct Segment {
    range: Range<usize>,
    rocks: u32,
}

fn setup_segments<FWidth, FDepth>(
    width: usize,
    depth: usize,
    squares: &Vec<UPoint>,
    width_dimension: FWidth,
    depth_dimension: FDepth,
) -> Vec<Vec<Segment>>
where
    FWidth: Fn(&UPoint) -> u32,
    FDepth: Fn(&UPoint) -> u32,
{
    let mut segments = vec![];
    segments.reserve_exact(width);

    for _ in 0..width {
        segments.push(vec![Segment {
            range: 0..depth,
            rocks: 0,
        }]);
    }

    for square in squares.iter() {
        let column = segments.get_mut(width_dimension(square) as usize).unwrap();
        let segment_index = column
            .binary_search_by(|segment| {
                if segment.range.contains(&(depth_dimension(square) as usize)) {
                    Ordering::Equal
                } else {
                    segment.range.start.cmp(&(depth_dimension(square) as usize))
                }
            })
            .unwrap();

        // Split segment
        {
            // Split low half of segment
            let segment = column.get_mut(segment_index).unwrap();
            let low_range = segment.range.start..depth_dimension(square) as usize;
            let high_range = (depth_dimension(square) as usize + 1)..segment.range.end;

            if low_range.len() > 0 {
                segment.range = low_range;
                // Add high half of segment
                column.insert(
                    segment_index + 1,
                    Segment {
                        range: high_range,
                        rocks: 0,
                    },
                );
            } else {
                segment.range = high_range;
            }
        };
    }

    segments
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
    let mut vertical_segments =
        setup_segments(input.width, input.height, &input.squares, |p| p.x, |p| p.y);

    let mut weight = 0;

    for round in input.rounds.iter() {
        let column = vertical_segments.get_mut(round.x as usize).unwrap();
        let segment_index = column
            .binary_search_by(|segment| {
                if segment.range.contains(&(round.y as usize)) {
                    Ordering::Equal
                } else {
                    segment.range.start.cmp(&(round.y as usize))
                }
            })
            .unwrap();
        let segment = column.get_mut(segment_index).unwrap();
        weight += (input.height - segment.range.start) as u32 - segment.rocks;
        segment.rocks += 1;
    }

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
/// assert_eq!(64, part2(&input));
/// ```
pub fn part2(input: &DishMap) -> u32 {
    let mut vertical_segments =
        setup_segments(input.width, input.height, &input.squares, |p| p.x, |p| p.y);
    let mut horizontal_segments =
        setup_segments(input.height, input.width, &input.squares, |p| p.y, |p| p.x);

    let lookup_table = {
        let mut table = vec![];
        for y in 0..input.height {
            let mut row_lookup = vec![];

            for x in 0..input.width {
                let vertical_segment_index = vertical_segments
                    .get_mut(x)
                    .unwrap()
                    .binary_search_by(|segment| {
                        if segment.range.contains(&y) {
                            Ordering::Equal
                        } else {
                            segment.range.start.cmp(&y)
                        }
                    });

                if let Ok(vertical_segment_index) = vertical_segment_index {
                    let horizontal_segment_index = horizontal_segments
                        .get_mut(y)
                        .unwrap()
                        .binary_search_by(|segment| {
                            if segment.range.contains(&x) {
                                Ordering::Equal
                            } else {
                                segment.range.start.cmp(&x)
                            }
                        })
                        .unwrap();
                    row_lookup.push(Some((vertical_segment_index, horizontal_segment_index)));
                } else {
                    row_lookup.push(None);
                }
            }
            table.push(row_lookup);
        }
        table
    };

    // First insert generates north rotation
    for round in input.rounds.iter() {
        let column = vertical_segments.get_mut(round.x as usize).unwrap();
        let segment_index = column
            .binary_search_by(|segment| {
                if segment.range.contains(&(round.y as usize)) {
                    Ordering::Equal
                } else {
                    segment.range.start.cmp(&(round.y as usize))
                }
            })
            .unwrap();
        let segment = column.get_mut(segment_index).unwrap();
        segment.rocks += 1;
    }

    let get_state = |horizontal_segments: &Vec<Vec<Segment>>| {
        let mut hasher = DefaultHasher::new();
        let mut load = 0;

        for (y, row) in horizontal_segments.iter().enumerate() {
            for segment in row {
                load += segment.rocks * (input.height - y) as u32;
                hasher.write_u32(segment.rocks);
            }
        }
        (hasher.finish(), load)
    };

    fn rotate<FLookup: Fn(usize, usize) -> usize>(
        from_segment: &mut Vec<Vec<Segment>>,
        to_segment: &mut Vec<Vec<Segment>>,
        segement_index_lookup: FLookup,
        reverse_order: bool,
    ) {
        for (from_index, column) in from_segment.iter_mut().enumerate() {
            for segment in column {
                let rock_range = if reverse_order {
                    segment.range.end - segment.rocks as usize..segment.range.end
                } else {
                    segment.range.start..segment.range.start + segment.rocks as usize
                };
                for to_index in rock_range {
                    let segment_index = segement_index_lookup(from_index, to_index);
                    let transfer_segment = to_segment[to_index].get_mut(segment_index).unwrap();
                    transfer_segment.rocks += 1;
                }
                segment.rocks = 0;
            }
        }
    }

    rotate(
        &mut vertical_segments,
        &mut horizontal_segments,
        |from_index, to_index| lookup_table[to_index][from_index].unwrap().1,
        false,
    );
    rotate(
        &mut horizontal_segments,
        &mut vertical_segments,
        |from_index, to_index| lookup_table[from_index][to_index].unwrap().0,
        false,
    );
    rotate(
        &mut vertical_segments,
        &mut horizontal_segments,
        |from_index, to_index| lookup_table[to_index][from_index].unwrap().1,
        true,
    );

    let mut loop_detect = HashMap::<u64, (u32, u32)>::new();
    let (state, load) = get_state(&horizontal_segments);
    loop_detect.insert(state, (1, load));

    for i in 2..=1000000000 {
        rotate(
            &mut horizontal_segments,
            &mut vertical_segments,
            |from_index, to_index| lookup_table[from_index][to_index].unwrap().0,
            true,
        );
        rotate(
            &mut vertical_segments,
            &mut horizontal_segments,
            |from_index, to_index| lookup_table[to_index][from_index].unwrap().1,
            false,
        );
        rotate(
            &mut horizontal_segments,
            &mut vertical_segments,
            |from_index, to_index| lookup_table[from_index][to_index].unwrap().0,
            false,
        );
        rotate(
            &mut vertical_segments,
            &mut horizontal_segments,
            |from_index, to_index| lookup_table[to_index][from_index].unwrap().1,
            true,
        );
        let (state, load) = get_state(&horizontal_segments);
        match loop_detect.insert(state, (i, load)) {
            Some((index, _)) => {
                let loop_index = ((1000000000 - index) % (i - index)) + index;
                return loop_detect
                    .values()
                    .find_map(|(index, load)| {
                        if *index == loop_index {
                            Some(*load)
                        } else {
                            None
                        }
                    })
                    .unwrap();
            }
            None => continue,
        }
    }

    unreachable!()
}
