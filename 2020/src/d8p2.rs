use crate::d8p1::{parse_instructions, run_instruction, Instruction, Operation, State};
use std::collections::HashSet;

fn run_instructions_until_loop_or_termination(instructions: &[Instruction]) -> (State, bool) {
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
            return (state, false);
        }

        if state.instruction_pointer == instructions.len() {
            return (state, true);
        }

        visited_instructions.insert(state.instruction_pointer);

        state = run_instruction(instructions, &state);
    }
}

fn mutate_instruction(
    instructions: &[Instruction],
    mutating_index: usize,
) -> Option<Vec<Instruction>> {
    let mut clone: Vec<Instruction> = Vec::new();
    clone.extend_from_slice(instructions);

    let mut to_mutate = clone
        .get_mut(mutating_index)
        .expect("mutating_index should exist");

    if to_mutate.operation == Operation::Jump {
        to_mutate.operation = Operation::Noop;
    } else if to_mutate.operation == Operation::Noop {
        to_mutate.operation = Operation::Jump;
    } else {
        return None;
    }

    Some(clone)
}

pub fn solve(input: &str) -> String {
    let instructions = parse_instructions(input);

    let mut mutating_index = 0;

    loop {
        if let Some(v) = mutate_instruction(&instructions, mutating_index) {
            let (state, terminated) = run_instructions_until_loop_or_termination(&v);

            if terminated {
                return state.accumulator.to_string();
            }
        }
        mutating_index += 1;
    }
}
