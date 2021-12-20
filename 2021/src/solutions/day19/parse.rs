use nom::{bytes::complete::tag, multi::separated_list1, sequence::tuple, IResult};

use crate::parsers::{full, lines, signed_int, unsigned_int};

pub type Position = [isize;3];
pub type Rotation = [u8;3];

#[derive(Debug, Clone)]
pub struct Scanner {
    pub id: usize,
    pub beacons: Vec<Position>,
    pub position: Option<Position>,
    pub rotation: Option<Rotation>,
}

fn parse_position(input: &str) -> IResult<&str, Position> {
    let (input, (x, _, y, _, z)) = tuple((
        signed_int::<isize>,
        tag(","),
        signed_int::<isize>,
        tag(","),
        signed_int::<isize>,
    ))(input)?;

    Ok((input, [x, y, z]))
}

fn parse_scanner(input: &str) -> IResult<&str, Scanner> {
    let (input, (_, id, _)) =
        tuple((tag("--- scanner "), unsigned_int::<usize>, tag(" ---\n")))(input)?;

    let (input, beacons) = lines(parse_position)(input)?;

    Ok((
        input,
        Scanner {
            id,
            beacons,
            position: None,
            rotation: None,
        },
    ))
}

pub fn parse_input(input: &str) -> Vec<Scanner> {
    full(separated_list1(tag("\n\n"), parse_scanner))(input)
        .unwrap()
        .1
}

