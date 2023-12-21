use advent::*;

advent_day!(Day21, parse, GardenMap, part1, part2);

pub fn parse(input: &str) -> GardenMap {
    GardenMap {
        map: input
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|plot| match plot {
                        b'.' => GardenPlot::Garden,
                        b'#' => GardenPlot::Rock,
                        b'S' => GardenPlot::Start,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GardenPlot {
    Start,
    Garden,
    Rock,
}

pub struct GardenMap {
    map: Vec<Vec<GardenPlot>>,
}

/// ```rust
/// use advent_of_code_2123::day21::*;
/// let input = parse(
/// r"...........
/// .....###.#.
/// .###.##..#.
/// ..#.#...#..
/// ....#.#....
/// .##..S####.
/// .##..#...#.
/// .......##..
/// .##.#.####.
/// .##..##.##.
/// ...........");
/// assert_eq!(16, part1(&input));
/// ```
pub fn part1(_input: &GardenMap) -> u32 {
    0
}

pub fn part2(_input: &GardenMap) -> u64 {
    0
}
