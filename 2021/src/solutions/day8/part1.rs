use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::recognize,
    multi::{many1, separated_list1},
    IResult,
};

use crate::parsers::{full, lines};

pub fn parse_line(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (input, patterns) = separated_list1(tag(" "), recognize(many1(alpha1)))(input)?;
    let (input, _) = tag(" | ")(input)?;
    let (input, digits) = separated_list1(tag(" "), recognize(many1(alpha1)))(input)?;

    assert!(patterns.len() == 10);
    assert!(digits.len() == 4);

    Ok((input, (patterns, digits)))
}

pub fn solve(input: &str) -> usize {
    let (_, entries) = full(lines(parse_line))(input).unwrap();

    entries
        .into_iter()
        .map(|(_, digits)| digits)
        .flatten()
        .filter(|digit| {
            digit.len() == 2 || digit.len() == 3 || digit.len() == 4 || digit.len() == 7
        })
        .count()
}
