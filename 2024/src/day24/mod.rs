use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    sync::atomic::{AtomicBool, Ordering},
};

use advent::*;
use regex::Regex;

advent_day!(
    Day24,
    parse,
    (HashMap<&'a str, bool>, Vec<OpNode<'a>>),
    part1,
    part2
);

pub fn parse(input: &str) -> InputType {
    let parser =
        Regex::new(r"(?<lhs>\w{3}) (?<op>AND|OR|XOR) (?<rhs>\w{3}) -> (?<out>\w{3})").unwrap();
    let init = HashMap::from_iter(
        input
            .lines()
            .take_while(|line| !line.is_empty())
            .filter_map(|line| line.split_once(": "))
            .map(|(label, value)| (label, value == "1")),
    );

    let nodes = input
        .lines()
        .skip_while(|line| !line.contains("->"))
        .map(|line| parser.captures(line).unwrap())
        .map(|cap| OpNode {
            lhs: cap.name("lhs").unwrap().as_str(),
            rhs: cap.name("rhs").unwrap().as_str(),
            out: cap.name("out").unwrap().as_str(),
            op: cap.name("op").unwrap().as_str().parse().unwrap(),
        })
        .collect::<Vec<_>>();
    (init, nodes)
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct OpNode<'a> {
    pub lhs: &'a str,
    pub rhs: &'a str,
    pub op: Operator,
    pub out: &'a str,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Operator {
    And,
    Or,
    Xor,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Operator::And),
            "OR" => Ok(Operator::Or),
            "XOR" => Ok(Operator::Xor),
            _ => Err(()),
        }
    }
}

/// ```rust
/// use advent_of_code_2024::day24::*;
/// let input = parse(
/// r"x00: 1
/// x01: 1
/// x02: 1
/// y00: 0
/// y01: 1
/// y02: 0
///
/// x00 AND y00 -> z00
/// x01 XOR y01 -> z01
/// x02 OR y02 -> z02");
/// assert_eq!(4, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2024::day24::*;
/// let input = parse(
/// r"x00: 1
/// x01: 0
/// x02: 1
/// x03: 1
/// x04: 0
/// y00: 1
/// y01: 1
/// y02: 1
/// y03: 1
/// y04: 1
///
/// ntg XOR fgs -> mjb
/// y02 OR x01 -> tnw
/// kwq OR kpj -> z05
/// x00 OR x03 -> fst
/// tgd XOR rvg -> z01
/// vdt OR tnw -> bfw
/// bfw AND frj -> z10
/// ffh OR nrd -> bqk
/// y00 AND y03 -> djm
/// y03 OR y00 -> psh
/// bqk OR frj -> z08
/// tnw OR fst -> frj
/// gnj AND tgd -> z11
/// bfw XOR mjb -> z00
/// x03 OR x00 -> vdt
/// gnj AND wpb -> z02
/// x04 AND y00 -> kjc
/// djm OR pbm -> qhw
/// nrd AND vdt -> hwm
/// kjc AND fst -> rvg
/// y04 OR y02 -> fgs
/// y01 AND x02 -> pbm
/// ntg OR kjc -> kwq
/// psh XOR fgs -> tgd
/// qhw XOR tgd -> z09
/// pbm OR djm -> kpj
/// x03 XOR y03 -> ffh
/// x00 XOR y04 -> ntg
/// bfw OR bqk -> z06
/// nrd XOR fgs -> wpb
/// frj XOR qhw -> z04
/// bqk OR frj -> z07
/// y03 OR x01 -> nrd
/// hwm AND bqk -> z03
/// tgd XOR rvg -> z12
/// tnw OR pbm -> gnj");
/// assert_eq!(2024, part1(&input));
/// ```
pub fn part1(input: &InputType) -> u64 {
    let (init, nodes) = input;

    struct NodeSolver<'a> {
        node: Option<&'a OpNode<'a>>,
        result: AtomicBool,
        resolved: AtomicBool,
    }

    impl NodeSolver<'_> {
        fn solve(&self, solvers: &HashMap<&str, NodeSolver<'_>>) -> bool {
            if self.resolved.load(Ordering::Relaxed) {
                self.result.load(Ordering::Relaxed)
            } else if let Some(node) = self.node {
                let result = Self::solve_node(node, solvers);
                self.result.store(result, Ordering::Relaxed);
                self.resolved.store(true, Ordering::Relaxed);
                result
            } else {
                panic!();
            }
        }

        fn solve_node(node: &OpNode<'_>, solvers: &HashMap<&str, NodeSolver<'_>>) -> bool {
            let lhs = Self::resolve_operand(node.lhs, solvers);
            let rhs = Self::resolve_operand(node.rhs, solvers);
            match node.op {
                Operator::And => lhs & rhs,
                Operator::Or => lhs | rhs,
                Operator::Xor => lhs ^ rhs,
            }
        }

        fn resolve_operand(label: &str, solvers: &HashMap<&str, NodeSolver<'_>>) -> bool {
            let node = solvers.get(label).unwrap();
            if node.resolved.load(Ordering::Relaxed) {
                node.result.load(Ordering::Relaxed)
            } else {
                node.solve(solvers)
            }
        }
    }

    let mut node_solvers = nodes
        .iter()
        .map(|node| {
            (
                node.out,
                NodeSolver {
                    node: Some(node),
                    result: AtomicBool::new(false),
                    resolved: AtomicBool::new(false),
                },
            )
        })
        .collect::<HashMap<_, _>>();

    node_solvers.extend(init.iter().map(|(&label, &value)| {
        (
            label,
            NodeSolver {
                node: None,
                result: AtomicBool::new(value),
                resolved: AtomicBool::new(true),
            },
        )
    }));

    let mut number = 0;
    for n in (0..45).rev() {
        if let Some(solver) = node_solvers.get(format!("z{:02}", n).as_str()) {
            number <<= 1;

            if solver.solve(&node_solvers) {
                number += 1;
            }
        }
    }

    number
}

