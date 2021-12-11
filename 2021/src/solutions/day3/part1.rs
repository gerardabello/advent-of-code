use nom::{branch::alt, bytes::complete::tag, multi::many1, IResult};

use crate::matrix::transposed_iter;
use crate::parsers::{binary_str_to_decimal, full, lines};

pub fn parse_0_1_line(input: &str) -> IResult<&str, Vec<&str>> {
    many1(alt((tag("0"), tag("1"))))(input)
}

pub fn solve(input: &str) -> usize {
    let (_, matrix) = full(lines(parse_0_1_line))(input).unwrap();
    let binary_length = matrix[0].len();
    let number_of_inputs = matrix.len();

    let gamma_bits = transposed_iter(&matrix)
        .map(|column| {
            // Sum all the column, sum the "1"s, and check if we have more then half the number
            match column
                .map(|v| str::parse::<usize>(v).unwrap())
                .sum::<usize>()
                > number_of_inputs / 2
            {
                true => "1",
                false => "0",
            }
        })
        .collect::<Vec<&str>>()
        .join("");

    let (_, gamma) = binary_str_to_decimal(&gamma_bits).unwrap();

    let epsilon = usize::pow(2, binary_length as u32) - gamma - 1;

    gamma * epsilon
}
