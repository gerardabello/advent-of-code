use crate::matrix::neighbours_with_diagonals;
use crate::parsers::{full, matrix_of_digits};

pub fn step(octopuses: &mut Vec<Vec<usize>>) -> usize {
    octopuses
        .iter_mut()
        .for_each(|row| row.iter_mut().for_each(|v| *v += 1));

    let mut positions_flashed = vec![];
    loop {
        let mut positions_to_flash = vec![];
        for (y, row) in octopuses.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if *value > 9 {
                    positions_to_flash.push((x, y));
                }
            }
        }

        positions_to_flash = positions_to_flash
            .into_iter()
            .filter(|pos| !positions_flashed.contains(pos))
            .collect();

        if positions_to_flash.is_empty() {
            break;
        }

        for (x, y) in positions_to_flash.iter() {
            for (_, nx, ny) in neighbours_with_diagonals(&octopuses.clone(), *x, *y) {
                octopuses[ny][nx] += 1;
            }
            positions_flashed.push((*x, *y));
        }
    }

    for (x, y) in positions_flashed.iter() {
        octopuses[*y][*x] = 0;
    }

    positions_flashed.len()
}

pub fn solve(input: &str) -> usize {
    let (_, octopuses) = full(matrix_of_digits)(input).unwrap();

    let mut total_flashes = 0;
    let mut octopuses_mut = octopuses;

    for _ in 0..100 {
        total_flashes += step(&mut octopuses_mut);
    }

    total_flashes
}
