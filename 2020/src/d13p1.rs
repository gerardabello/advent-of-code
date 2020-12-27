use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::digit1,
    combinator::{map, map_res, recognize},
    multi::separated_list1,
    IResult,
};

fn parse_unsigned_integer_64(input: &str) -> IResult<&str, u64> {
    map_res(recognize(digit1), str::parse)(input)
}

fn parse_bus_id(input: &str) -> IResult<&str, Option<u64>> {
    let parser = alt((tag("x"), is_a("0123456789")));

    map(parser, |s| match s {
        "x" => None,
        num => Some(num.parse::<u64>().unwrap()),
    })(input)
}

pub fn parse_input(input: &str) -> IResult<&str, (u64, Vec<Option<u64>>)> {
    let (input, arrival_time) = parse_unsigned_integer_64(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, result_ints) = separated_list1(tag(","), parse_bus_id)(input)?;

    Ok((input, (arrival_time, result_ints)))
}

pub fn next_bus_wait (timestamp: u64, bus_id: u64) -> u64 {
    bus_id - (timestamp % bus_id)
}

fn first_bus(timestamp: u64, bus_ids: &[u64]) -> Option<(u64, u64)> {
    bus_ids
        .iter()
        .map(|id| (*id, next_bus_wait(timestamp, *id)))
        .min_by(|a, b| a.1.cmp(&b.1))
}

pub fn solve(input: &str) -> String {
    let (_, (arrival_time, bus_ids_o)) = parse_input(input).unwrap();

    let bus_ids: Vec<u64> = bus_ids_o
        .iter()
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect();

    let (id, wait_time) = first_bus(arrival_time, &bus_ids).unwrap();

    (id * wait_time).to_string()
}
