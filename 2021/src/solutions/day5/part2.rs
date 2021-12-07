
use crate::parsers::{full, lines};
use crate::solutions::day5::part1::{
    is_line_horizontal, is_line_vertical, parse_line, Line, Map,
};

pub fn mark_line_in_map(map: &mut Map, line: &Line) {
    if is_line_vertical(line) {
        let x = line.0 .0;
        let start = line.0 .1;
        let end = line.1 .1;
        let iter = if start > end {
            end..(start + 1)
        } else {
            start..(end + 1)
        };
        for y in iter {
            map[y][x] += 1;
        }
        return;
    }

    if is_line_horizontal(line) {
        let y = line.0 .1;
        let start = line.0 .0;
        let end = line.1 .0;
        let iter = if start > end {
            end..(start + 1)
        } else {
            start..(end + 1)
        };
        for x in iter {
            map[y][x] += 1;
        }
        return;
    }

    let startx = line.0 .0;
    let endx = line.1 .0;
    let starty = line.0 .1;
    let endy = line.1 .1;

    let ascx = startx < endx;
    let ascy = starty < endy;

    let len = if !ascx {
        startx - endx + 1
    } else {
        endx - startx + 1
    };

    for i in 0..len {
        let x = if ascx { startx + i } else { startx - i };
        let y = if ascy { starty + i } else { starty - i };
        map[y][x] += 1;
    }
}

pub fn solve(input: &str) -> usize {
    let (_, lines) = full(lines(parse_line))(input).unwrap();

    let mut map = vec![vec![0; 1000]; 1000];

    for line in lines.iter() {
        mark_line_in_map(&mut map, line);
    }

    map.iter().flatten().filter(|v| **v >= 2).count()
}
