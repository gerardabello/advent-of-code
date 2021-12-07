use nom::{bytes::complete::tag, multi::separated_list1};

use crate::parsers::{full, unsigned_int};

pub fn distance_cost(distance: u128) -> u128 {
    let mut cost = 0;

    for i in 1..(distance + 1){
        cost += i;
    }

    cost
}

pub fn sum_of_distances(slice: &[u128], point: u128) -> u128 {
    slice
        .iter()
        .map(|v| ((point as isize) - (*v as isize)).abs() as u128)
        .map(distance_cost)
        .sum()
}

pub fn geometric_mean(slice: &[u128]) -> u128 {
    (0..1000)
        .map(|i| (i, sum_of_distances(slice, i)))
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap()
        .1
}

pub fn solve(input: &str) -> u128 {
    let (_, positions) = full(separated_list1(tag(","), unsigned_int::<u128>))(input).unwrap();

    geometric_mean(&positions)
}
