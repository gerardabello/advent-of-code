use super::parse::{parse_input, Position, Scanner};
use std::collections::HashSet;

use crate::point::{rotate_3d_90x_xyz, ALL_3D_90X_ROTATIONS};

fn find_position_rotation(scanner: &Scanner, positioned_scanners: &[Scanner]) -> Option<Scanner> {
    for ps in positioned_scanners {
        let beacon_set_ps: HashSet<Position> = ps
            .beacons
            .iter()
            .map(|b| rotate_3d_90x_xyz(b, &ps.rotation.unwrap()))
            .map(|b| translate_position(&b, &ps.position.unwrap()))
            .collect();

        for rotation in ALL_3D_90X_ROTATIONS {
            let beacons: Vec<Position> = scanner
                .beacons
                .iter()
                .map(|b| rotate_3d_90x_xyz(b, &rotation))
                .collect();
            for bs in &beacons {
                for bps in &beacon_set_ps {
                    let tx = bps[0] - bs[0];
                    let ty = bps[1] - bs[1];
                    let tz = bps[2] - bs[2];

                    let translation = [tx, ty, tz];

                    let beacon_set_s: HashSet<Position> = beacons
                        .iter()
                        .map(|b| translate_position(b, &translation))
                        .collect();

                    let intersection_count = beacon_set_s.intersection(&beacon_set_ps).count();
                    assert!(intersection_count >= 1);

                    if intersection_count >= 12 {
                        let mut scanner = scanner.clone();
                        scanner.rotation = Some(rotation);
                        scanner.position = Some(translation);
                        return Some(scanner);
                    }
                }
            }
        }
    }

    None
}

fn translate_position(position: &Position, translation: &Position) -> Position {
    [
        position[0] + translation[0],
        position[1] + translation[1],
        position[2] + translation[2],
    ]
}

pub fn find_positions_of_scanners_relative_to_first(mut scanners: Vec<Scanner>) -> Vec<Scanner> {
    let mut positioned_scanners = vec![];

    let mut scanner0 = scanners.remove(0);
    scanner0.position = Some([0, 0, 0]);
    scanner0.rotation = Some([0, 0, 0]);
    positioned_scanners.push(scanner0);

    let mut scaners_left = scanners.len();
    loop {
        scanners = scanners
            .into_iter()
            .filter(|scanner| {
                let position_result = find_position_rotation(scanner, &positioned_scanners);
                if let Some(s) = position_result {
                    positioned_scanners.push(s);
                    return false;
                }
                true
            })
            .collect();

        if scaners_left == scanners.len() {
            panic!("not progressing at {}", scaners_left);
        } else {
            scaners_left = scanners.len();
            println!("{} scanners left", scaners_left);
        }

        if scaners_left == 0 {
            break;
        }
    }

    positioned_scanners
}

pub fn solve(input: &str) -> usize {
    let scanners = parse_input(input);

    let positioned_scanners = find_positions_of_scanners_relative_to_first(scanners);

    let all_positions: HashSet<Position> = positioned_scanners
        .into_iter()
        .flat_map(|s| {
            s.beacons
                .iter()
                .map(|b| rotate_3d_90x_xyz(b, &s.rotation.unwrap()))
                .map(|b| translate_position(&b, &s.position.unwrap()))
                .collect::<Vec<Position>>()
        })
        .collect();

    all_positions.len()
}
