use crate::parsers::{full, lines, unsigned_int};

pub fn parse_input(input: &str) -> Vec<usize> {
    full(lines(unsigned_int::<usize>))(input).unwrap().1
}

pub fn solve(input: &str) -> usize {
    let _ = parse_input(input);

    panic!("Not implemented");
}
