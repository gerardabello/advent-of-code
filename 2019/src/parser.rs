use nom::{character::complete::digit1, combinator::map_res, multi::separated_list1, IResult};

pub fn number_list<'a>(
    separator: impl Fn(&'a str) -> IResult<&'a str, char>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<u32>> {
    return separated_list1(separator, map_res(digit1, |s: &str| s.parse::<u32>()));
}
