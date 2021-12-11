use crate::matrix::transposed_iter;
use crate::parsers::{binary_str_to_decimal, full, lines};
use crate::solutions::day3::part1::parse_0_1_line;

fn filter_rows_by_value_at_column<T: std::cmp::PartialEq>(
    matrix: Vec<Vec<T>>,
    column: usize,
    value: T,
) -> Vec<Vec<T>> {
    matrix
        .into_iter()
        .filter(|row| row[column] == value)
        .collect()
}

fn most_common_bit<'a>(matrix: &[Vec<&'a str>], column: usize) -> &'a str {
    let number_of_1 = transposed_iter(matrix)
        .nth(column)
        .unwrap()
        .filter(|v| **v == "1")
        .count();
    let number_of_0 = transposed_iter(matrix)
        .nth(column)
        .unwrap()
        .filter(|v| **v == "0")
        .count();

    if number_of_1 >= number_of_0 {
        "1"
    } else {
        "0"
    }
}

fn least_common_bit<'a>(matrix: &[Vec<&'a str>], column: usize) -> &'a str {
    let number_of_1 = transposed_iter(matrix)
        .nth(column)
        .unwrap()
        .filter(|v| **v == "1")
        .count();
    let number_of_0 = transposed_iter(matrix)
        .nth(column)
        .unwrap()
        .filter(|v| **v == "0")
        .count();

    if number_of_0 <= number_of_1 {
        "0"
    } else {
        "1"
    }
}

pub fn solve(input: &str) -> usize {
    let (_, matrix) = full(lines(parse_0_1_line))(input).unwrap();

    let binary_length = matrix[0].len();

    let mut oxigen_mat = matrix.clone();
    for column in 0..binary_length {
        let common_bit = most_common_bit(&oxigen_mat, column);
        oxigen_mat = filter_rows_by_value_at_column(oxigen_mat, column, common_bit);
        if oxigen_mat.len() == 1 {
            break;
        }
    }

    let mut co2_mat = matrix.clone();
    for column in 0..binary_length {
        let common_bit = least_common_bit(&co2_mat, column);
        co2_mat = filter_rows_by_value_at_column(co2_mat, column, common_bit);
        if co2_mat.len() == 1 {
            break;
        }
    }

    let oxigen_bits = oxigen_mat[0].join("");
    let co2_bits = co2_mat[0].join("");

    let (_, oxigen) = binary_str_to_decimal(&oxigen_bits).unwrap();
    let (_, co2) = binary_str_to_decimal(&co2_bits).unwrap();

    oxigen * co2
}
