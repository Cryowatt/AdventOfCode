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
        bounds: Point::new(
            input.lines().next().unwrap().len() as u32,
            input.lines().count() as u32,
        ),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GardenPlot {
    Start,
    Garden,
    Rock,
}

pub struct GardenMap {
    map: Vec<Vec<GardenPlot>>,
    bounds: UPoint,
}

/// ```rust
/// use advent_of_code_2023::day21::*;
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
/// assert_eq!(16, step_count(&input, 6));
/// ```
pub fn part1(input: &GardenMap) -> u32 {
    step_count(input, 64)
}

pub fn step_count(input: &GardenMap, max_steps: u32) -> u32 {
    let start = input
        .map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, cell)| {
                if *cell == GardenPlot::Start {
                    Some(Point::new(x as u32, y as u32))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let mut visited: Vec<Vec<u32>> = vec![];
    for _y in 0..input.bounds.y {
        let mut row = vec![];
        for _x in 0..input.bounds.x {
            row.push(u32::MAX);
        }
        visited.push(row);
    }

    let mut pending: Vec<(UPoint, u32)> = vec![(start, 0)];

    fn queue_if_better(
        point: Option<UPoint>,
        steps: u32,
        map: &Vec<Vec<GardenPlot>>,
        visited: &mut Vec<Vec<u32>>,
        pending: &mut Vec<(UPoint, u32)>,
    ) {
        // Bounds check
        if let Some(point) = point {
            // Terrain check
            if map[point.y as usize][point.x as usize] != GardenPlot::Rock {
                // Visited check
                if steps < visited[point.y as usize][point.x as usize] {
                    visited[point.y as usize][point.x as usize] = steps;
                    pending.push((point, steps));
                }
            }
        }
    }

    while let Some((point, steps)) = pending.pop() {
        if steps < max_steps {
            let next_step = steps + 1;
            queue_if_better(
                point.north_checked(),
                next_step,
                &input.map,
                &mut visited,
                &mut pending,
            );
            queue_if_better(
                point.west_checked(),
                next_step,
                &input.map,
                &mut visited,
                &mut pending,
            );
            queue_if_better(
                point.south_checked(&input.bounds),
                next_step,
                &input.map,
                &mut visited,
                &mut pending,
            );
            queue_if_better(
                point.east_checked(&input.bounds),
                next_step,
                &input.map,
                &mut visited,
                &mut pending,
            );
        }
    }

    let check = max_steps % 2;
    let mut plots = 0;

    for y in 0..input.bounds.y {
        for x in 0..input.bounds.x {
            let visited = visited[y as usize][x as usize];
            let plot = input.map[y as usize][x as usize];

            if visited % 2 == check {
                plots += 1;
                print!("O");
            } else {
                print!(
                    "{}",
                    match plot {
                        GardenPlot::Start => "S",
                        GardenPlot::Garden => ".",
                        GardenPlot::Rock => "#",
                    }
                );
            }
        }
        println!();
    }

    plots
}

pub fn part2(_input: &GardenMap) -> u64 {
    0
}
