pub mod part1;
pub mod part2;

pub use part1::solve as part1;
pub use part2::solve as part2;

use crate::parser::number_list;

use nom::character::complete::char;

pub fn parse_input(input: &str) -> Vec<u32> {
    return number_list(char(','))(input).unwrap().1;
}
