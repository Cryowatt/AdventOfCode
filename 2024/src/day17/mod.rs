use advent::*;
use regex::Regex;

advent_day!(Day17, parse, (Registers, Vec<u8>), part1, part2);

#[derive(Debug, Clone, Copy)]
pub struct Registers {
    pub a: u64,
    pub b: u64,
    pub c: u64,
}
#[derive(Debug)]
pub struct Computer<'a> {
    pub registers: Registers,
    ip: u32,
    program: &'a Vec<u8>,
    output: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
pub enum Fault {
    IllegalOp(u8),
    IllegalCombo(u8),
    Reserved,
    EOF,
    DivideByZero,
}

impl<'a> Computer<'a> {
    fn new(registers: Registers, program: &'a Vec<u8>) -> Self {
        Self {
            registers,
            ip: 0,
            program,
            output: vec![],
        }
    }

    fn cycle(&mut self) -> Result<(), Fault> {
        let op = self.program.get(self.ip as usize).ok_or(Fault::EOF)?;
        let operand = self.program.get((self.ip + 1) as usize).ok_or(Fault::EOF)?;
        self.ip += 2;

        match op {
            0 => self.adv(*operand),
            1 => self.bxl(*operand),
            2 => self.bst(*operand),
            3 => self.jnz(*operand),
            4 => self.bxc(*operand),
            5 => self.out(*operand),
            6 => self.bdv(*operand),
            7 => self.cdv(*operand),
            _ => Err(Fault::IllegalOp(*op)),
        }
    }

    fn combo(&mut self, operand: u8) -> Result<u64, Fault> {
        match operand {
            0..=3 => Ok(operand as u64),
            4 => Ok(self.registers.a),
            5 => Ok(self.registers.b),
            6 => Ok(self.registers.c),
            7 => Err(Fault::Reserved),
            _ => Err(Fault::IllegalCombo(operand)),
        }
    }

    fn div(&mut self, operand: u8) -> Result<u64, Fault> {
        Ok(self
            .registers
            .a
            .checked_div(2u64.pow(self.combo(operand)? as u32))
            .ok_or(Fault::DivideByZero)?)
    }

    fn adv(&mut self, operand: u8) -> Result<(), Fault> {
        self.registers.a = self.div(operand)?;
        Ok(())
    }

    fn bxl(&mut self, operand: u8) -> Result<(), Fault> {
        self.registers.b = self.registers.b ^ operand as u64;
        Ok(())
    }

    fn bst(&mut self, operand: u8) -> Result<(), Fault> {
        self.registers.b = self.combo(operand)? % 8;
        Ok(())
    }

    fn jnz(&mut self, operand: u8) -> Result<(), Fault> {
        if self.registers.a != 0 {
            self.ip = operand as u32;
        }
        Ok(())
    }

    fn bxc(&mut self, _operand: u8) -> Result<(), Fault> {
        self.registers.b = self.registers.b ^ self.registers.c;
        Ok(())
    }

    fn out(&mut self, operand: u8) -> Result<(), Fault> {
        let value = (self.combo(operand)? % 8) as u8;
        self.output.push(value);
        Ok(())
    }

    fn bdv(&mut self, operand: u8) -> Result<(), Fault> {
        self.registers.b = self.div(operand)?;
        Ok(())
    }

    fn cdv(&mut self, operand: u8) -> Result<(), Fault> {
        self.registers.c = self.div(operand)?;
        Ok(())
    }
}

pub fn parse(input: &str) -> InputType<'_> {
    let pattern = Regex::new(
        r"Register A: (?<A>\d+)
Register B: (?<B>\d+)
Register C: (?<C>\d+)

Program: (?<P>\d(,\d)+)",
    )
    .unwrap();

    let captures = pattern.captures(input).unwrap();
    (
        Registers {
            a: captures.name("A").unwrap().as_str().parse().unwrap(),
            b: captures.name("B").unwrap().as_str().parse().unwrap(),
            c: captures.name("C").unwrap().as_str().parse().unwrap(),
        },
        captures
            .name("P")
            .unwrap()
            .as_str()
            .split(',')
            .map(|b| b.parse().unwrap())
            .collect(),
    )
}

