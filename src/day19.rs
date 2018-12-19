day!(
    day19,
    "https://adventofcode.com/2018/day/19/input",
    part1,
    part2
);

use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Register(u8);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Immediate(u8);
type Registers = [i64; 6];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    opcode: Opcode,
    target: Register,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Opcode {
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
struct Program {
    ip_register: u8,
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProgramState<'a> {
    program: &'a Program,
    registers: Registers,
    ip: usize,
}

impl Register {
    fn resolve(&self, registers: &Registers) -> i64 {
        registers[self.0 as usize]
    }
}
impl Immediate {
    fn resolve(&self, _registers: &Registers) -> i64 {
        self.0 as i64
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

impl FromStr for Program {
    type Err = Error;
    fn from_str(input: &str) -> Result<Program> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([a-z]{4}) (\d+) (\d+) (\d+)$").unwrap();
        }

        let mut lines = input.lines();
        let line = lines.next().ok_or(Error::Input("expected IP directive"))?;
        let ip_register: u8 = if line.starts_with("#ip ") {
            line[4..].parse()?
        } else {
            return Err(Error::Input("expected IP directive"));
        };

        lines
            .map(|line| {
                let capture = RE
                    .captures(line)
                    .ok_or(Error::Input("invalid instruction"))?;
                let a = capture[2].parse::<u8>()?;
                let b = capture[3].parse::<u8>()?;
                let c = capture[4].parse::<u8>()?;
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
    fn new_state<'s>(&'s self) -> ProgramState<'s> {
        ProgramState {
            program: self,
            registers: [0; 6],
            ip: 0,
        }
    }
}

impl ProgramState<'_> {
    fn execute(&mut self) -> bool {
        if self.ip >= self.program.instructions.len() {
            return false;
        }

        let ip_register = self.program.ip_register as usize;
        self.registers[ip_register] = self.ip as i64;
        self.registers = self.program.instructions[self.ip].execute(&self.registers);
        self.ip = (self.registers[ip_register] + 1) as usize;

        true
    }

    fn run(&mut self) -> u64 {
        let mut count = 0;
        while self.execute() {
            count += 1;
        }
        count
    }
}

fn part1(input: &str) -> Result<i64> {
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
                ))
            }
        }
        instruction.target.0 as usize
    };

    // Run till the loop back around
    let mut state = program.new_state();
    state.registers[0] = 1;
    let mut last_ip = state.ip;
    while last_ip != program.instructions.len() - 1 {
        if !state.execute() {
            return Err(Error::Input("unexpected early termination"));
        }
        last_ip = state.ip;
    }

    // Fetch the target input
    let input = state.registers[input_register] as u64;

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
