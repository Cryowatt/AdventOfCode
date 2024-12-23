use std::collections::{HashMap, HashSet};

use advent::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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
pub fn part1(input: &InputType) -> u64 {
    let mut edges: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut t_nodes = HashSet::new();

    for (node_a, node_b) in input.iter() {
        if node_a.contains('t') {
            t_nodes.insert(node_a);
        }

        if node_b.contains('t') {
            t_nodes.insert(node_b);
        }

        match edges.get_mut(node_a) {
            Some(node_edges) => node_edges,
            None => edges.insert(node_a, HashSet::new()).unwrap(),
        }

        match edges.get_mut(node_b) {
            Some(node_edges) => node_edges.push(node_a),
            None => _ = edges.insert(node_b, vec![*node_a]),
        }
    }

    let mut count = 0;
    let mut net = vec![];
    for node in t_nodes {
        net.push(node);
        for child_node in edges.get(node).unwrap().iter()
        // .filter(|child_node| !child_node.contains('t') || node < child_node)
        {
            net.push(child_node);

            for grandchild_node in edges.get(child_node).unwrap().iter()
            // .filter(|&grandchild_node| {
            //     grandchild_node != node
            //         && (!grandchild_node.contains('t') || node < grandchild_node)
            // })
            {
                net.push(grandchild_node);
                println!("{:?}", net);
                count += 1;
                net.pop();
            }
            net.pop();
        }
        net.pop();
    }
    println!("COUNT: {}", count);

    0
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
pub fn part2(input: &InputType) -> i32 {
    0
}
