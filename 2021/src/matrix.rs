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
            print!("{}", v);
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
                print!("\x1b[0;31m{}\x1b[0m", v);
            } else {
                print!("{}", v);
            }
        }
        println!();
    }
}
