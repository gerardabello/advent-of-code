use nom::{
    bytes::complete::tag,
    bytes::complete::take,
    character::complete::multispace0,
    combinator::eof,
    combinator::{map, map_opt, map_res},
    error::ParseError,
    multi::{many0, many1, many_m_n},
    sequence::tuple,
    IResult, Parser,
};

use crate::parsers::full;

// runs parser and asserts that the remaining input is [maybe some 0s][maybe some whitespace][eof]
fn full_with_zeros<'a, O, E, P>(parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    P: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    map(
        tuple((parser, many0(tag("0")), multispace0, eof)),
        |(o1, _, _, _)| o1,
    )
}

#[derive(Debug, Clone)]
pub struct PacketHeader {
    pub version: u8,
    pub type_id: u8,
}

#[derive(Debug, Clone)]
pub enum Packet {
    LiteralValue {
        header: PacketHeader,
        value: usize,
    },
    Operator {
        header: PacketHeader,
        children: Vec<Packet>,
    },
}

fn three_char_binary(input: &str) -> IResult<&str, u8> {
    map_opt(take(3_usize), |b: &str| match b {
        "000" => Some(0),
        "001" => Some(1),
        "010" => Some(2),
        "011" => Some(3),
        "100" => Some(4),
        "101" => Some(5),
        "110" => Some(6),
        "111" => Some(7),
        _ => None,
    })(input)
}

pub fn hex_to_binary(input: &str) -> String {
    let mut s = String::new();

    for c in input.chars() {
        let binary_string = match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            '\n' => "",
            c => unreachable!("unexpected char in hex string: '{}'", c),
        };
        s.push_str(binary_string);
    }

    s
}

fn literal_number(input: &str) -> IResult<&str, usize> {
    let mut binary_string = String::new();
    let mut acc_input = input;
    loop {
        let (new_input, first_bit) = take(1_usize)(acc_input)?;
        acc_input = new_input;
        let (new_input, four_bits) = take(4_usize)(acc_input)?;
        acc_input = new_input;

        binary_string.push_str(four_bits);

        if first_bit == "0" {
            return Ok((acc_input, usize::from_str_radix(&binary_string, 2).unwrap()));
        }
    }
}

fn literal_value_packet(header: PacketHeader, input: &str) -> IResult<&str, Packet> {
    let (input, value) = literal_number(input)?;
    Ok((input, Packet::LiteralValue { header, value }))
}

fn operator_packet_bits_length(header: PacketHeader, input: &str) -> IResult<&str, Packet> {
    let (input, bits_length) = map_res(take(15_usize), |s| usize::from_str_radix(s, 2))(input)?;

    let (input, subpackets_input) = take(bits_length)(input)?;
    let (_, children) = full(many1(parse_packet))(subpackets_input)?;

    Ok((input, Packet::Operator { header, children }))
}

fn operator_packet_subpackets_length(header: PacketHeader, input: &str) -> IResult<&str, Packet> {
    let (input, subpackets_length) =
        map_res(take(11_usize), |s| usize::from_str_radix(s, 2))(input)?;

    let (input, children) = many_m_n(subpackets_length, subpackets_length, parse_packet)(input)?;

    Ok((input, Packet::Operator { header, children }))
}

fn operator_packet(header: PacketHeader, input: &str) -> IResult<&str, Packet> {
    let (input, type_length_id) = take(1_usize)(input)?;

    match type_length_id {
        "0" => operator_packet_bits_length(header, input),
        "1" => operator_packet_subpackets_length(header, input),
        _ => unreachable!(),
    }
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    let (input, version) = three_char_binary(input)?;
    let (input, type_id) = three_char_binary(input)?;

    let header = PacketHeader { version, type_id };

    if type_id == 4 {
        literal_value_packet(header, input)
    } else {
        operator_packet(header, input)
    }
}

pub fn parse_input(input: &str) -> Packet {
    full_with_zeros(parse_packet)(input).unwrap().1
}

fn sum_of_versions(packet: &Packet) -> usize {
    match packet {
        Packet::LiteralValue { header, .. } => header.version as usize,
        Packet::Operator { header, children } => {
            header.version as usize + children.iter().map(sum_of_versions).sum::<usize>()
        }
    }
}

pub fn solve(input: &str) -> usize {
    let packet = parse_input(&hex_to_binary(input));

    sum_of_versions(&packet)
}
