#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct UPoint {
    pub x: u32,
    pub y: u32,
}

impl UPoint {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn left(&self) -> Option<Self> {
        self.x.checked_sub(1).map(|x| Self::new(x, self.y))
    }

    pub fn right(&self) -> Option<Self> {
        self.x.checked_add(1).map(|x| Self::new(x, self.y))
    }

    pub fn up(&self) -> Option<Self> {
        self.y.checked_sub(1).map(|y| Self::new(self.x, y))
    }

    pub fn down(&self) -> Option<Self> {
        self.y.checked_add(1).map(|y| Self::new(self.x, y))
    }

    pub fn manhattan(&self, other: &UPoint) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[macro_export]
macro_rules! run_day {
    ($id:literal, $day:path) => {
        print!("Day {}# [{:<15}]", $id, <$day>::part1());
        println!(" [{:<15}]", <$day>::part2());
    };
}

#[macro_export]
macro_rules! advent_day {
    ($day:ident, $parser:expr, $input_type:ty, $part1_func:ident, $part2_func:ident) => {
        pub const INPUT: &'static str = include_str!("input.txt");

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
