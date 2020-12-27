use std::collections::HashSet;
use std::convert::TryInto;
use std::fmt;

use nom::{
    bytes::complete::{tag, take, take_until},
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

pub const IMAGE_PIECE_SIZE: usize = 10;

#[derive(Clone, Debug)]
pub enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

pub struct ImageData {
    pub id: u32,
    data: [[bool; IMAGE_PIECE_SIZE]; IMAGE_PIECE_SIZE],
}

#[derive(Clone)]
pub struct ImagePiece {
    pub id: u32,
    pub rotation: Rotation,
    pub flip_v: bool,
    pub flip_h: bool,
}

impl fmt::Debug for ImagePiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.id).expect("Print should work");
        match self.rotation {
            Rotation::Deg0 => write!(f, "r0").expect("Print should work"),
            Rotation::Deg90 => write!(f, "r1").expect("Print should work"),
            Rotation::Deg180 => write!(f, "r2").expect("Print should work"),
            Rotation::Deg270 => write!(f, "r3").expect("Print should work"),
        }
        if self.flip_h {
            write!(f, "h").expect("Print should work");
        }
        if self.flip_v {
            write!(f, "v").expect("Print should work");
        }
        Ok(())
    }
}

    fn rotate_coords_90(size: usize, coords: (usize, usize)) -> (usize, usize) {
        (coords.1, size - 1 - coords.0)
    }

    pub fn rotate_coords(size: usize, coords: (usize, usize), rotation: &Rotation) -> (usize, usize) {
        let x = coords.0;
        let y = coords.1;
        match rotation {
            Rotation::Deg0 => (x, y),
            Rotation::Deg90 => rotate_coords_90(size,(x, y)),
            Rotation::Deg180 => rotate_coords_90(size,rotate_coords_90(size,(x, y))),
            Rotation::Deg270 => rotate_coords_90(size,rotate_coords_90(size,
                rotate_coords_90(size,(x, y)),
            )),
        }
    }

    pub fn flip_horizontal(size: usize, coords: (usize, usize), flip: bool) -> (usize, usize) {
        let x = coords.0;
        let y = coords.1;
        if flip {
            return (size - 1 - x, y);
        }

        (x, y)
    }

    pub fn flip_vertical(size: usize, coords: (usize, usize), flip: bool) -> (usize, usize) {
        let x = coords.0;
        let y = coords.1;
        if flip {
            return (x, size - 1 - y);
        }

        (x, y)
    }

impl ImagePiece {

    fn rotate_coords(coords: (usize, usize), rotation: &Rotation) -> (usize, usize) {
        rotate_coords(IMAGE_PIECE_SIZE, coords, rotation)
    }

    fn flip_horizontal(coords: (usize, usize), flip: bool) -> (usize, usize) {
        flip_horizontal(IMAGE_PIECE_SIZE, coords, flip)
    }

    fn flip_vertical(coords: (usize, usize), flip: bool) -> (usize, usize) {
        flip_vertical(IMAGE_PIECE_SIZE, coords, flip)
    }

    pub fn get_data_at_pos(&self, data: &ImageData, x: usize, y: usize) -> Option<bool> {
        if x >= IMAGE_PIECE_SIZE || y >= IMAGE_PIECE_SIZE {
            return None;
        }

        assert!(self.id == data.id);

        let (rx, ry) = ImagePiece::flip_vertical(
            ImagePiece::flip_horizontal(
                ImagePiece::rotate_coords((x, y), &self.rotation),
                self.flip_h,
            ),
            self.flip_v,
        );

        Some(data.data[ry][rx])
    }
}

fn vec_into_image_size_array<T>(v: Vec<T>) -> [T; IMAGE_PIECE_SIZE] {
    v.try_into().unwrap_or_else(|v: Vec<T>| {
        panic!(
            "Expected a Vec of length {} but it was {}",
            IMAGE_PIECE_SIZE,
            v.len()
        )
    })
}

fn parse_data_row(input: &str) -> IResult<&str, [bool; IMAGE_PIECE_SIZE]> {
    let (input, row) = take_until("\n")(input)?;

    let vector: Vec<_> = row.chars().map(|c| c == '#').collect();

    Ok((input, vec_into_image_size_array(vector)))
}

fn parse_data(input: &str) -> IResult<&str, [[bool; IMAGE_PIECE_SIZE]; IMAGE_PIECE_SIZE]> {
    let (input, to_parse) = take(IMAGE_PIECE_SIZE * IMAGE_PIECE_SIZE + (IMAGE_PIECE_SIZE))(input)?;
    let (_, vector) = separated_list1(tag("\n"), parse_data_row)(to_parse)?;

    Ok((input, vec_into_image_size_array(vector)))
}

