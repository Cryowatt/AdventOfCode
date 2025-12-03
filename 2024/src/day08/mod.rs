use std::collections::{HashMap, HashSet};

use advent::*;

advent_day!(Day08, parse, Vec<Vec<u8>>, part1, part2);

pub fn parse(input: &str) -> InputType<'_> {
    input
        .lines()
        .map(|line| line.as_bytes().iter().cloned().collect())
        .collect()
}

/// ```rust
/// use advent_of_code_2024::day08::*;
/// let input = parse(
/// r"............
/// ........0...
/// .....0......
/// .......0....
/// ....0.......
/// ......A.....
/// ............
/// ............
/// ........A...
/// .........A..
/// ............
/// ............");
/// assert_eq!(14, part1(&input));
/// ```
pub fn part1(input: &InputType) -> usize {
    let bounds = Point::new(input.first().unwrap().len() as i32, input.len() as i32);
    let mut antinodes = HashSet::new();
    let mut frequency_antenna = HashMap::new();
    for (f, node) in input.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter_map(move |(x, cell)| {
            if *cell == b'.' {
                None
            } else {
                Some((*cell, Point::new(x as i32, y as i32)))
            }
        })
    }) {
        frequency_antenna
            .entry(f)
            .and_modify(|peers: &mut Vec<Point<i32>>| {
                for &peer in peers.iter() {
                    let distance = node - peer;
                    add_node(&mut antinodes, node + distance, bounds);
                    add_node(&mut antinodes, peer - distance, bounds);
                }
                peers.push(node);
            })
            .or_insert_with(|| vec![node]);
    }

    antinodes.len()
}

/// ```rust
/// use advent_of_code_2024::day08::*;
/// let input = parse(
/// r"............
/// ........0...
/// .....0......
/// .......0....
/// ....0.......
/// ......A.....
/// ............
/// ............
/// ........A...
/// .........A..
/// ............
/// ............");
/// assert_eq!(34, part2(&input));
/// ```
pub fn part2(input: &InputType) -> usize {
    let bounds = Point::new(input.first().unwrap().len() as i32, input.len() as i32);
    let mut antinodes = HashSet::new();
    let mut frequency_antenna = HashMap::new();
    for (f, node) in input.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter_map(move |(x, cell)| {
            if *cell == b'.' {
                None
            } else {
                Some((*cell, Point::new(x as i32, y as i32)))
            }
        })
    }) {
        frequency_antenna
            .entry(f)
            .and_modify(|peers: &mut Vec<Point<i32>>| {
                for &peer in peers.iter() {
                    add_node(&mut antinodes, node, bounds);
                    add_node(&mut antinodes, peer, bounds);
                    let distance = node - peer;
                    let mut harmonic = node + distance;
                    while in_bounds(&harmonic, &bounds) {
                        add_node(&mut antinodes, harmonic, bounds);
                        harmonic = harmonic + distance;
                    }
                    let mut harmonic = peer - distance;
                    while in_bounds(&harmonic, &bounds) {
                        add_node(&mut antinodes, harmonic, bounds);
                        harmonic = harmonic - distance;
                    }
                }
                peers.push(node);
            })
            .or_insert_with(|| vec![node]);
    }

    fn in_bounds(node: &Point<i32>, bounds: &Point<i32>) -> bool {
        node.x >= 0 && node.y >= 0 && node.x < bounds.x && node.y < bounds.y
    }

    antinodes.len()
}

fn add_node(nodes: &mut HashSet<Point<i32>>, node: Point<i32>, bounds: Point<i32>) -> bool {
    if node.x >= 0 && node.y >= 0 && node.x < bounds.x && node.y < bounds.y {
        nodes.insert(node)
    } else {
        false
    }
}
