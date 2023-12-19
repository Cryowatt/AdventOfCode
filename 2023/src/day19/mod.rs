use std::{collections::HashMap, sync::OnceLock};

use advent::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

advent_day!(Day19, parse, PartsSystem, part1, part2);

pub struct PartsSystem<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
    parts: Vec<MachinePart>,
}

struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
    fallback: Target<'a>,
}

impl Workflow<'_> {
    fn apply(&self, part: &MachinePart) -> &Target {
        match self.rules.iter().filter_map(|rule| rule.apply(part)).next() {
            Some(target) => target,
            None => &self.fallback,
        }
    }
}

enum Category {
    X,
    M,
    A,
    S,
}
enum Operator {
    GreaterThan,
    LessThan,
}

enum Target<'a> {
    Accept,
    Reject,
    Workflow(&'a str),
}

struct Rule<'a> {
    category: Category,
    operator: Operator,
    operand: u16,
    target: Target<'a>,
}

impl<'a> Rule<'a> {
    fn apply(&'a self, part: &MachinePart) -> Option<&Target<'a>> {
        let value = match self.category {
            Category::X => part.x,
            Category::M => part.m,
            Category::A => part.a,
            Category::S => part.s,
        };

        let success = match self.operator {
            Operator::GreaterThan => value > self.operand,
            Operator::LessThan => value < self.operand,
        };

        if success {
            Some(&self.target)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MachinePart {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl MachinePart {
    fn score(&self) -> u32 {
        self.x as u32 + self.m as u32 + self.a as u32 + self.s as u32
    }
}

static WORKFLOW_MATCH: OnceLock<Regex> = OnceLock::new();
static PART_MATCH: OnceLock<Regex> = OnceLock::new();
static RULE_MATCH: OnceLock<Regex> = OnceLock::new();

pub fn parse(input: &str) -> PartsSystem {
    let workflow_match = WORKFLOW_MATCH.get_or_init(|| Regex::new(r"(?P<name>[[:lower:]]+)\{(?P<rules>(?:(?:[xmas])(?:[<>])(?:\d+):(?:[AR]|\w+))(?:,(?:(?:[xmas])(?:[<>])(?:\d+):(?:[AR]|\w+)))*),(?P<fallback>[AR]|\w+)\}").unwrap());
    let part_match = PART_MATCH.get_or_init(|| {
        Regex::new(r"\{x=(?P<x>\d+),m=(?P<m>\d+),a=(?P<a>\d+),s=(?P<s>\d+)\}").unwrap()
    });

    fn parse_workflow<'a>(name: &'a str, rules: &'a str, fallback: &'a str) -> Workflow<'a> {
        let rule_match = RULE_MATCH.get_or_init(|| {
            Regex::new(r"(?:(?P<cat>[xmas])(?P<op>[<>])(?P<operand>\d+):(?P<target>[AR]|\w+))")
                .unwrap()
        });

        Workflow {
            name,
            rules: rule_match
                .captures_iter(rules)
                .map(|rule_capture| Rule {
                    category: parse_category(rule_capture.name("cat").unwrap().as_str()),
                    operator: parse_operator(rule_capture.name("op").unwrap().as_str()),
                    operand: rule_capture
                        .name("operand")
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap(),
                    target: parse_target(rule_capture.name("target").unwrap().as_str()),
                })
                .collect(),
            fallback: parse_target(fallback),
        }
    }

    fn parse_category(category: &str) -> Category {
        match category.bytes().next().unwrap() {
            b'x' => Category::X,
            b'm' => Category::M,
            b'a' => Category::A,
            b's' => Category::S,
            _ => unreachable!(),
        }
    }

    fn parse_operator(operator: &str) -> Operator {
        match operator.bytes().next().unwrap() {
            b'<' => Operator::LessThan,
            b'>' => Operator::GreaterThan,
            _ => unreachable!(),
        }
    }

    fn parse_target(target: &str) -> Target {
        match target.bytes().next().unwrap() {
            b'A' => Target::Accept,
            b'R' => Target::Reject,
            _ => Target::Workflow(target),
        }
    }

    PartsSystem {
        workflows: input
            .lines()
            .filter_map(|line| {
                workflow_match.captures(line).map(|workflow| {
                    (
                        workflow.name("name").unwrap().as_str(),
                        parse_workflow(
                            workflow.name("name").unwrap().as_str(),
                            workflow.name("rules").unwrap().as_str(),
                            workflow.name("fallback").unwrap().as_str(),
                        ),
                    )
                })
            })
            .collect(),
        parts: input
            .lines()
            .filter_map(|line| {
                part_match.captures(line).map(|part| MachinePart {
                    x: part.name("x").unwrap().as_str().parse().unwrap(),
                    m: part.name("m").unwrap().as_str().parse().unwrap(),
                    a: part.name("a").unwrap().as_str().parse().unwrap(),
                    s: part.name("s").unwrap().as_str().parse().unwrap(),
                })
            })
            .collect(),
    }
}

/// ```rust
/// use advent_of_code_2023::day19::*;
/// let input = parse(
/// r"px{a<2006:qkq,m>2090:A,rfg}
/// pv{a>1716:R,A}
/// lnx{m>1548:A,A}
/// rfg{s<537:gd,x>2440:R,A}
/// qs{s>3448:A,lnx}
/// qkq{x<1416:A,crn}
/// crn{x>2662:A,R}
/// in{s<1351:px,qqz}
/// qqz{s>2770:qs,m<1801:hdj,R}
/// gd{a>3333:R,R}
/// hdj{m>838:A,pv}
///
/// {x=787,m=2655,a=1222,s=2876}
/// {x=1679,m=44,a=2067,s=496}
/// {x=2036,m=264,a=79,s=2244}
/// {x=2461,m=1339,a=466,s=291}
/// {x=2127,m=1623,a=2188,s=1013}");
/// assert_eq!(19114, part1(&input));
/// ```
pub fn part1(system: &PartsSystem) -> u32 {
    system
        .parts
        .par_iter()
        .filter_map(|part| {
            let mut workflow = system.workflows.get("in").unwrap();

            loop {
                match workflow.apply(part) {
                    Target::Accept => return Some(part.score()),
                    Target::Reject => return None,
                    Target::Workflow(target_workflow) => {
                        workflow = system.workflows.get(target_workflow).unwrap()
                    }
                }
            }
        })
        .sum()
}

/// ```rust
/// use advent_of_code_2023::day19::*;
/// let input = parse(
/// r"px{a<2006:qkq,m>2090:A,rfg}
/// pv{a>1716:R,A}
/// lnx{m>1548:A,A}
/// rfg{s<537:gd,x>2440:R,A}
/// qs{s>3448:A,lnx}
/// qkq{x<1416:A,crn}
/// crn{x>2662:A,R}
/// in{s<1351:px,qqz}
/// qqz{s>2770:qs,m<1801:hdj,R}
/// gd{a>3333:R,R}
/// hdj{m>838:A,pv}
///
/// {x=787,m=2655,a=1222,s=2876}
/// {x=1679,m=44,a=2067,s=496}
/// {x=2036,m=264,a=79,s=2244}
/// {x=2461,m=1339,a=466,s=291}
/// {x=2127,m=1623,a=2188,s=1013}");
/// assert_eq!(167409079868000, part2(&input));
/// ```
pub fn part2(system: &PartsSystem) -> u64 {
    // FUCK
    0
}
