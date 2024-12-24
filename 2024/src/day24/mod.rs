use std::{
    collections::HashMap,
    str::FromStr,
    sync::atomic::{AtomicBool, Ordering},
};

use advent::*;
use rayon::prelude::*;
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

#[derive(Debug, Clone, Copy)]
pub struct OpNode<'a> {
    pub lhs: &'a str,
    pub rhs: &'a str,
    pub op: Operator,
    pub out: &'a str,
}

#[derive(Debug, Clone, Copy)]
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
    for n in (0..64).rev() {
        if let Some(solver) = node_solvers.get(format!("z{:02}", n).as_str()) {
            number <<= 1;

            if solver.solve(&node_solvers) {
                number += 1;
            }
        }
    }

    number
}

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
/// assert_eq!(0, part2(&input));
/// ```
pub fn part2(input: &InputType) -> usize {
    0
}
