use std::collections::HashMap;
use std::ops::Range;

use super::part1::{parse_input, Cuboid, Step};

pub fn cuboid_area(cuboid: &Cuboid) -> u128 {
    (cuboid[0].start - cuboid[0].end).abs() as u128
        * (cuboid[1].start - cuboid[1].end).abs() as u128
        * (cuboid[2].start - cuboid[2].end).abs() as u128
}

pub fn cuboid_interesection(c1: &Cuboid, c2: &Cuboid) -> Option<Cuboid> {
    let range_x = Range {
        start: isize::max(c1[0].start, c2[0].start),
        end: isize::min(c1[0].end, c2[0].end),
    };

    if range_x.start >= range_x.end {
        return None;
    };

    let range_y = Range {
        start: isize::max(c1[1].start, c2[1].start),
        end: isize::min(c1[1].end, c2[1].end),
    };

    if range_y.start >= range_y.end {
        return None;
    };

    let range_z = Range {
        start: isize::max(c1[2].start, c2[2].start),
        end: isize::min(c1[2].end, c2[2].end),
    };

    if range_z.start >= range_z.end {
        return None;
    };

    Some([range_x, range_y, range_z])
}

pub fn get_cuboid(step: &Step) -> Cuboid {
    match step {
        Step::On(cuboid) => cuboid.clone(),
        Step::Off(cuboid) => cuboid.clone(),
    }
}

pub fn solve(input: &str) -> u128 {
    let steps = parse_input(input);

    let mut cuboid_with_weights  : HashMap<Cuboid, i128> = HashMap::new();

    for step in steps {
        let mut new_cuboid_with_weights = cuboid_with_weights.clone();
        let cuboid = get_cuboid(&step);
        for (c2, weight) in cuboid_with_weights {
            if let Some(intersection) = cuboid_interesection(&c2, &cuboid) {
                *new_cuboid_with_weights.entry(intersection).or_insert(0) -= weight;
            }
        }

        if matches!(step, Step::On(_)) {
            *new_cuboid_with_weights.entry(cuboid).or_insert(0) += 1;
        }
        cuboid_with_weights = new_cuboid_with_weights;
    }

    cuboid_with_weights
        .into_iter()
        .map(|(cuboid, weight)| cuboid_area(&cuboid) as i128 * weight)
        .sum::<i128>() as u128
}
