use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::{many1, separated_list1},
    IResult,
};
use std::collections::HashMap;

pub enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, s) = alt((
        tag("e"),
        tag("w"),
        tag("sw"),
        tag("nw"),
        tag("se"),
        tag("ne"),
    ))(input)?;

    let dir = match s {
        "e" => Direction::East,
        "w" => Direction::West,
        "se" => Direction::SouthEast,
        "ne" => Direction::NorthEast,
        "sw" => Direction::SouthWest,
        "nw" => Direction::NorthWest,
        _ => unreachable!(),
    };

    Ok((input, dir))
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Direction>>> {
    separated_list1(tag("\n"), many1(parse_direction))(input)
}

pub fn dir_to_hex_coords(dir: &Direction) -> (isize, isize) {
    match dir {
        Direction::East => (1, 0),
        Direction::West => (-1, 0),

        Direction::NorthEast => (0, 1),
        Direction::NorthWest => (-1, 1),
        Direction::SouthWest => (0, -1),
        Direction::SouthEast => (1, -1),
    }
}

fn add_dir(dir: &Direction, coords: (isize, isize)) -> (isize, isize) {
    let relative_coords = dir_to_hex_coords(dir);

    (coords.0 + relative_coords.0, coords.1 + relative_coords.1)
}

pub fn dirs_to_hex_coords(dirs: &[Direction]) -> (isize, isize) {
    dirs.iter().fold((0, 0), |acc, dir| add_dir(dir, acc))
}

fn count_slice_by_key<T: Clone + std::hash::Hash + std::cmp::Eq>(
    slice: &[T],
) -> HashMap<T, usize> {
    let mut hm: HashMap<T, usize> = HashMap::new();
    for v in slice {
        if hm.contains_key(v) {
            hm.insert(v.clone(), hm.get(v).unwrap() + 1);
        } else {
            hm.insert(v.clone(), 0);
        }
    }

    hm
}

pub fn solve(input: &str) -> String {
    let (_, directions) = parse_input(input).unwrap();
    let coords: Vec<(isize, isize)> = directions
        .into_iter()
        .map(|dirs| dirs_to_hex_coords(&dirs))
        .collect();

    let counts = count_slice_by_key(&coords);

    counts
        .iter()
        .filter(|(_, v)| *v % 2 == 0)
        .count()
        .to_string()
}
