use advent::*;

use fixed::traits::Fixed;
use kiddo::{
    fixed::{
        distance::{Manhattan, SquaredEuclidean},
        kdtree::KdTree,
    },
    traits::DistanceMetric,
};

// type Axis = fixed::FixedU32<fixed::types::extra::U0>;
// type Tree = kiddo::fixed::kdtree::KdTree<Axis, usize, 2, 32, u32>;

advent_day!(Day09, 9, Vec<IPoint>);
struct Area;
impl<A: kiddo::fixed::kdtree::Axis, const K: usize> DistanceMetric<A, K> for Area {
    #[inline]
    fn dist(a: &[A; K], b: &[A; K]) -> A {
        a.iter()
            .zip(b.iter())
            .map(|(&a_val, &b_val)| {
                let diff: A = a_val.dist(b_val);
                diff
            })
            .reduce(|a, b| a * b)
            .unwrap()
    }

    #[inline]
    fn dist1(a: A, b: A) -> A {
        let diff: A = a.dist(b);
        diff
    }
}

impl DayParser<Day> for Day {
    fn parse(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| {
                    let (x, y) = line.split_once(',').unwrap();
                    IPoint::new(x.parse().unwrap(), y.parse().unwrap())
                })
                .collect(),
        )
    }
}

impl Day {
    /// ```rust
    /// use advent::*;
    /// use advent_of_code_2025::day09::*;
    /// let day = Day::parse(
    /// r"7,1
    /// 11,1
    /// 11,7
    /// 9,7
    /// 9,5
    /// 2,5
    /// 2,3
    /// 7,3");
    /// assert_eq!(50, day.part1_impl(IPoint::new(6,4)));
    /// ```
    pub fn part1_impl(&self, center: IPoint) -> u64 {
        let mut max_quadrant = [(0, IPoint::origin()); 4];
        for &point in self.input() {
            let offset_point = point - center;
            let quadrant =
                if offset_point.x < 0 { 1 } else { 0 } + if offset_point.y < 0 { 2 } else { 0 };
            let area = (offset_point.x * offset_point.y).abs();

            if max_quadrant[quadrant].0 < area {
                max_quadrant[quadrant] = (area, offset_point);
            }
        }
        let (_, top_right) = max_quadrant[0];
        let (_, top_left) = max_quadrant[1];
        let (_, bottom_right) = max_quadrant[2];
        let (_, bottom_left) = max_quadrant[3];
        let nesw = (bottom_right.x.abs_diff(top_left.x) + 1) as u64
            * (bottom_right.y.abs_diff(top_left.y) + 1) as u64;
        let nwse = (bottom_left.x.abs_diff(top_right.x) + 1) as u64
            * (bottom_left.y.abs_diff(top_right.y) + 1) as u64;
        nesw.max(nwse)
    }
}

impl AdventDay for Day {
    fn part1(&self) -> String {
        self.part1_impl(IPoint::new(50000, 50000)).to_string()
    }

    /// ```rust
    /// use advent::*;
    /// use advent_of_code_2025::day09::*;
    /// let day = Day::parse(
    /// r"7,1
    /// 11,1
    /// 11,7
    /// 9,7
    /// 9,5
    /// 2,5
    /// 2,3
    /// 7,3");
    /// assert_eq!("24", day.part2());
    /// ```
    fn part2(&self) -> String {
        "".to_string()
    }
}
