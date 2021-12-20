use nom::{bytes::complete::tag, multi::many1, sequence::tuple};

use crate::matrix::{neighbours_with_diagonals_and_self, pad};
use crate::parsers::{full, hash_dot_bool, matrix_of};

pub fn parse_input(input: &str) -> ([bool; 512], Vec<Vec<bool>>) {
    let (_, (filter, _, map)) = full(tuple((
        many1(hash_dot_bool),
        tag("\n\n"),
        matrix_of(hash_dot_bool),
    )))(input)
    .unwrap();

    (filter.try_into().expect("filter should be 512 long"), map)
}

fn bool_slice_to_filter_value(filter: &[bool; 512], bool_string: &[bool; 9]) -> bool {
    let index: usize = bool_string
        .iter()
        .rev()
        .enumerate()
        .map(|(i, b)| if *b { 2_usize.pow(i as u32) } else { 0 })
        .sum();

    filter[index]
}

fn enhance(filter: &[bool; 512], map: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut new_map = vec![vec![false; map[0].len() - 2]; map.len() - 2];

    for (y, row) in (&map[1..map.len() - 2]).iter().enumerate() {
        for (x, _) in (&row[1..row.len() - 2]).iter().enumerate() {
            let bs: [bool; 9] = neighbours_with_diagonals_and_self(map, x + 1, y + 1)
                .map(|(v, _, _)| v)
                .collect::<Vec<bool>>()
                .try_into()
                .expect("neighbours_with_diagonals_and_self should return 9 elements");
            new_map[y][x] = bool_slice_to_filter_value(filter, &bs)
        }
    }

    new_map
}

pub fn enhance_times(filter: &[bool; 512], map: &[Vec<bool>], n: usize) -> Vec<Vec<bool>> {
    let mut enhanced_map = pad(map, n * 2 + 1, false);
    for _ in 0..n {
        enhanced_map = enhance(filter, &enhanced_map);
    }

    enhanced_map
}

pub fn solve(input: &str) -> usize {
    let (filter, map) = parse_input(input);

    let enhanced_map = enhance_times(&filter, &map, 2);

    enhanced_map
        .iter()
        .map(|row| row.iter())
        .flatten()
        .filter(|b| **b)
        .count()
}
