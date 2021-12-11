use crate::solutions::day11::part1::step;

use crate::parsers::{full, matrix_of_digits};

pub fn solve(input: &str) -> usize {
    let (_, octopuses) = full(matrix_of_digits)(input).unwrap();

    let number_of_octopuses = octopuses.len() * octopuses[0].len();

    let mut octopuses_mut = octopuses;

    for i in 1..usize::MAX {
        let flashes = step(&mut octopuses_mut);
        if flashes == number_of_octopuses {
            return i;
        }
    }

    panic!("Not found");
}
