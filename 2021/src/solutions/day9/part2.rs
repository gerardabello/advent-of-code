use nom::multi::many1;

use crate::matrix;
use crate::parsers::full;
use crate::solutions::day9::part1::{find_minimums, parse_line};

pub fn grow_basin(map: &[Vec<usize>], base: (usize, usize), basin: &mut Vec<(usize, usize)>) {
    for (neighbour_val, neighbour_x, neighbour_y) in matrix::neighbours(map, base.0, base.1) {
        let neighbour_pos = (neighbour_x, neighbour_y);
        if neighbour_val < 9 && !basin.contains(&neighbour_pos) {
            basin.push(neighbour_pos);
            grow_basin(map, neighbour_pos, basin);
        }
    }
}

pub fn solve(input: &str) -> usize {
    let (_, map) = full(many1(parse_line))(input).unwrap();

    let minimums = find_minimums(&map);

    let mut basin_sizes: Vec<usize> = minimums
        .iter()
        .map(|minimum| {
            let mut basin = vec![];
            grow_basin(&map, *minimum, &mut basin);
            basin.len()
        })
        .collect();

    basin_sizes.sort_unstable();
    basin_sizes = basin_sizes.into_iter().rev().collect();

    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}
