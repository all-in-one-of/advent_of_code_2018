day!(
    day19,
    "https://adventofcode.com/2018/day/19/input",
    part1,
    part2
);

use regex::Regex;
use std::fmt::{self, Display};
use std::str::FromStr;

pub type IntType = u64;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Register(pub IntType);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Immediate(pub IntType);
pub type Registers = [IntType; 6];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Argument {
    Immediate(Immediate),
    Register(Register),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub target: Register,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    AddR(Register, Register),
    AddI(Register, Immediate),
    MulR(Register, Register),
    MulI(Register, Immediate),
    BanR(Register, Register),
    BanI(Register, Immediate),
    BorR(Register, Register),
    BorI(Register, Immediate),
    SetR(Register),
    SetI(Immediate),
    GtIR(Immediate, Register),
    GtRI(Register, Immediate),
    GtRR(Register, Register),
    EqIR(Immediate, Register),
    EqRI(Register, Immediate),
    EqRR(Register, Register),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub ip_register: IntType,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramState<'a> {
    pub program: &'a Program,
    pub registers: Registers,
    pub ip: IntType,
}

impl Register {
    fn resolve(&self, registers: &Registers) -> IntType {
        registers[self.0 as usize]
    }
}
impl Immediate {
    fn resolve(&self, _registers: &Registers) -> IntType {
        self.0
    }
}

impl Instruction {
    fn execute(&self, reg: &Registers) -> Registers {
        let mut result = reg.clone();
        let new_value = #[rustfmt::skip] match self.opcode {
            Opcode::AddR(a, b) => a.resolve(reg) + b.resolve(reg),
            Opcode::AddI(a, b) => a.resolve(reg) + b.resolve(reg),
            Opcode::MulR(a, b) => a.resolve(reg) * b.resolve(reg),
            Opcode::MulI(a, b) => a.resolve(reg) * b.resolve(reg),
            Opcode::BanR(a, b) => a.resolve(reg) & b.resolve(reg),
            Opcode::BanI(a, b) => a.resolve(reg) & b.resolve(reg),
            Opcode::BorR(a, b) => a.resolve(reg) | b.resolve(reg),
            Opcode::BorI(a, b) => a.resolve(reg) | b.resolve(reg),
            Opcode::SetR(a   ) => a.resolve(reg),
            Opcode::SetI(a   ) => a.resolve(reg),
            Opcode::GtIR(a, b) => if a.resolve(reg)  > b.resolve(reg) { 1 } else { 0 },
            Opcode::GtRI(a, b) => if a.resolve(reg)  > b.resolve(reg) { 1 } else { 0 },
            Opcode::GtRR(a, b) => if a.resolve(reg)  > b.resolve(reg) { 1 } else { 0 },
            Opcode::EqIR(a, b) => if a.resolve(reg) == b.resolve(reg) { 1 } else { 0 },
            Opcode::EqRI(a, b) => if a.resolve(reg) == b.resolve(reg) { 1 } else { 0 },
            Opcode::EqRR(a, b) => if a.resolve(reg) == b.resolve(reg) { 1 } else { 0 },
        };
        result[self.target.0 as usize] = new_value;
        result
    }
}

impl Argument {
    fn value(&self) -> IntType {
        match self {
            &Argument::Register(register) => register.0,
            &Argument::Immediate(immediate) => immediate.0,
        }
    }
}

impl From<Register> for Argument {
    fn from(register: Register) -> Argument {
        Argument::Register(register)
    }
}
impl From<Immediate> for Argument {
    fn from(immediate: Immediate) -> Argument {
        Argument::Immediate(immediate)
    }
}

