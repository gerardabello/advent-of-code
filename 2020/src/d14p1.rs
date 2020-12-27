use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::digit1,
    combinator::{map, map_res},
    multi::separated_list1,
    IResult,
};

#[derive(Debug, Clone)]
pub struct Mask(pub String);

fn apply_mask(mask: &str, value: u64) -> u64 {
    let binary = format!("{:b}", value);
    let binary_padded = format!("{:0>36}", binary);

    let masked_rev: String = binary_padded
        .chars()
        .rev()
        .enumerate()
        .map(|(i, b)| match mask.chars().rev().nth(i) {
            Some('0') => '0',
            Some('1') => '1',
            Some('X') => b,
            None => b,
            _ => unreachable!(),
        })
        .collect();

    let masked: String = masked_rev.chars().rev().collect();

    u64::from_str_radix(&masked, 2).unwrap()
}

#[derive(Debug)]
pub struct Mem {
    pub address: u64,
    pub value: u64,
}

#[derive(Debug)]
pub enum Instruction {
    Mask(Mask),
    Mem(Mem),
}

struct VM {
    memory: HashMap<u64, u64>,
    mask: Mask,
}

impl VM {
    fn new() -> Self {
        Self {
            memory: HashMap::new(),
            mask: Mask("".to_owned()),
        }
    }

    fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Mask(m) => self.mask = (*m).clone(),
            Instruction::Mem(m) => {
                let val = apply_mask(&self.mask.0, m.value);

                self.memory.insert(m.address, val);
            }
        };
    }
}

fn parse_mask(input: &str) -> IResult<&str, Mask> {
    let (input, _) = tag(" = ")(input)?;
    let (input, mask) = is_a("0X1")(input)?;
    Ok((input, Mask(mask.to_owned())))
}

fn parse_mem(input: &str) -> IResult<&str, Mem> {
    let (input, _) = tag("[")(input)?;
    let (input, address) = map_res(digit1, |s: &str| s.parse::<u64>())(input)?;
    let (input, _) = tag("] = ")(input)?;
    let (input, value) = map_res(digit1, |s: &str| s.parse::<u64>())(input)?;
    Ok((input, Mem { address, value }))
}

fn parse_intruction(input: &str) -> IResult<&str, Instruction> {
    let (input, name) = alt((tag("mask"), tag("mem")))(input)?;

    if name == "mem" {
        return map(parse_mem, Instruction::Mem)(input);
    }

    if name == "mask" {
        return map(parse_mask, Instruction::Mask)(input);
    }

    unreachable!();
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(tag("\n"), parse_intruction)(input)
}

pub fn solve(input: &str) -> String {
    let (_, instructions) = parse_input(input).unwrap();

    let mut vm = VM::new();

    for ins in &instructions {
        vm.run(&ins);
    }

    vm.memory.values().sum::<u64>().to_string()
}
