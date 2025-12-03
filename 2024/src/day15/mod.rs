use advent::*;
use array2d::Array2D;

advent_day!(Day15, parse, Warehouse, part1, part2);

#[derive(Debug)]
pub struct Warehouse {
    floorplan: Array2D<Tile>,
    moveset: Vec<Direction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Box,
    BoxL,
    BoxR,
    Robot,
    Wall,
    Empty,
}

pub fn parse(input: &str) -> InputType<'_> {
    let width = input.lines().next().unwrap().len();
    let floorplan = input.lines().take(width).flat_map(|line| {
        line.as_bytes().iter().map(|tile| match tile {
            b'#' => Tile::Wall,
            b'@' => Tile::Robot,
            b'O' => Tile::Box,
            _ => Tile::Empty,
        })
    });

    let movesets = input
        .lines()
        .skip(width + 1)
        .flat_map(|line| {
            line.as_bytes().iter().map(|dir| match dir {
                b'>' => Direction::East,
                b'^' => Direction::North,
                b'<' => Direction::West,
                b'v' => Direction::South,
                _ => panic!("At the disco"),
            })
        })
        .collect();

    Warehouse {
        floorplan: Array2D::from_iter_row_major(floorplan, width, width).unwrap(),
        moveset: movesets,
    }
}

/// ```rust
/// use advent_of_code_2024::day15::*;
/// let input = parse(
/// r"########
/// ##..O.O.#
/// ###@.O..#
/// ##...O..#
/// ##.#.O..#
/// ##...O..#
/// ##......#
/// #########
///
/// <^^>>>vv<v>>v<<");
/// assert_eq!(2028, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2024::day15::*;
/// let input = parse(
/// r"##########
/// ##..O..O.O#
/// ##......O.#
/// ##.OO..O.O#
/// ##..O@..O.#
/// ##O#..O...#
/// ##O..O..O.#
/// ##.OO.O.OO#
/// ##....O...#
/// ###########
///
/// <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
/// vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
/// ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
/// <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
/// ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
/// ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
/// >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
/// <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
/// ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
/// v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^");
/// assert_eq!(10092, part1(&input));
/// ```
pub fn part1(input: &InputType) -> usize {
    let ((row, col), _) = input
        .floorplan
        .enumerate_row_major()
        .find(|(_, &tile)| tile == Tile::Robot)
        .unwrap();
    let mut robot = UPoint::new(col as u32, row as u32);
    let mut warehouse = input.floorplan.clone();
    warehouse[(robot.y as usize, robot.x as usize)] = Tile::Empty;
    let bounds = UPoint::new(warehouse.column_len() as u32, warehouse.row_len() as u32);

    fn push_tile(
        warehouse: &mut Array2D<Tile>,
        bounds: &UPoint,
        position: UPoint,
        direction: Direction,
    ) -> Result<UPoint, ()> {
        if let Some(target) = position.direction_checked(direction, bounds) {
            match warehouse[(target.y as usize, target.x as usize)] {
                Tile::Empty => {
                    swap(warehouse, position, target);
                    Ok(target)
                }
                Tile::Box => {
                    if let Ok(_) = push_tile(warehouse, bounds, target, direction) {
                        swap(warehouse, position, target);
                        Ok(target)
                    } else {
                        Err(())
                    }
                }
                Tile::Wall => Err(()),
                _ => unreachable!(),
            }
        } else {
            Err(())
        }
    }

    for command in input.moveset.iter() {
        if let Ok(position) = push_tile(&mut warehouse, &bounds, robot, *command) {
            robot = position;
        }
    }

    gps(warehouse)
}

