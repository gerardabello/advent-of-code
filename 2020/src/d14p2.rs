use std::collections::HashMap;

use crate::d14p1::{parse_input, Instruction, Mask};

fn apply_mask(mask: &str, address: u64) -> Vec<u64> {
    let binary = format!("{:b}", address);
    let binary_padded = format!("{:0>36}", binary);

    let masked_rev: String = binary_padded
        .chars()
        .rev()
        .enumerate()
        .map(|(i, b)| match mask.chars().rev().nth(i) {
            Some('0') => b,
            Some('1') => '1',
            Some('X') => 'X',
            None => b,
            _ => unreachable!(),
        })
        .collect();

    let masked: String = masked_rev.chars().rev().collect();

    // The number of X to replace in the masked address
    let n_x = masked.chars().filter(|c| *c == 'X').count();

    // The different combinations of 1 and 0 to replace
    let replacers: Vec<String> = (0..(u64::pow(2, n_x as u32)))
        .map(|v| format!("{:b}", v))
        .collect();

    // for each replacer, replace the X to get the address
    replacers
        .iter()
        .map(|r| {
            let mut i_x = 0;
            let mut new_address = "".to_string();
            for c in masked.chars() {
                if c == 'X' {
                    let replacer_val = match r.chars().rev().nth(i_x) {
                        Some(c) => c,
                        None => '0',
                    };
                    new_address = format!("{}{}", &new_address, replacer_val);
                    i_x += 1;
                } else {
                    new_address = format!("{}{}", &new_address, c);
                }
            }

            u64::from_str_radix(&new_address, 2).unwrap()
        })
        .collect()
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
                let addresses = apply_mask(&self.mask.0, m.address);

                for a in &addresses {
                    self.memory.insert(*a, m.value);
                }
            }
        };
    }
}

pub fn solve(input: &str) -> String {
    let (_, instructions) = parse_input(input).unwrap();

    let mut vm = VM::new();

    for ins in &instructions {
        vm.run(&ins);
    }

    vm.memory.values().sum::<u64>().to_string()
}
