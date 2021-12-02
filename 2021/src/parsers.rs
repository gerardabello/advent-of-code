use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    character::complete::multispace0,
    combinator::eof,
    combinator::{map_res, recognize},
    error::ParseError,
    multi::separated_list1,
    sequence::pair,
    IResult, Parser,
};

/// Parses the input with `parser` and returns it if the remaining input is empty or only
/// whitespace.
#[allow(dead_code)]
pub fn full<'a, O, E, P>(mut parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    P: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    move |input: &'a str| {
        let (input, o1) = parser.parse(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = eof(input)?;
        Ok((input, o1))
    }
}

pub trait UnsignedInt {}
impl UnsignedInt for usize {}
impl UnsignedInt for u8 {}
impl UnsignedInt for u16 {}
impl UnsignedInt for u32 {}
impl UnsignedInt for u64 {}
impl UnsignedInt for u128 {}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn signed_int<T: SignedInt + std::str::FromStr>(input: &str) -> IResult<&str, T> {
    alt((
        map_res(recognize(digit1), str::parse),
        map_res(recognize(pair(tag("-"), digit1)), str::parse),
        map_res(recognize(pair(tag("+"), digit1)), str::parse),
    ))(input)
}

// Uses `parser` on multiple lines and returns each output inside a Vec
#[allow(dead_code)]
pub fn lines<'a, O, E, P>(parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>, E>
where
    P: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    separated_list1(tag("\n"), parser)
}
