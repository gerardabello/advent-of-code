#![allow(dead_code)]

use std::ops::Neg;

fn rotate_3d_90_z<T>(point: &[T; 3]) -> [T; 3]
where
    T: Neg<Output = T> + Copy,
{
    [point[1], -point[0], point[2]]
}

fn rotate_3d_90_y<T>(point: &[T; 3]) -> [T; 3]
where
    T: Neg<Output = T> + Copy,
{
    [-point[2], point[1], point[0]]
}

fn rotate_3d_90_x<T>(point: &[T; 3]) -> [T; 3]
where
    T: Neg<Output = T> + Copy,
{
    [point[0], point[2], -point[1]]
}

pub fn rotate_3d_90x_xyz<T>(point: &[T; 3], amounts: &[u8; 3]) -> [T; 3]
where
    T: Neg<Output = T> + Copy,
{
    let mut pos = *point;

    for _ in 0..(amounts[0] % 4) {
        pos = rotate_3d_90_x(&pos);
    }

    for _ in 0..(amounts[1] % 4) {
        pos = rotate_3d_90_y(&pos);
    }

    for _ in 0..(amounts[2] % 4) {
        pos = rotate_3d_90_z(&pos);
    }

    pos
}

fn rotate_2d_90<T>(point: &[T; 2]) -> [T; 2]
where
    T: Neg<Output = T> + Copy,
{
    [point[1], -point[0]]
}

pub fn rotate_2d_90x<T>(point: &[T; 2], amount: u8) -> [T; 2]
where
    T: Neg<Output = T> + Copy,
{
    let mut pos = *point;

    for _ in 0..(amount % 4) {
        pos = rotate_2d_90(&pos);
    }

    pos
}

// This is was calculated by generating all permutations and checking which ones produce the same
// output.
pub const ALL_3D_90X_ROTATIONS: [[u8; 3]; 24] = [
    [0, 0, 2],
    [1, 0, 2],
    [0, 2, 0],
    [1, 2, 0],
    [0, 2, 3],
    [0, 1, 3],
    [0, 0, 3],
    [0, 3, 3],
    [1, 2, 3],
    [0, 3, 2],
    [1, 0, 3],
    [0, 1, 0],
    [0, 2, 2],
    [1, 2, 2],
    [0, 0, 0],
    [1, 0, 0],
    [0, 0, 1],
    [0, 3, 1],
    [0, 2, 1],
    [0, 1, 1],
    [1, 0, 1],
    [0, 1, 2],
    [1, 2, 1],
    [0, 3, 0],
];
