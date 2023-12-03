use crate::{advent_day, UPoint};

advent_day!(Day3, part1, part2);

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

    struct Grid<'a> {
        cells: &'a [u8],
        width: u32,
        height: u32,
        stride: u32,
    }

    let grid = Grid {
        cells: input.as_bytes(),
        width: width as u32,
        height: height as u32,
        stride: stride as u32,
    };

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
/// let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
/// assert_eq!(0, part2(input));
/// ```
pub fn part2(_input: &str) -> u32 {
    0
}
