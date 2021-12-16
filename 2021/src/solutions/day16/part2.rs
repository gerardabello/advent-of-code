use crate::solutions::day16::part1::{hex_to_binary, parse_input, Packet};

pub fn eval_packet(packet: &Packet) -> usize {
    match packet {
        Packet::LiteralValue { value, .. } => *value,
        Packet::Operator { header, children } => match header.type_id {
            0 => children.iter().map(eval_packet).sum(),
            1 => children.iter().map(eval_packet).product(),
            2 => children.iter().map(eval_packet).min().unwrap(),
            3 => children.iter().map(eval_packet).max().unwrap(),
            5 => {
                assert!(children.len() == 2);

                if eval_packet(&children[0]) > eval_packet(&children[1]) {
                    1
                } else {
                    0
                }
            }

            6 => {
                assert!(children.len() == 2);

                if eval_packet(&children[0]) < eval_packet(&children[1]) {
                    1
                } else {
                    0
                }
            }

            7 => {
                assert!(children.len() == 2);

                if eval_packet(&children[0]) == eval_packet(&children[1]) {
                    1
                } else {
                    0
                }
            }

            _ => unreachable!("Unexpected type_id '{}'", header.type_id),
        },
    }
}

pub fn solve(input: &str) -> usize {
    let packet = parse_input(&hex_to_binary(input));

    eval_packet(&packet)
}
