use bitflags::bitflags;

use advent::*;

advent_day!(Day10, parse, PipeMap, part1, part2);

pub fn parse(input: &str) -> PipeMap {
    PipeMap {
        rows: input
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|cell| match cell {
                        b'|' => PipeEnds::North.union(PipeEnds::South),
                        b'-' => PipeEnds::East.union(PipeEnds::West),
                        b'L' => PipeEnds::North.union(PipeEnds::East),
                        b'J' => PipeEnds::North.union(PipeEnds::West),
                        b'7' => PipeEnds::South.union(PipeEnds::West),
                        b'F' => PipeEnds::South.union(PipeEnds::East),
                        b'.' => PipeEnds::empty(),
                        b'S' => PipeEnds::Start,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
        bounds: UPoint::new(
            input.lines().next().unwrap().len() as u32,
            input.lines().count() as u32,
        ),
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PipeEnds: u8 {
        const North = 0b00000001;
        const South = 0b00000010;
        const West =  0b00000100;
        const East =  0b00001000;
        const Start = 0b10010000;
        const Visited = 0b10000000;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PipeMap {
    rows: Vec<Vec<PipeEnds>>,
    bounds: UPoint,
}

impl PipeMap {
    fn cell(&self, position: UPoint) -> PipeEnds {
        self.rows[position.y as usize][position.x as usize]
    }

    fn mark_cell(&mut self, position: UPoint) -> PipeEnds {
        let pipe_ends = self.rows[position.y as usize][position.x as usize];
        self.rows[position.y as usize][position.x as usize] = pipe_ends | PipeEnds::Visited;
        pipe_ends
    }
}

/// ```rust
/// use advent_of_code_2023::day10::*;
/// let input = parse(
/// r".....
/// .S-7.
/// .|.|.
/// .L-J.
/// .....");
/// assert_eq!(4, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day10::*;
/// let input = parse(
/// r"7-F7-
/// .FJ|7
/// SJLL7
/// |F--J
/// LJ.LJ");
/// assert_eq!(8, part1(&input));
/// ```
pub fn part1(map: &PipeMap) -> u32 {
    // Find start
    let start = map
        .rows
        .iter()
        .enumerate()
        .filter_map(|row| {
            match row
                .1
                .iter()
                .enumerate()
                .filter_map(|cell| {
                    if *cell.1 == PipeEnds::Start {
                        Some(cell.0)
                    } else {
                        None
                    }
                })
                .next()
            {
                Some(x) => Some(UPoint::new(x as u32, row.0 as u32)),
                None => None,
            }
        })
        .next()
        .unwrap();

    let check_direction = |point: Option<UPoint>, from_direction: PipeEnds| -> bool {
        point.map_or(false, |up| map.cell(up).contains(from_direction))
    };

    // Check around the start position for the first step
    let mut position = if check_direction(start.north_checked(), PipeEnds::South) {
        (PipeEnds::South, start.north_checked().unwrap())
    } else if check_direction(start.west_checked(), PipeEnds::East) {
        (PipeEnds::East, start.west_checked().unwrap())
    } else if check_direction(start.east_checked(&map.bounds), PipeEnds::West) {
        (PipeEnds::West, start.east_checked(&map.bounds).unwrap())
    } else if check_direction(start.south_checked(&map.bounds), PipeEnds::North) {
        (PipeEnds::North, start.south_checked(&map.bounds).unwrap())
    } else {
        unreachable!();
    };

    let mut distance = 1;

    // Walk the route
    while position.1 != start {
        let cell = map.cell(position.1);
        position = match cell.difference(position.0) {
            PipeEnds::North => (PipeEnds::South, position.1.north_checked().unwrap()),
            PipeEnds::South => (
                PipeEnds::North,
                position.1.south_checked(&map.bounds).unwrap(),
            ),
            PipeEnds::East => (
                PipeEnds::West,
                position.1.east_checked(&map.bounds).unwrap(),
            ),
            PipeEnds::West => (PipeEnds::East, position.1.west_checked().unwrap()),
            _ => unreachable!(),
        };
        distance += 1;
    }

    distance / 2
}

/// ```rust
/// use advent_of_code_2023::day10::*;
/// let input = parse(
/// r"...........
/// .S-------7.
/// .|F-----7|.
/// .||.....||.
/// .||.....||.
/// .|L-7.F-J|.
/// .|..|.|..|.
/// .L--J.L--J.
/// ...........");
/// assert_eq!(4, part2(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day10::*;
/// let input = parse(
/// r".F----7F7F7F7F-7....
/// .|F--7||||||||FJ....
/// .||.FJ||||||||L7....
/// FJL7L7LJLJ||LJ.L-7..
/// L--J.L7...LJS7F-7L7.
/// ....F-J..F7FJ|L7L7L7
/// ....L7.F7||L7|.L7L7|
/// .....|FJLJ|FJ|F7|.LJ
/// ....FJL-7.||.||||...
/// ....L---J.LJ.LJLJ...");
/// assert_eq!(8, part2(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day10::*;
/// let input = parse(
/// r"FF7FSF7F7F7F7F7F---7
/// L|LJ||||||||||||F--J
/// FL-7LJLJ||||||LJL-77
/// F--JF--7||LJLJ7F7FJ-
/// L---JF-JLJ.||-FJLJJ7
/// |F|F-JF---7F7-L7L|7|
/// |FFJF7L7F-JF7|JL---7
/// 7-L-JL7||F7|L7F-7F7|
/// L.L7LFJ|||||FJL7||LJ
/// L7JLJL-JLJLJL--JLJ.L");
/// assert_eq!(10, part2(&input));
/// ```
pub fn part2(map: &PipeMap) -> u32 {
    let mut map = (*map).clone();

    // Find start
    let start = map
        .rows
        .iter()
        .enumerate()
        .filter_map(|row| {
            match row
                .1
                .iter()
                .enumerate()
                .filter_map(|cell| {
                    if *cell.1 == PipeEnds::Start {
                        Some(cell.0)
                    } else {
                        None
                    }
                })
                .next()
            {
                Some(x) => Some(UPoint::new(x as u32, row.0 as u32)),
                None => None,
            }
        })
        .next()
        .unwrap();

    let check_direction = |point: Option<UPoint>, from_direction: PipeEnds| -> bool {
        point.map_or(false, |up| map.cell(up).contains(from_direction))
    };

    // Check around the start position for the first step
    let mut position = if check_direction(start.north_checked(), PipeEnds::South) {
        map.rows[start.y as usize][start.x as usize] |= PipeEnds::South;
        (PipeEnds::South, start.north_checked().unwrap())
    } else if check_direction(start.west_checked(), PipeEnds::East) {
        map.rows[start.y as usize][start.x as usize] |= PipeEnds::East;
        (PipeEnds::East, start.west_checked().unwrap())
    } else if check_direction(start.east_checked(&map.bounds), PipeEnds::West) {
        map.rows[start.y as usize][start.x as usize] |= PipeEnds::West;
        (PipeEnds::West, start.east_checked(&map.bounds).unwrap())
    } else if check_direction(start.south_checked(&map.bounds), PipeEnds::North) {
        map.rows[start.y as usize][start.x as usize] |= PipeEnds::North;
        (PipeEnds::North, start.south_checked(&map.bounds).unwrap())
    } else {
        unreachable!();
    };

    // Walk the route
    while position.1 != start {
        let cell = map.mark_cell(position.1);
        position = match cell.difference(position.0) {
            PipeEnds::North => (PipeEnds::South, position.1.north_checked().unwrap()),
            PipeEnds::South => (
                PipeEnds::North,
                position.1.south_checked(&map.bounds).unwrap(),
            ),
            PipeEnds::East => (
                PipeEnds::West,
                position.1.east_checked(&map.bounds).unwrap(),
            ),
            PipeEnds::West => (PipeEnds::East, position.1.west_checked().unwrap()),
            _ => unreachable!(),
        };
    }

    map.rows[position.1.y as usize][position.1.x as usize] |= position.0;

    let mut inside = 0;

    for row in map.rows {
        let mut loop_orientation = PipeEnds::empty();
        let mut is_inside = false;
        for cell in row {
            if cell.contains(PipeEnds::Visited) {
                const NORTH_SOUTH: PipeEnds = PipeEnds::North.union(PipeEnds::South);
                let path_orientation = cell.intersection(NORTH_SOUTH);

                if !is_inside {
                    loop_orientation = path_orientation;
                    is_inside = true;
                } else {
                    loop_orientation ^= path_orientation;
                    if loop_orientation.is_empty() {
                        is_inside = false;
                    }
                }
            } else {
                if is_inside {
                    inside += 1;
                }
            }
        }
    }

    inside
}
