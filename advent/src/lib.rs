pub type UPoint = Point<u32>;
pub type IPoint = Point<i32>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: num_traits::CheckedSub
        + num_traits::CheckedAdd
        + num_traits::identities::One
        + num_traits::identities::Zero
        + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    pub fn north(&self) -> Option<Self> {
        self.up()
    }

    pub fn south(&self) -> Option<Self> {
        self.down()
    }

    pub fn east(&self) -> Option<Self> {
        self.right()
    }

    pub fn west(&self) -> Option<Self> {
        self.left()
    }

    pub fn left(&self) -> Option<Self> {
        self.x.checked_sub(&T::one()).map(|x| Self::new(x, self.y))
    }

    pub fn right(&self) -> Option<Self> {
        self.x.checked_add(&T::one()).map(|x| Self::new(x, self.y))
    }

    pub fn up(&self) -> Option<Self> {
        self.y.checked_sub(&T::one()).map(|y| Self::new(self.x, y))
    }

    pub fn down(&self) -> Option<Self> {
        self.y.checked_add(&T::one()).map(|y| Self::new(self.x, y))
    }
}

pub enum Direction {
    North,
    South,
    East,
    West,
}

pub trait Manhattan {
    type Output;
    fn manhattan(&self, other: &Self) -> Self::Output;
}

impl Manhattan for UPoint {
    type Output = u32;

    fn manhattan(&self, other: &Self) -> Self::Output {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[macro_export]
macro_rules! run_day {
    ($id:literal, $day:path) => {
        print!("Day {}#\t[{:<15}]", $id, <$day>::part1());
        println!(" [{:<15}]", <$day>::part2());
    };
}

#[macro_export]
macro_rules! advent_day {
    ($day:ident, $parser:expr, $input_type:ty, $part1_func:ident, $part2_func:ident) => {
        pub const INPUT: &'static str = include_str!("input.txt");
        // type InputType = $input_type;

        pub struct $day;

        impl $day {
            pub fn part1() -> String {
                $part1_func(&$parser(INPUT)).to_string()
            }

            pub fn part2() -> String {
                $part2_func(&$parser(INPUT)).to_string()
            }
        }

        #[cfg(test)]
        mod bench {
            extern crate test;

            #[bench]
            fn part1_bench(b: &mut test::Bencher) {
                let input = super::parse(super::INPUT);
                b.iter(|| test::black_box(super::$part1_func(&input)));
            }

            #[bench]
            fn part2_bench(b: &mut test::Bencher) {
                let input = super::parse(super::INPUT);
                b.iter(|| test::black_box(super::$part2_func(&input)));
            }
        }
    };
}

#[macro_export]
macro_rules! advent_bench {
    ($parser:ident, $module:ident, $part1_func:ident) => {
        #[cfg(test)]
        mod $module {
            extern crate test;

            #[cfg(feature = "cursed")]
            #[bench]
            fn bench(b: &mut test::Bencher) {
                let input = super::$parser(include_str!("input.txt"));
                b.iter(|| test::black_box(super::$part1_func(&input)));
            }
        }
    };
    ($parser:ident, $module:ident, $part1_func:ident, $part2_func:ident) => {
        #[cfg(test)]
        mod $module {
            extern crate test;

            #[cfg(feature = "cursed")]
            #[bench]
            fn part1_bench(b: &mut test::Bencher) {
                let input = super::$parser(include_str!("input.txt"));
                b.iter(|| test::black_box(super::$part1_func(&input)));
            }

            #[cfg(feature = "cursed")]
            #[bench]
            fn part2_bench(b: &mut test::Bencher) {
                let input = super::$parser(include_str!("input.txt"));
                b.iter(|| test::black_box(super::$part1_func(&input)));
            }
        }
    };
}
