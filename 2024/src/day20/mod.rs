use advent::*;
use array2d::Array2D;

advent_day!(Day20, parse, Array2D<Tile>, part1, part2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Start,
    End,
    Empty,
}

pub fn parse(input: &str) -> InputType<'_> {
    Array2D::from_rows(
        &input
            .lines()
            .map(|line| {
                line.as_bytes()
                    .iter()
                    .map(|tile| match tile {
                        b'.' => Tile::Empty,
                        b'#' => Tile::Wall,
                        b'S' => Tile::Start,
                        b'E' => Tile::End,
                        _ => panic!("Illegal character"),
                    })
                    .collect()
            })
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// ```rust
/// use advent_of_code_2024::day20::*;
/// let input = parse(
/// r"###############
///# #...#...#.....#
///# #.#.#.#.#.###.#
///# #S#...#.#.#...#
///# #######.#.#.###
///# #######.#.#...#
///# #######.#.###.#
///# ###..E#...#...#
///# ###.#######.###
///# #...###...#...#
///# #.#####.#.###.#
///# #.#...#.#.#...#
///# #.#.#.#.#.#.###
///# #...#...#...###
///# ###############");
/// // There are 14 cheats that save 2 picoseconds.
/// assert_eq!(14, find_cheats(&input, |cheat_length| cheat_length == 2));
/// // There are 14 cheats that save 4 picoseconds.
/// assert_eq!(14, find_cheats(&input, |cheat_length| cheat_length == 4));
/// //There are 2 cheats that save 6 picoseconds.
/// assert_eq!(2, find_cheats(&input, |cheat_length| cheat_length == 6));
/// //There are 4 cheats that save 8 picoseconds.
/// assert_eq!(4, find_cheats(&input, |cheat_length| cheat_length == 8));
/// //There are 2 cheats that save 10 picoseconds.
/// assert_eq!(2, find_cheats(&input, |cheat_length| cheat_length == 10));
/// //There are 3 cheats that save 12 picoseconds.
/// assert_eq!(3, find_cheats(&input, |cheat_length| cheat_length == 12));
/// //There is one cheat that saves 20 picoseconds.
/// assert_eq!(1, find_cheats(&input, |cheat_length| cheat_length == 20));
/// //There is one cheat that saves 36 picoseconds.
/// assert_eq!(1, find_cheats(&input, |cheat_length| cheat_length == 36));
/// //There is one cheat that saves 38 picoseconds.
/// assert_eq!(1, find_cheats(&input, |cheat_length| cheat_length == 38));
/// //There is one cheat that saves 40 picoseconds.
/// assert_eq!(1, find_cheats(&input, |cheat_length| cheat_length == 40));
/// //There is one cheat that saves 64 picoseconds.
/// assert_eq!(1, find_cheats(&input, |cheat_length| cheat_length == 64));
/// ```
pub fn part1(input: &InputType) -> usize {
    find_cheats(input, |cheat_length| cheat_length >= 100)
}

pub fn find_cheats<P>(input: &InputType, predicate: P) -> usize
where
    P: Fn(i32) -> bool,
{
    let map = input;
    let bounds = UPoint::new(input.row_len() as u32, input.column_len() as u32);
    let start = input
        .enumerate_row_major()
        .find_map(|((row, column), &tile)| {
            (tile == Tile::Start).then(|| UPoint::new(column as u32, row as u32))
        })
        .unwrap();

    let mut current = start.clone();
    let mut direction = Direction::North;

    let distance_map = &mut Array2D::filled_with(None, input.row_len(), input.column_len());
    let mut steps = 0;
    distance_map[(start.y as usize, start.x as usize)] = Some(steps);
    let mut cheats = 0;

    fn cheat(
        current: UPoint,
        direction: Direction,
        bounds: &Point<u32>,
    ) -> (Option<UPoint>, Option<UPoint>) {
        let next = current.direction_checked(direction, bounds);
        let cheat = next.and_then(|next| next.direction_checked(direction, bounds));
        (next, cheat)
    }

    loop {
        // Look for cheats
        let left_turn = direction.left();
        let right_turn = direction.right();

        let (forward, cheat_forward) = cheat(current, direction, &bounds);
        let (left, left_cheat) = cheat(current, left_turn, &bounds);
        let (right, right_cheat) = cheat(current, right_turn, &bounds);
        cheats += [cheat_forward, left_cheat, right_cheat]
            .iter()
            .map(|cheat| {
                if let Some(point) = cheat {
                    if let Some(distance) = distance_map[(point.y as usize, point.x as usize)] {
                        let cheat_distance = steps - (distance + 2);
                        if predicate(cheat_distance) {
                            return 1;
                        }
                    }
                }

                0
            })
            .sum::<usize>();

        steps += 1;

        if let Some(next) = forward {
            if map[(next.y as usize, next.x as usize)] != Tile::Wall {
                distance_map[(next.y as usize, next.x as usize)] = Some(steps);
                current = next;
                continue;
            }
        }

        if let Some(next) = left {
            if map[(next.y as usize, next.x as usize)] != Tile::Wall {
                distance_map[(next.y as usize, next.x as usize)] = Some(steps);
                direction = left_turn;
                current = next;
                continue;
            }
        }

        if let Some(next) = right {
            if map[(next.y as usize, next.x as usize)] != Tile::Wall {
                distance_map[(next.y as usize, next.x as usize)] = Some(steps);
                direction = right_turn;
                current = next;
                continue;
            }
        }

        // No more valid moves, so this is the end
        break;
    }

    cheats
}

/// ```rust
/// use advent_of_code_2024::day20::*;
/// let input = parse(
/// r"###############
///# #...#...#.....#
///# #.#.#.#.#.###.#
///# #S#...#.#.#...#
///# #######.#.#.###
///# #######.#.#...#
///# #######.#.###.#
///# ###..E#...#...#
///# ###.#######.###
///# #...###...#...#
///# #.#####.#.###.#
///# #.#...#.#.#...#
///# #.#.#.#.#.#.###
///# #...#...#...###
///# ###############");
/// //There are 32 cheats that save 50 picoseconds.
/// assert_eq!(32, find_long_cheats(&input, |cheat_length| cheat_length == 50));
/// //There are 31 cheats that save 52 picoseconds.
/// assert_eq!(31, find_long_cheats(&input, |cheat_length| cheat_length == 52));
/// //There are 29 cheats that save 54 picoseconds.
/// assert_eq!(29, find_long_cheats(&input, |cheat_length| cheat_length == 54));
/// //There are 39 cheats that save 56 picoseconds.
/// assert_eq!(39, find_long_cheats(&input, |cheat_length| cheat_length == 56));
/// //There are 25 cheats that save 58 picoseconds.
/// assert_eq!(25, find_long_cheats(&input, |cheat_length| cheat_length == 58));
/// //There are 23 cheats that save 60 picoseconds.
/// assert_eq!(23, find_long_cheats(&input, |cheat_length| cheat_length == 60));
/// //There are 20 cheats that save 62 picoseconds.
/// assert_eq!(20, find_long_cheats(&input, |cheat_length| cheat_length == 62));
/// //There are 19 cheats that save 64 picoseconds.
/// assert_eq!(19, find_long_cheats(&input, |cheat_length| cheat_length == 64));
/// //There are 12 cheats that save 66 picoseconds.
/// assert_eq!(12, find_long_cheats(&input, |cheat_length| cheat_length == 66));
/// //There are 14 cheats that save 68 picoseconds.
/// assert_eq!(14, find_long_cheats(&input, |cheat_length| cheat_length == 68));
/// //There are 12 cheats that save 70 picoseconds.
/// assert_eq!(12, find_long_cheats(&input, |cheat_length| cheat_length == 70));
/// //There are 22 cheats that save 72 picoseconds.
/// assert_eq!(22, find_long_cheats(&input, |cheat_length| cheat_length == 72));
/// //There are 4 cheats that save 74 picoseconds.
/// assert_eq!(4, find_long_cheats(&input, |cheat_length| cheat_length == 74));
/// //There are 3 cheats that save 76 picoseconds.
/// assert_eq!(3, find_long_cheats(&input, |cheat_length| cheat_length == 76));
/// ```
pub fn part2(input: &InputType) -> usize {
    find_long_cheats(input, |cheat_length| cheat_length >= 100)
}

pub fn find_long_cheats<P>(input: &InputType, predicate: P) -> usize
where
    P: Fn(u32) -> bool,
{
    let map = input;
    let bounds = UPoint::new(input.row_len() as u32, input.column_len() as u32);
    let start = input
        .enumerate_row_major()
        .find_map(|((row, column), &tile)| {
            (tile == Tile::Start).then(|| UPoint::new(column as u32, row as u32))
        })
        .unwrap();

    let mut current = start.clone();
    let mut direction = Direction::North;

    let distance_map = &mut Array2D::filled_with(None, input.row_len(), input.column_len());
    let mut steps = 0;
    distance_map[(start.y as usize, start.x as usize)] = Some(steps);
    let mut cheats = 0;

    loop {
        // Look for cheats
        let left_turn = direction.left();
        let right_turn = direction.right();

        cheats += (0..=20)
            .flat_map(|x| {
                (0..=(20 - x)).flat_map(move |y| match (x, y) {
                    (0, 0) => vec![],
                    (0, _) => vec![
                        (current.x as i32, current.y as i32 + y),
                        (current.x as i32, current.y as i32 - y),
                    ],
                    (_, 0) => vec![
                        (current.x as i32 + x, current.y as i32),
                        (current.x as i32 - x, current.y as i32),
                    ],
                    (_, _) => vec![
                        (current.x as i32 + x, current.y as i32 + y),
                        (current.x as i32 + x, current.y as i32 - y),
                        (current.x as i32 - x, current.y as i32 + y),
                        (current.x as i32 - x, current.y as i32 - y),
                    ],
                })
            })
            .filter_map(|(cheat_x, cheat_y)| {
                (cheat_x >= 0
                    && cheat_y >= 0
                    && cheat_x < bounds.x as i32
                    && cheat_y < bounds.y as i32)
                    .then(|| UPoint::new(cheat_x as u32, cheat_y as u32))
            })
            .filter(|point| {
                if let Some(distance) = distance_map[(point.y as usize, point.x as usize)] {
                    let cheat_jump = point.manhattan(&current);
                    let cheat_distance = steps - (distance + cheat_jump);
                    predicate(cheat_distance)
                } else {
                    false
                }
            })
            .count();

        steps += 1;

        if let Some(next) = current.direction_checked(direction, &bounds) {
            if map[(next.y as usize, next.x as usize)] != Tile::Wall {
                distance_map[(next.y as usize, next.x as usize)] = Some(steps);
                current = next;
                continue;
            }
        }

        if let Some(next) = current.direction_checked(left_turn, &bounds) {
            if map[(next.y as usize, next.x as usize)] != Tile::Wall {
                distance_map[(next.y as usize, next.x as usize)] = Some(steps);
                direction = left_turn;
                current = next;
                continue;
            }
        }

        if let Some(next) = current.direction_checked(right_turn, &bounds) {
            if map[(next.y as usize, next.x as usize)] != Tile::Wall {
                distance_map[(next.y as usize, next.x as usize)] = Some(steps);
                direction = right_turn;
                current = next;
                continue;
            }
        }

        // No more valid moves, so this is the end
        break;
    }

    cheats
}
