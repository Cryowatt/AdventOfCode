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
/// assert_eq!(16, step_count( &input, 6));
/// ```
pub fn part1(input: &GardenMap) -> u32 {
    step_count(input, 64)
}

pub fn step_count(input: &GardenMap, max_steps: u32) -> u32 {
    let start = UPoint::new(input.bounds.x / 2, input.bounds.y / 2);

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
        max_steps: u32,
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
                    if steps < max_steps {
                        pending.push((point, steps));
                    }
                }
            }
        }
    }

    while let Some((point, steps)) = pending.pop() {
        // if steps < max_steps {
        let next_step = steps + 1;
        queue_if_better(
            point.north_checked(),
            next_step,
            max_steps,
            &input.map,
            &mut visited,
            &mut pending,
        );
        queue_if_better(
            point.west_checked(),
            next_step,
            max_steps,
            &input.map,
            &mut visited,
            &mut pending,
        );
        queue_if_better(
            point.south_checked(&input.bounds),
            next_step,
            max_steps,
            &input.map,
            &mut visited,
            &mut pending,
        );
        queue_if_better(
            point.east_checked(&input.bounds),
            next_step,
            max_steps,
            &input.map,
            &mut visited,
            &mut pending,
        );
        // }
    }

    let check = max_steps % 2;
    let mut plots = 0;

    for y in 0..input.bounds.y {
        for x in 0..input.bounds.x {
            let visited = visited[y as usize][x as usize];

            if visited % 2 == check {
                plots += 1;
            }
        }
    }

    plots
}

// 9.38173766097547% is rocks
// a full diamond with max step 26501365 is
//  (((26501365 ^ 2 + 26501366 ^ 2) * (1-9.38173766097547%)) / 2)
// ~= 636432330761187
pub fn part2(input: &GardenMap) -> u64 {
    const MAX_STEPS: u32 = 26501365;
    let block_size = input.bounds.x;

    // Number of steps required to exit the center plot
    let exit_steps = block_size / 2;
    let blocks_to_edge = (MAX_STEPS - exit_steps) / block_size;

    let start = Point::new(exit_steps, exit_steps);

    // Corners
    let corner_start = (((MAX_STEPS - exit_steps) / block_size) - 1) * block_size + exit_steps + 1;
    let (_, north) = plot_count(
        Point::new(start.x, block_size - 1),
        input,
        corner_start,
        MAX_STEPS,
    );
    let (_, south) = plot_count(Point::new(start.x, 0), input, corner_start, MAX_STEPS);
    let (_, east) = plot_count(Point::new(0, start.y), input, corner_start, MAX_STEPS);
    let (_, west) = plot_count(
        Point::new(block_size - 1, start.y),
        input,
        corner_start,
        MAX_STEPS,
    );

    // Inner edges
    let inner_start = corner_start + exit_steps + 1;
    let (_, inner_south_east) = plot_count(Point::new(0, 0), input, inner_start, MAX_STEPS);
    let (_, inner_south_west) =
        plot_count(Point::new(block_size - 1, 0), input, inner_start, MAX_STEPS);
    let (_, inner_north_east) =
        plot_count(Point::new(0, block_size - 1), input, inner_start, MAX_STEPS);
    let (_, inner_north_west) = plot_count(
        Point::new(block_size - 1, block_size - 1),
        input,
        inner_start,
        MAX_STEPS,
    );

    // This handles the left/right corners and top/bottom rows
    let mut total_plots = north as u64
        + south as u64
        + east as u64
        + west as u64
        + inner_south_east as u64
        + inner_south_west as u64
        + inner_north_east as u64
        + inner_north_west as u64;

    // Outer edges
    let edge_start = corner_start - block_size + exit_steps + 1;
    let (_, south_east) = plot_count(Point::new(0, 0), input, edge_start, MAX_STEPS);
    let (_, south_west) = plot_count(Point::new(block_size - 1, 0), input, edge_start, MAX_STEPS);
    let (_, north_east) = plot_count(Point::new(0, block_size - 1), input, edge_start, MAX_STEPS);
    let (_, north_west) = plot_count(
        Point::new(block_size - 1, block_size - 1),
        input,
        edge_start,
        MAX_STEPS,
    );

    // Full block
    let (full_even, full_odd) = plot_count(start, input, 0, MAX_STEPS);

    let row_steps = |y| {
        let full_blocks_length = ((blocks_to_edge * 2) + 1) - 2 - (y * 2);

        let odd_blocks = full_blocks_length / 2;
        let even_blocks = full_blocks_length - odd_blocks;

        ((odd_blocks * full_odd) + (even_blocks * full_even)) as u64
    };

    total_plots += row_steps(0);

    for y in 1..blocks_to_edge {
        total_plots += north_east as u64
            + north_west as u64
            + south_east as u64
            + south_west as u64
            + inner_north_east as u64
            + inner_north_west as u64
            + inner_south_east as u64
            + inner_south_west as u64
            + (row_steps(y) * 2);
    }

    total_plots
}

fn plot_count(start: Point<u32>, map: &GardenMap, steps: u32, max_steps: u32) -> (u32, u32) {
    let mut visited: Vec<Vec<u32>> = vec![];
    for _y in 0..map.bounds.y {
        let mut row = vec![];
        for _x in 0..map.bounds.x {
            row.push(u32::MAX);
        }
        visited.push(row);
    }

    let mut pending: Vec<(UPoint, u32)> = vec![];

    fn queue_if_better(
        point: Option<UPoint>,
        steps: u32,
        max_steps: u32,
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
                    if steps < max_steps {
                        pending.push((point, steps));
                    }
                }
            }
        }
    }

    queue_if_better(
        Some(start),
        steps,
        max_steps,
        &map.map,
        &mut visited,
        &mut pending,
    );

    while let Some((point, steps)) = pending.pop() {
        let next_step = steps + 1;
        queue_if_better(
            point.north_checked(),
            next_step,
            max_steps,
            &map.map,
            &mut visited,
            &mut pending,
        );
        queue_if_better(
            point.west_checked(),
            next_step,
            max_steps,
            &map.map,
            &mut visited,
            &mut pending,
        );
        queue_if_better(
            point.south_checked(&map.bounds),
            next_step,
            max_steps,
            &map.map,
            &mut visited,
            &mut pending,
        );
        queue_if_better(
            point.east_checked(&map.bounds),
            next_step,
            max_steps,
            &map.map,
            &mut visited,
            &mut pending,
        );
    }

    let mut even_plots = 0;
    let mut odd_plots = 0;

    for y in 0..map.bounds.y {
        for x in 0..map.bounds.x {
            let visited = visited[y as usize][x as usize];

            if visited < u32::MAX {
                if visited % 2 == 0 {
                    even_plots += 1;
                } else {
                    odd_plots += 1;
                }
            }
        }
    }
    (even_plots, odd_plots)
}
