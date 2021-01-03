pub mod part1;
pub mod part2;

pub use part1::solve as part1;
pub use part2::solve as part2;

use nom::{branch::alt, bytes::complete::tag, multi::many1, IResult};

pub enum Direction {
    North,
    South,
    East,
    West,
}

pub fn move_direction(pos: (i32, i32), dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::North => (pos.0, pos.1 - 1),
        Direction::South => (pos.0, pos.1 + 1),
        Direction::West => (pos.0 - 1, pos.1),
        Direction::East => (pos.0 + 1, pos.1),
    }
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, c) = alt((tag("^"), tag("v"), tag(">"), tag("<")))(input)?;

    match c {
        "^" => Ok((input, Direction::North)),
        "v" => Ok((input, Direction::South)),
        ">" => Ok((input, Direction::East)),
        "<" => Ok((input, Direction::West)),
        other => unreachable!("{}", other),
    }
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(parse_direction)(input)
}
