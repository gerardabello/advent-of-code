use nom::{
    bytes::complete::tag, character::complete::alpha1, combinator::recognize,
    sequence::separated_pair, IResult,
};
use std::collections::HashMap;

use crate::parsers::{full, lines};

pub fn parse_line(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(recognize(alpha1), tag("-"), recognize(alpha1))(input)
}

pub fn add_value_to_hashmap<'a>(
    hm: &mut HashMap<&'a str, Vec<&'a str>>,
    key: &'a str,
    value: &'a str,
) {
    match hm.get_mut(key) {
        Some(arr) => arr.push(value),
        None => {
            hm.insert(key, vec![value]);
        }
    };
}

pub fn edges_to_hashmap<'a>(edges: &[(&'a str, &'a str)]) -> HashMap<&'a str, Vec<&'a str>> {
    let mut hm: HashMap<&str, Vec<&str>> = HashMap::new();
    for (start, end) in edges {
        add_value_to_hashmap(&mut hm, start, end);
        add_value_to_hashmap(&mut hm, end, start);
    }

    hm
}

fn grow_path<'a>(hm: &HashMap<&'a str, Vec<&'a str>>, base: &[&'a str]) -> Vec<Vec<&'a str>> {
    let last = *(base.last().unwrap());
    let candidates = hm.get(last).unwrap();

    let mut paths = vec![];

    for candidate in candidates {
        if *candidate == "end" {
            let mut new_path = base.to_vec();
            new_path.push("end");

            paths.append(&mut vec![new_path]);
            continue
        }

        if candidate.chars().all(|c| c.is_lowercase()) && base.contains(candidate) {
            continue;
        }

        let mut new_base = base.to_vec();
        new_base.push(candidate);

        paths.append(&mut grow_path(hm, &new_base));
    }

    paths
}

fn find_all_paths<'a>(hm: &HashMap<&'a str, Vec<&'a str>>) -> Vec<Vec<&'a str>> {
    grow_path(hm, &["start"])
}

pub fn solve(input: &str) -> usize {
    let (_, edges_list) = full(lines(parse_line))(input).unwrap();

    let hm = edges_to_hashmap(&edges_list);

    find_all_paths(&hm).len()
}
