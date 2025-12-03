use advent::*;

advent_day!(Day12, parse, Vec<Vec<u8>>, part1, part2);

pub fn parse(input: &str) -> InputType<'_> {
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

    (0..bounds.y)
        .flat_map(|y| (0..bounds.x).map(move |x| UPoint::new(x, y)))
        .filter_map(|position| measure(position, input, visited, &bounds))
        .sum()
}

/// ```rust
/// use advent_of_code_2024::day12::*;
/// let input = parse(
/// r"AAAA
/// BBCD
/// BBCC
/// EEEC");
/// assert_eq!(80, part2(&input));
/// ```
/// ```rust
/// use advent_of_code_2024::day12::*;
/// let input = parse(
/// r"EEEEE
/// EXXXX
/// EEEEE
/// EXXXX
/// EEEEE");
/// assert_eq!(236, part2(&input));
/// ```
/// ```rust
/// use advent_of_code_2024::day12::*;
/// let input = parse(
/// r"AAAAAA
/// AAABBA
/// AAABBA
/// ABBAAA
/// ABBAAA
/// AAAAAA");
/// assert_eq!(368, part2(&input));
/// ```
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
pub fn part2(input: &InputType) -> u32 {
    let bounds = UPoint::new(input.first().unwrap().len() as u32, input.len() as u32);
    let visited = &mut (0..bounds.y)
        .map(|_| (0..bounds.y).map(|_| false).collect())
        .collect::<Vec<Vec<_>>>();

    let corners: &Vec<Vec<u8>> = &(0..bounds.y)
        .map(|y| {
            (0..bounds.x)
                .map(move |x| count_corners(UPoint::new(x, y), input, &bounds))
                .collect()
        })
        .collect();

    fn count_corners(position: UPoint, map: &InputType, bounds: &UPoint) -> u8 {
        let mut corners = 0;
        fn get_cell(position: Option<UPoint>, map: &InputType) -> Option<u8> {
            if let Some(position) = position {
                Some(map[position.y as usize][position.x as usize])
            } else {
                None
            }
        }
        let center = Some(map[position.y as usize][position.x as usize]);
        let east = position.east_checked(bounds);
        let west = position.west_checked();
        let northeast = get_cell(east.and_then(|p| p.north_checked()), map);
        let northwest = get_cell(west.and_then(|p| p.north_checked()), map);
        let southeast = get_cell(east.and_then(|p| p.south_checked(bounds)), map);
        let southwest = get_cell(west.and_then(|p| p.south_checked(bounds)), map);
        let north = get_cell(position.north_checked(), map);
        let south = get_cell(position.south_checked(bounds), map);
        let east = get_cell(east, map);
        let west = get_cell(west, map);

        if (northeast != center && north == center && east == center)
            || (north != center && east != center)
        {
            corners += 1;
        }
        if (northwest != center && north == center && west == center)
            || (north != center && west != center)
        {
            corners += 1;
        }
        if (southeast != center && south == center && east == center)
            || (south != center && east != center)
        {
            corners += 1;
        }
        if (southwest != center && south == center && west == center)
            || (south != center && west != center)
        {
            corners += 1;
        }

        corners
    }

    fn measure(
        start: UPoint,
        map: &InputType,
        bounds: &UPoint,
        corner_map: &Vec<Vec<u8>>,
        visited: &mut Vec<Vec<bool>>,
    ) -> Option<u32> {
        let current_region = map[start.y as usize][start.x as usize];
        let mut area = 0;
        let mut corners = 0u32;
        let mut pending = vec![start];

        if visited[start.y as usize][start.x as usize] {
            None
        } else {
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

                corners += corner_map[position.y as usize][position.x as usize] as u32;

                if let Some(adjacent) =
                    check_adjacent(position, Direction::North, &bounds, map, current_region)
                {
                    pending.push(adjacent);
                }

                if let Some(adjacent) =
                    check_adjacent(position, Direction::East, &bounds, map, current_region)
                {
                    pending.push(adjacent);
                }

                if let Some(adjacent) =
                    check_adjacent(position, Direction::South, &bounds, map, current_region)
                {
                    pending.push(adjacent);
                }

                if let Some(adjacent) =
                    check_adjacent(position, Direction::West, &bounds, map, current_region)
                {
                    pending.push(adjacent);
                }
            }

            Some(area * corners)
        }
    }

    (0..bounds.y)
        .flat_map(|y| (0..bounds.x).map(move |x| UPoint::new(x, y)))
        .filter_map(|position| measure(position, input, &bounds, corners, visited))
        .sum()
}
