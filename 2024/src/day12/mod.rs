use advent::*;

advent_day!(Day12, parse, Vec<Vec<u8>>, part1, part2);

pub fn parse(input: &str) -> InputType {
    input
        .lines()
        .map(|line| line.as_bytes().iter().map(|b| *b).collect())
        .collect()
}

/// ```rust
/// use advent_of_code_2024::day12::*;
/// let input = parse(
/// r"RRRRIICCFF
/// RRRRIICCCF
/// VVRRRCCFFF
/// VVRCCCJFFF
/// VVVVCJJCFE
/// VVIVCCJJEE
/// VVIIICJJEE
/// MIIIIIJJEE
/// MIIISIJEEE
/// MMMISSJEEE");
/// assert_eq!(1930, part1(&input));
/// ```
pub fn part1(input: &InputType) -> u32 {
    let bounds = UPoint::new(input.first().unwrap().len() as u32, input.len() as u32);
    let visited = &mut (0..bounds.y)
        .map(|_| (0..bounds.y).map(|_| false).collect())
        .collect::<Vec<Vec<_>>>();

    fn measure(
        start: UPoint,
        map: &InputType,
        visited: &mut Vec<Vec<bool>>,
        bounds: &UPoint,
    ) -> Option<u32> {
        let current_region = map[start.y as usize][start.x as usize];
        if current_region == 0xff {
            None
        } else {
            let mut area = 0;
            let mut perimeter = 0;
            let mut pending = vec![start];

            fn check_adjacent(
                position: Point<u32>,
                direction: Direction,
                bounds: &UPoint,
                map: &Vec<Vec<u8>>,
                current_region: u8,
            ) -> Option<UPoint> {
                if let Some(adjacent) = position.direction_checked(direction, bounds) {
                    let adjacent_region = map[adjacent.y as usize][adjacent.x as usize];
                    if adjacent_region == current_region {
                        Some(adjacent)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            while let Some(position) = pending.pop() {
                if visited[position.y as usize][position.x as usize] {
                    continue;
                }
                visited[position.y as usize][position.x as usize] = true;
                area += 1;

                if let Some(adjacent) =
                    check_adjacent(position, Direction::North, &bounds, map, current_region)
                {
                    pending.push(adjacent);
                } else {
                    perimeter += 1;
                }

                if let Some(adjacent) =
                    check_adjacent(position, Direction::East, &bounds, map, current_region)
                {
                    pending.push(adjacent);
                } else {
                    perimeter += 1;
                }

                if let Some(adjacent) =
                    check_adjacent(position, Direction::South, &bounds, map, current_region)
                {
                    pending.push(adjacent);
                } else {
                    perimeter += 1;
                }

                if let Some(adjacent) =
                    check_adjacent(position, Direction::West, &bounds, map, current_region)
                {
                    pending.push(adjacent);
                } else {
                    perimeter += 1;
                }
            }

            Some(area * perimeter)
        }
    }

    (0..bounds.y)
        .flat_map(|y| (0..bounds.x).map(move |x| UPoint::new(x, y)))
        .filter_map(|position| measure(position, input, visited, &bounds))
        .sum()
}

/// ```rust
/// use advent_of_code_2024::day12::*;
/// let input = parse(
/// r"RRRRIICCFF
/// RRRRIICCCF
/// VVRRRCCFFF
/// VVRCCCJFFF
/// VVVVCJJCFE
/// VVIVCCJJEE
/// VVIIICJJEE
/// MIIIIIJJEE
/// MIIISIJEEE
/// MMMISSJEEE");
/// assert_eq!(1206, part2(&input));
/// ```
pub fn part2(input: &InputType) -> u64 {
    0
}
