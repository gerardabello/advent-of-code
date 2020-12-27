use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::{map, map_res, recognize},
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Noop,
    Add,
    Jump,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub operation: Operation,
    pub value: i32,
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let parser = alt((tag("nop"), tag("acc"), tag("jmp")));

    map(parser, |s| match s {
        "nop" => Operation::Noop,
        "acc" => Operation::Add,
        "jmp" => Operation::Jump,
        _ => unreachable!(),
    })(input)
}

fn parse_integer_32(input: &str) -> IResult<&str, i32> {
    let (input, sign_s) = take(1usize)(input)?;
    let (input, num) = map_res(recognize(digit1), str::parse)(input)?;

    Ok((
        input,
        match sign_s {
            "+" => num,
            "-" => -num,
            _ => unreachable!(),
        },
    ))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, operation) = parse_operation(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, value) = parse_integer_32(input)?;

    Ok((input, Instruction { operation, value }))
}

pub fn parse_instructions(input: &str) -> Vec<Instruction> {
    let instructions: Vec<Instruction> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|l| parse_instruction(l).unwrap())
        .map(|(_, instruction)| instruction)
        .collect();

    instructions
}


#[derive(Debug)]
pub struct State {
    pub accumulator: i64,
    pub instruction_pointer: usize,
}

pub fn run_instruction(instructions: &[Instruction], state: &State) -> State {
    let instruction = &instructions[state.instruction_pointer];
    match instruction.operation {
        Operation::Noop => State {
            accumulator: state.accumulator,
            instruction_pointer: state.instruction_pointer + 1,
        },
        Operation::Add => State {
            accumulator: state.accumulator + instruction.value as i64,
            instruction_pointer: state.instruction_pointer + 1,
        },
        Operation::Jump => State {
            accumulator: state.accumulator,
            instruction_pointer: (state.instruction_pointer as i32 + instruction.value) as usize,
        },
    }
}

fn run_instructions_until_loop(instructions: &[Instruction]) -> State {
    let mut visited_instructions: HashSet<usize> = HashSet::new();

    let mut state = State {
        accumulator: 0,
        instruction_pointer: 0,
    };

    loop {
        if visited_instructions
            .get(&state.instruction_pointer)
            .is_some()
        {
            return state;
        }

        visited_instructions.insert(state.instruction_pointer);

        state = run_instruction(instructions, &state);
    }
}

pub fn solve(input: &str) -> String {
    let instructions: Vec<Instruction> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|l| parse_instruction(l).unwrap())
        .map(|(_, instruction)| instruction)
        .collect();

    let last_state = run_instructions_until_loop(&instructions);

    last_state.accumulator.to_string()
}