impl Opcode {
    #[rustfmt::skip]
    pub fn key(&self) -> &'static str {
        match self {
            &Opcode::AddR(_, _) => "addr",
            &Opcode::AddI(_, _) => "addi",
            &Opcode::MulR(_, _) => "mulr",
            &Opcode::MulI(_, _) => "muli",
            &Opcode::BanR(_, _) => "banr",
            &Opcode::BanI(_, _) => "bani",
            &Opcode::BorR(_, _) => "borr",
            &Opcode::BorI(_, _) => "bori",
            &Opcode::SetR(_   ) => "setr",
            &Opcode::SetI(_   ) => "seti",
            &Opcode::GtIR(_, _) => "gtir",
            &Opcode::GtRI(_, _) => "gtri",
            &Opcode::GtRR(_, _) => "gtrr",
            &Opcode::EqIR(_, _) => "eqir",
            &Opcode::EqRI(_, _) => "eqri",
            &Opcode::EqRR(_, _) => "eqrr",
        }
    }

    #[rustfmt::skip]
    pub fn args(&self) -> (Argument, Option<Argument>) {
        match self {
            &Opcode::AddR(a, b) => (a.into(), Some(b.into())),
            &Opcode::AddI(a, b) => (a.into(), Some(b.into())),
            &Opcode::MulR(a, b) => (a.into(), Some(b.into())),
            &Opcode::MulI(a, b) => (a.into(), Some(b.into())),
            &Opcode::BanR(a, b) => (a.into(), Some(b.into())),
            &Opcode::BanI(a, b) => (a.into(), Some(b.into())),
            &Opcode::BorR(a, b) => (a.into(), Some(b.into())),
            &Opcode::BorI(a, b) => (a.into(), Some(b.into())),
            &Opcode::SetR(a   ) => (a.into(), None          ),
            &Opcode::SetI(a   ) => (a.into(), None          ),
            &Opcode::GtIR(a, b) => (a.into(), Some(b.into())),
            &Opcode::GtRI(a, b) => (a.into(), Some(b.into())),
            &Opcode::GtRR(a, b) => (a.into(), Some(b.into())),
            &Opcode::EqIR(a, b) => (a.into(), Some(b.into())),
            &Opcode::EqRI(a, b) => (a.into(), Some(b.into())),
            &Opcode::EqRR(a, b) => (a.into(), Some(b.into())),
        }
    }

    #[rustfmt::skip]
    pub fn map_registers<F>(&self, mut f: F) -> Opcode
    where
        F: FnMut(Register) -> Register,
    {
        match self {
            &Opcode::AddR(a, b) => Opcode::AddR(f(a), f(b)),
            &Opcode::AddI(a, b) => Opcode::AddI(f(a),   b ),
            &Opcode::MulR(a, b) => Opcode::MulR(f(a), f(b)),
            &Opcode::MulI(a, b) => Opcode::MulI(f(a),   b ),
            &Opcode::BanR(a, b) => Opcode::BanR(f(a), f(b)),
            &Opcode::BanI(a, b) => Opcode::BanI(f(a),   b ),
            &Opcode::BorR(a, b) => Opcode::BorR(f(a), f(b)),
            &Opcode::BorI(a, b) => Opcode::BorI(f(a),   b ),
            &Opcode::SetR(a   ) => Opcode::SetR(f(a)      ),
            &Opcode::SetI(a   ) => Opcode::SetI(  a       ),
            &Opcode::GtIR(a, b) => Opcode::GtIR(  a , f(b)),
            &Opcode::GtRI(a, b) => Opcode::GtRI(f(a),   b ),
            &Opcode::GtRR(a, b) => Opcode::GtRR(f(a), f(b)),
            &Opcode::EqIR(a, b) => Opcode::EqIR(  a , f(b)),
            &Opcode::EqRI(a, b) => Opcode::EqRI(f(a),   b ),
            &Opcode::EqRR(a, b) => Opcode::EqRR(f(a), f(b)),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (a, b) = self.opcode.args();
        write!(
            f,
            "{} {} {} {}",
            self.opcode.key(),
            a.value(),
            b.unwrap_or(Argument::Immediate(Immediate(0))).value(),
            self.target.0
        )
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#ip {}", self.ip_register)?;
        for instruction in &self.instructions {
            write!(f, "\n{}", instruction)?;
        }
        Ok(())
    }
}

