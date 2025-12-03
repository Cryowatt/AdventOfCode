use std::{
    collections::{HashMap, HashSet},
    sync::atomic::{self, AtomicUsize},
};

use advent::*;

advent_day!(Day06, parse, Vec<Vec<u8>>, part1, part2);

pub fn parse(input: &str) -> InputType<'_> {
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
    let mut path = HashMap::new();
    let mut pos = start;
    let loops = AtomicUsize::new(0);

    rayon::scope(|s| {
        let loops = &loops;
        let mut dir = Direction::North;
        path.insert(start, dir);
        while let Some(next) = pos.direction_checked(dir, &bounds) {
            match input[next.y][next.x] {
                b'#' => dir = dir.right(),
                _ => {
                    if let None = path.insert(next, dir) {
                        // Haven't walked here, drop an obstruction down
                        let mut current_path = path.clone();
                        s.spawn(move |_| {
                            if test_loop(input, bounds, next, pos, dir, &mut current_path) {
                                loops.fetch_add(1, atomic::Ordering::Relaxed);
                            }
                        });
                    }
                    pos = next;
                }
            }
        }
    });

    fn test_loop(
        map: &Vec<Vec<u8>>,
        bounds: Point<usize>,
        obstruction: Point<usize>,
        mut pos: Point<usize>,
        mut dir: Direction,
        path: &mut HashMap<Point<usize>, Direction>,
    ) -> bool {
        while let Some(next) = pos.direction_checked(dir, &bounds) {
            if next == obstruction {
                dir = dir.right()
            } else {
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
        }

        false
    }

    loops.load(atomic::Ordering::Relaxed)
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
