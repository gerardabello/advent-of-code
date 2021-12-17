use nom::{
    bytes::complete::tag,
    sequence::{preceded, separated_pair},
};

use crate::parsers::{full, signed_int};

pub fn parse_input(input: &str) -> ((isize, isize), (isize, isize)) {
    full(preceded(
        tag("target area: "),
        separated_pair(
            preceded(
                tag("x="),
                separated_pair(signed_int::<isize>, tag(".."), signed_int::<isize>),
            ),
            tag(", "),
            preceded(
                tag("y="),
                separated_pair(signed_int::<isize>, tag(".."), signed_int::<isize>),
            ),
        ),
    ))(input)
    .unwrap()
    .1
}

pub fn update_probe(
    position: (isize, isize),
    velocity: (isize, isize),
) -> ((isize, isize), (isize, isize)) {
    (
        (position.0 + velocity.0, position.1 + velocity.1),
        (
            if velocity.0 > 0 {
                velocity.0 - 1
            } else if velocity.0 < 0 {
                velocity.0 + 1
            } else {
                0
            },
            velocity.1 - 1,
        ),
    )
}

pub fn is_inside_target(
    position: (isize, isize),
    target_area: ((isize, isize), (isize, isize)),
) -> bool {
    position.0 >= target_area.0 .0
        && position.0 <= target_area.0 .1
        && position.1 >= target_area.1 .0
        && position.1 <= target_area.1 .1
}

pub fn is_past_target(
    position: (isize, isize),
    target_area: ((isize, isize), (isize, isize)),
) -> bool {
    position.0 > target_area.0 .1
        ||
    position.1 < target_area.1 .0
}

pub fn find_max_height(
    velocity: (isize, isize),
    target_area: ((isize, isize), (isize, isize)),
) -> Option<isize> {
    let mut vel = velocity;
    let mut pos = (0, 0);
    let mut max_height = 0;


    loop {
        let (new_pos, new_vel) = update_probe(pos, vel);
        pos = new_pos;
        vel = new_vel;

        if pos.1 > max_height {
            max_height = pos.1;
        }

        if is_inside_target(pos, target_area) {
            return Some(max_height);
        }

        if is_past_target(pos, target_area) {
            return None;
        }

    }
}

pub fn solve(input: &str) -> usize {
    let target_area = parse_input(input);

    let mut max_height = 0;
    for vx in 0..1000 {
        for vy in -1000..1000 {
            let mx = find_max_height((vx, vy), target_area);
            if let Some(h) = mx {
                if h > max_height {
                    max_height = h;
                }
            }
        }
    }

    max_height as usize
}