pub fn part2<'a>(input: &'a InputType) -> String {
    const X_NODES: [&'static str; 45] = [
        "x00", "x01", "x02", "x03", "x04", "x05", "x06", "x07", "x08", "x09", "x10", "x11", "x12",
        "x13", "x14", "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24", "x25",
        "x26", "x27", "x28", "x29", "x30", "x31", "x32", "x33", "x34", "x35", "x36", "x37", "x38",
        "x39", "x40", "x41", "x42", "x43", "x44",
    ];
    const Y_NODES: [&'static str; 45] = [
        "y00", "y01", "y02", "y03", "y04", "y05", "y06", "y07", "y08", "y09", "y10", "y11", "y12",
        "y13", "y14", "y15", "y16", "y17", "y18", "y19", "y20", "y21", "y22", "y23", "y24", "y25",
        "y26", "y27", "y28", "y29", "y30", "y31", "y32", "y33", "y34", "y35", "y36", "y37", "y38",
        "y39", "y40", "y41", "y42", "y43", "y44",
    ];
    const Z_NODES: [&'static str; 45] = [
        "z00", "z01", "z02", "z03", "z04", "z05", "z06", "z07", "z08", "z09", "z10", "z11", "z12",
        "z13", "z14", "z15", "z16", "z17", "z18", "z19", "z20", "z21", "z22", "z23", "z24", "z25",
        "z26", "z27", "z28", "z29", "z30", "z31", "z32", "z33", "z34", "z35", "z36", "z37", "z38",
        "z39", "z40", "z41", "z42", "z43", "z44",
    ];
    let (_init, nodes) = input;

    let dependency_lookup: &mut HashMap<&str, HashSet<&OpNode<'_>>> = &mut HashMap::new();
    let node_map = &mut HashMap::new();

    for node in nodes {
        node_map.insert(node.out, node);
        if let Some(edges) = dependency_lookup.get_mut(node.lhs) {
            edges.insert(node);
        } else {
            dependency_lookup.insert(node.lhs, HashSet::from_iter([node]));
        }

        if let Some(edges) = dependency_lookup.get_mut(node.rhs) {
            edges.insert(node);
        } else {
            dependency_lookup.insert(node.rhs, HashSet::from_iter([node]));
        }
    }

    let bad_nodes: &mut HashSet<&'a str> = &mut HashSet::new();

    let (_, half_carry0) = find_half_adder(X_NODES[0], Y_NODES[0], dependency_lookup, bad_nodes);

    let mut carry = half_carry0;

    for bit in 1..45 {
        let (_, full_carry) = find_full_adder(
            X_NODES[bit],
            Y_NODES[bit],
            carry,
            Z_NODES[bit],
            dependency_lookup,
            bad_nodes,
        );
        carry = full_carry;
    }

    fn find_half_adder<'a>(
        input_a: &'a str,
        input_b: &'a str,
        dependency_lookup: &mut HashMap<&str, HashSet<&'a OpNode<'a>>>,
        bad_nodes: &mut HashSet<&'a str>,
    ) -> (&'a str, &'a str) {
        let sum =
            find_node(input_a, input_b, Operator::Xor, dependency_lookup).unwrap_or_else(|| {
                let (pair, bad_input_id) =
                    partial_find_node(input_a, input_b, Operator::Xor, dependency_lookup);
                bad_nodes.insert(bad_input_id);

                pair
            });
        let carry =
            find_node(input_a, input_b, Operator::And, dependency_lookup).unwrap_or_else(|| {
                let (pair, bad_input_id) =
                    partial_find_node(input_a, input_b, Operator::And, dependency_lookup);
                bad_nodes.insert(bad_input_id);
                pair
            });

        (sum.out, carry.out)
    }

    fn find_full_adder<'a>(
        input_a: &'a str,
        input_b: &'a str,
        carry_in: &'a str,
        expected_out: &'a str,
        dependency_lookup: &mut HashMap<&str, HashSet<&'a OpNode<'a>>>,
        bad_nodes: &mut HashSet<&'a str>,
    ) -> (&'a str, &'a str) {
        let mut last_error = None;
        let (half_sum, half_carry) =
            find_half_adder(input_a, input_b, dependency_lookup, bad_nodes);

        if half_carry == expected_out {
            bad_nodes.insert(half_carry);
            last_error.replace(half_carry);
        }

        if half_sum == expected_out {
            bad_nodes.insert(half_sum);
            last_error.replace(half_sum);
        }

        let (full_sum, full_carry) =
            find_half_adder(half_sum, carry_in, dependency_lookup, bad_nodes);

        if full_sum != expected_out {
            bad_nodes.insert(full_sum);
            last_error.replace(full_sum);
        }

        if full_carry == expected_out {
            bad_nodes.insert(full_carry);
            last_error.replace(full_carry);
        }

        let carry_out = find_node(half_carry, full_carry, Operator::Or, dependency_lookup)
            .unwrap_or_else(|| {
                let (node, bad_input) =
                    partial_find_node(half_carry, full_carry, Operator::Or, dependency_lookup);

                bad_nodes.insert(bad_input);
                node
            });

        if carry_out.out == expected_out {
            bad_nodes.insert(carry_out.out);

            (full_sum, last_error.unwrap())
        } else {
            (full_sum, carry_out.out)
        }
    }

    fn partial_find_node<'a>(
        input_a: &'a str,
        input_b: &'a str,
        operator: Operator,
        dependency_lookup: &mut HashMap<&str, HashSet<&'a OpNode<'a>>>,
    ) -> (&'a OpNode<'a>, &'a str) {
        let node_a = dependency_lookup
            .get(input_a)
            .map(|deps| deps.iter().filter(|dep| dep.op == operator).next())
            .flatten();

        let node_b = dependency_lookup
            .get(input_b)
            .map(|deps| deps.iter().filter(|dep| dep.op == operator).next())
            .flatten();

        if node_a.is_some() {
            (node_a.unwrap(), input_b)
        } else if node_b.is_some() {
            (node_b.unwrap(), input_a)
        } else {
            panic!("Can't find candidate");
        }
    }

    fn find_node<'a>(
        input_a: &str,
        input_b: &str,
        operator: Operator,
        dependency_lookup: &mut HashMap<&str, HashSet<&'a OpNode<'a>>>,
    ) -> Option<&'a OpNode<'a>> {
        let deps_a = dependency_lookup.get(input_a)?;
        let deps_b = dependency_lookup.get(input_b)?;

        deps_a
            .intersection(deps_b)
            .filter(|dep| dep.op == operator)
            .cloned()
            .next()
    }

    let mut answer_nodes = Vec::from_iter(bad_nodes.iter().cloned());
    answer_nodes.sort();
    answer_nodes.join(",")
}
