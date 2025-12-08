use std::collections::{BinaryHeap, HashSet};

use advent::*;

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

        // Fill the kdtree and rank nearest neighbours
        for (index, [x, y, z]) in points.iter().enumerate() {
            let point = &[Axis::from_num(*x), Axis::from_num(*y), Axis::from_num(*z)];

            for nearest in point_tree.best_n_within::<SquaredEuclidean>(
                point,
                distance_rank.peek().map_or(Axis::MAX, |x| x.distance),
                NEAREST,
            ) {
                distance_rank.push(Connection {
                    distance: nearest.distance,
                    a: index,
                    b: nearest.item,
                });
                if distance_rank.len() > NEAREST {
                    distance_rank.pop();
                }
            }
            point_tree.add(point, index);
        }

        let mut network_map: Vec<HashSet<usize>> = vec![];

        for _ in 0..NEAREST {
            let connection = distance_rank.pop().unwrap();

            let point_a = connection.a;
            let point_b = connection.b;
            let net_a_id = network_map
                .iter()
                .position(|network| network.contains(&point_a));
            let net_b_id = network_map
                .iter()
                .position(|network| network.contains(&point_b));

            if let Some(net_a_id) = net_a_id {
                // Point A is already on a network
                if let Some(net_b_id) = net_b_id {
                    if net_a_id == net_b_id {
                        // Already connected
                    } else {
                        // Network merge
                        let low_id = net_a_id.min(net_b_id);
                        let high_id = net_a_id.max(net_b_id);
                        let net = network_map
                            .swap_remove(high_id)
                            .union(&network_map.swap_remove(low_id))
                            .copied()
                            .collect();
                        network_map.push(net);
                    }
                } else {
                    // Add Point B to Network A
                    network_map.get_mut(net_a_id).unwrap().insert(point_b);
                }
            } else if let Some(net_b_id) = net_b_id {
                // Add Point A to Network B
                network_map.get_mut(net_b_id).unwrap().insert(point_a);
            } else {
                // This is a new network
                let net = HashSet::from([point_a, point_b]);
                network_map.push(net);
            }
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

// Too high:  400309
// Too high: 1078000

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
    /// assert_eq!("", day.part2());
    /// ```
    fn part2(&self) -> String {
        "".to_string()
    }
}
