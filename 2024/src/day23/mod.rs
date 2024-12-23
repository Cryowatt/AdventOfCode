use std::collections::{HashMap, HashSet};

use advent::*;
use rayon::prelude::*;

advent_day!(Day23, parse, Vec<(&'a str, &'a str)>, part1, part2);

pub fn parse(input: &str) -> InputType {
    input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect()
}

/// ```rust
/// use advent_of_code_2024::day23::*;
/// let input = parse(
/// r"kh-tc
/// qp-kh
/// de-cg
/// ka-co
/// yn-aq
/// qp-ub
/// cg-tb
/// vc-aq
/// tb-ka
/// wh-tc
/// yn-cg
/// kh-ub
/// ta-co
/// de-co
/// tc-td
/// tb-wq
/// wh-td
/// ta-ka
/// td-qp
/// aq-cg
/// wq-ub
/// ub-vc
/// de-ta
/// wq-aq
/// wq-vc
/// wh-yn
/// ka-de
/// kh-ta
/// co-tc
/// wh-qp
/// tb-vc
/// td-yn");
/// assert_eq!(7, part1(&input));
/// ```
/// TOO HIGH: 2335
pub fn part1(input: &InputType) -> usize {
    let mut edges: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut all_nodes = HashSet::new();

    for (node_a, node_b) in input.iter() {
        all_nodes.insert(node_a);
        all_nodes.insert(node_b);

        if let Some(edge_set) = edges.get_mut(node_a) {
            edge_set.insert(node_b);
        } else {
            edges.insert(node_a, HashSet::from([*node_b]));
        }

        if let Some(edge_set) = edges.get_mut(node_b) {
            edge_set.insert(node_a);
        } else {
            edges.insert(node_b, HashSet::from([*node_a]));
        }
    }

    all_nodes
        .par_iter()
        .map(|&node| {
            let pairs = edges.get(node).unwrap();
            pairs
                .iter()
                .filter(|&pair_node| node < pair_node)
                .map(|&pair_node| {
                    let trios = edges
                        .get(pair_node)
                        .unwrap()
                        .intersection(pairs)
                        .filter(|&trio_node| pair_node < trio_node);

                    trios
                        .filter(|&trio_node| {
                            node.starts_with('t')
                                || pair_node.starts_with('t')
                                || trio_node.starts_with('t')
                        })
                        .count()
                })
                .sum::<usize>()
        })
        .sum()
}

/// ```rust
/// use advent_of_code_2024::day23::*;
/// let input = parse(
/// r"kh-tc
/// qp-kh
/// de-cg
/// ka-co
/// yn-aq
/// qp-ub
/// cg-tb
/// vc-aq
/// tb-ka
/// wh-tc
/// yn-cg
/// kh-ub
/// ta-co
/// de-co
/// tc-td
/// tb-wq
/// wh-td
/// ta-ka
/// td-qp
/// aq-cg
/// wq-ub
/// ub-vc
/// de-ta
/// wq-aq
/// wq-vc
/// wh-yn
/// ka-de
/// kh-ta
/// co-tc
/// wh-qp
/// tb-vc
/// td-yn");
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(_input: &InputType) -> i32 {
    0
}
