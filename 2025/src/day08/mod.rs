use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use advent::*;

use fixed::traits::Fixed;
use kiddo::fixed::{distance::SquaredEuclidean, kdtree::KdTree};

type Axis = fixed::FixedU64<fixed::types::extra::U0>;
type Tree = kiddo::fixed::kdtree::KdTree<Axis, usize, 3, 32, u32>;

#[derive(Debug, PartialOrd, PartialEq, Eq)]
struct Connection {
    distance: Axis,
    a: usize,
    b: usize,
}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

advent_day!(Day08, 8, Vec<[u32; 3]>);

impl DayParser<Day> for Day {
    fn parse(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| {
                    line.split(',')
                        .map(|v| v.parse::<u32>().unwrap())
                        .collect_array()
                        .unwrap()
                })
                .collect::<Vec<_>>(),
        )
    }
}

impl Day {
    /// ```rust
    /// use advent::*;
    /// use advent_of_code_2025::day08::*;
    /// let day = Day::parse(
    /// r"162,817,812
    /// 57,618,57
    /// 906,360,560
    /// 592,479,940
    /// 352,342,300
    /// 466,668,158
    /// 542,29,236
    /// 431,825,988
    /// 739,650,466
    /// 52,470,668
    /// 216,146,977
    /// 819,987,18
    /// 117,168,530
    /// 805,96,715
    /// 346,949,466
    /// 970,615,88
    /// 941,993,340
    /// 862,61,35
    /// 984,92,344
    /// 425,690,689");
    /// assert_eq!("40", day.part1_impl::<10>());
    /// ```
    pub fn part1_impl<const NEAREST: usize>(&self) -> String {
        let points = self.input();
        let mut point_tree: Tree = KdTree::with_capacity(1000);
        let mut distance_rank: BinaryHeap<Connection> = BinaryHeap::with_capacity(NEAREST + 1);

        let mut worst_distance = Axis::MAX;

        // Fill the kdtree and rank nearest neighbours
        for (index, [x, y, z]) in points.iter().enumerate() {
            let point = &[Axis::from_num(*x), Axis::from_num(*y), Axis::from_num(*z)];

            for nearest in
                point_tree.best_n_within::<SquaredEuclidean>(point, worst_distance, NEAREST)
            {
                distance_rank.push(Connection {
                    distance: nearest.distance,
                    a: index,
                    b: nearest.item,
                });
                if distance_rank.len() > NEAREST {
                    worst_distance = distance_rank.pop().unwrap().distance;
                }
            }
            point_tree.add(point, index);
        }

        let mut node_map = (0..points.len()).collect::<Vec<_>>();
        let mut network_map: Vec<HashSet<usize>> = points
            .iter()
            .enumerate()
            .map(|(index, _)| HashSet::from([index]))
            .collect::<Vec<_>>();
        let network_map = &mut network_map;

        for _ in 0..NEAREST {
            let connection = distance_rank.pop().unwrap();

            let point_a = connection.a;
            let point_b = connection.b;
            let net_a_id = node_map[point_a];
            let net_b_id = node_map[point_b];
            if net_a_id == net_b_id {
                // Already connected
                continue;
            }

            let [net_a, net_b] = network_map.get_disjoint_mut([net_a_id, net_b_id]).unwrap();
            net_a.extend(net_b.iter());
            net_b.iter().for_each(|node| node_map[*node] = net_a_id);
            net_b.clear();
        }

        let mut network_sizes = network_map
            .iter()
            .map(|net| net.len() as u64)
            .collect::<BinaryHeap<_>>();

        let mut answer = 1;
        for _ in 0..3 {
            let size = network_sizes.pop().unwrap();
            answer *= size;
        }
        answer.to_string()
    }
}

impl AdventDay for Day {
    fn part1(&self) -> String {
        self.part1_impl::<1000>()
    }

    /// ```rust
    /// use advent::*;
    /// use advent_of_code_2025::day08::*;
    /// let day = Day::parse(
    /// r"162,817,812
    /// 57,618,57
    /// 906,360,560
    /// 592,479,940
    /// 352,342,300
    /// 466,668,158
    /// 542,29,236
    /// 431,825,988
    /// 739,650,466
    /// 52,470,668
    /// 216,146,977
    /// 819,987,18
    /// 117,168,530
    /// 805,96,715
    /// 346,949,466
    /// 970,615,88
    /// 941,993,340
    /// 862,61,35
    /// 984,92,344
    /// 425,690,689");
    /// assert_eq!("25272", day.part2());
    /// ```
    fn part2(&self) -> String {
        let points = self.input();
        let mut point_tree: Tree = KdTree::with_capacity(1000);
        let mut distance_rank: BinaryHeap<Reverse<Connection>> = BinaryHeap::new();

        // Fill the kdtree and rank nearest neighbours
        for (index, [x, y, z]) in points.iter().enumerate() {
            let point = &[Axis::from_num(*x), Axis::from_num(*y), Axis::from_num(*z)];

            for nearest in point_tree.nearest_n::<SquaredEuclidean>(point, 6) {
                distance_rank.push(Reverse(Connection {
                    distance: nearest.distance,
                    a: index,
                    b: nearest.item,
                }));
            }
            point_tree.add(point, index);
        }

        let mut node_map = (0..points.len()).collect::<Vec<_>>();
        let mut network_map: Vec<HashSet<usize>> = points
            .iter()
            .enumerate()
            .map(|(index, _)| HashSet::from([index]))
            .collect::<Vec<_>>();
        let network_map = &mut network_map;
        while network_map.len() > 1 {
            let Reverse(connection) = distance_rank.pop().unwrap();

            let point_a = connection.a;
            let point_b = connection.b;
            let net_a_id = node_map[point_a];
            let net_b_id = node_map[point_b];

            if net_a_id == net_b_id {
                continue;
            }

            let [net_a, net_b] = network_map.get_disjoint_mut([net_a_id, net_b_id]).unwrap();
            net_a.extend(net_b.iter());
            net_b.iter().for_each(|node| node_map[*node] = net_a_id);
            net_b.clear();

            if network_map[net_a_id].len() == points.len() {
                return (points[point_a][0] as u64 * points[point_b][0] as u64).to_string();
            }
        }

        unreachable!()
    }
}
