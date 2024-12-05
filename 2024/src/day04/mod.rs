use advent::*;

advent_day!(Day04, parse, Vec<Vec<u8>>, part1, part2);

pub fn parse(input: &str) -> InputType {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

/// ```rust
/// use advent_of_code_2024::day04::*;
/// let input = parse(
/// r"MMMSXXMASM
/// MSAMXMSMSA
/// AMXSXMAAMM
/// MSAMASMSMX
/// XMASAMXAMM
/// XXAMMXXAMA
/// SMSMSASXSS
/// SAXAMASAAA
/// MAMMMXMMMM
/// MXMXAXMASX");
/// assert_eq!(18, part1(&input));
/// ```
pub fn part1(input: &InputType) -> usize {
    const PATHS: [[Point<isize>; 3]; 8] = [
        [Point::new(1, 0), Point::new(2, 0), Point::new(3, 0)], // E
        [Point::new(1, -1), Point::new(2, -2), Point::new(3, -3)], // NE
        [Point::new(0, -1), Point::new(0, -2), Point::new(0, -3)], // N
        [Point::new(-1, -1), Point::new(-2, -2), Point::new(-3, -3)], // NW
        [Point::new(-1, 0), Point::new(-2, 0), Point::new(-3, 0)], // W
        [Point::new(-1, 1), Point::new(-2, 2), Point::new(-3, 3)], // SW
        [Point::new(0, 1), Point::new(0, 2), Point::new(0, 3)], // S
        [Point::new(1, 1), Point::new(2, 2), Point::new(3, 3)], // SE
    ];

    fn count_matches(input: &InputType, origin: Point<isize>) -> usize {
        if cell_contains(input, origin, b'X') {
            PATHS
                .iter()
                .filter(|path| {
                    cell_contains(input, origin + path[0], b'M')
                        && cell_contains(input, origin + path[1], b'A')
                        && cell_contains(input, origin + path[2], b'S')
                })
                .count()
        } else {
            0
        }
        // if input
    }
    (0..input.len())
        .flat_map(move |y| {
            (0..input[y].len())
                .map(move |x| count_matches(input, Point::new(x as isize, y as isize)))
        })
        .sum()
}

/// ```rust
/// use advent_of_code_2024::day04::*;
/// let input = parse(
/// r"MMMSXXMASM
/// MSAMXMSMSA
/// AMXSXMAAMM
/// MSAMASMSMX
/// XMASAMXAMM
/// XXAMMXXAMA
/// SMSMSASXSS
/// SAXAMASAAA
/// MAMMMXMMMM
/// MXMXAXMASX");
/// assert_eq!(9, part2(&input));
/// ```
pub fn part2(input: &InputType) -> usize {
    fn is_xmas(input: &InputType, origin: Point<isize>) -> bool {
        if let Ok(b'A') = get_cell(input, origin) {
            let ne = get_cell(input, origin + Point::new(1, -1)).unwrap();
            let nw = get_cell(input, origin + Point::new(-1, -1)).unwrap();
            let sw = get_cell(input, origin + Point::new(-1, 1)).unwrap();
            let se = get_cell(input, origin + Point::new(1, 1)).unwrap();

            ((ne == b'M' && sw == b'S') || (ne == b'S' && sw == b'M'))
                && ((nw == b'M' && se == b'S') || (nw == b'S' && se == b'M'))
        } else {
            false
        }
    }

    (1..input.len() - 1)
        .map(move |y| {
            (1..input[y].len() - 1)
                .filter(move |x| is_xmas(input, Point::new(*x as isize, y as isize)))
                .count()
        })
        .sum()
}

fn get_cell(input: &InputType, point: Point<isize>) -> Result<u8, ()> {
    if let Some(row) = input.get(point.y as usize) {
        if let Some(cell) = row.get(point.x as usize) {
            Ok(*cell)
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

fn cell_contains(input: &InputType, point: Point<isize>, value: u8) -> bool {
    if let Ok(cell) = get_cell(input, point) {
        cell == value
    } else {
        false
    }
}
