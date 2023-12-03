use crate::{advent_day, UPoint};

advent_day!(Day3, part1, part2);

struct Grid<'a> {
    cells: &'a [u8],
    width: u32,
    height: u32,
    stride: u32,
}

impl<'a> Grid<'a> {
    fn has_at<F: FnOnce(u8) -> bool>(&self, position: UPoint, predicate: F) -> bool {
        if position.x >= self.width {
            false
        } else if position.y >= self.height {
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
        let mut num:u32 = 0;
        for cell in &self.cells[offset as usize..(offset as usize +length)] {
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

        let mut left = start.left();
        while maybe_has_digit_at(left) {
            start = left.unwrap();
            left = start.left();
            length += 1;
        }

        let mut right = position.right();
        while maybe_has_digit_at(right) {
            right = right.unwrap().right();
            length += 1;
        }

        Some(grid.read_number(start, length))
    } else {
        None
    }
}

/// ```rust
/// use advent_of_code::day3::*;
/// let input = r"467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..";
/// assert_eq!(4361, part1(input));
/// ```
pub fn part1(input: &str) -> u32 {
    let width = input.find('\n').unwrap();
    let stride = width + 1;
    let height = input.len().div_ceil(width + 1);

    let grid = Grid {
        cells: input.as_bytes(),
        width: width as u32,
        height: height as u32,
        stride: stride as u32,
    };

    let mut sum = 0;

    for y in 0..height {
        for x in 0..width {
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
                            check_cell(pos.left()) + check_cell(pos.right())
                        }
                    })
                };

                sum += check_cell(here.left());
                sum += check_cell(here.right());
                sum += check_row(here.up());
                sum += check_row(here.down());
            }
        }
    }

    sum
}

/// ```rust
/// use advent_of_code::day3::*;
/// let input = r"467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..";
/// assert_eq!(467835, part2(input));
/// ```
pub fn part2(input: &str) -> u32 {
    let width = input.find('\n').unwrap();
    let stride = width + 1;
    let height = input.len().div_ceil(width + 1);

    let grid = Grid {
        cells: input.as_bytes(),
        width: width as u32,
        height: height as u32,
        stride: stride as u32,
    };
    
    let mut sum = 0;

    for y in 0..height {
        for x in 0..width {
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
                                checks[1] = check_cell(pos.left());
                                checks[2] = check_cell(pos.right())
                            }
                        }
                    );
                    }
                };

                let mut checks: [Option<u32>; 8] = [None; 8];

                // I probably could short-circuit something here if the 2-adjacent requirement is violated
                checks[0] = check_cell(here.left());
                checks[1] = check_cell(here.right());
                check_row(here.up(), &mut checks[2..5]);
                check_row(here.down(), &mut checks[5..8]);

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
