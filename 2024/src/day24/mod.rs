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

pub fn shuffle_attempt() -> bool {
    let input = parse(&INPUT);
    let swaps = part2(&input).split(',').collect::<Vec<_>>();

    // for

    // false
    true
}

/// ```rust
/// use advent_of_code_2024::day24::*;
/// assert_eq!(true, shuffle_attempt());
/// ```
pub fn part2(input: &InputType) -> String {
    println!();
    println!();
    println!();
    let (_init, nodes) = input;
    // Expectations:
    // x + y into AND and XOR
    // XOR -> z

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

    let mut bad_nodes = vec![];

    for node in nodes {
        let deps = dependency_lookup.get(node.out);
        match node.op {
            Operator::And => {
                // AND -> OR
                if node.lhs == "x00" || node.rhs == "x00" {
                    // This is fine
                } else {
                    if let Some(deps) = deps {
                        if deps.len() == 1 {
                            if deps.iter().next().unwrap().op != Operator::Or {
                                println!("WRONG DEP OP {:?} {:?}", node, deps);
                                bad_nodes.push(node.out);
                            }
                        } else {
                            bad_nodes.push(node.out);
                            println!("UNEXPECTED DEPS {:?} {:?}", node, deps);
                        }
                    } else {
                        bad_nodes.push(node.out);
                        println!("NO DEPS? {:?}", node);
                    }
                }
            }
            Operator::Or => {
                // OR -> [XOR, AND]
                if node.out == "z45" {
                    // This is fine
                } else {
                    if let Some(deps) = deps {
                        if deps.len() == 2 {
                            if !deps.iter().any(|node| node.op == Operator::Xor)
                                || !deps.iter().any(|node| node.op == Operator::And)
                            {
                                bad_nodes.push(node.out);
                                println!("WRONG DEPS {:?} {:?}", node, deps);
                            }
                        } else {
                            bad_nodes.push(node.out);
                            println!("UNEXPECTED DEPS {:?} {:?}", node, deps);
                        }
                    } else {
                        bad_nodes.push(node.out);
                        println!("NO DEPS? {:?}", node);
                    }
                }
            }
            Operator::Xor => {
                // XOR -> [XOR, AND]
                // XOR -> [OUT]
                if node.out.starts_with("z") {
                    // Output node
                } else {
                    if let Some(deps) = deps {
                        if deps.len() == 2 {
                            if !deps.iter().any(|node| {
                                node.op == Operator::Xor
                                    && dependency_lookup[&node.out].out.starts_with("z")
                            }) || !deps.iter().any(|node| node.op == Operator::And)
                            {
                                bad_nodes.push(node.out);
                                println!("WRONG DEPS {:?} {:?}", node, deps);
                            }
                        } else {
                            bad_nodes.push(node.out);
                            println!("UNEXPECTED DEPS {:?} {:?}", node, deps);
                        }
                    } else {
                        bad_nodes.push(node.out);
                        println!("NO DEPS? {:?}", node);
                    }
                }
            }
        }
    }

    // println!();
    // println!();
    println!();
    println!();
    println!();
    println!("THE BADDIES {:?}", bad_nodes);

    bad_nodes.sort();
    bad_nodes.join(",")

    // fn guess_node_op<'a>(
    //     node: &'a OpNode<'a>,
    //     dependency_lookup: &mut HashMap<&str, HashSet<&'a OpNode<'a>>>,
    // ) {
    //     // node
    // }

    // // println!("{:?}", dependency_lookup);
    // for xor_node in nodes.iter().filter(|node| node.op == Operator::Xor) {
    //     if (xor_node.lhs.starts_with('x') && xor_node.rhs.starts_with('y'))
    //         || (xor_node.lhs.starts_with('y') && xor_node.rhs.starts_with('x'))
    //             && xor_node.out != "z00"
    //     {
    //         // Node is half-sum, should output to full-sum (XOR) and carry-chain (AND)
    //         if let Some(deps) = dependency_lookup.get(xor_node.out) {
    //             if deps.iter().find(|node| node.op == Operator::Xor).is_none()
    //                 || deps.iter().find(|node| node.op == Operator::And).is_none()
    //             {
    //                 println!("BAD ADDER OUT: {:?}", xor_node);
    //             }
    //         } else {
    //             println!("BAD ADDER: {:?}", xor_node);
    //         }
    //     } else if !xor_node.out.starts_with('z') {
    //         // Node is full-sum, should output to z
    //         println!("BAD SUM OUT: {:?}", xor_node);
    //     }
    // }

    // let fk = INPUT;

    // // All XORs come from x+y _or_ out z
    // for node in nodes
    //     .iter()
    //     .filter(|node| node.op != Operator::Xor && node.out.starts_with("z") && node.out != "z45")
    // {
    //     println!("BAD OUT {:?}", node);
    // }

    // for or_node in nodes.iter().filter(|node| node.op == Operator::Or) {
    //     // Only carry
    //     // Carry -> XOR -> ZOut
    //     // AND & AND -> Carry

    //     if let Some(deps) = dependency_lookup.get(or_node.out) {
    //         if deps
    //             .iter()
    //             .find(|node| node.out.starts_with("z") && node.op == Operator::Xor)
    //             .is_none()
    //         {
    //             println!("BAD CARRY {:?}", or_node);
    //         }
    //     } else {
    //         println!("BAD CARRY OUT {:?}", or_node);
    //     }
    // }

    // for and_node in nodes.iter().filter(|node| node.op == Operator::And) {
    //     // Only outputs to OR
    //     let deps = dependency_lookup.get(and_node.out).unwrap();
    //     if !deps.iter().all(|node| node.op == Operator::Or) {
    //         println!("BAD CARRY BLOCK {:?} {:?}", and_node, deps);
    //     }
    // }

    // // println!("Swapped pairs");
    // // for node in nodes.iter().filter(|node| {
    // //     node.op == Operator::Xor
    // //         && !node.out.starts_with("z")
    // //         && !(node.lhs.starts_with('x') || node.rhs.starts_with('x'))
    // // }) {
    // //     println!("{:?}", node);
    // // }

    // fn find_node<'a>(
    //     input_a: &str,
    //     input_b: &str,
    //     dependency_lookup: &mut HashMap<&str, HashSet<&'a OpNode<'a>>>,
    // ) -> Option<Vec<&'a OpNode<'a>>> {
    //     // println!("FInd node {} {}", input_a, input_b);
    //     let deps_a = dependency_lookup.get(input_a)?;
    //     let deps_b = dependency_lookup.get(input_b)?;

    //     let nodes = deps_a.intersection(deps_b).cloned().collect::<Vec<_>>();
    //     if nodes.is_empty() {
    //         println!("Failed to find node {}&{}: {:?}", input_a, input_b, nodes);
    //         None
    //     } else {
    //         Some(nodes)
    //     }
    // }

    // enum FindResult<'a> {
    //     Err,
    //     Partial(&'a OpNode<'a>, &'a str),
    //     Ok(&'a OpNode<'a>),
    // }

    // // fn find_node_op<'a>(
    // //     input_a: &str,
    // //     input_b: &str,
    // //     dependency_lookup: &mut HashMap<&str, HashSet<&'a OpNode<'a>>>,
    // // ) -> FindResult<'a> {
    // //     // println!("FInd node {} {}", input_a, input_b);
    // //     let deps_a = dependency_lookup.get(input_a).unwrap();
    // //     let deps_b = dependency_lookup.get(input_b).unwrap();

    // //     let nodes = deps_a.intersection(deps_b).cloned().collect::<Vec<_>>();
    // //     if nodes.is_empty() {
    // //         println!("Failed to find node {}&{}: {:?}", input_a, input_b, nodes);
    // //         None
    // //     } else {
    // //         Some(nodes)
    // //     }
    // // }

    // let mut full_sum: Option<&OpNode<'_>> = None;
    // let mut full_carry: Option<&OpNode<'_>> = None;

    // for node in find_node("x00", "y00", dependency_lookup).unwrap() {
    //     if let Some(_dupe) = match node.op {
    //         Operator::Xor => full_sum.replace(node),
    //         Operator::And => full_carry.replace(node),
    //         Operator::Or => {
    //             panic!("Fuck")
    //         }
    //     } {
    //         panic!("Fuck")
    //     }
    // }

    // for bit in 1..45 {
    //     let x_label = format!("x{:02}", bit);
    //     let y_label = format!("y{:02}", bit);

    //     print!(
    //         "Bit {} A:{}, B:{}, carry-in:{} ",
    //         bit,
    //         x_label,
    //         y_label,
    //         full_carry.unwrap().out
    //     );

    //     let mut half_sum: Option<&OpNode<'_>> = None;
    //     let mut half_carry: Option<&OpNode<'_>> = None;
    //     let mut carry_chain: Option<&OpNode<'_>> = None;

    //     // let x = dependency_lookup.get(&x_label);
    //     // let y = dependency_lookup.get(&y_label);

    //     for node in find_node(&x_label, &y_label, dependency_lookup).unwrap() {
    //         if let Some(dupe) = match node.op {
    //             Operator::Xor => half_sum.replace(node),
    //             Operator::And => half_carry.replace(node),
    //             Operator::Or => {
    //                 panic!("Fuck")
    //             }
    //         } {
    //             println!("Found duplicate for {}: {:?} and {:?}", bit, node, dupe);
    //         }
    //     }

    //     for node in find_node(
    //         half_sum.unwrap().out,
    //         full_carry.unwrap().out,
    //         dependency_lookup,
    //     )
    //     .unwrap()
    //     {
    //         match node.op {
    //             Operator::And => carry_chain.replace(node),
    //             Operator::Or => panic!("fuck"),
    //             Operator::Xor => full_sum.replace(node),
    //         };
    //     }

    //     full_carry.replace(
    //         find_node(
    //             half_carry.unwrap().out,
    //             carry_chain.unwrap().out,
    //             dependency_lookup,
    //         )
    //         .unwrap()
    //         .first()
    //         .unwrap(),
    //     );

    //     println!(
    //         "half-sum:{}, full-sum:{}, half-carry:{}, carry-chair:{}, carry-out: {}",
    //         half_sum.unwrap().out,
    //         full_sum.unwrap().out,
    //         half_carry.unwrap().out,
    //         carry_chain.unwrap().out,
    //         full_carry.unwrap().out,
    //     );
    // // Full sum is half sum + carry-in (the previous full-carry)
    // full_sum.replace(
    //     find_node(
    //         half_sum.unwrap().out,
    //         full_carry.unwrap().out,
    //         dependency_lookup,
    //     )
    //     .iter()
    //     .find(|node| node.op == Operator::And)
    //     .unwrap(),
    // );

    // // Carry chain is half-sum and carry in
    // carry_chain.replace(
    //     find_node(
    //         half_sum.unwrap().out,
    //         full_carry.unwrap().out,
    //         dependency_lookup,
    //     )
    //     .iter()
    //     .find(|node| node.op == Operator::And)
    //     .unwrap(),
    // );

    // let fk = find_node(
    //     half_sum.unwrap().out,
    //     half_carry.unwrap().out,
    //     dependency_lookup,
    // );
    // if let Some(deps) = dependency_lookup.get(half_carry.unwrap().out) {
    //     if deps.len() > 1 {
    //         println!("{:?}", deps);
    //     }
    // } else {
    //     println!("Missing full-carry bit{:02}", bit);
    // }
    // println!("{:?}", fk);
    // let bit_sum =
    // let bit_carry
    // }

    // nodes.iter().flat_map(|node| [(node.lhs, node.out), (node.rhs,node.out)])

    // let mut nodes = nodes
    //     .iter()
    //     .map(|node| (node.out, node))
    //     .collect::<HashMap<_, _>>();

    // let mut bit_sum = [None; 64];
    // let mut bit_carry = [None; 64];

    // for node in nodes {
    //     if node.lhs.starts_with("x") {
    //         if node.rhs.starts_with("y") && node.rhs.ends_with(&node.lhs[1..]) {
    //             let bit = node.lhs[1..].parse::<u8>().unwrap();
    //             match node.op {
    //                 Operator::And => bit_carry[bit as usize] = Some(node.out),
    //                 Operator::Xor => bit_sum[bit as usize] = Some(node.out),
    //                 Operator::Or => panic!(),
    //             }
    //         } else {
    //             panic!();
    //         }
    //     } else if node.lhs.starts_with("y") {
    //         if node.rhs.starts_with("x") && node.rhs.ends_with(&node.rhs[1..]) {
    //             let bit = node.lhs[1..].parse::<u8>().unwrap();
    //             match node.op {
    //                 Operator::And => bit_carry[bit as usize] = Some(node.out),
    //                 Operator::Xor => bit_sum[bit as usize] = Some(node.out),
    //                 Operator::Or => panic!(),
    //             }
    //         } else {
    //             panic!();
    //         }
    //     }
    // }

    // let mut bit_carry_sum = [None; 64];

    // for node in nodes {
    //     if node.lhs.starts_with("x") {
    //         if node.rhs.starts_with("y") && node.rhs.ends_with(&node.lhs[1..]) {
    //             let bit = node.lhs[1..].parse::<u8>().unwrap();
    //             match node.op {
    //                 Operator::And => bit_carry[bit as usize] = Some(node.out),
    //                 Operator::Xor => bit_sum[bit as usize] = Some(node.out),
    //                 Operator::Or => panic!(),
    //             }
    //         } else {
    //             panic!();
    //         }
    //     } else if node.lhs.starts_with("y") {
    //         if node.rhs.starts_with("x") && node.rhs.ends_with(&node.rhs[1..]) {
    //             let bit = node.lhs[1..].parse::<u8>().unwrap();
    //             match node.op {
    //                 Operator::And => bit_carry[bit as usize] = Some(node.out),
    //                 Operator::Xor => bit_sum[bit as usize] = Some(node.out),
    //                 Operator::Or => panic!(),
    //             }
    //         } else {
    //             panic!();
    //         }
    //     }
    // }
    // let mut node_solvers = nodes
    //     .iter()
    //     .map(|node| (node.out, node))
    //     .collect::<HashMap<_, _>>();
    // println!("{:?}", node_solvers);
    // let fk = super::day24::INPUT;
    // String::new()
}
