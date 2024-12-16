use std::collections::BinaryHeap;

use advent::*;
use array2d::Array2D;

advent_day!(Day16, parse, Array2D<Tile>, part1, part2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
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
    let end = input
        .enumerate_row_major()
        .find(|(_, &tile)| tile == Tile::End)
        .map(|((row, column), _)| UPoint::new(column as u32, row as u32))
        .unwrap();
    pending.push(Step::new(0, start, Direction::East));

    while let Some(Step {
        score,
        position,
        direction,
    }) = pending.pop()
    {
        // println!("{:?}", Step::new(score, position, direction));
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
/// assert_eq!(0, part2(&input));
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
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &InputType) -> usize {
    0
}
