use super::{Gift, parse_input};

fn wrapping_paper_needed(gift: &Gift) -> u32 {
    let sides = [gift.0 * gift.1, gift.0 * gift.2, gift.1 * gift.2];

    let smallest = *sides.iter().min().unwrap();

    sides.iter().sum::<u32>() * 2 + smallest
}

pub fn solve(input: &str) -> u32 {
    let (_, boxes) = parse_input(input).unwrap();

    boxes.iter().map(wrapping_paper_needed).sum()
}
