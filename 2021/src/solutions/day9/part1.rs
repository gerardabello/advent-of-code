use nom::{
    bytes::complete::{tag, take},
    combinator::map,
    multi::{many1, many_till},
    IResult,
};

use crate::matrix;
use crate::parsers::full;

pub fn parse_line(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, (numbers, _)) = many_till(
        map(take(1_usize), |c: &str| c.parse::<usize>().unwrap()),
        tag("\n"),
    )(input)?;

    Ok((input, numbers))
}

pub fn get_xy(map: &[Vec<usize>], x: i32, y: i32) -> Option<usize> {
    let height = map.len();
    let width = map[0].len();

    if x < 0 || x >= width as i32 {
        return None;
    }

    if y < 0 || y >= height as i32 {
        return None;
    }

    Some(map[y as usize][x as usize])
}

pub fn find_minimums(map: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut minimums: Vec<(usize, usize)> = vec![];

    for (y, row) in map.iter().enumerate() {
        'main: for (x, val) in row.iter().enumerate() {
            for x_surround in -1..2 {
                for y_surround in -1..2 {
                    if x_surround != 0 || y_surround != 0 {
                        let surround_result =
                            get_xy(map, x as i32 + x_surround, y as i32 + y_surround);
                        if surround_result.is_some() && surround_result.unwrap() <= *val {
                            continue 'main;
                        }
                    }
                }
            }
            minimums.push((x, y));
        }
    }

    minimums
}

pub fn solve(input: &str) -> usize {
    let (_, map) = full(many1(parse_line))(input).unwrap();

    let minimums = find_minimums(&map);

    matrix::print_with_highlights(&map, |x, y, _| minimums.contains(&(x, y)));

    minimums
        .into_iter()
        .map(|(x, y)| get_xy(&map, x as i32, y as i32).unwrap())
        .map(|h| h + 1)
        .sum()
}
