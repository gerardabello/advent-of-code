#[allow(dead_code)]
pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[allow(dead_code)]
pub fn filter_rows_by_value_at_column<T: std::cmp::PartialEq>(
    matrix: Vec<Vec<T>>,
    column: usize,
    value: T,
) -> Vec<Vec<T>> {
    matrix
        .into_iter()
        .filter(|row| row[column] == value)
        .collect()
}

#[allow(dead_code)]
pub fn print<T: std::fmt::Display>(map: &[Vec<T>]) {
    for row in map {
        for v in row {
            print!("{:<2}", v);
        }
        println!();
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn get_xy_signed_index<T: Copy>(map: &[Vec<T>], x: isize, y: isize) -> Option<T> {
    if are_valid_isize_coordinates(map, x, y) {
        return Some(map[y as usize][x as usize]);
    }

    None
}

#[allow(dead_code)]
pub fn get_xy<T: Copy>(map: &[Vec<T>], x: usize, y: usize) -> Option<T> {
    if are_valid_usize_coordinates(map, x, y) {
        return Some(map[y as usize][x as usize]);
    }

    None
}

const SIMPLE_NEIGHBOURS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const DIAGINAL_NEIGHBOURS: [(isize, isize); 4] = [(-1, -1), (1, 1), (1, -1), (-1, 1)];

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn mutate_all<T, F>(map: &mut Vec<Vec<T>>, f: F)
where
    F: Fn(&mut T),
{
    map.iter_mut()
        .for_each(|row| row.iter_mut().for_each(|v| f(v)));
}
