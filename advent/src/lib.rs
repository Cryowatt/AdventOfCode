use std::str::FromStr;

pub type UPoint = Point<u32>;
pub type IPoint = Point<i32>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: num_traits::Unsigned
        + PartialOrd
        + num_traits::CheckedSub
        + num_traits::CheckedAdd
        + num_traits::identities::One
        + num_traits::identities::Zero
        + Copy,
{
    pub fn south_checked(&self, bounds: &Point<T>) -> Option<Self> {
        let y = self.y + T::one();
        if y < bounds.y {
            Some(Self::new(self.x, y))
        } else {
            None
        }
    }

    pub fn north_checked(&self) -> Option<Self> {
        Some(Self::new(self.x, self.y.checked_sub(&T::one())?))
    }

    pub fn east_checked(&self, bounds: &Point<T>) -> Option<Self> {
        let x = self.x + T::one();
        if x < bounds.x {
            Some(Self::new(x, self.y))
        } else {
            None
        }
    }

    pub fn west_checked(&self) -> Option<Self> {
        Some(Self::new(self.x.checked_sub(&T::one())?, self.y))
    }

    pub fn direction_checked(&self, direction: Direction, bounds: &Point<T>) -> Option<Self> {
        match direction {
            Direction::North => self.north_checked(),
            Direction::South => self.south_checked(bounds),
            Direction::East => self.east_checked(bounds),
            Direction::West => self.west_checked(),
        }
    }
}

impl<T> Point<T>
where
    T: num_traits::identities::Zero + Copy,
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
}

impl<T> Point<T>
where
    T: num_traits::Signed + num_traits::identities::One + num_traits::identities::Zero + Copy,
{
    pub fn north(&self) -> Self {
        self.up()
    }

    pub fn south(&self) -> Self {
        self.down()
    }

    pub fn east(&self) -> Self {
        self.right()
    }

    pub fn west(&self) -> Self {
        self.left()
    }

    pub fn left(&self) -> Self {
        Self::new(self.x - T::one(), self.y)
    }

    pub fn right(&self) -> Self {
        Self::new(self.x + T::one(), self.y)
    }

    pub fn up(&self) -> Self {
        Self::new(self.x, self.y - T::one())
    }

    pub fn down(&self) -> Self {
        Self::new(self.x, self.y + T::one())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    pub fn right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }
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
        print!("Day {}#\t[{:<20}]", $id, <$day>::part1());
        println!(" [{:<20}]", <$day>::part2());
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
                b.bytes = super::INPUT.len() as u64;
                let input = super::parse(super::INPUT);
                b.iter(|| test::black_box(super::$part1_func(&input)));
            }

            #[bench]
            fn part2_bench(b: &mut test::Bencher) {
                b.bytes = super::INPUT.len() as u64;
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> Point3D<T>
where
    T: num_traits::identities::Zero + Copy,
{
    pub fn origin() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }
}

#[derive(Debug)]
pub enum PointParseError<Err> {
    Terminator,
    InnerValueError(Err),
}

impl<T> FromStr for Point3D<T>
where
    T: FromStr,
{
    type Err = PointParseError<T::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_value<T, TErr>(
            value: Option<&str>,
        ) -> Result<T, PointParseError<<T as FromStr>::Err>>
        where
            T: FromStr,
        {
            value
                .ok_or(PointParseError::Terminator)?
                .trim()
                .parse::<T>()
                .map_err(|inner| PointParseError::InnerValueError(inner))
        }

        let mut values = s.split_terminator(", ");
        Ok(Point3D::new(
            parse_value::<_, Self::Err>(values.next())?,
            parse_value::<_, Self::Err>(values.next())?,
            parse_value::<_, Self::Err>(values.next())?,
        ))
    }
}
