use std::cmp::Ordering;
use std::collections::HashSet;

use nom::{bytes::complete::tag, bytes::complete::take, sequence::tuple, IResult};

use crate::parsers::{full, lines, unsigned_int};

#[derive(Debug, Copy, Clone)]
pub enum Fold {
    Left(usize),
    Up(usize),
}

fn parse_coordinate(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, (x, _, y)) =
        tuple((unsigned_int::<usize>, tag(","), unsigned_int::<usize>))(input)?;
    Ok((input, (x, y)))
}

fn parse_fold(input: &str) -> IResult<&str, Fold> {
    let (input, (_, dir, _, val)) = tuple((
        tag("fold along "),
        take(1_usize),
        tag("="),
        unsigned_int::<usize>,
    ))(input)?;

    Ok((
        input,
        match dir {
            "x" => Fold::Left(val),
            "y" => Fold::Up(val),
            _ => unreachable!(),
        },
    ))
}

fn fold_up(coordinates: &[(usize, usize)], line: usize) -> Vec<(usize, usize)> {
    coordinates
        .iter()
        .map(|(x, y)| match line.cmp(y) {
            Ordering::Equal => unreachable!(), // by the problem definition
            Ordering::Less => (*x, line - (*y - line)),
            Ordering::Greater => (*x, *y),
        })
        .collect()
}

fn fold_left(coordinates: &[(usize, usize)], line: usize) -> Vec<(usize, usize)> {
    coordinates
        .iter()
        .map(|(x, y)| match line.cmp(x) {
            Ordering::Equal => unreachable!(), // by the problem definition
            Ordering::Less => (line - (*x - line), *y),
            Ordering::Greater => (*x, *y),
        })
        .collect()
}

pub fn fold_coordinates(coordinates: &[(usize, usize)], fold: Fold) -> Vec<(usize, usize)> {
    match fold {
        Fold::Left(line) => fold_left(coordinates, line),
        Fold::Up(line) => fold_up(coordinates, line),
    }
}

pub fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<Fold>) {
    let (_, (dot_coordinates, _, folds)) = full(tuple((
        lines(parse_coordinate),
        tag("\n\n"),
        lines(parse_fold),
    )))(input)
    .unwrap();
    (dot_coordinates, folds)
}

pub fn solve(input: &str) -> usize {
    let (dot_coordinates, folds) = parse_input(input);

    assert!(!folds.is_empty());

    let folded_coordinates = fold_coordinates(&dot_coordinates, folds[0]);

    let coordinate_set: HashSet<(usize, usize)> = folded_coordinates.into_iter().collect();

    coordinate_set.len()
}
