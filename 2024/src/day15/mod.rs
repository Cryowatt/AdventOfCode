use std::process::Command;

use advent::*;
use array2d::Array2D;
use onig::EncodedChars;
use rayon::iter::Positions;

advent_day!(Day15, parse, Warehouse, part1, part2);

#[derive(Debug)]
pub struct Warehouse {
    floorplan: Array2D<Tile>,
    moveset: Vec<Direction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Robot,
    Empty,
    Box,
}

pub fn parse(input: &str) -> InputType {
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
        .find(|((row, col), &tile)| tile == Tile::Robot)
        .unwrap();
    let mut robot = UPoint::new(col as u32, row as u32);
    println!("Robot at {:?}", robot);
    let mut warehouse = input.floorplan.clone();
    let bounds = UPoint::new(warehouse.column_len() as u32, warehouse.row_len() as u32);

    for command in input.moveset.iter() {
        if let Ok(position) = push_tile(&mut warehouse, &bounds, robot, *command) {
            robot = position;
        }
        print_warehouse(&warehouse);
    }

    fn print_warehouse(warehouse: &Array2D<Tile>) {
        for row in warehouse.as_rows() {
            for tile in row {
                let c = match tile {
                    Tile::Wall => '#',
                    Tile::Robot => '@',
                    Tile::Empty => '.',
                    Tile::Box => 'O',
                };
                print!("{}", c);
            }
            println!();
        }
    }

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

    fn gps(warehouse: Array2D<Tile>) -> usize {
        warehouse
            .enumerate_row_major()
            .filter_map(|((row, col), tile)| match tile {
                Tile::Box => Some(row * 100 + col),
                _ => None,
            })
            .sum()
    }

    gps(warehouse)
}

fn swap(warehouse: &mut Array2D<Tile>, position: Point<u32>, target: Point<u32>) {
    let current = warehouse[(position.y as usize, position.x as usize)];
    warehouse[(target.y as usize, target.x as usize)] = current;
    warehouse[(position.y as usize, position.x as usize)] = Tile::Empty;
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
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &InputType) -> i64 {
    0
}
