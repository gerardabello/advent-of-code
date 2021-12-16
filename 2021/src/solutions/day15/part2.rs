use crate::parsers::{full, lines};
use crate::solutions::day15::part1::{find_shortest_path, parse_line};

pub fn solve(input: &str) -> usize {
    let (_, map) = full(lines(parse_line))(input).unwrap();

    let height = map.len();
    let width = map[0].len();


    let mut big_map = vec![vec![0; width * 5]; height * 5];

    for bx in 0..5 {
        for by in 0..5 {
            for (y, row) in map.iter().enumerate() {
                for (x, v) in row.iter().enumerate() {
                    let fx = bx * width + x;
                    let fy = by * height + y;
                    big_map[fy][fx] = ((v - 1 + bx + by) % 9) + 1;
                }
            }
        }
    }

    find_shortest_path(&big_map)
}
