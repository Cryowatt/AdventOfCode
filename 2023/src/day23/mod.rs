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

struct Map {
    tiles: Vec<Vec<Tile>>,
    start: UPoint,
    end: UPoint,
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
    Map { tiles, start, end }
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
/// assert_eq!(5, part1(&input));
/// ```
pub fn part1(map: &Map) -> u64 {
    todo!()
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
