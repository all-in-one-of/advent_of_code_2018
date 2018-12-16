day!(
    day16,
    "https://adventofcode.com/2018/day/16/input",
    part1,
    part2
);

use regex::Regex;
use smallvec::SmallVec;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Register(u8);
#[derive(Debug, Clone, Copy)]
struct Immediate(u8);
type Registers = [i64; 4];

#[derive(Debug, Clone)]
struct Sample {
    before: Registers,
    placeholder: PlaceholderInstruction,
    after: Registers,
}

#[derive(Debug, Clone, Copy)]
struct PlaceholderInstruction(u8, u8, u8, u8);
#[derive(Debug, Clone, Copy)]
struct Instruction {
    opcode: Opcode,
    target: Register,
}

#[derive(Debug, Clone, Copy)]
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

impl From<PlaceholderInstruction> for Instruction {
    fn from(placeholder: PlaceholderInstruction) -> Instruction {
        let opcode = #[rustfmt::skip] match placeholder.0 {
            0  => Opcode::AddR( Register(placeholder.1),  Register(placeholder.2)),
            1  => Opcode::AddI( Register(placeholder.1), Immediate(placeholder.2)),
            2  => Opcode::MulR( Register(placeholder.1),  Register(placeholder.2)),
            3  => Opcode::MulI( Register(placeholder.1), Immediate(placeholder.2)),
            4  => Opcode::BanR( Register(placeholder.1),  Register(placeholder.2)),
            5  => Opcode::BanI( Register(placeholder.1), Immediate(placeholder.2)),
            6  => Opcode::BorR( Register(placeholder.1),  Register(placeholder.2)),
            7  => Opcode::BorI( Register(placeholder.1), Immediate(placeholder.2)),
            8  => Opcode::SetR( Register(placeholder.1)),
            9  => Opcode::SetI(Immediate(placeholder.1)),
            10 => Opcode::GtIR(Immediate(placeholder.1),  Register(placeholder.2)),
            11 => Opcode::GtRI( Register(placeholder.1), Immediate(placeholder.2)),
            12 => Opcode::GtRR( Register(placeholder.1),  Register(placeholder.2)),
            13 => Opcode::EqIR(Immediate(placeholder.1),  Register(placeholder.2)),
            14 => Opcode::EqRI( Register(placeholder.1), Immediate(placeholder.2)),
            15 => Opcode::EqRR( Register(placeholder.1),  Register(placeholder.2)),
            _ => panic!("opcode out of range"),
        };
        Instruction {
            opcode,
            target: Register(placeholder.3),
        }
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

fn parse_input(input: &str) -> Result<(Vec<Sample>, Vec<PlaceholderInstruction>)> {
    let split_point = input.find("\n\n\n").ok_or(Error::Input("no split point"))?;

    lazy_static! {
        static ref RE1: Regex = Regex::new(r"Before: \[(\d+), (\d+), (\d+), (\d+)\]\n(\d+) (\d+) (\d+) (\d+)\nAfter:  \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
        static ref RE2: Regex = Regex::new(r"(?m)^(\d+) (\d+) (\d+) (\d+)$").unwrap();
    }

    let samples = &input[0..split_point];
    let samples = RE1
        .captures_iter(samples)
        .map(|captures| {
            let before: Registers = [
                captures[1].parse()?,
                captures[2].parse()?,
                captures[3].parse()?,
                captures[4].parse()?,
            ];
            let placeholder = PlaceholderInstruction(
                captures[5].parse()?,
                captures[6].parse()?,
                captures[7].parse()?,
                captures[8].parse()?,
            );
            let after: Registers = [
                captures[9].parse()?,
                captures[10].parse()?,
                captures[11].parse()?,
                captures[12].parse()?,
            ];
            Ok(Sample {
                before,
                placeholder,
                after,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let instructions = &input[split_point + 3..];
    let instructions = RE2
        .captures_iter(instructions)
        .map(|captures| {
            Ok(PlaceholderInstruction(
                captures[1].parse()?,
                captures[2].parse()?,
                captures[3].parse()?,
                captures[4].parse()?,
            ))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok((samples, instructions))
}

fn find_possible_opcodes(sample: &Sample) -> SmallVec<[u8; 16]> {
    let mut res = SmallVec::new();

    for i in 0..16 {
        let mut current_instruction = sample.placeholder;
        current_instruction.0 = i;
        let old_registers = sample.before;
        let instruction: Instruction = current_instruction.into();
        let new_registers = instruction.execute(&old_registers);
        if new_registers == sample.after {
            res.push(i);
        }
    }

    res
}

fn part1(input: &str) -> Result<usize> {
    let (samples, _) = parse_input(input)?;
    Ok(samples
        .into_iter()
        .filter(|sample| find_possible_opcodes(sample).len() >= 3)
        .count())
}

fn part2(input: &str) -> Result<i64> {
    let (samples, instructions) = parse_input(input)?;
    let mut mapping: HashMap<u8, u8> = HashMap::new();
    let mut new_mapping = Vec::new();
    while mapping.len() < 16 {
        new_mapping.extend(
            samples
                .iter()
                .filter(|sample| !mapping.contains_key(&sample.placeholder.0))
                .filter_map(|sample| {
                    let mut possible_opcodes = find_possible_opcodes(sample);
                    for i in (0..possible_opcodes.len()).rev() {
                        let mapped_to = possible_opcodes[i];
                        if mapping.values().any(|x| *x == mapped_to) {
                            possible_opcodes.remove(i);
                        }
                    }
                    if possible_opcodes.len() == 1 {
                        Some((sample.placeholder.0, possible_opcodes[0]))
                    } else {
                        None
                    }
                }),
        );
        mapping.extend(new_mapping.drain(..));
    }

    let mut registers = [0, 0, 0, 0];
    for mut placeholder in instructions {
        placeholder.0 = mapping[&placeholder.0];
        let instruction: Instruction = placeholder.into();
        registers = instruction.execute(&registers);
    }

    Ok(registers[0])
}

#[test]
fn day16_test() {
    // Before: [3, 2, 1, 1]
    // 9 2 1 2
    // After:  [3, 2, 2, 1]

    assert_eq!(
        smallvec![1u8, 2u8, 9u8] as SmallVec<[u8; 16]>,
        find_possible_opcodes(&Sample {
            placeholder: PlaceholderInstruction(9, 2, 1, 2),
            before: [3, 2, 1, 1],
            after: [3, 2, 2, 1],
        })
    );
}
