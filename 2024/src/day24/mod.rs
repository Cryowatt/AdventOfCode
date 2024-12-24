use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
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
pub fn part1(input: &InputType) -> usize {
    0
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
