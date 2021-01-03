use std::collections::HashSet;
use std::iter::Iterator;

use super::{move_direction, parse_input, Direction};

fn get_visited_set<'a, T: Iterator<Item = &'a Direction>>(dirs: T) -> HashSet<(i32, i32)> {
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    let mut pos = (0, 0);
    for dir in dirs {
        set.insert(pos);
        pos = move_direction(pos, &dir);
    }

    set
}

pub fn solve(input: &str) -> usize {
    let (_, dirs) = parse_input(input).unwrap();

    let santa_dirs = dirs.iter().step_by(2);
    let robo_dirs = dirs.iter().skip(1).step_by(2);

    let mut santa_houses = get_visited_set(santa_dirs);
    let robo_houses = get_visited_set(robo_dirs);

    santa_houses.extend(robo_houses);
    santa_houses.len()
}
