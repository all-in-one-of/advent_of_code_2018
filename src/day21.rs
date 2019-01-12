day!(
    day21,
    "https://adventofcode.com/2018/day/21/input",
    part1,
    part2
);

use crate::day19::{Instruction, Opcode, Program, ProgramState, Register};
use std::collections::HashSet;

fn parse_input(input: &str) -> Result<(Program, u64)> {
    let program: Program = input.parse()?;
    
    // Validate the opcode keys
    if program.instructions.len() != 31 {
        return Err(Error::Input("expected 31 instructions"));
    }

    macro_rules! validate_opcode {
        ($program:expr, $idx:expr, $expected:expr) => {
            if program.instructions[$idx].opcode.key() != $expected {
                return Err(Error::Input("invalid opcode structure in input"))
            }
        };
    }
    validate_opcode!(program, 00, "seti");
    validate_opcode!(program, 01, "bani");
    validate_opcode!(program, 02, "eqri");
    validate_opcode!(program, 03, "addr");
    validate_opcode!(program, 04, "seti");
    validate_opcode!(program, 05, "seti");
    validate_opcode!(program, 06, "bori");
    validate_opcode!(program, 07, "seti");
    validate_opcode!(program, 08, "bani");
    validate_opcode!(program, 09, "addr");
    validate_opcode!(program, 10, "bani");
    validate_opcode!(program, 11, "muli");
    validate_opcode!(program, 12, "bani");
    validate_opcode!(program, 13, "gtir");
    validate_opcode!(program, 14, "addr");
    validate_opcode!(program, 15, "addi");
    validate_opcode!(program, 16, "seti");
    validate_opcode!(program, 17, "seti");
    validate_opcode!(program, 18, "addi");
    validate_opcode!(program, 19, "muli");
    validate_opcode!(program, 20, "gtrr");
    validate_opcode!(program, 21, "addr");
    validate_opcode!(program, 22, "addi");
    validate_opcode!(program, 23, "seti");
    validate_opcode!(program, 24, "addi");
    validate_opcode!(program, 25, "seti");
    validate_opcode!(program, 26, "setr");
    validate_opcode!(program, 27, "seti");
    validate_opcode!(program, 28, "eqrr");
    validate_opcode!(program, 29, "addr");
    validate_opcode!(program, 30, "seti");

    let mut register_map = [0; 6];
    register_map[program.ip_register as usize] = 1;
    register_map[program.instructions[0].target.0 as usize] = 2;
    register_map[program.instructions[6].target.0 as usize] = 3;
    register_map[program.instructions[8].target.0 as usize] = 4;
    register_map[program.instructions[18].target.0 as usize] = 5;

    let new_program = Program {
        ip_register: 1,
        instructions: program.instructions.iter().map(|instruction| {
            let map_register = |r: Register| -> Register { Register(register_map[r.0 as usize]) };
            Instruction {
                target: map_register(instruction.target),
                opcode: instruction.opcode.map_registers(map_register),
            }
        }).collect()
    };
    let initial_state = match new_program.instructions[7].opcode {
        Opcode::SetI(a) => a.0,
        _ => unreachable!(),
    };

    Ok((new_program, initial_state))
}

fn puzzle(max_iter: u64, find_first: bool, initial_state: u64) -> Option<u64> {
    let mut last = None;
    let mut previous_values = HashSet::new();
    let mut a: u64 = 0;
    for _ in 0..max_iter {
        let mut b: u64 = a | 65536;
        a = initial_state;
        loop {
            a = a + (b & 255);
            a = a & 16777215;
            a = a * 65899;
            a = a & 16777215;
            if 256 > b { break; }
            let mut c: u64 = 0;
            loop {
                let mut d: u64 = c + 1;
                d = d * 256;
                d = if d > b { 1 } else { 0 };
                if d == 1 { break; }
                c = c + 1;
            }
            b = c;
        }
        if find_first {
            return Some(a);
        }
        if previous_values.insert(a) {
            last = Some(a);
        }
        else {
            return last;
        }
    }
    last
}

fn part1(input: &str) -> Result<u64> {
    let (_, initial_state) = parse_input(input)?;
    puzzle(1_000_000, true, initial_state).ok_or(Error::Input(
        "cannot reach instruction in 1_000_000 iterations",
    ))
}

fn part2(input: &str) -> Result<u64> {
    let (_, initial_state) = parse_input(input)?;
    puzzle(1_000_000_000_000, false, initial_state).ok_or(Error::Input(
        "cannot reach instruction in 1_000_000 iterations",
    ))
}

#[test]
fn day21_test() {
    const EXAMPLE: &str = "#ip 1
seti 123 0 2
bani 2 456 2
eqri 2 72 2
addr 2 1 1
seti 0 0 1
seti 0 3 2
bori 2 65536 5
seti 4843319 1 2
bani 5 255 4
addr 2 4 2
bani 2 16777215 2
muli 2 65899 2
bani 2 16777215 2
gtir 256 5 4
addr 4 1 1
addi 1 1 1
seti 27 4 1
seti 0 7 4
addi 4 1 3
muli 3 256 3
gtrr 3 5 3
addr 3 1 1
addi 1 1 1
seti 25 0 1
addi 4 1 4
seti 17 0 1
setr 4 1 5
seti 7 3 1
eqrr 2 0 4
addr 4 1 1
seti 5 3 1";
    assert_results!(part1, EXAMPLE => 8797248);
    assert_results!(part2, EXAMPLE => 3007673);
}
