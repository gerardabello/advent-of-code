use crate::intcode::run_intcode;

use super::parse_input;

pub fn solve(input: &str) -> usize {
    let input = parse_input(input);

    run_intcode(&input, 12, 2).unwrap()
}
