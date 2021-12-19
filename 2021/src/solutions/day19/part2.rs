use super::parse::parse_input;
use super::part1::find_positions_of_scanners_relative_to_first;

pub fn solve(input: &str) -> usize {
    let scanners = parse_input(input);

    let positioned_scanners = find_positions_of_scanners_relative_to_first(scanners);

    let mut max = 0;

    for s1 in &positioned_scanners {
        for s2 in &positioned_scanners {
            let distance = (s1.position.unwrap().0 - s2.position.unwrap().0).abs()
                + (s1.position.unwrap().1 - s2.position.unwrap().1).abs()
                + (s1.position.unwrap().2 - s2.position.unwrap().2).abs();

            if distance > max {
                max = distance;
            }
        }
    }

    max as usize
}
