use nom::multi::many1;

use crate::parsers::full;
use crate::solutions::day9::part1::{find_minimums, get_xy, parse_line};

pub fn grow_basin(map: &[Vec<usize>], base: (usize, usize), basin: &mut Vec<(usize, usize)>) {
    for (x_surround, y_surround) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let neighbour_x = base.0 as i32 + x_surround;
        let neighbour_y = base.1 as i32 + y_surround;
        let neighbour_v = get_xy(map, neighbour_x, neighbour_y);
        if neighbour_v.is_some() && neighbour_v.unwrap() < 9 {
            // here we know that neighbour coordinates are within bounds (so not negative)
            let neighbour_usize = (neighbour_x as usize, neighbour_y as usize);
            if !basin.contains(&neighbour_usize) {
                basin.push(neighbour_usize);
                grow_basin(map, neighbour_usize, basin);
            }
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