fn parse_header(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("Tile ")(input)?;
    let (input, id) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = tag(":")(input)?;

    Ok((input, id))
}

fn parse_image_data(input: &str) -> IResult<&str, ImageData> {
    let (input, id) = parse_header(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, data) = parse_data(input)?;

    Ok((input, ImageData { id, data }))
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<ImageData>> {
    separated_list1(tag("\n"), parse_image_data)(input)
}

pub fn extract_top_border(piece: &ImagePiece, data: &ImageData) -> [bool; IMAGE_PIECE_SIZE] {
    let mut ret: [bool; IMAGE_PIECE_SIZE] = [false; IMAGE_PIECE_SIZE];
    for i in 0..IMAGE_PIECE_SIZE {
        ret[i] = piece.get_data_at_pos(data, i, 0).unwrap();
    }

    ret
}

pub fn extract_bottom_border(piece: &ImagePiece, data: &ImageData) -> [bool; IMAGE_PIECE_SIZE] {
    let mut ret: [bool; IMAGE_PIECE_SIZE] = [false; IMAGE_PIECE_SIZE];
    for i in 0..IMAGE_PIECE_SIZE {
        ret[i] = piece
            .get_data_at_pos(data, i, IMAGE_PIECE_SIZE - 1)
            .unwrap();
    }

    ret
}

pub fn extract_left_border(piece: &ImagePiece, data: &ImageData) -> [bool; IMAGE_PIECE_SIZE] {
    let mut ret: [bool; IMAGE_PIECE_SIZE] = [false; IMAGE_PIECE_SIZE];
    for i in 0..IMAGE_PIECE_SIZE {
        ret[i] = piece.get_data_at_pos(data, 0, i).unwrap();
    }

    ret
}

pub fn extract_right_border(piece: &ImagePiece, data: &ImageData) -> [bool; IMAGE_PIECE_SIZE] {
    let mut ret: [bool; IMAGE_PIECE_SIZE] = [false; IMAGE_PIECE_SIZE];
    for i in 0..IMAGE_PIECE_SIZE {
        ret[i] = piece
            .get_data_at_pos(data, IMAGE_PIECE_SIZE - 1, i)
            .unwrap();
    }

    ret
}

pub fn extract_borders(piece: &ImagePiece, data: &ImageData) -> [[bool; IMAGE_PIECE_SIZE]; 4] {
    [
        extract_left_border(piece, data),
        extract_right_border(piece, data),
        extract_top_border(piece, data),
        extract_bottom_border(piece, data),
    ]
}

pub fn flip_image_size_array<T: Copy>(a: &[T; IMAGE_PIECE_SIZE]) -> [T; IMAGE_PIECE_SIZE] {
    vec_into_image_size_array(a.iter().rev().cloned().collect())
}

pub fn find_edges<'a>(
    pieces: &'a [ImagePiece],
    datas: &[ImageData],
    only_corners: bool,
) -> Vec<&'a ImagePiece> {
    let mut corners: Vec<&ImagePiece> = Vec::new();

    for piece in pieces {
        let mut set: HashSet<[bool; IMAGE_PIECE_SIZE]> = HashSet::new();

        for other_piece in pieces.iter().filter(|p| p.id != piece.id) {
            for border in &extract_borders(
                other_piece,
                datas.iter().find(|d| d.id == other_piece.id).unwrap(),
            ) {
                set.insert(*border);
                set.insert(flip_image_size_array(border));
            }
        }

        let borders: [[bool; 10]; 4] =
            extract_borders(piece, datas.iter().find(|d| d.id == piece.id).unwrap());
        let found = borders.iter().filter(|b| set.contains(*b)).count();
        if only_corners {
            if found <= 2 {
                corners.push(piece);
            }
        } else if found <= 3 {
            corners.push(piece);
        }
    }

    corners
}

pub fn solve(input: &str) -> String {
    let (_, datas) = parse_input(input).unwrap();

    let pieces = datas.iter().map(|d| ImagePiece {
        id: d.id,
        rotation: Rotation::Deg0,
        flip_h: false,
        flip_v: false,
    }).collect::<Vec<_>>();

    let corners = find_edges(&pieces, &datas, true);

    corners
        .iter()
        .map(|p| p.id as u128)
        .product::<u128>()
        .to_string()
}
