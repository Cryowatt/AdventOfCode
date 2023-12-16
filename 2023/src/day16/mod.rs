use bitflags::bitflags;

use advent::*;

advent_day!(Day16, parse, LaserChessBoard, part1, part2);

pub fn parse(input: &str) -> LaserChessBoard {
    let width = input.lines().nth(0).unwrap().len();
    let height = input.lines().count();

    let mut board = LaserChessBoard {
        rows: vec![],
        columns: vec![],
        width,
        height,
    };

    for x in 0..width {
        board.columns.push(vec![
            (Tile {
                location: IPoint::new(x as i32, -1),
                tile_type: TileType::Oblivion,
            }),
        ]);
    }

    for y in 0..height {
        board.rows.push(vec![
            (Tile {
                location: IPoint::new(-1, y as i32),
                tile_type: TileType::Oblivion,
            }),
        ]);
    }

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, cell)| {
            if let Some(tile_type) = match cell {
                '.' => None,
                '/' => Some(TileType::ForwardMirror),
                '\\' => Some(TileType::BackMirror),
                '-' => Some(TileType::HorizontalSplitter),
                '|' => Some(TileType::VerticalSplitter),
                _ => unreachable!(),
            } {
                board.rows[y].push(Tile {
                    location: IPoint::new(x as i32, y as i32),
                    tile_type,
                });
                board.columns[x].push(Tile {
                    location: IPoint::new(x as i32, y as i32),
                    tile_type,
                });
            };
        });

        board.rows[y].push(Tile {
            location: IPoint::new(width as i32, y as i32),
            tile_type: TileType::Oblivion,
        });
    });

    for x in 0..width {
        board.columns[x].push(Tile {
            location: IPoint::new(x as i32, height as i32 + 1),
            tile_type: TileType::Oblivion,
        });
    }

    board
}