impl FromStr for Program {
    type Err = Error;
    fn from_str(input: &str) -> Result<Program> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([a-z]{4}) (\d+) (\d+) (\d+)$").unwrap();
        }

        let mut lines = input.lines();
        let line = lines.next().ok_or(Error::Input("expected IP directive"))?;
        let ip_register: IntType = if line.starts_with("#ip ") {
            line[4..].parse()?
        } else {
            return Err(Error::Input("expected IP directive"));
        };

        lines
            .map(|line| {
                let capture = RE
                    .captures(line)
                    .ok_or(Error::Input("invalid instruction"))?;
                let a = capture[2].parse::<IntType>()?;
                let b = capture[3].parse::<IntType>()?;
                let c = capture[4].parse::<IntType>()?;
                let opcode = match &capture[1] {
                    "addr" => Opcode::AddR(Register(a), Register(b)),
                    "addi" => Opcode::AddI(Register(a), Immediate(b)),
                    "mulr" => Opcode::MulR(Register(a), Register(b)),
                    "muli" => Opcode::MulI(Register(a), Immediate(b)),
                    "banr" => Opcode::BanR(Register(a), Register(b)),
                    "bani" => Opcode::BanI(Register(a), Immediate(b)),
                    "borr" => Opcode::BorR(Register(a), Register(b)),
                    "bori" => Opcode::BorI(Register(a), Immediate(b)),
                    "setr" => Opcode::SetR(Register(a)),
                    "seti" => Opcode::SetI(Immediate(a)),
                    "gtir" => Opcode::GtIR(Immediate(a), Register(b)),
                    "gtri" => Opcode::GtRI(Register(a), Immediate(b)),
                    "gtrr" => Opcode::GtRR(Register(a), Register(b)),
                    "eqir" => Opcode::EqIR(Immediate(a), Register(b)),
                    "eqri" => Opcode::EqRI(Register(a), Immediate(b)),
                    "eqrr" => Opcode::EqRR(Register(a), Register(b)),
                    _ => return Err(Error::Input("invalid instruction")),
                };
                Ok(Instruction {
                    opcode,
                    target: Register(c),
                })
            })
            .collect::<Result<Vec<_>>>()
            .map(|instructions| Program {
                ip_register,
                instructions,
            })
    }
}

impl Program {
    pub fn new_state<'s>(&'s self) -> ProgramState<'s> {
        ProgramState {
            program: self,
            registers: [0; 6],
            ip: 0,
        }
    }
}

impl ProgramState<'_> {
    pub fn execute(&mut self) -> bool {
        if self.ip as usize >= self.program.instructions.len() {
            return false;
        }

        let ip_register = self.program.ip_register as usize;
        self.registers[ip_register] = self.ip;
        self.registers = self.program.instructions[self.ip as usize].execute(&self.registers);
        self.ip = self.registers[ip_register] + 1;

        true
    }

    pub fn run(&mut self) -> IntType {
        let mut count = 0;
        while self.execute() {
            count += 1;
        }
        count
    }
}

fn part1(input: &str) -> Result<IntType> {
    let program = Program::from_str(input)?;
    let mut state = program.new_state();
    state.run();
    Ok(state.registers[0])
}

fn part2(input: &str) -> Result<String> {
    let program = Program::from_str(input)?;

    if program.instructions.len() < 5 {
        return Err(Error::Input("expected more instructions"));
    }

    let last_instruction = Instruction {
        opcode: Opcode::SetI(Immediate(0)),
        target: Register(program.ip_register),
    };
    let second_to_last_instruction = Instruction {
        opcode: Opcode::SetI(Immediate(0)),
        target: Register(0),
    };

    if &program.instructions[program.instructions.len() - 1] != &last_instruction
        || &program.instructions[program.instructions.len() - 2] != &second_to_last_instruction
    {
        return Err(Error::Input("expect a pattern of instructions at the end"));
    }

    let input_register = {
        let instruction = &program.instructions[program.instructions.len() - 3];
        match instruction.opcode {
            Opcode::AddR(x, _) if x == instruction.target => {}
            _ => {
                return Err(Error::Input(
                    "expected input to be set at instruction[..len-3]",
                ));
            }
        }
        instruction.target.0 as usize
    };

    // Run till the loop back around
    let mut state = program.new_state();
    state.registers[0] = 1;
    let mut last_ip = state.ip;
    while last_ip as usize != program.instructions.len() - 1 {
        if !state.execute() {
            return Err(Error::Input("unexpected early termination"));
        }
        last_ip = state.ip;
    }

    // Fetch the target input
    let input = state.registers[input_register] as IntType;

    let mut sigma = 0;
    for divisor in 1..=input {
        if input % divisor == 0 {
            sigma += divisor;
        }
    }

    Ok(format!("sigma({}) = {}", input, sigma))
}

#[test]
fn day19_test() {
    const EXAMPLE: &str = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

    assert_results!(part1, EXAMPLE => 6);
}
