use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use advent::*;
use num::Integer;
use regex::Regex;

advent_day!(Day20, parse, Vec<ModuleDefinition>, part1, part2);

pub fn parse(input: &str) -> Vec<ModuleDefinition> {
    let module_pattern =
        Regex::new(r"(?P<name>(?:[%|&]\w+)|broadcaster) -> (?P<targets>\w+(?:, \w+)*)").unwrap();

    input
        .lines()
        .map(|line| {
            let matches = module_pattern.captures(line).unwrap();
            let name = matches.name("name").unwrap().as_str();
            let (name, type_code) = if name.starts_with("%") {
                (&name[1..], ModuleType::FlipFlop)
            } else if name.starts_with("&") {
                (&name[1..], ModuleType::Conjunction)
            } else if name == "broadcaster" {
                (name, ModuleType::Broadcast)
            } else {
                unreachable!()
            };
            ModuleDefinition {
                name,
                type_code,
                targets: matches
                    .name("targets")
                    .unwrap()
                    .as_str()
                    .split_terminator(",")
                    .map(|target| target.trim())
                    .collect(),
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ModuleType {
    Broadcast,
    Conjunction,
    FlipFlop,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleDefinition<'a> {
    name: &'a str,
    type_code: ModuleType,
    targets: Vec<&'a str>,
}

trait InputModule: Module {}

trait Module {
    fn connect(&self, module: (Rc<dyn Module>, usize));
    fn pulse(&self, high: bool, context: usize, pulses: &mut VecDeque<Pulse>);
    fn reserve_input(&self) -> usize;
}

struct Button {
    broadcast: Rc<dyn Module>,
}

impl InputModule for Button {}

impl Module for Button {
    fn connect(&self, _module: (Rc<dyn Module>, usize)) {
        unimplemented!()
    }

    fn pulse(&self, high: bool, _context: usize, pulses: &mut VecDeque<Pulse>) {
        pulses.push_back(Pulse {
            high,
            target: (self.broadcast.clone(), 0),
        })
    }

    fn reserve_input(&self) -> usize {
        unimplemented!()
    }
}

impl Button {
    fn new(broadcast: Rc<dyn Module>) -> Self {
        Self { broadcast }
    }
}

struct Broadcast {
    targets: RefCell<Vec<(Rc<dyn Module>, usize)>>,
}

impl Broadcast {
    fn new() -> Self {
        Self {
            targets: RefCell::new(vec![]),
        }
    }
}

impl InputModule for Broadcast {}

impl Module for Broadcast {
    fn connect(&self, module: (Rc<dyn Module>, usize)) {
        self.targets.borrow_mut().push(module);
    }

    fn pulse(&self, high: bool, _context: usize, pulses: &mut VecDeque<Pulse>) {
        for target in self.targets.borrow().iter() {
            pulses.push_back(Pulse::new(high, target.clone()));
        }
    }

    fn reserve_input(&self) -> usize {
        0
    }
}

struct Conjunction {
    targets: RefCell<Vec<(Rc<dyn Module>, usize)>>,
    input_states: RefCell<Vec<bool>>,
    loop_detect: RefCell<Vec<u32>>,
    current_cycle: Cell<u32>,
}

impl Conjunction {
    fn new() -> Self {
        Self {
            targets: RefCell::new(vec![]),
            input_states: RefCell::new(vec![]),
            loop_detect: RefCell::new(vec![]),
            current_cycle: Cell::new(0),
        }
    }

    fn cycle_check(&self, button_count: u32) -> Option<u64> {
        self.current_cycle.set(button_count);
        if self.loop_detect.borrow().iter().all(|input| *input > 0) {
            Some(
                self.loop_detect
                    .borrow()
                    .iter()
                    .fold(1u64, |lcm: u64, cycle| lcm.lcm(&(*cycle as u64))),
            )
        } else {
            None
        }
    }
}

impl InputModule for Conjunction {}

impl Module for Conjunction {
    fn connect(&self, module: (Rc<dyn Module>, usize)) {
        self.targets.borrow_mut().push(module);
    }

    fn pulse(&self, high: bool, context: usize, pulses: &mut VecDeque<Pulse>) {
        self.input_states.borrow_mut()[context] = high;
        if high && self.current_cycle.get() > 0 {
            self.loop_detect.borrow_mut()[context] = self.current_cycle.get();
        }
        let high = !self.input_states.borrow_mut().iter().all(|state| *state);

        for target in self.targets.borrow().iter() {
            pulses.push_back(Pulse::new(high, target.clone()));
        }
    }

    fn reserve_input(&self) -> usize {
        let index = self.input_states.borrow().len();
        self.input_states.borrow_mut().push(false);
        self.loop_detect.borrow_mut().push(0);
        index
    }
}

struct FlipFlop {
    targets: RefCell<Vec<(Rc<dyn Module>, usize)>>,
    state: Cell<bool>,
}

impl FlipFlop {
    fn new() -> Self {
        Self {
            targets: RefCell::new(vec![]),
            state: Cell::new(false),
        }
    }
}

impl InputModule for FlipFlop {}

impl Module for FlipFlop {
    fn connect(&self, module: (Rc<dyn Module>, usize)) {
        self.targets.borrow_mut().push(module);
    }

    fn pulse(&self, high: bool, _context: usize, pulses: &mut VecDeque<Pulse>) {
        if !high {
            let high = !self.state.get();
            self.state.replace(high);

            for target in self.targets.borrow().iter() {
                pulses.push_back(Pulse::new(high, target.clone()));
            }
        }
    }

    fn reserve_input(&self) -> usize {
        0
    }
}

struct NullModule {}

impl NullModule {
    fn new() -> Self {
        Self {}
    }
}

impl InputModule for NullModule {}

impl Module for NullModule {
    fn connect(&self, _module: (Rc<dyn Module>, usize)) {
        todo!()
    }

    fn pulse(&self, _high: bool, _context: usize, _pulses: &mut VecDeque<Pulse>) {}

    fn reserve_input(&self) -> usize {
        todo!()
    }
}

struct Pulse {
    high: bool,
    target: (Rc<dyn Module>, usize),
}

impl Pulse {
    fn new(high: bool, target: (Rc<dyn Module>, usize)) -> Self {
        Self { high, target }
    }
}

/// ```rust
/// use advent_of_code_2023::day20::*;
/// let input = parse(
/// r"broadcaster -> a, b, c
/// %a -> b
/// %b -> c
/// %c -> inv
/// &inv -> a");
/// assert_eq!(32000000, part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2023::day20::*;
/// let input = parse(
/// r"broadcaster -> a
/// %a -> inv, con
/// &inv -> b
/// %b -> con
/// &con -> output");
/// assert_eq!(11687500, part1(&input));
/// ```
pub fn part1(input: &Vec<ModuleDefinition>) -> u32 {
    let null_module = Rc::new(NullModule::new());
    let broadcast = Rc::new(Broadcast::new());
    let mut conjunctions: HashMap<&str, Rc<Conjunction>> = HashMap::new();
    let mut flipflops: HashMap<&str, Rc<FlipFlop>> = HashMap::new();

    // First pass, create modules
    for module_definition in input {
        match module_definition.type_code {
            ModuleType::Broadcast => {}
            ModuleType::Conjunction => {
                let _ = conjunctions.insert(module_definition.name, Rc::new(Conjunction::new()));
            }
            ModuleType::FlipFlop => {
                let _ = flipflops.insert(module_definition.name, Rc::new(FlipFlop::new()));
            }
        }
    }

    // Second pass, connect modules
    let button = Button::new(broadcast.clone());
    for module_definition in input {
        let find_target = |target| {
            conjunctions
                .get(target)
                .map_or_else(
                    || {
                        flipflops
                            .get(target)
                            .map(|ff| (ff.clone() as Rc<dyn Module>, ff.reserve_input()))
                    },
                    |cj| Some((cj.clone() as Rc<dyn Module>, cj.reserve_input())),
                )
                .or_else(|| Some((null_module.clone() as Rc<dyn Module>, 0)))
                .unwrap()
        };

        match module_definition.type_code {
            ModuleType::Broadcast => {
                for target in module_definition.targets.iter() {
                    broadcast.connect(find_target(target));
                }
            }
            ModuleType::Conjunction => {
                let source = conjunctions.get(module_definition.name).unwrap();
                for target in module_definition.targets.iter() {
                    source.connect(find_target(target));
                }
            }
            ModuleType::FlipFlop => {
                let source = flipflops.get(module_definition.name).unwrap();
                for target in module_definition.targets.iter() {
                    source.connect(find_target(target));
                }
            }
        }
    }

    let mut pulses = VecDeque::new();
    let mut low_count = 0;
    let mut high_count = 0;

    for _button_smash in 0..1000 {
        button.pulse(false, 0, &mut pulses);

        while let Some(pulse) = pulses.pop_front() {
            if pulse.high {
                high_count += 1;
            } else {
                low_count += 1;
            }

            pulse
                .target
                .0
                .pulse(pulse.high, pulse.target.1, &mut pulses);
        }
    }

    low_count * high_count
}

pub fn part2(input: &Vec<ModuleDefinition>) -> u64 {
    let null_module = Rc::new(NullModule::new());
    let broadcast = Rc::new(Broadcast::new());
    let mut conjunctions: HashMap<&str, Rc<Conjunction>> = HashMap::new();
    let mut flipflops: HashMap<&str, Rc<FlipFlop>> = HashMap::new();
    let mut end_node: Option<Rc<Conjunction>> = None;

    // First pass, create modules
    for module_definition in input {
        match module_definition.type_code {
            ModuleType::Broadcast => {}
            ModuleType::Conjunction => {
                let _ = conjunctions.insert(module_definition.name, Rc::new(Conjunction::new()));
            }
            ModuleType::FlipFlop => {
                let _ = flipflops.insert(module_definition.name, Rc::new(FlipFlop::new()));
            }
        }
    }

    // Second pass, connect modules
    let button = Button::new(broadcast.clone());
    for module_definition in input {
        let find_target = |target| {
            conjunctions
                .get(target)
                .map_or_else(
                    || {
                        flipflops
                            .get(target)
                            .map(|ff| (ff.clone() as Rc<dyn Module>, ff.reserve_input()))
                    },
                    |cj| Some((cj.clone() as Rc<dyn Module>, cj.reserve_input())),
                )
                .or_else(|| Some((null_module.clone() as Rc<dyn Module>, 0)))
                .unwrap()
        };

        match module_definition.type_code {
            ModuleType::Broadcast => {
                for target in module_definition.targets.iter() {
                    broadcast.connect(find_target(target));
                }
            }
            ModuleType::Conjunction => {
                let source = conjunctions.get(module_definition.name).unwrap();

                for target in module_definition.targets.iter() {
                    let target_module = find_target(target);

                    if Rc::ptr_eq(&target_module.0, &(null_module.clone() as Rc<dyn Module>)) {
                        end_node = Some(source.clone());
                    }

                    source.connect(target_module);
                }
            }
            ModuleType::FlipFlop => {
                let source = flipflops.get(module_definition.name).unwrap();
                for target in module_definition.targets.iter() {
                    source.connect(find_target(target));
                }
            }
        }
    }

    let end_node = end_node.unwrap();
    let mut pulses = VecDeque::new();
    let mut button_count = 0;

    loop {
        button.pulse(false, 0, &mut pulses);
        button_count += 1;

        if let Some(result) = end_node.cycle_check(button_count) {
            return result;
        }

        while let Some(pulse) = pulses.pop_front() {
            pulse
                .target
                .0
                .pulse(pulse.high, pulse.target.1, &mut pulses);
        }
    }
}