/// ```rust
/// use advent_of_code_2024::day15::*;
/// let input = parse(
/// r"##########
/// ##..O..O.O#
/// ##......O.#
/// ##.OO..O.O#
/// ##..O@..O.#
/// ##O#..O...#
/// ##O..O..O.#
/// ##.OO.O.OO#
/// ##....O...#
/// ###########
///
/// <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
/// vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
/// ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
/// <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
/// ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
/// ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
/// >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
/// <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
/// ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
/// v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^");
/// assert_eq!(9021, part2(&input));
/// ```
pub fn part2(input: &InputType) -> usize {
    let bounds = UPoint::new(
        (input.floorplan.column_len() * 2) as u32,
        input.floorplan.row_len() as u32,
    );

    // Enthiccen
    let thicc = input
        .floorplan
        .elements_row_major_iter()
        .flat_map(|tile| match tile {
            Tile::Box => [Tile::BoxL, Tile::BoxR],
            Tile::Robot => [Tile::Robot, Tile::Empty],
            Tile::Wall => [Tile::Wall, Tile::Wall],
            _ => [Tile::Empty, Tile::Empty],
        });
    let mut warehouse =
        Array2D::from_iter_row_major(thicc, bounds.y as usize, bounds.x as usize).unwrap();
    let ((row, col), _) = warehouse
        .enumerate_row_major()
        .find(|(_, &tile)| tile == Tile::Robot)
        .unwrap();
    warehouse[(row, col)] = Tile::Empty;
    let mut robot = UPoint::new(col as u32, row as u32);

    fn can_push_vertical(
        warehouse: &mut Array2D<Tile>,
        bounds: &UPoint,
        position: UPoint,
        direction: Direction,
    ) -> bool {
        if let Some(target) = position.direction_checked(direction, bounds) {
            match warehouse[(target.y as usize, target.x as usize)] {
                Tile::Empty => true,
                Tile::BoxL => {
                    can_push_vertical(warehouse, bounds, target, direction)
                        && can_push_vertical(
                            warehouse,
                            bounds,
                            UPoint::new(target.x + 1, target.y),
                            direction,
                        )
                }
                Tile::BoxR => {
                    can_push_vertical(warehouse, bounds, target, direction)
                        && can_push_vertical(
                            warehouse,
                            bounds,
                            UPoint::new(target.x - 1, target.y),
                            direction,
                        )
                }
                Tile::Wall => false,
                _ => unreachable!(),
            }
        } else {
            false
        }
    }

    fn push_tile_vertical(
        warehouse: &mut Array2D<Tile>,
        bounds: &UPoint,
        position: UPoint,
        direction: Direction,
    ) -> Result<UPoint, ()> {
        if let Some(target) = position.direction_checked(direction, bounds) {
            match warehouse[(target.y as usize, target.x as usize)] {
                Tile::Empty => {
                    swap(warehouse, position, target);
                    Ok(target)
                }
                Tile::BoxL => {
                    if push_tile_vertical(warehouse, bounds, target, direction).is_ok()
                        && push_tile_vertical(
                            warehouse,
                            bounds,
                            UPoint::new(target.x + 1, target.y),
                            direction,
                        )
                        .is_ok()
                    {
                        swap(warehouse, position, target);
                        Ok(target)
                    } else {
                        Err(())
                    }
                }
                Tile::BoxR => {
                    if push_tile_vertical(warehouse, bounds, target, direction).is_ok()
                        && push_tile_vertical(
                            warehouse,
                            bounds,
                            UPoint::new(target.x - 1, target.y),
                            direction,
                        )
                        .is_ok()
                    {
                        swap(warehouse, position, target);
                        Ok(target)
                    } else {
                        Err(())
                    }
                }
                Tile::Wall => Err(()),
                _ => unreachable!(),
            }
        } else {
            Err(())
        }
    }

    fn push_tile_horizontal(
        warehouse: &mut Array2D<Tile>,
        bounds: &UPoint,
        position: UPoint,
        direction: Direction,
    ) -> Result<UPoint, ()> {
        if let Some(target) = position.direction_checked(direction, bounds) {
            match warehouse[(target.y as usize, target.x as usize)] {
                Tile::Empty => {
                    swap(warehouse, position, target);
                    Ok(target)
                }
                Tile::BoxL | Tile::BoxR => {
                    if let Ok(_) = push_tile_horizontal(warehouse, bounds, target, direction) {
                        swap(warehouse, position, target);
                        Ok(target)
                    } else {
                        Err(())
                    }
                }
                Tile::Wall => Err(()),
                _ => panic!("FK"),
            }
        } else {
            Err(())
        }
    }

    for command in input.moveset.iter() {
        if let Ok(position) = match command {
            Direction::North | Direction::South => {
                if can_push_vertical(&mut warehouse, &bounds, robot, *command) {
                    push_tile_vertical(&mut warehouse, &bounds, robot, *command)
                } else {
                    Err(())
                }
            }
            Direction::East | Direction::West => {
                push_tile_horizontal(&mut warehouse, &bounds, robot, *command)
            }
        } {
            robot = position;
        }
    }

    gps(warehouse)
}

fn gps(warehouse: Array2D<Tile>) -> usize {
    warehouse
        .enumerate_row_major()
        .filter_map(|((row, col), tile)| match tile {
            Tile::Box | Tile::BoxL => Some(row * 100 + col),
            _ => None,
        })
        .sum()
}

fn swap(warehouse: &mut Array2D<Tile>, position: Point<u32>, target: Point<u32>) {
    let current = warehouse[(position.y as usize, position.x as usize)];
    warehouse[(target.y as usize, target.x as usize)] = current;
    warehouse[(position.y as usize, position.x as usize)] = Tile::Empty;
}
