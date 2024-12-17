use std::collections::BinaryHeap;

use advent::*;
use array2d::Array2D;

advent_day!(Day16, parse, Array2D<Tile>, part1, part2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Seat,
    Empty,
    Start,
    End,
}

pub fn parse(input: &str) -> InputType {
    let rows = input.lines().count();
    let colums = input.lines().next().unwrap().len();
    Array2D::from_iter_row_major(
        input.lines().flat_map(|line| {
            line.as_bytes().iter().map(|tile| match tile {
                b'.' => Tile::Empty,
                b'#' => Tile::Wall,
                b'S' => Tile::Start,
                b'E' => Tile::End,
                _ => Tile::Empty,
            })
        }),
        rows,
        colums,
    )
    .unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Step {
    score: u32,
    position: UPoint,
    direction: Direction,
}

impl Step {
    pub const fn new(score: u32, position: UPoint, direction: Direction) -> Self {
        Self {
            score,
            position,
            direction,
        }
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}
impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// ```rust
/// use advent_of_code_2024::day16::*;
/// let input = parse(
/// r"###############
/// ##.......#....E#
/// ##.#.###.#.###.#
/// ##.....#.#...#.#
/// ##.###.#####.#.#
/// ##.#.#.......#.#
/// ##.#.#####.###.#
/// ##...........#.#
/// ####.#.#####.#.#
/// ##...#.....#.#.#
/// ##.#.#.###.#.#.#
/// ##.....#...#.#.#
/// ##.###.#.#.#.#.#
/// ##S..#.....#...#
/// ################");
/// assert_eq!(7036, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2024::day16::*;
/// let input = parse(
/// r"#################
/// ##...#...#...#..E#
/// ##.#.#.#.#.#.#.#.#
/// ##.#.#.#...#...#.#
/// ##.#.#.#.###.#.#.#
/// ##...#.#.#.....#.#
/// ##.#.#.#.#.#####.#
/// ##.#...#.#.#.....#
/// ##.#.#####.#.###.#
/// ##.#.#.......#...#
/// ##.#.###.#####.###
/// ##.#.#...#.....#.#
/// ##.#.#.#####.###.#
/// ##.#.#.........#.#
/// ##.#.#.#########.#
/// ##S#.............#
/// ##################");
/// assert_eq!(11048, part1(&input));
/// ```
pub fn part1(input: &InputType) -> u32 {
    let mut maze = input.clone();
    let mut pending = BinaryHeap::new();
    let bounds = UPoint::new(input.column_len() as u32, input.row_len() as u32);
    let start = input
        .enumerate_row_major()
        .find(|(_, &tile)| tile == Tile::Start)
        .map(|((row, column), _)| UPoint::new(column as u32, row as u32))
        .unwrap();
    pending.push(Step::new(0, start, Direction::East));

    while let Some(Step {
        score,
        position,
        direction,
    }) = pending.pop()
    {
        if maze[(position.y as usize, position.x as usize)] == Tile::End {
            return score;
        }

        maze[(position.y as usize, position.x as usize)] = Tile::Wall;

        if let Some(next) = position.direction_checked(direction, &bounds) {
            if maze[(next.y as usize, next.x as usize)] != Tile::Wall {
                pending.push(Step::new(score + 1, next, direction));
            }
        }
        let left = direction.left();
        if let Some(next) = position.direction_checked(left, &bounds) {
            if maze[(next.y as usize, next.x as usize)] != Tile::Wall {
                pending.push(Step::new(score + 1001, next, left));
            }
        }
        let right = direction.right();
        if let Some(next) = position.direction_checked(right, &bounds) {
            if maze[(next.y as usize, next.x as usize)] != Tile::Wall {
                pending.push(Step::new(score + 1001, next, right));
            }
        }
    }

    panic!("Couldn't find end");
}

/// ```rust
/// use advent_of_code_2024::day16::*;
/// let input = parse(
/// r"###############
/// ##.......#....E#
/// ##.#.###.#.###.#
/// ##.....#.#...#.#
/// ##.###.#####.#.#
/// ##.#.#.......#.#
/// ##.#.#####.###.#
/// ##...........#.#
/// ####.#.#####.#.#
/// ##...#.....#.#.#
/// ##.#.#.###.#.#.#
/// ##.....#...#.#.#
/// ##.###.#.#.#.#.#
/// ##S..#.....#...#
/// ################");
/// assert_eq!(45, part2(&input));
/// ```
/// ```rust
/// use advent_of_code_2024::day16::*;
/// let input = parse(
/// r"#################
/// ##...#...#...#..E#
/// ##.#.#.#.#.#.#.#.#
/// ##.#.#.#...#...#.#
/// ##.#.#.#.###.#.#.#
/// ##...#.#.#.....#.#
/// ##.#.#.#.#.#####.#
/// ##.#...#.#.#.....#
/// ##.#.#####.#.###.#
/// ##.#.#.......#...#
/// ##.#.###.#####.###
/// ##.#.#...#.....#.#
/// ##.#.#.#####.###.#
/// ##.#.#.........#.#
/// ##.#.#.#########.#
/// ##S#.............#
/// ##################");
/// assert_eq!(64, part2(&input));
/// ```
/// 432 is too low
pub fn part2(input: &InputType) -> u32 {
    let mut pending = BinaryHeap::new();
    let bounds = UPoint::new(input.column_len() as u32, input.row_len() as u32);
    let mut maze_score = Array2D::filled_with(
        [u32::MAX, u32::MAX, u32::MAX, u32::MAX],
        input.row_len(),
        input.column_len(),
    );
    let start = input
        .enumerate_row_major()
        .find(|(_, &tile)| tile == Tile::Start)
        .map(|((row, column), _)| UPoint::new(column as u32, row as u32))
        .unwrap();
    let end = input
        .enumerate_row_major()
        .find(|(_, &tile)| tile == Tile::End)
        .map(|((row, column), _)| UPoint::new(column as u32, row as u32))
        .unwrap();
    pending.push(Step::new(0, start, Direction::East));

    fn score_direction(score: &mut [u32; 4], direction: Direction) -> &mut u32 {
        match direction {
            Direction::North => score.get_mut(0).unwrap(),
            Direction::South => score.get_mut(1).unwrap(),
            Direction::East => score.get_mut(2).unwrap(),
            Direction::West => score.get_mut(3).unwrap(),
        }
    }

    while let Some(Step {
        score,
        position,
        direction,
    }) = pending.pop()
    {
        let current_score = maze_score
            .get_mut(position.y as usize, position.x as usize)
            .unwrap();

        if input[(position.y as usize, position.x as usize)] == Tile::End {
            break;
        }

        if let Some(next) = position.direction_checked(direction, &bounds) {
            if input[(next.y as usize, next.x as usize)] != Tile::Wall {
                let target = score_direction(current_score, direction);
                if *target > score + 1 {
                    pending.push(Step::new(score + 1, next, direction));
                    *target = score + 1;
                }
            }
        }
        let left = direction.left();
        if let Some(next) = position.direction_checked(left, &bounds) {
            if input[(next.y as usize, next.x as usize)] != Tile::Wall {
                let target = score_direction(current_score, left);
                if *target > score + 1001 {
                    pending.push(Step::new(score + 1001, next, left));
                    *target = score + 1001;
                }
            }
        }
        let right = direction.right();
        if let Some(next) = position.direction_checked(right, &bounds) {
            if input[(next.y as usize, next.x as usize)] != Tile::Wall {
                let target = score_direction(current_score, right);
                if *target > score + 1001 {
                    pending.push(Step::new(score + 1001, next, right));
                    *target = score + 1001;
                }
            }
        }
    }

    let mut pending = vec![(u32::MAX, end)];
    let mut seats = 0;
    let mut maze = input.clone();

    while let Some((score, position)) = pending.pop() {
        let tile = maze
            .get_mut(position.y as usize, position.x as usize)
            .unwrap();

        // Marking off things as walls to track visitation
        if *tile == Tile::Seat {
            continue;
        } else {
            *tile = Tile::Seat;
        }
        seats += 1;

        if let Some(next) = position.north_checked() {
            let next_score = *score_direction(
                &mut maze_score[(next.y as usize, next.x as usize)],
                Direction::South,
            );
            if next_score < score {
                pending.push((next_score, next));
            }
        }
        if let Some(next) = position.west_checked() {
            let next_score = *score_direction(
                &mut maze_score[(next.y as usize, next.x as usize)],
                Direction::East,
            );
            if next_score < score {
                pending.push((next_score, next));
            }
        }
        if let Some(next) = position.south_checked(&bounds) {
            let next_score = *score_direction(
                &mut maze_score[(next.y as usize, next.x as usize)],
                Direction::North,
            );
            if next_score < score {
                pending.push((next_score, next));
            }
        }
        if let Some(next) = position.east_checked(&bounds) {
            let next_score = *score_direction(
                &mut maze_score[(next.y as usize, next.x as usize)],
                Direction::West,
            );
            if next_score < score {
                pending.push((next_score, next));
            }
        }
    }

    seats
}
