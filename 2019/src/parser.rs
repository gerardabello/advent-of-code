use std::str::FromStr;

use nom::{character::complete::digit1, combinator::map_res, multi::separated_list1, IResult};

pub fn number_list<'a, T: FromStr>(
    separator: impl Fn(&'a str) -> IResult<&'a str, char>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<T>> {
    return separated_list1(separator, map_res(digit1, |s: &str| s.parse::<T>()));
}
