use std::collections::HashMap;

use crate::parsers::{full, lines};
use crate::solutions::day12::part1::{edges_to_hashmap, parse_line};

fn node_is_lowercase(node: &str) -> bool {
    node.chars().all(|c| c.is_lowercase())
}

fn grow_path<'a>(
    hm: &HashMap<&'a str, Vec<&'a str>>,
    base: &[&'a str],
    paths: &mut Vec<Vec<&'a str>>,
    has_visited_small_cave_twice: bool,
) {
    let last = *(base.last().unwrap());
    let candidates = hm.get(last).unwrap();

    for candidate in candidates {
        if *candidate == "start" {
            continue;
        }

        if *candidate == "end" {
            let mut new_path = base.to_vec();
            new_path.push("end");

            paths.push(new_path);
            continue;
        }

        let mut new_has_visited_small_cave_twice = has_visited_small_cave_twice;

        if node_is_lowercase(candidate) && base.contains(candidate) {
            if has_visited_small_cave_twice {
                continue;
            } else {
                new_has_visited_small_cave_twice = true;
            }
        }

        let mut new_base = base.to_vec();
        new_base.push(candidate);

        grow_path(hm, &new_base, paths, new_has_visited_small_cave_twice);
    }
}

fn find_all_paths<'a>(hm: &HashMap<&'a str, Vec<&'a str>>) -> Vec<Vec<&'a str>> {
    let mut paths = vec![];
    grow_path(hm, &["start"], &mut paths, false);
    paths
}

pub fn solve(input: &str) -> usize {
    let (_, edges_list) = full(lines(parse_line))(input).unwrap();

    let hm = edges_to_hashmap(&edges_list);

    find_all_paths(&hm).len()
}
