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

pub fn filter_rows_by_value_at_column<T: std::cmp::PartialEq>(
    matrix: Vec<Vec<T>>,
    column: usize,
    value: T,
) -> Vec<Vec<T>>
{
    matrix
        .into_iter()
        .filter(|row| row[column] == value).collect()
}
