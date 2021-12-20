use super::part1::{enhance_times, parse_input};

pub fn solve(input: &str) -> usize {
    let (filter, map) = parse_input(input);

    let enhanced_map = enhance_times(&filter, &map, 50);

    enhanced_map
        .iter()
        .map(|row| row.iter())
        .flatten()
        .filter(|b| **b)
        .count()
}
