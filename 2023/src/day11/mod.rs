use std::{
    cell::{RefCell, RefMut},
    ops::Deref,
    rc::Rc,
};

use advent::*;

advent_day!(Day11, parse, PipeMap, part1, part2);

pub fn parse(input: &str) -> GalaxyMap {
    let galaxy = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes().enumerate().filter_map(move |(x, char)| {
                if char == b'#' {
                    Some(UPoint::new(x as u32, y as u32))
                } else {
                    None
                }
            })
        })
        .collect();
    GalaxyMap { galaxy }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GalaxyMap {
    galaxy: Vec<UPoint>,
}

/// ```rust
/// use advent_of_code_2023::day11::*;
/// let input = parse(
/// r"...#......
/// .......#..
/// #.........
/// ..........
/// ......#...
/// .#........
/// .........#
/// ..........
/// .......#..
/// #...#.....");
/// assert_eq!(374, part1(&input));
/// ```
pub fn part1(map: &GalaxyMap) -> u64 {
    expand_universe(map, 2)
}

/// ```rust
/// use advent_of_code_2023::day11::*;
/// let input = parse(
/// r"...#......
/// .......#..
/// #.........
/// ..........
/// ......#...
/// .#........
/// .........#
/// ..........
/// .......#..
/// #...#.....");
/// assert_eq!(1030, expand_universe(&input, 10));
/// ```
/// ```rust
/// use advent_of_code_2023::day11::*;
/// let input = parse(
/// r"...#......
/// .......#..
/// #.........
/// ..........
/// ......#...
/// .#........
/// .........#
/// ..........
/// .......#..
/// #...#.....");
/// assert_eq!(8410, expand_universe(&input, 100));
/// ```
pub fn expand_universe(map: &GalaxyMap, expansion_rate: u32) -> u64 {
    let expansion_rate = expansion_rate - 1;
    let galaxies: Vec<_> = map
        .galaxy
        .iter()
        .map(|star| Rc::new(RefCell::new(*star)))
        .collect();
    let mut row_map = vec![];
    let mut col_map = vec![];

    // map galaxy locations to col/row
    for galaxy in galaxies.iter() {
        let x = galaxy.borrow().x as usize;
        let y = galaxy.borrow().y as usize;

        fn map_stripe(
            map: &mut Vec<Vec<Rc<RefCell<UPoint>>>>,
            i: usize,
            galaxy: Rc<RefCell<UPoint>>,
        ) {
            if map.len() <= i as usize {
                map.resize_with(i + 1, || vec![]);
            }

            map[i].push(galaxy);
        }

        map_stripe(&mut col_map, x, galaxy.clone());
        map_stripe(&mut row_map, y, galaxy.clone());
    }

    fn expand_universe<F: Fn(RefMut<'_, UPoint>, u32)>(
        map: &Vec<Vec<Rc<RefCell<UPoint>>>>,
        axis: F,
        expansion_rate: u32,
    ) {
        map.iter().fold(0, |expansion, stripe| {
            stripe.iter().for_each(|galaxy| {
                axis(galaxy.as_ref().borrow_mut(), expansion);
            });

            if stripe.is_empty() {
                expansion + expansion_rate
            } else {
                expansion
            }
        });
    }

    // expand the universe horizontally
    expand_universe(
        &col_map,
        |mut galaxy, expansion| galaxy.x += expansion,
        expansion_rate,
    );
    // expand the universe horizontally
    expand_universe(
        &row_map,
        |mut galaxy, expansion| galaxy.y += expansion,
        expansion_rate,
    );

    galaxies
        .iter()
        .enumerate()
        .map(|(i, galaxy)| {
            let galaxy = galaxy.borrow();
            galaxies
                .iter()
                .skip(i + 1)
                .map(|to_galaxy| galaxy.manhattan(to_galaxy.borrow().deref()) as u64)
                .sum::<u64>()
        })
        .sum()
}

pub fn part2(map: &GalaxyMap) -> u64 {
    expand_universe(map, 1_000_000)
}
