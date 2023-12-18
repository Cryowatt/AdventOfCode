use advent::*;

advent_day!(Day03, parse, Grid<'_>, part1, part2);

pub fn parse<'a>(input: &'a str) -> Grid<'a> {
    let width = input.find(['\r', '\n']).unwrap();
    let terminator_length = input
        .get(width..)
        .unwrap()
        .find(|c: char| !c.is_whitespace())
        .unwrap();
    let stride = width + terminator_length;
    let height = input.len().div_ceil(stride);

    Grid {
        cells: input.as_bytes(),
        bounds: UPoint::new(width as u32, height as u32),
        stride: stride as u32,
    }
}

pub struct Grid<'a> {
    cells: &'a [u8],
    stride: u32,
    bounds: UPoint,
}

impl<'a> Grid<'a> {
    fn has_at<F: FnOnce(u8) -> bool>(&self, position: UPoint, predicate: F) -> bool {
        if position.x >= self.bounds.x {
            false
        } else if position.y >= self.bounds.y {
            false
        } else {
            match (position.y * self.stride).checked_add(position.x) {
                Some(offset) => {
                    let cell = self.cells[offset as usize];
                    predicate(cell)
                }
                None => false,
            }
        }
    }

    fn has_symbol_at(&self, position: UPoint) -> bool {
        self.has_at(position, |cell| !cell.is_ascii_digit() && cell != b'.')
    }

    fn has_gear_at(&self, position: UPoint) -> bool {
        self.has_at(position, |cell| cell == b'*')
    }

    fn has_digit_at(&self, position: UPoint) -> bool {
        self.has_at(position, |cell| cell.is_ascii_digit())
    }

    fn read_number(&self, position: UPoint, length: usize) -> u32 {
        let offset = (position.y * self.stride) + position.x;
        let mut num: u32 = 0;
        for cell in &self.cells[offset as usize..(offset as usize + length)] {
            num = num * 10 + ((*cell) as char).to_digit(10).unwrap()
        }
        num
    }
}

fn part_number(grid: &Grid, position: UPoint) -> Option<u32> {
    if grid.has_digit_at(position) {
        let mut start = position;
        let mut length = 1;

        let maybe_has_digit_at = |position: Option<UPoint>| {
            if let Some(position) = position {
                grid.has_digit_at(position)
            } else {
                false
            }
        };

        let mut left = start.west_checked();
        while maybe_has_digit_at(left) {
            start = left.unwrap();
            left = start.west_checked();
            length += 1;
        }

        let mut right = position.east_checked(&grid.bounds);
        while maybe_has_digit_at(right) {
            right = right.unwrap().east_checked(&grid.bounds);
            length += 1;
        }

        Some(grid.read_number(start, length))
    } else {
        None
    }
}

/// ```rust
/// use advent_of_code_2023::day03::*;
/// let input = parse(
/// r"467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..");
/// assert_eq!(4361, part1(&input));
/// ```
pub fn part1(grid: &Grid<'_>) -> u32 {
    let mut sum = 0;

    for y in 0..grid.bounds.y {
        for x in 0..grid.bounds.x {
            let here = UPoint::new(x as u32, y as u32);

            if grid.has_symbol_at(here) {
                // Found a symbol, now go look for parts
                let check_cell = |maybe_position: Option<UPoint>| {
                    maybe_position.map_or(0, |pos| part_number(&grid, pos).unwrap_or_default())
                };

                let check_row = |maybe_position: Option<UPoint>| {
                    maybe_position.map_or(0, |pos| {
                        if let Some(part_num) = part_number(&grid, pos) {
                            part_num
                        } else {
                            check_cell(pos.west_checked())
                                + check_cell(pos.east_checked(&grid.bounds))
                        }
                    })
                };

                sum += check_cell(here.west_checked());
                sum += check_cell(here.east_checked(&grid.bounds));
                sum += check_row(here.north_checked());
                sum += check_row(here.south_checked(&grid.bounds));
            }
        }
    }

    sum
}

/// ```rust
/// use advent_of_code_2023::day03::*;
/// let input = parse(
/// r"467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..");
/// assert_eq!(467835, part2(&input));
/// ```
pub fn part2(grid: &Grid<'_>) -> u32 {
    let mut sum = 0;

    for y in 0..grid.bounds.y {
        for x in 0..grid.bounds.x {
            let here = UPoint::new(x as u32, y as u32);

            if grid.has_gear_at(here) {
                // Found a gear, now go look for parts
                let check_cell = |maybe_position: Option<UPoint>| {
                    maybe_position.map(|pos| part_number(&grid, pos)).unwrap()
                };

                let check_row = |maybe_position: Option<UPoint>, checks: &mut [Option<u32>]| {
                    checks[0] = check_cell(maybe_position);

                    if checks[0].is_none() {
                        maybe_position.map(|pos| {
                            if None == checks[0] {
                                checks[1] = check_cell(pos.west_checked());
                                checks[2] = check_cell(pos.east_checked(&grid.bounds))
                            }
                        });
                    }
                };

                let mut checks: [Option<u32>; 8] = [None; 8];

                // I probably could short-circuit something here if the 2-adjacent requirement is violated
                checks[0] = check_cell(here.west_checked());
                checks[1] = check_cell(here.east_checked(&grid.bounds));
                check_row(here.north_checked(), &mut checks[2..5]);
                check_row(here.south_checked(&grid.bounds), &mut checks[5..8]);

                let mut product = 1;
                let mut count = 0;

                for i in 0..8 {
                    if let Some(part) = checks[i] {
                        product *= part;
                        count += 1;
                    }
                }

                if count == 2 {
                    sum += product
                }
            }
        }
    }

    sum
}
