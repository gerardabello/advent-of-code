use nom::{branch::alt, character::complete::char, multi::many1, IResult};

use crate::parsers::{full, lines};

pub fn is_open(c: char) -> bool {
    c == '(' || c == '[' || c == '{' || c == '<'
}

pub fn matching_close(open: char) -> char {
    match open {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn check_chunk(input: &[char]) -> Result<bool, char> {
    let mut state = vec![];

    for c in input {
        if is_open(*c) {
            state.push(c);
        } else {
            let last_state = state.last();
            if last_state.is_some() && *c == matching_close(**last_state.unwrap()) {
                state.pop();
            } else {
                return Err(*c);
            }
        }
    }

    Ok(state.is_empty())
}

pub fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
    many1(alt((
        char('('),
        char('{'),
        char('['),
        char('<'),
        char(')'),
        char('}'),
        char(']'),
        char('>'),
    )))(input)
}

pub fn solve(input: &str) -> usize {
    let (_, lines) = full(lines(parse_line))(input).unwrap();

    lines
        .iter()
        .map(|line| check_chunk(line))
        .filter(|r| r.is_err())
        .map(|r| match r {
            Err(c) => c,
            _ => unreachable!(),
        })
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum()
}
