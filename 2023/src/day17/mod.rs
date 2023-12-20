use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

use advent::*;

advent_day!(Day17, parse, HeatMap, part1, part2);

pub struct HeatMap {
    blocks: Vec<Vec<u8>>,
    bounds: UPoint,
    end: UPoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PathState {
    point: UPoint,
    distance: u32,
    heat: u32,
    direction: Direction,
    steps: u8,
}

impl PathState {
    fn fitness(&self) -> u32 {
        self.heat + (self.distance)
    }

    fn next_state(&self, direction: Direction, map: &HeatMap, max_step: u8) -> Option<PathState> {
        if self.direction.opposite() == direction {
            None
        } else {
            if let Some(point) = self.point.direction_checked(direction, &map.bounds) {
                let steps = if self.direction == direction {
                    self.steps + 1
                } else {
                    1
                };

                if steps <= max_step {
                    Some(PathState {
                        point,
                        distance: point.manhattan(&map.end),
                        heat: self.heat + map.blocks[point.y as usize][point.x as usize] as u32,
                        direction,
                        steps,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fitness().cmp(&other.fitness())
    }
}

impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Record {
    north: Option<PathState>,
    south: Option<PathState>,
    east: Option<PathState>,
    west: Option<PathState>,
}

impl Record {
    fn is_better(&mut self, candidate: PathState) -> bool {
        let best_state = match candidate.direction {
            Direction::North => &mut self.north,
            Direction::South => &mut self.south,
            Direction::East => &mut self.east,
            Direction::West => &mut self.west,
        };

        if let Some(best_state) = best_state {
            if candidate.heat < best_state.heat {
                *best_state = candidate;
                true
            } else {
                false
            }
        } else {
            *best_state = Some(candidate);
            true
        }
    }
}

pub fn parse(input: &str) -> HeatMap {
    let bounds = UPoint::new(
        input.lines().next().unwrap().len() as u32,
        input.lines().count() as u32,
    );
    HeatMap {
        blocks: input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|b| b.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect(),
        bounds,
        end: UPoint::new(bounds.x - 1, bounds.y - 1),
        // end: UPoint::new(5, 0),
    }
}

/// ```rust
/// use advent_of_code_2023::day17::*;
/// let input = parse(
/// r"2413432311323
/// 3215453535623
/// 3255245654254
/// 3446585845452
/// 4546657867536
/// 1438598798454
/// 4457876987766
/// 3637877979653
/// 4654967986887
/// 4564679986453
/// 1224686865563
/// 2546548887735
/// 4322674655533");
/// assert_eq!(102, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day17::*;
/// let input = parse(
/// r"9419932319999
/// 9915459939999
/// 9999999954299
/// 9999999999499
/// 9999999999539
/// 9999999999959
/// 9999999999969
/// 9999999999953
/// 9999999999997
/// 9999999999993
/// 9999999999963
/// 9999999999939
/// 9999999999933");
/// assert_eq!(102, part1(&input));
/// ```
pub fn part1(map: &HeatMap) -> u32 {
    let mut priority_queue = BinaryHeap::<Reverse<PathState>>::new();
    let mut best_heat: Vec<Vec<Record>> = vec![];

    for _y in 0..map.bounds.y {
        let mut row = vec![];
        for _x in 0..map.bounds.x {
            row.push(Record {
                north: None,
                south: None,
                east: None,
                west: None,
            });
        }
        best_heat.push(row);
    }

    priority_queue.push(Reverse(PathState {
        point: UPoint::origin(),
        distance: UPoint::origin().manhattan(&map.end),
        heat: 0,
        direction: Direction::East,
        steps: 0,
    }));
    priority_queue.push(Reverse(PathState {
        point: UPoint::origin(),
        distance: UPoint::origin().manhattan(&map.end),
        heat: 0,
        direction: Direction::South,
        steps: 0,
    }));

    while let Some(state) = priority_queue.pop() {
        let state = state.0;

        if state.point == map.end {
            return state.heat;
        }

        let right = state.direction.right();
        let rights = (0..3).scan(state, |last, _| {
            if let Some(valid_state) = last.next_state(right, map, 3) {
                *last = valid_state;
                Some(valid_state)
            } else {
                None
            }
        });
        let left = state.direction.left();
        let lefts = (0..3).scan(state, |last, _| {
            if let Some(valid_state) = last.next_state(left, map, 3) {
                *last = valid_state;
                Some(valid_state)
            } else {
                None
            }
        });

        for valid_state in rights.chain(lefts) {
            if best_heat[valid_state.point.y as usize][valid_state.point.x as usize]
                .is_better(valid_state)
            {
                priority_queue.push(Reverse(valid_state));
            }
        }
    }

    unreachable!()
}

/// ```rust
/// use advent_of_code_2023::day17::*;
/// let input = parse(
/// r"2413432311323
/// 3215453535623
/// 3255245654254
/// 3446585845452
/// 4546657867536
/// 1438598798454
/// 4457876987766
/// 3637877979653
/// 4654967986887
/// 4564679986453
/// 1224686865563
/// 2546548887735
/// 4322674655533");
/// assert_eq!(94, part2(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day17::*;
/// let input = parse(
/// r"111111111111
/// 999999999991
/// 999999999991
/// 999999999991
/// 999999999991");
/// assert_eq!(71, part2(&input));
/// ```
pub fn part2(map: &HeatMap) -> u32 {
    let mut priority_queue = BinaryHeap::<Reverse<PathState>>::new();
    let mut best_heat: Vec<Vec<Record>> = vec![];

    for _y in 0..map.bounds.y {
        let mut row = vec![];
        for _x in 0..map.bounds.x {
            row.push(Record {
                north: None,
                south: None,
                east: None,
                west: None,
            });
        }
        best_heat.push(row);
    }

    priority_queue.push(Reverse(PathState {
        point: UPoint::origin(),
        distance: UPoint::origin().manhattan(&map.end),
        heat: 0,
        direction: Direction::East,
        steps: 0,
    }));
    priority_queue.push(Reverse(PathState {
        point: UPoint::origin(),
        distance: UPoint::origin().manhattan(&map.end),
        heat: 0,
        direction: Direction::South,
        steps: 0,
    }));

    while let Some(state) = priority_queue.pop() {
        let state = state.0;

        if state.point == map.end && state.steps >= 4 {
            return state.heat;
        }

        let right = state.direction.right();
        let rights = (0..10).scan(state, |last, _| {
            if let Some(valid_state) = last.next_state(right, map, 10) {
                *last = valid_state;
                Some(valid_state)
            } else {
                None
            }
        });
        let left = state.direction.left();
        let lefts = (0..10).scan(state, |last, _| {
            if let Some(valid_state) = last.next_state(left, map, 10) {
                *last = valid_state;
                Some(valid_state)
            } else {
                None
            }
        });

        for valid_state in rights.chain(lefts) {
            if valid_state.steps >= 4 {
                if best_heat[valid_state.point.y as usize][valid_state.point.x as usize]
                    .is_better(valid_state)
                {
                    priority_queue.push(Reverse(valid_state));
                }
            }
        }
    }

    unreachable!()
}
