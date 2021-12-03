use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::tuple, IResult};

use crate::parsers::{full, lines, unsigned_int};

pub enum Direction {
    Up,
    Down,
    Forward,
}

pub struct Command {
    pub direction: Direction,
    pub amount: usize,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        map(tag("forward"), |_| Direction::Forward),
        map(tag("up"), |_| Direction::Up),
        map(tag("down"), |_| Direction::Down),
    ))(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    map(
        tuple((parse_direction, tag(" "), unsigned_int::<usize>)),
        |(direction, _, amount)| Command { direction, amount },
    )(input)
}

pub fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    lines(parse_command)(input)
}

pub fn solve(input: &str) -> usize {
    let (_, commands) = full(parse_commands)(input).unwrap();

    let mut depth: usize = 0;
    let mut position: usize = 0;
    for command in commands {
        match command.direction {
            Direction::Up => depth -= command.amount,
            Direction::Down => depth += command.amount,
            Direction::Forward => position += command.amount,
        }
    }

    depth * position
}
