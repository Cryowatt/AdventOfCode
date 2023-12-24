use std::collections::{HashMap, HashSet, VecDeque};

use advent::*;

advent_day!(Day23, parse, Map, part1, part2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Path,
    Forest,
    North,
    East,
    West,
    South,
}

pub struct Map {
    tiles: Vec<Vec<Tile>>,
    start: UPoint,
    end: UPoint,
    bounds: UPoint,
}

pub fn parse(input: &str) -> Map {
    let tiles: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|tile| match tile {
                    b'#' => Tile::Forest,
                    b'.' => Tile::Path,
                    b'^' => Tile::North,
                    b'>' => Tile::East,
                    b'v' => Tile::South,
                    b'<' => Tile::West,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    let start = tiles
        .first()
        .unwrap()
        .iter()
        .enumerate()
        .find_map(|(x, tile)| match tile {
            Tile::Path => Some(UPoint::new(x as u32, 0)),
            _ => None,
        })
        .unwrap();
    let end = tiles
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .find_map(|(x, tile)| match tile {
            Tile::Path => Some(UPoint::new(x as u32, tiles.len() as u32 - 1)),
            _ => None,
        })
        .unwrap();
    let bounds = UPoint::new(tiles.first().unwrap().len() as u32, tiles.len() as u32);

    Map {
        tiles,
        start,
        end,
        bounds,
    }
}

/// ```rust
/// use advent_of_code_2023::day23::*;
/// let input = parse(
/// r"#.#####################
///##.......#########...###
///########.#########.#.###
///####.....#.>.>.###.#.###
///####v#####.#v#.###.#.###
///####.>...#.#.#.....#...#
///####v###.#.#.#########.#
///####...#.#.#.......#...#
///######.#.#.#######.#.###
///##.....#.#.#.......#...#
///##.#####.#.#.#########v#
///##.#...#...#...###...>.#
///##.#.#v#######v###.###v#
///##...#.>.#...>.>.#.###.#
///######v#.#.###v#.#.###.#
///##.....#...#...#.#.#...#
///##.#########.###.#.#.###
///##...###...#...#...#.###
///####.###.#.###v#####v###
///##...#...#.#.>.>.#.>.###
///##.###.###.#.###.#.#v###
///##.....###...###...#...#
///######################.#");
/// assert_eq!(94, part1(&input));
/// ```
pub fn part1(map: &Map) -> u64 {
    let mut nodes = HashMap::new();
    let mut edges: HashMap<Point<u32>, Vec<Point<u32>>> = HashMap::new();

    // Trace path to the fork
    let mut open_paths = VecDeque::from([(map.start, Direction::South)]);
    let mut closed_paths = HashSet::new();

    while let Some((start_position, start_direction)) = open_paths.pop_front() {
        if closed_paths.contains(&start_position) {
            continue;
        }

        let mut current = Some((start_position, start_direction));
        let mut path_length = 1;

        while let Some((position, direction)) = current {
            let valid_directions = [direction, direction.left(), direction.right()];
            current = valid_directions
                .into_iter()
                .filter_map(|next_direction| {
                    position
                        .direction_checked(next_direction, &map.bounds)
                        .map(|valid_point| (valid_point, next_direction))
                })
                .find_map(|(valid_point, next_direction)| {
                    if valid_point == map.end {
                        closed_paths.insert(valid_point);
                        nodes.insert(start_position, path_length);
                        None
                    } else {
                        match map.tiles[valid_point.y as usize][valid_point.x as usize] {
                            Tile::Path => {
                                // Found a path, keep walking
                                path_length += 1;
                                Some((valid_point, next_direction))
                            }
                            Tile::Forest => {
                                // Ouch, a tree
                                None
                            }
                            _ => {
                                closed_paths.insert(start_position);
                                path_length += 2;
                                nodes.insert(start_position, path_length);
                                let end_point = valid_point
                                    .direction_checked(next_direction, &map.bounds)
                                    .unwrap();

                                [
                                    next_direction,
                                    next_direction.left(),
                                    next_direction.right(),
                                ]
                                .into_iter()
                                .filter_map(|start_direction| {
                                    if let Some(valid_start) =
                                        end_point.direction_checked(start_direction, &map.bounds)
                                    {
                                        let valid_slope = match map.tiles[valid_start.y as usize]
                                            [valid_start.x as usize]
                                        {
                                            Tile::North => start_direction == Direction::North,
                                            Tile::East => start_direction == Direction::East,
                                            Tile::West => start_direction == Direction::West,
                                            Tile::South => start_direction == Direction::South,
                                            _ => false,
                                        };

                                        if valid_slope {
                                            if let Some(edge_list) = edges.get_mut(&start_position)
                                            {
                                                edge_list.push(valid_start);
                                            } else {
                                                edges.insert(start_position, vec![valid_start]);
                                            }
                                            Some((valid_start, start_direction))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                })
                                .for_each(|start| {
                                    if !closed_paths.contains(&start.0) {
                                        open_paths.push_back(start)
                                    }
                                });

                                None
                            }
                        }
                    }
                });
        }
    }

    let mut longest_path = HashMap::new();
    let mut unvisited = VecDeque::from([(map.start, 0)]);

    while let Some((node, distance)) = unvisited.pop_front() {
        let distance = distance + nodes[&node];

        if let Some(edge_list) = edges.get(&node) {
            for next_path in edge_list.into_iter() {
                if let Some(longest_distance) = longest_path.get_mut(next_path) {
                    if distance > *longest_distance {
                        *longest_distance = distance;
                        unvisited.push_back((*next_path, distance));
                    }
                } else {
                    longest_path.insert(*next_path, distance);
                    unvisited.push_back((*next_path, distance));
                }
            }
        } else {
            if let Some(longest_distance) = longest_path.get_mut(&map.end) {
                if distance > *longest_distance {
                    *longest_distance = distance;
                }
            } else {
                longest_path.insert(map.end, distance);
            }
        }
    }

    longest_path.get(&map.end).unwrap().to_owned() as u64
}

/// ```rust
/// use advent_of_code_2023::day23::*;
/// let input = parse(
/// r"#.#####################
///##.......#########...###
///########.#########.#.###
///####.....#.>.>.###.#.###
///####v#####.#v#.###.#.###
///####.>...#.#.#.....#...#
///####v###.#.#.#########.#
///####...#.#.#.......#...#
///######.#.#.#######.#.###
///##.....#.#.#.......#...#
///##.#####.#.#.#########v#
///##.#...#...#...###...>.#
///##.#.#v#######v###.###v#
///##...#.>.#...>.>.#.###.#
///######v#.#.###v#.#.###.#
///##.....#...#...#.#.#...#
///##.#########.###.#.#.###
///##...###...#...#...#.###
///####.###.#.###v#####v###
///##...#...#.#.>.>.#.>.###
///##.###.###.#.###.#.#v###
///##.....###...###...#...#
///######################.#");
/// //assert_eq!(?, part2(&input));
/// ```
pub fn part2(map: &Map) -> u64 {
    todo!()
}
