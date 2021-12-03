use crate::matrix::{filter_rows_by_value_at_column, transpose};
use crate::parsers::{binary_str_to_decimal, full, lines};
use crate::solutions::day3::part1::parse_0_1_line;

fn most_common_bit(matrix: Vec<Vec<&str>>, column: usize) -> &str {
    let transposed = transpose(matrix);

    let number_of_1 = transposed[column].iter().filter(|v| **v == "1").count();
    let number_of_0 = transposed[column].iter().filter(|v| **v == "0").count();

    if number_of_1 >= number_of_0 {
        "1"
    } else {
        "0"
    }
}

fn least_common_bit(matrix: Vec<Vec<&str>>, column: usize) -> &str {
    let transposed = transpose(matrix);

    let number_of_1 = transposed[column].iter().filter(|v| **v == "1").count();
    let number_of_0 = transposed[column].iter().filter(|v| **v == "0").count();

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
        let common_bit = most_common_bit(oxigen_mat.clone(), column);
        oxigen_mat = filter_rows_by_value_at_column(oxigen_mat, column, common_bit);
        if oxigen_mat.len() == 1 {
            break;
        }
    }

    let mut co2_mat = matrix.clone();
    for column in 0..binary_length {
        let common_bit = least_common_bit(co2_mat.clone(), column);
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
