#![allow(dead_code)]

pub fn transposed_iter<'iter, Column, Item: 'iter>(
    columns: &'iter [Column],
) -> impl Iterator<Item = impl Iterator<Item = &'iter Item>>
where
    &'iter Column: IntoIterator<Item = &'iter Item>,
{
    (0..).scan((), move |&mut (), row_idx| {
        Some({
            let mut columns_iterator = columns.iter();
            let first_column = columns_iterator.next()?;
            let first: &'iter Item = first_column.into_iter().nth(row_idx)?;
            Iterator::chain(
                ::core::iter::once(first),
                columns_iterator.map(move |column| {
                    column.into_iter().nth(row_idx).unwrap() // assumes the columns are of equal length
                }),
            )
        })
    })
}

pub fn print<T: std::fmt::Display>(map: &[Vec<T>]) {
    print_with_highlights(map, |_, _, _| false)
}

pub fn print_with_highlights<
    T: std::fmt::Display + std::marker::Sized,
    F: Fn(usize, usize, &T) -> bool,
>(
    map: &[Vec<T>],
    filter: F,
) {
    for (y, row) in map.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            if filter(x, y, v) {
                print!("\x1b[0;31m{:<2}\x1b[0m", v);
            } else {
                print!("{:<2}", v);
            }
        }
        println!();
    }
}

pub fn print_bool(map: &[Vec<bool>]) {
    for row in map.iter() {
        for v in row.iter() {
            if *v {
                print!("# ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

pub fn are_valid_isize_coordinates<T>(map: &[Vec<T>], x: isize, y: isize) -> bool {
    let height = map.len();
    let width = map[0].len();

    if x < 0 || x >= width as isize {
        return false;
    }

    if y < 0 || y >= height as isize {
        return false;
    }

    true
}

pub fn are_valid_usize_coordinates<T>(map: &[Vec<T>], x: usize, y: usize) -> bool {
    let height = map.len();
    let width = map[0].len();

    if x >= width {
        return false;
    }

    if y >= height {
        return false;
    }

    true
}

pub fn get_xy_signed_index<T: Copy>(map: &[Vec<T>], x: isize, y: isize) -> Option<T> {
    if are_valid_isize_coordinates(map, x, y) {
        return Some(map[y as usize][x as usize]);
    }

    None
}

pub fn get_xy<T: Copy>(map: &[Vec<T>], x: usize, y: usize) -> Option<T> {
    if are_valid_usize_coordinates(map, x, y) {
        return Some(map[y as usize][x as usize]);
    }

    None
}

const SIMPLE_NEIGHBOURS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const DIAGINAL_NEIGHBOURS: [(isize, isize); 4] = [(-1, -1), (1, 1), (1, -1), (-1, 1)];

pub fn neighbours<T: Copy>(
    map: &[Vec<T>],
    x: usize,
    y: usize,
) -> impl Iterator<Item = (T, usize, usize)> + '_ {
    SIMPLE_NEIGHBOURS
        .iter()
        .map(move |(rx, ry)| (x as isize + rx, y as isize + ry))
        .filter(|(nx, ny)| are_valid_isize_coordinates(map, *nx, *ny))
        .map(|(nx, ny)| (nx as usize, ny as usize))
        .map(|(nx, ny)| (get_xy(map, nx, ny).unwrap(), nx, ny))
}

pub fn neighbours_with_diagonals<T: Copy>(
    map: &[Vec<T>],
    x: usize,
    y: usize,
) -> impl Iterator<Item = (T, usize, usize)> + '_ {
    SIMPLE_NEIGHBOURS
        .iter()
        .chain(DIAGINAL_NEIGHBOURS.iter())
        .map(move |(rx, ry)| (x as isize + rx, y as isize + ry))
        .filter(|(nx, ny)| are_valid_isize_coordinates(map, *nx, *ny))
        .map(|(nx, ny)| (nx as usize, ny as usize))
        .map(|(nx, ny)| (get_xy(map, nx, ny).unwrap(), nx, ny))
}

pub fn mutate_all<T, F>(map: &mut Vec<Vec<T>>, f: F)
where
    F: Fn(&mut T),
{
    map.iter_mut()
        .for_each(|row| row.iter_mut().for_each(|v| f(v)));
}

pub fn bool_from_coordinates(coordinates: &[(usize, usize)]) -> Vec<Vec<bool>> {
    let max_x = coordinates.iter().map(|(x, _)| *x).max().unwrap();
    let max_y = coordinates.iter().map(|(_, y)| *y).max().unwrap();

    let mut map = vec![vec![false; max_x + 1]; max_y + 1];

    for (x, y) in coordinates {
        map[*y][*x] = true;
    }

    map
}