pub struct LaserChessBoard {
    rows: Vec<Vec<Tile>>,
    columns: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

struct Tile {
    location: IPoint,
    tile_type: TileType,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct LaserDirection: u8 {
        const North = 0b00000001;
        const South = 0b00000010;
        const West =  0b00000100;
        const East =  0b00001000;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum TileType {
    ForwardMirror,
    BackMirror,
    HorizontalSplitter,
    VerticalSplitter,
    Oblivion,
}

/// ```rust
/// use advent_of_code_2023::day16::*;
/// let input = parse(
/// r".|...\....
/// |.-.\.....
/// .....|-...
/// ........|.
/// ..........
/// .........\
/// ..../.\\..
/// .-.-/..|..
/// .|....-|.\
/// ..//.|....");
/// assert_eq!(46, part1(&input));
/// ```
pub fn part1(input: &LaserChessBoard) -> u32 {
    let starting_laser = (IPoint::origin(), LaserDirection::East);

    energized_tiles(input, starting_laser)
}

fn energized_tiles(input: &LaserChessBoard, starting_laser: (Point<i32>, LaserDirection)) -> u32 {
    let mut beam_paths: Vec<Vec<LaserDirection>> = vec![];
    for _y in 0..input.height {
        beam_paths.push(vec![LaserDirection::empty(); input.width]);
    }

    let mut pending_lasers = vec![(starting_laser)];

    let queue_laser = |pending_lasers: &mut Vec<(IPoint, LaserDirection)>,
                       incident_laser: LaserDirection,
                       tile: &Tile| match tile.tile_type {
        TileType::Oblivion => {}
        TileType::ForwardMirror => match incident_laser {
            LaserDirection::North => {
                pending_lasers.push((tile.location.east().unwrap(), LaserDirection::East))
            }
            LaserDirection::East => {
                pending_lasers.push((tile.location.north().unwrap(), LaserDirection::North))
            }
            LaserDirection::West => {
                pending_lasers.push((tile.location.south().unwrap(), LaserDirection::South))
            }
            LaserDirection::South => {
                pending_lasers.push((tile.location.west().unwrap(), LaserDirection::West))
            }
            _ => unreachable!(),
        },
        TileType::BackMirror => match incident_laser {
            LaserDirection::North => {
                pending_lasers.push((tile.location.west().unwrap(), LaserDirection::West))
            }
            LaserDirection::East => {
                pending_lasers.push((tile.location.south().unwrap(), LaserDirection::South))
            }
            LaserDirection::West => {
                pending_lasers.push((tile.location.north().unwrap(), LaserDirection::North))
            }
            LaserDirection::South => {
                pending_lasers.push((tile.location.east().unwrap(), LaserDirection::East))
            }
            _ => unreachable!(),
        },
        TileType::HorizontalSplitter => match incident_laser {
            LaserDirection::North => {
                pending_lasers.push((tile.location.west().unwrap(), LaserDirection::West));
                pending_lasers.push((tile.location.east().unwrap(), LaserDirection::East));
            }
            LaserDirection::East => {
                pending_lasers.push((tile.location.east().unwrap(), LaserDirection::East))
            }
            LaserDirection::West => {
                pending_lasers.push((tile.location.west().unwrap(), LaserDirection::West))
            }
            LaserDirection::South => {
                pending_lasers.push((tile.location.west().unwrap(), LaserDirection::West));
                pending_lasers.push((tile.location.east().unwrap(), LaserDirection::East));
            }
            _ => unreachable!(),
        },
        TileType::VerticalSplitter => match incident_laser {
            LaserDirection::North => {
                pending_lasers.push((tile.location.north().unwrap(), LaserDirection::North))
            }
            LaserDirection::East => {
                pending_lasers.push((tile.location.north().unwrap(), LaserDirection::North));
                pending_lasers.push((tile.location.south().unwrap(), LaserDirection::South));
            }
            LaserDirection::West => {
                pending_lasers.push((tile.location.north().unwrap(), LaserDirection::North));
                pending_lasers.push((tile.location.south().unwrap(), LaserDirection::South));
            }
            LaserDirection::South => {
                pending_lasers.push((tile.location.south().unwrap(), LaserDirection::South))
            }
            _ => unreachable!(),
        },
    };

    while let Some((location, direction)) = pending_lasers.pop() {
        // Eliminate out-of-bounds lasers
        if location.x < 0
            || location.y < 0
            || location.x >= input.width as i32
            || location.y >= input.height as i32
        {
            continue;
        }

        // Loop detection
        let tile_beam_path = beam_paths[location.y as usize][location.x as usize];
        if tile_beam_path.contains(direction) {
            // Laser path followed, skip
            continue;
        }

        // find nearest target
        match direction {
            LaserDirection::East => {
                let target_index = input.rows[location.y as usize]
                    .binary_search_by(|tile| tile.location.x.cmp(&location.x))
                    .unwrap_or_else(|index| index);
                let target = input.rows[location.y as usize].get(target_index);

                let target_location = if let Some(tile) = target {
                    queue_laser(&mut pending_lasers, direction, tile);
                    tile.location.x.clamp(0, input.width as i32 - 1)
                } else {
                    input.width as i32 - 1
                };

                for x in location.x..=target_location {
                    beam_paths[location.y as usize][x as usize] =
                        beam_paths[location.y as usize][x as usize].union(direction);
                }
            }
            LaserDirection::West => {
                let target_index = input.rows[location.y as usize]
                    .binary_search_by(|tile| tile.location.x.cmp(&location.x))
                    .unwrap_or_else(|index| index - 1);
                let target = input.rows[location.y as usize].get(target_index);

                let target_location = if let Some(tile) = target {
                    queue_laser(&mut pending_lasers, direction, tile);
                    tile.location.x.clamp(0, input.width as i32 - 1)
                } else {
                    0
                };

                for x in target_location..=location.x {
                    beam_paths[location.y as usize][x as usize] =
                        beam_paths[location.y as usize][x as usize].union(direction);
                }
            }
            LaserDirection::South => {
                let target_index = input.columns[location.x as usize]
                    .binary_search_by(|tile| tile.location.y.cmp(&location.y))
                    .unwrap_or_else(|index| index);
                let target = input.columns[location.x as usize].get(target_index);

                let target_location = if let Some(tile) = target {
                    queue_laser(&mut pending_lasers, direction, tile);
                    tile.location.y.clamp(0, input.height as i32 - 1)
                } else {
                    input.height as i32 - 1
                };

                for y in location.y..=target_location {
                    beam_paths[y as usize][location.x as usize] =
                        beam_paths[y as usize][location.x as usize].union(direction);
                }
            }
            LaserDirection::North => {
                let target_index = input.columns[location.x as usize]
                    .binary_search_by(|tile| tile.location.y.cmp(&location.y))
                    .unwrap_or_else(|index| index - 1);
                let target = input.columns[location.x as usize].get(target_index);

                let target_location = if let Some(tile) = target {
                    queue_laser(&mut pending_lasers, direction, tile);
                    tile.location.y.clamp(0, input.height as i32 - 1)
                } else {
                    0
                };

                for y in target_location..=location.y {
                    beam_paths[y as usize][location.x as usize] =
                        beam_paths[y as usize][location.x as usize].union(direction);
                }
            }
            _ => unreachable!(),
        };
    }

    let mut beam_count = 0;

    for y in 0..input.height {
        for x in 0..input.width {
            if !beam_paths[y][x].is_empty() {
                beam_count += 1;
            }
        }
    }

    beam_count
}

/// ```rust
/// use advent_of_code_2023::day16::*;
/// let input = parse(
/// r".|...\....
/// |.-.\.....
/// .....|-...
/// ........|.
/// ..........
/// .........\
/// ..../.\\..
/// .-.-/..|..
/// .|....-|.\
/// ..//.|....");
/// assert_eq!(51, part2(&input));
/// ```
pub fn part2(input: &LaserChessBoard) -> u32 {
    let max_left_edge = (0..input.height as i32)
        .map(|y| energized_tiles(input, (IPoint::new(0, y), LaserDirection::East)))
        .max()
        .unwrap();

    let max_right_edge = (0..input.height as i32)
        .map(|y| {
            energized_tiles(
                input,
                (IPoint::new(input.width as i32 - 1, y), LaserDirection::West),
            )
        })
        .max()
        .unwrap();

    let max_top_edge = (0..input.width as i32)
        .map(|x| energized_tiles(input, (IPoint::new(x, 0), LaserDirection::South)))
        .max()
        .unwrap();

    let max_bottom_edge = (0..input.width as i32)
        .map(|x| {
            energized_tiles(
                input,
                (
                    IPoint::new(x, input.height as i32 - 1),
                    LaserDirection::North,
                ),
            )
        })
        .max()
        .unwrap();

    max_left_edge
        .max(max_right_edge)
        .max(max_top_edge)
        .max(max_bottom_edge)
}
