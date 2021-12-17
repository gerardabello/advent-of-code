use super::part1::{parse_input, find_max_height};

pub fn solve(input: &str) -> usize {
    let target_area = parse_input(input);

    let mut count = 0;
    for vx in 0..1000 {
        for vy in -1000..1000 {
            let mx = find_max_height((vx, vy), target_area);
            if mx.is_some() {
                count += 1;
            }
        }
    }

    count
}
