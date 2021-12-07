use nom::{bytes::complete::tag, multi::separated_list1};

use crate::parsers::{full, unsigned_int};

fn advance_day(state: &[usize]) -> Vec<usize> {
    let mut new_state = vec![];

    for s in state {
        match s {
            0 => {
                new_state.push(8);
                new_state.push(6);
            }
            days => {
                new_state.push(days - 1);
            }
        }
    }

    new_state
}

pub fn solve(input: &str) -> usize {
    let (_, initial_state) = full(separated_list1(tag(","), unsigned_int::<usize>))(input).unwrap();

    let mut state = initial_state;
    for _ in 0..80 {
        state = advance_day(&state);
    }

    state.len()
}
