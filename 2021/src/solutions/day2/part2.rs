use crate::parsers::full; 
use crate::solutions::day2::part1::{parse_commands, Direction};

pub fn solve(input: &str) -> isize {
    let (_, commands) = full(parse_commands)(input).unwrap();

    let mut depth: isize = 0;
    let mut position: isize = 0;
    let mut aim: isize = 0;
    for command in commands {
        match command.direction {
            Direction::Up => aim -= command.amount as isize,
            Direction::Down => aim += command.amount as isize,
            Direction::Forward => {
                position += command.amount as isize;
                depth += command.amount as isize * aim;
            },
        }
    }

    depth * position
}
