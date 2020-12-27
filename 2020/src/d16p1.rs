use nom::{
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

pub type Ticket = Vec<u32>;

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub name: String,
    pub ranges: Vec<(u32, u32)>,
}

impl Rule {
    pub fn validate(&self, n: u32) -> bool {
        self.ranges.iter().any(|r| n >= r.0 && n <= r.1)
    }
}

fn parse_i32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_ticket(input: &str) -> IResult<&str, Ticket> {
    separated_list1(tag(","), parse_i32)(input)
}

fn parse_range(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, min) = parse_i32(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, max) = parse_i32(input)?;

    Ok((input, (min, max)))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, name) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, ranges) = separated_list1(tag(" or "), parse_range)(input)?;

    Ok((
        input,
        Rule {
            name: name.to_owned(),
            ranges,
        },
    ))
}

pub fn parse_input(input: &str) -> IResult<&str, (Vec<Rule>, Ticket, Vec<Ticket>)> {
    let (input, rules) = separated_list1(tag("\n"), parse_rule)(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, _) = tag("your ticket:\n")(input)?;
    let (input, my_ticket) = parse_ticket(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, _) = tag("nearby tickets:\n")(input)?;
    let (input, tickets) = separated_list1(tag("\n"), parse_ticket)(input)?;

    Ok((input, (rules, my_ticket, tickets)))
}

pub fn solve(input: &str) -> String {
    let (_, (rules, _, tickets)) = parse_input(input).unwrap();

    tickets
        .iter()
        .flatten()
        .filter(|n| rules.iter().all(|r| !r.validate(**n)))
        .sum::<u32>()
        .to_string()
}
