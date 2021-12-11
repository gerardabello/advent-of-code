#![allow(dead_code)]

use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take,
    character::complete::digit1,
    character::complete::multispace0,
    combinator::eof,
    combinator::{map, map_res, recognize},
    error::ParseError,
    multi::{many1, separated_list1},
    sequence::{pair, tuple},
    IResult, Parser,
};

/* CHEATSHEET
========================
# Tags to Enum
    alt((
        map(tag("left"), |_| Direction::Left),
        map(tag("right"), |_| Direction::Right),
        map(tag("up"), |_| Direction::Up),
        map(tag("down"), |_| Direction::Down),
    ))

# Custom parser
    pub fn custom<'a, O, E, P>(parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        P: Parser<&'a str, O, E>,
        E: ParseError<&'a str>,
    {
        move |input: &'a str| {
            let (input, out) = parser.parse(input)?;
            ... more stuff ...
            Ok((input, out))
        }
    }


*/

/// Parses the input with `parser` and returns it if the remaining input is empty or only
/// whitespace.
pub fn full<'a, O, E, P>(parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    P: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    map(tuple((parser, multispace0, eof)), |(o1, _, _)| o1)
}

pub trait UnsignedInt {}
impl UnsignedInt for usize {}
impl UnsignedInt for u8 {}
impl UnsignedInt for u16 {}
impl UnsignedInt for u32 {}
impl UnsignedInt for u64 {}
impl UnsignedInt for u128 {}

pub fn unsigned_int<T: UnsignedInt + std::str::FromStr>(input: &str) -> IResult<&str, T> {
    map_res(recognize(digit1), str::parse)(input)
}

pub trait SignedInt {}
impl SignedInt for isize {}
impl SignedInt for i8 {}
impl SignedInt for i16 {}
impl SignedInt for i32 {}
impl SignedInt for i64 {}
impl SignedInt for i128 {}

pub fn signed_int<T: SignedInt + std::str::FromStr>(input: &str) -> IResult<&str, T> {
    alt((
        map_res(recognize(digit1), str::parse),
        map_res(recognize(pair(tag("-"), digit1)), str::parse),
        map_res(recognize(pair(tag("+"), digit1)), str::parse),
    ))(input)
}

// Uses `parser` on multiple lines and returns each output inside a Vec
pub fn lines<'a, O, E, P>(parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>, E>
where
    P: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    separated_list1(tag("\n"), parser)
}

pub fn binary_str_to_decimal(input: &str) -> IResult<&str, usize> {
    map_res(recognize(many1(alt((tag("0"), tag("1"))))), |bin_str| {
        usize::from_str_radix(bin_str, 2)
    })(input)
}

pub fn single_digit(input: &str) -> IResult<&str, usize> {
    map_res(take(1_usize), str::parse)(input)
}

pub fn matrix_of_digits(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    map_res(lines(many1(single_digit)), |matrix| {
        if matrix.iter().all(|row| row.len() == matrix[0].len()) {
            Ok(matrix)
        } else {
            Err("uneven rows in parsed matrix")
        }
    })(input)
}
