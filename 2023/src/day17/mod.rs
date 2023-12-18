use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

use advent::*;
use bitflags::Flags;
use crossterm::style::Stylize;

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

    fn next_state_ultra(&self, direction: Direction, map: &HeatMap) -> Option<PathState> {
        if self.direction.opposite() == direction {
            None
        } else {
            if self.direction != direction {
                // Freshly turned, take 4 steps if possible
                let fk = self
                    .next_state(direction, map, 10)
                    .and_then(|state| state.next_state(direction, map, 10))
                    .and_then(|state| state.next_state(direction, map, 10))
                    .and_then(|state| state.next_state(direction, map, 10));
                // if let Some(fk) = fk {
                //     println!(
                //         "Turn {} step: {} ({}, {}) => ({}, {})",
                //         fk.steps, fk.heat, self.point.x, self.point.y, fk.point.x, fk.point.y
                //     );
                // }
                fk
            } else if let Some(point) = self.point.direction_checked(direction, &map.bounds) {
                let steps = self.steps + 1;

                if steps <= 10 {
                    let heat = self.heat + map.blocks[point.y as usize][point.x as usize] as u32;
                    Some(PathState {
                        point,
                        distance: point.manhattan(&map.end),
                        heat,
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
        // self.fitness()
        //     .cmp(&other.fitness())
        self.heat
            .cmp(&other.heat)
            // other
            //     .fitness()
            //     .cmp(&self.fitness())
            // other
            //     .heat
            //     .cmp(&self.heat)
            // .then_with(|| other.fitness().cmp(&self.fitness()))
            // .then_with(|| other.distance.cmp(&self.distance))
            // .then_with(|| self.heat.cmp(&other.heat))
            .then_with(|| self.steps.cmp(&other.steps))

        // ot
        // .then_with(|| (self.point.x + self.point.y).cmp(&(other.point.x + other.point.y)))
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
    fn add_if_better(&mut self, candidate: PathState) -> bool {
        let best_state = match candidate.direction {
            Direction::North => &mut self.north,
            Direction::South => &mut self.south,
            Direction::East => &mut self.east,
            Direction::West => &mut self.west,
        };

        if let Some(best_state) = best_state {
            if candidate < *best_state {
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

    fn add_if_better_ultra(&mut self, candidate: PathState) -> bool {
        let best_state = match candidate.direction {
            Direction::North => &mut self.north,
            Direction::South => &mut self.south,
            Direction::East => &mut self.east,
            Direction::West => &mut self.west,
        };

        if let Some(best_state) = best_state {
            if candidate < *best_state {
                // println!(
                //     "{} ({}) is better than {} ({})",
                //     candidate.heat,
                //     candidate.fitness(),
                //     best_state.heat,
                //     best_state.fitness()
                // );
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
/// //assert_eq!(102, part1(&input));
/// ```
pub fn part1(input: &HeatMap) -> u32 {
    return 0;
    println!(
        "Bounds {},{}  End {},{}",
        input.bounds.x, input.bounds.y, input.end.x, input.end.y
    );
    fn enumerate_moves(
        state: PathState,
        heap: &mut BinaryHeap<Reverse<PathState>>,
        best_heat: &mut Vec<Vec<Record>>,
        map: &HeatMap,
    ) {
        for potential_state in [
            state.next_state(Direction::North, map, 3),
            state.next_state(Direction::South, map, 3),
            state.next_state(Direction::West, map, 3),
            state.next_state(Direction::East, map, 3),
        ] {
            if let Some(valid_state) = potential_state {
                if best_heat[valid_state.point.y as usize][valid_state.point.x as usize]
                    .add_if_better(valid_state)
                {
                    // println!(
                    //     "Queue heat {} {} {} ({}, {})",
                    //     valid_state.heat,
                    //     valid_state.fitness(),
                    //     valid_state.steps,
                    //     valid_state.point.x,
                    //     valid_state.point.y
                    // );
                    heap.push(Reverse(valid_state));
                }
            }
        }
    }

    let mut priority_queue = BinaryHeap::<Reverse<PathState>>::new();
    let mut best_heat: Vec<Vec<Record>> = vec![];

    for _y in 0..input.bounds.y {
        let mut row = vec![];
        for _x in 0..input.bounds.x {
            row.push(Record {
                north: None,
                south: None,
                east: None,
                west: None,
            });
        }
        best_heat.push(row);
    }

    let start = PathState {
        point: UPoint::origin(),
        distance: UPoint::origin().manhattan(&input.end),
        heat: 0,
        direction: Direction::East,
        steps: 0,
    };

    best_heat[0][0].north = Some(start);
    best_heat[0][0].south = Some(start);
    best_heat[0][0].east = Some(start);
    best_heat[0][0].west = Some(start);

    priority_queue.push(Reverse(start));

    let mut max_fit = 0;
    let mut max_heat = 0;

    while let Some(state) = priority_queue.pop() {
        let state = state.0;
        if state.fitness() > max_fit {
            println!("FIT {}", state.fitness());
            max_fit = state.fitness();
        }
        if state.heat > max_heat {
            println!("HEAT {}", state.heat);
            max_heat = state.heat;
        }
        if state.point == input.end {
            println!(
                "SANITY FAILING {:?} {},{} {},{}",
                state.direction, state.point.x, state.point.y, input.end.x, input.end.y
            );
            for y in 0..input.bounds.y as usize {
                for x in 0..input.bounds.x as usize {
                    let fk = &best_heat[y][x];
                    print!("[");
                    match fk.north {
                        Some(b) => print!("{},", b.heat),
                        None => print!("_,"),
                    }
                    match fk.south {
                        Some(b) => print!("{},", b.heat),
                        None => print!("_,"),
                    }
                    match fk.east {
                        Some(b) => print!("{},", b.heat),
                        None => print!("_,"),
                    }
                    match fk.west {
                        Some(b) => print!("{}", b.heat),
                        None => print!("_"),
                    }
                    print!("]\t");
                }
                println!();
            }
            // println!("END STATE H{} F{}", state.heat, state.fitness());
            // println!("END NODE {},{}", input.end.x, input.end.y);
            // while let Some(left_state) = priority_queue.pop() {
            //     if left_state.fitness() <= state.fitness() + 1 {
            //         println!(
            //             "Remaining nodes: H{} F{} ({},{})",
            //             left_state.heat,
            //             left_state.fitness(),
            //             left_state.point.x,
            //             left_state.point.y
            //         );
            //     }
            // }
            return state.heat;
        }

        enumerate_moves(state, &mut priority_queue, &mut best_heat, input);
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
/// //assert_eq!(94, part2(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day17::*;
/// let input = parse(
/// r"111111111111
/// 999999999991
/// 999999999991
/// 999999999991
/// 999999999991");
/// //assert_eq!(71, part2(&input));
/// ```
pub fn part2(input: &HeatMap) -> u32 {
    return 0;
    fn enumerate_moves(
        state: PathState,
        heap: &mut BinaryHeap<Reverse<PathState>>,
        best_heat: &mut Vec<Vec<Record>>,
        map: &HeatMap,
    ) {
        for potential_state in [
            state.next_state_ultra(Direction::North, map),
            state.next_state_ultra(Direction::South, map),
            state.next_state_ultra(Direction::West, map),
            state.next_state_ultra(Direction::East, map),
        ] {
            if let Some(valid_state) = potential_state {
                // if valid_state.steps < 4 {
                //     heap.push(valid_state);
                // } else
                if best_heat[valid_state.point.y as usize][valid_state.point.x as usize]
                    .add_if_better_ultra(valid_state)
                {
                    // println!(
                    //     "Queue heat H{} F{} ({}, {})",
                    //     state.heat,
                    //     state.fitness(),
                    //     valid_state.point.x,
                    //     valid_state.point.y
                    // );
                    heap.push(Reverse(valid_state));
                }
            }
        }
    }

    let mut priority_queue = BinaryHeap::<Reverse<PathState>>::new();
    let mut best_heat: Vec<Vec<Record>> = vec![];
    let end = UPoint::new(input.bounds.x - 1, input.bounds.y - 1);

    for _y in 0..input.bounds.y {
        let mut row = vec![];
        for _x in 0..input.bounds.x {
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
        distance: UPoint::origin().manhattan(&input.end),
        heat: 0,
        direction: Direction::East,
        steps: 0,
    }));

    // let mut best_heat = PathState {
    //     point: UPoint::origin(),
    //     distance: u32::MAX,
    //     heat: u32::MAX,
    //     direction: Direction::East,
    //     steps: u8::MAX,
    // };

    while let Some(state) = priority_queue.pop() {
        let state = state.0;
        // print!("{}, ", state.heat);
        // println!(
        //     "State {} ({}, {})",
        //     state.fitness(),
        //     state.point.x,
        //     state.point.y
        // );
        if state.point == end {
            return state.heat;
            // if state < best_heat {
            //     best_heat = state;

            //     if st
            // }
        }

        enumerate_moves(state, &mut priority_queue, &mut best_heat, input);
    }

    unreachable!()
}
