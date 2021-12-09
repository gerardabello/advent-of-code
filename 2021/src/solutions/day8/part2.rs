use std::collections::HashSet;

use crate::parsers::{full, lines};
use crate::solutions::day8::part1::parse_line;

pub fn solve_entry(patterns: Vec<&str>, digits: Vec<&str>) -> usize {
    let mut ordered_patters: [Option<HashSet<char>>; 10] =
        [None, None, None, None, None, None, None, None, None, None];

    let patterns_left: Vec<HashSet<char>> =
        patterns.into_iter().map(|p| p.chars().collect()).collect();

    let mut patterns_left_5 = vec![];
    let mut patterns_left_6 = vec![];

    for pattern in patterns_left {
        match pattern.len() {
            2 => ordered_patters[1] = Some(pattern),
            3 => ordered_patters[7] = Some(pattern),
            4 => ordered_patters[4] = Some(pattern),
            7 => ordered_patters[8] = Some(pattern),
            5 => patterns_left_5.push(pattern),
            6 => patterns_left_6.push(pattern),
            _ => unreachable!(),
        }
    }

    assert!(patterns_left_5.len() == 3);
    assert!(patterns_left_6.len() == 3);

    let top_segment: HashSet<char> = ordered_patters[7]
        .as_ref()
        .unwrap()
        .difference(ordered_patters[1].as_ref().unwrap())
        .copied()
        .collect();

    let all_minus_bottom_left_and_bottom: HashSet<char> = ordered_patters[4]
        .as_ref()
        .unwrap()
        .union(ordered_patters[7].as_ref().unwrap())
        .copied()
        .collect();

    ordered_patters[9] = Some(
        patterns_left_6
            .iter()
            .find(|p| p.difference(&all_minus_bottom_left_and_bottom).count() == 1)
            .unwrap()
            .clone(),
    );

    patterns_left_6 = patterns_left_6
        .into_iter()
        .filter(|p| p != ordered_patters[9].as_ref().unwrap())
        .collect();

    assert!(patterns_left_6.len() == 2);

    let bottom_left_and_bottom: HashSet<char> = ordered_patters[8]
        .as_ref()
        .unwrap()
        .difference(&all_minus_bottom_left_and_bottom)
        .copied()
        .collect();

    let bottom_left: HashSet<char> = ordered_patters[8]
        .as_ref()
        .unwrap()
        .difference(ordered_patters[9].as_ref().unwrap())
        .copied()
        .collect();

    let bottom: HashSet<char> = bottom_left_and_bottom
        .difference(&bottom_left)
        .copied()
        .collect();

    ordered_patters[0] = Some(
        patterns_left_6
            .iter()
            .find(|p| p.intersection(ordered_patters[1].as_ref().unwrap()).count() == 2)
            .unwrap()
            .clone(),
    );

    ordered_patters[6] = Some(
        patterns_left_6
            .iter()
            .find(|p| p.intersection(ordered_patters[1].as_ref().unwrap()).count() == 1)
            .unwrap()
            .clone(),
    );

    std::mem::drop(patterns_left_6);

    let middle: HashSet<char> = ordered_patters[8]
        .as_ref()
        .unwrap()
        .difference(ordered_patters[0].as_ref().unwrap())
        .copied()
        .collect();

    ordered_patters[3] = Some(
        ordered_patters[1]
            .as_ref()
            .unwrap()
            .union(&top_segment)
            .cloned()
            .collect::<HashSet<char>>()
            .union(&middle)
            .cloned()
            .collect::<HashSet<char>>()
            .union(&bottom)
            .cloned()
            .collect::<HashSet<char>>(),
    );

    ordered_patters[5] = Some(
        ordered_patters[6]
            .as_ref()
            .unwrap()
            .difference(&bottom_left)
            .cloned()
            .collect::<HashSet<char>>(),
    );

    ordered_patters[2] = Some(
        patterns_left_5
            .iter()
            .find(|p| p.intersection(&bottom_left).count() == 1)
            .unwrap()
            .clone(),
    );

    let hash_digits = digits.into_iter().map(|p| p.chars().collect());

    let num_digits: Vec<usize> = hash_digits
        .map(|digit| {
            ordered_patters
                .iter()
                .enumerate()
                .find(|(_, pattern)| pattern.as_ref().unwrap().clone() == digit)
                .unwrap()
                .0
        })
        .collect();

    num_digits[3] + num_digits[2] * 10 + num_digits[1] * 100 + num_digits[0] * 1000
}

pub fn solve(input: &str) -> usize {
    let (_, entries) = full(lines(parse_line))(input).unwrap();

    entries.into_iter().map(|e| solve_entry(e.0.clone(), e.1.clone())).sum()
}
