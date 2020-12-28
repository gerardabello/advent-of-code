pub mod part1;
pub mod part2;

pub use part1::solve as part1;
pub use part2::solve as part2;

use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    IResult,
};

pub type Gift = (u32, u32, u32);

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_gift(input: &str) -> IResult<&str, Gift> {
    let (input, l) = parse_u32(input)?;
    let (input, _) = tag("x")(input)?;
    let (input, w) = parse_u32(input)?;
    let (input, _) = tag("x")(input)?;
    let (input, h) = parse_u32(input)?;

    Ok((input, (l, w, h)))
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Gift>> {
    separated_list1(tag("\n"), parse_gift)(input)
}
