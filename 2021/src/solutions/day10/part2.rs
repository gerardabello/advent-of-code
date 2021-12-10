use crate::parsers::{full, lines};
use crate::solutions::day10::part1::{is_open, matching_close, parse_line};

fn complete_chunk(input: &[char]) -> Option<String> {
    let mut state = vec![];

    for c in input {
        if is_open(*c) {
            state.push(c);
        } else {
            let last_state = state.last();
            if last_state.is_some() && *c == matching_close(**last_state.unwrap()) {
                state.pop();
            } else {
                return None;
            }
        }
    }

    Some(state.iter().rev().map(|c| matching_close(**c)).collect())
}

pub fn solve(input: &str) -> usize {
    let (_, lines) = full(lines(parse_line))(input).unwrap();

    let mut scores: Vec<usize> = lines
        .iter()
        .map(|line| complete_chunk(line))
        .flatten()
        .map(|completion| {
            completion
                .chars()
                .map(|c| match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => unreachable!(),
                })
                .fold(0, |acc, score| acc * 5 + score)
        })
        .collect();

    scores.sort_unstable();
    scores[scores.len() / 2]
}
