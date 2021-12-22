use std::collections::HashSet;
use std::ops::Range;

use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

use crate::parsers::{full, lines, signed_int};

pub type Cuboid = [Range<isize>; 3];

#[derive(Clone, Debug)]
pub enum Step {
    On(Cuboid),
    Off(Cuboid),
}

fn parse_range(input: &str) -> IResult<&str, Range<isize>> {
    let (input, start) = signed_int::<isize>(input)?;
    let (input, _) = tag("..")(input)?;
    let (input, end) = signed_int::<isize>(input)?;

    Ok((input, start..end + 1))
}

fn parse_line(input: &str) -> IResult<&str, Step> {
    let (input, on) = alt((map(tag("on"), |_| true), map(tag("off"), |_| false)))(input)?;
    let (input, _) = tag(" ")(input)?;

    let (input, _) = tag("x=")(input)?;
    let (input, range_x) = parse_range(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, _) = tag("y=")(input)?;
    let (input, range_y) = parse_range(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, _) = tag("z=")(input)?;
    let (input, range_z) = parse_range(input)?;

    let cuboid = [range_x, range_y, range_z];
    Ok((
        input,
        match on {
            true => Step::On(cuboid),
            false => Step::Off(cuboid),
        },
    ))
}

pub fn parse_input(input: &str) -> Vec<Step> {
    full(lines(parse_line))(input).unwrap().1
}

fn check_cuboid(cuboid: &Cuboid) -> bool {
    cuboid[0].start >= -50
        && cuboid[0].end <= 51
        && cuboid[1].start >= -50
        && cuboid[1].end <= 51
        && cuboid[2].start >= -50
        && cuboid[2].end <= 51
}

pub fn solve(input: &str) -> usize {
    let steps = parse_input(input);

    let mut hs = HashSet::new();

    for step in steps.iter().filter(|step| match step {
        Step::On(cuboid) => check_cuboid(cuboid),
        Step::Off(cuboid) => check_cuboid(cuboid),
    }) {
        match step {
            Step::On(cuboid) => {
                for x in cuboid[0].clone() {
                    for y in cuboid[1].clone() {
                        for z in cuboid[2].clone() {
                            hs.insert([x, y, z]);
                        }
                    }
                }
            }
            Step::Off(cuboid) => {
                for x in cuboid[0].clone() {
                    for y in cuboid[1].clone() {
                        for z in cuboid[2].clone() {
                            hs.remove(&[x, y, z]);
                        }
                    }
                }
            }
        }
    }

    hs.len()
}
