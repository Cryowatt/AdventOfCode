use std::collections::{HashMap, HashSet};

use advent::*;

advent_day!(Day06, parse, Vec<Vec<u8>>, part1, part2);

pub fn parse(input: &str) -> InputType {
    input
        .lines()
        .map(|line| line.as_bytes().iter().cloned().collect())
        .collect()
}

/// ```rust
/// use advent_of_code_2024::day06::*;
/// let input = parse(
/// r"....#.....
/// .........#
/// ..........
/// ..#.......
/// .......#..
/// ..........
/// .#..^.....
/// ........#.
/// #.........
/// ......#...");
/// assert_eq!(41, part1(&input));
/// ```
pub fn part1(input: &InputType) -> usize {
    let bounds = Point::new(input.first().unwrap().len(), input.len());
    let start = find_start(input).unwrap();
    let mut path = HashSet::new();
    let mut pos = start;
    let mut dir = Direction::North;
    path.insert(start);

    while let Some(next) = pos.direction_checked(dir, &bounds) {
        match input[next.y][next.x] {
            b'#' => dir = dir.right(),
            _ => {
                pos = next;
                path.insert(pos);
            }
        }
    }

    path.len()
}

/// ```rust
/// use advent_of_code_2024::day06::*;
/// let input = parse(
/// r"....#.....
/// .........#
/// ..........
/// ..#.......
/// .......#..
/// ..........
/// .#..^.....
/// ........#.
/// #.........
/// ......#...");
/// assert_eq!(6, part2(&input));
/// ```
pub fn part2(input: &InputType) -> usize {
    let bounds = Point::new(input.first().unwrap().len(), input.len());
    let start = find_start(input).unwrap();
    let mut path = HashSet::new();
    let mut pos = start;
    let mut dir = Direction::North;
    path.insert(start);
    let mut loops = 0;

    while let Some(next) = pos.direction_checked(dir, &bounds) {
        match input[next.y][next.x] {
            b'#' => dir = dir.right(),
            _ => {
                if path.insert(next) {
                    // Haven't walked here, drop an obstruction down
                    if test_loop(input.clone(), bounds, pos, dir) {
                        loops += 1;
                    }
                }
                pos = next;
            }
        }
    }

    fn test_loop(
        mut map: Vec<Vec<u8>>,
        bounds: Point<usize>,
        mut pos: Point<usize>,
        mut dir: Direction,
    ) -> bool {
        let mut path = HashMap::<Point<usize>, Direction>::new();
        path.insert(pos, dir);
        let obstruction = pos.direction_checked(dir, &bounds).unwrap();
        map[obstruction.y][obstruction.x] = b'#';
        while let Some(next) = pos.direction_checked(dir, &bounds) {
            match map[next.y][next.x] {
                b'#' => dir = dir.right(),
                _ => {
                    if let Some(breadcrumb) = path.insert(next, dir) {
                        if breadcrumb == dir {
                            // Loop
                            return true;
                        }
                    }
                    pos = next;
                }
            }
        }

        false
    }

    loops
}

fn find_start(input: &InputType) -> Result<Point<usize>, ()> {
    for y in 0..input.len() {
        for x in 0..input.first().unwrap().len() {
            if input[y][x] == b'^' {
                return Ok(Point::new(x, y));
            }
        }
    }
    Err(())
}
