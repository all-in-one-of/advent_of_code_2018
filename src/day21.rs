day!(
    day21,
    "https://adventofcode.com/2018/day/21/input",
    part1,
    part2
);

use crate::day19::{Instruction, Opcode, Program, ProgramState, Register};
use std::collections::HashSet;

fn get_goal_register(p: &Program) -> Result<usize> {
    if p.instructions.len() != 31 {
        return Err(Error::Input("not enough instructions in input"));
    }
    Ok(match &p.instructions[28] {
        Instruction {
            opcode: Opcode::EqRR(Register(a), Register(b)),
            target: _,
        } if (*a == 0) != (*b == 0) => {
            if *a == 0 {
                *b
            } else {
                *a
            }
        }
        _ => return Err(Error::Input("unexpected 28th instruction")),
    } as usize)
}

fn part1(input: &str) -> Result<u64> {
    let p: Program = input.parse()?;
    let goal_register = get_goal_register(&p)?;

    let mut s = p.new_state();
    for _ in 0..1_000_000 {
        s.execute();
        if s.ip == 28 {
            return Ok(s.registers[goal_register]);
        }
    }
    Err(Error::Input(
        "cannot reach instruction in 1_000_000 iterations",
    ))
}

fn part2(input: &str) -> Result<u64> {
    let p: Program = input.parse()?;
    let goal_register = get_goal_register(&p)?;

    let mut s = p.new_state();
    let mut termination_values = HashSet::new();
    let mut last = 0;
    for _ in 0..1_000_000_000_000u64 {
        s.execute();
        if s.ip == 28 {
            if !termination_values.insert(s.registers[goal_register]) {
                return Ok(last);
            }
            last = s.registers[goal_register];
        }
    }

    Err(Error::Input(
        "cannot reach instruction in 1_000_000_000_000 iterations",
    ))
}
