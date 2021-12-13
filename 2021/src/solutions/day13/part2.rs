use crate::matrix::{bool_from_coordinates, print_bool};
use crate::solutions::day13::part1::{fold_coordinates, parse_input};

pub fn solve(input: &str) -> &'static str {
    let (dot_coordinates, folds) = parse_input(input);

    let mut folded_coordinates = dot_coordinates;

    for fold in folds {
        folded_coordinates = fold_coordinates(&folded_coordinates, fold);
    }

    let map = bool_from_coordinates(&folded_coordinates);
    print_bool(&map);

    "Read string ☝️"
}
