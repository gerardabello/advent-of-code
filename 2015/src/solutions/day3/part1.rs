use std::collections::HashSet;

use super::{move_direction, parse_input};

pub fn solve(input: &str) -> usize {
    let (_, dirs) = parse_input(input).unwrap();
    let mut set: HashSet<(i32, i32)> = HashSet::new();

    let mut pos = (0, 0);
    for dir in dirs {
        set.insert(pos);
        pos = move_direction(pos, &dir);
    }

    set.len()
}
