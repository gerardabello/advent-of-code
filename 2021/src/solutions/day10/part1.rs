use nom::IResult;

use crate::parsers::{full, lines, unsigned_int};

pub fn parse_line(input: &str) -> IResult<&str, usize> {
    unsigned_int::<usize>(input)
}

pub fn solve(input: &str) -> usize {
    let (_, _) = full(lines(parse_line))(input).unwrap();

    panic!("Not implemented");
}
