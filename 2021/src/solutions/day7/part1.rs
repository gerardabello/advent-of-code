use nom::{
    bytes::complete::tag,
    multi::separated_list1,
};

use crate::parsers::{full, unsigned_int};

pub fn sum_of_distances(slice: &[usize], point: usize) -> usize {
    slice
        .iter()
        .map(|v| ((point as isize) - (*v as isize)).abs() as usize)
        .sum()
}

pub fn geometric_mean(slice: &[usize]) -> usize {
    (0..1000)
        .map(|i| (i, sum_of_distances(slice, i)))
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap().1
}

pub fn solve(input: &str) -> usize {
    let (_, positions) = full(separated_list1(tag(","), unsigned_int::<usize>))(input).unwrap();

    geometric_mean(&positions)
}