/// ```rust
/// use advent_of_code_2024::day17::*;
/// let input = parse(
/// r"Register A: 0
/// Register B: 0
/// Register C: 9
///
/// Program: 2,6");
/// assert_eq!(1, compute(&input).registers.b);
/// ```
/// ```rust
/// use advent_of_code_2024::day17::*;
/// let input = parse(
/// r"Register A: 10
/// Register B: 0
/// Register C: 0
///
/// Program: 5,0,5,1,5,4");
/// assert_eq!("0,1,2".to_string(), part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2024::day17::*;
/// let input = parse(
/// r"Register A: 2024
/// Register B: 0
/// Register C: 0
///
/// Program: 0,1,5,4,3,0");
/// assert_eq!("4,2,5,6,7,7,7,7,3,1,0".to_string(), part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2024::day17::*;
/// let input = parse(
/// r"Register A: 0
/// Register B: 29
/// Register C: 0
///
/// Program: 1,7");
/// assert_eq!(26, compute(&input).registers.b);
/// ```
/// ```rust
/// use advent_of_code_2024::day17::*;
/// let input = parse(
/// r"Register A: 0
/// Register B: 2024
/// Register C: 43690
///
/// Program: 4,0");
/// assert_eq!(44354, compute(&input).registers.b);
/// ```
/// ```rust
/// use advent_of_code_2024::day17::*;
/// let input = parse(
/// r"Register A: 729
/// Register B: 0
/// Register C: 0
///
/// Program: 0,1,5,4,3,0");
/// assert_eq!("4,6,3,5,6,3,5,2,1,0".to_string(), part1(&input));
/// ```
pub fn part1(input: &InputType) -> String {
    let computer = compute(input);

    let mut output_buffer = String::with_capacity(computer.output.len() * 2);
    let mut output = computer.output.iter();
    if let Some(value) = output.next() {
        output_buffer.push((b'0' + *value) as char);
        for value in output {
            output_buffer.push(',');
            output_buffer.push((b'0' + *value) as char);
        }
    }
    output_buffer
}

pub fn compute<'a>(input: &'a InputType) -> Computer<'a> {
    let (registers, program) = input;
    let mut computer = Computer::new(registers.clone(), program);
    loop {
        match computer.cycle() {
            Ok(_) => continue,
            Err(Fault::EOF) => break,
            Err(e) => panic!("FAULT! {:?} {:?}", e, computer),
        }
    }

    computer
}

/// ```rust
/// use advent_of_code_2024::day17::*;
/// let input = parse(
/// r"Register A: 117440
/// Register B: 0
/// Register C: 0
///
/// Program: 0,3,5,4,3,0");
/// assert_eq!("0,3,5,4,3,0".to_string(), part1(&input));
/// ```
/// ```rust
/// use advent_of_code_2024::day17::*;
/// let input = parse(
/// r"Register A: 2024
/// Register B: 0
/// Register C: 0
///
/// Program: 0,3,5,4,3,0");
/// assert_eq!(117440, part2(&input));
/// ```

pub fn part2(input: &InputType) -> u64 {
    let (registers, program) = input;

    let mut octits = program.clone();
    octits.fill(7);

    fn from_oct(octits: &Vec<u8>) -> u64 {
        octits
            .iter()
            .rev()
            .fold(0, |a, &digit| (a << 3) + digit as u64)
    }

    fn crack_digit(
        octits: &mut Vec<u8>,
        digit: usize,
        registers: &Registers,
        program: &Vec<u8>,
    ) -> Option<()> {
        for v in 0..8u8 {
            let mut computer = Computer::new(registers.clone(), program);
            octits[digit] = v;
            computer.registers.a = from_oct(&octits);
            loop {
                match computer.cycle() {
                    Ok(_) => continue,
                    Err(Fault::EOF) => break,
                    Err(e) => panic!("FAULT! {:?} {:?}", e, computer),
                }
            }

            if digit < computer.output.len() && computer.output[digit] == program[digit] {
                if digit == 0 {
                    return Some(());
                } else if crack_digit(octits, digit - 1, registers, program).is_some() {
                    return Some(());
                }
            }
        }

        None
    }

    crack_digit(&mut octits, program.len() - 1, registers, program);

    from_oct(&octits)
}
