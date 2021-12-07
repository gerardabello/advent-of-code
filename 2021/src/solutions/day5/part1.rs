use nom::{bytes::complete::tag, sequence::tuple, IResult};

use crate::parsers::{full, lines, unsigned_int};

pub type Position = (usize, usize);
pub type Line = (Position, Position);
pub type Map = Vec<Vec<usize>>;

pub fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, (x1, _, y1, _, x2, _, y2)) = tuple((
        unsigned_int::<usize>,
        tag(","),
        unsigned_int::<usize>,
        tag(" -> "),
        unsigned_int::<usize>,
        tag(","),
        unsigned_int::<usize>,
    ))(input)?;

    Ok((input, ((x1, y1), (x2, y2))))
}

pub fn is_line_vertical(line: &Line) -> bool {
    line.0 .0 == line.1 .0
}

pub fn is_line_horizontal(line: &Line) -> bool {
    line.0 .1 == line.1 .1
}

fn mark_line_in_map(map: &mut Map, line: &Line) {
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

    panic!("Line not supported");
}

pub fn solve(input: &str) -> usize {
    let (_, lines) = full(lines(parse_line))(input).unwrap();

    let horizontal_or_vertical_lines: Vec<Line> = lines
        .into_iter()
        .filter(|line| is_line_vertical(line) || is_line_horizontal(line))
        .collect();

    let mut map = vec![vec![0; 1000]; 1000];

    for line in horizontal_or_vertical_lines.iter() {
        mark_line_in_map(&mut map, line);
    }

    map.iter().flatten().filter(|v| **v >= 2).count()
}
