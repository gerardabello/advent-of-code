use crate::matrix;
use crate::parsers::{full, matrix_of_digits};

pub fn find_minimums(map: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut minimums: Vec<(usize, usize)> = vec![];

    for (y, row) in map.iter().enumerate() {
        'main: for (x, val) in row.iter().enumerate() {
            for (n_val, _, _) in matrix::neighbours(map, x, y) {
                if n_val <= *val {
                    continue 'main;
                }
            }
            minimums.push((x, y));
        }
    }

    minimums
}

pub fn solve(input: &str) -> usize {
    let (_, map) = full(matrix_of_digits)(input).unwrap();

    let minimums = find_minimums(&map);

    matrix::print_with_highlights(&map, |x, y, _| minimums.contains(&(x, y)));

    minimums
        .into_iter()
        .map(|(x, y)| matrix::get_xy(&map, x, y).unwrap())
        .map(|h| h + 1)
        .sum()
}
