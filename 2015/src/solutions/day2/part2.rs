use super::{parse_input, Gift};

fn ribbon_for_wrap(gift: &Gift) -> u32 {
    let perimeters = [
        2 * (gift.0 + gift.1),
        2 * (gift.0 + gift.2),
        2 * (gift.1 + gift.2),
    ];

    *perimeters.iter().min().unwrap()
}

fn ribbon_for_bow(gift: &Gift) -> u32 {
    gift.0 * gift.1 * gift.2
}

fn ribbon_needed(gift: &Gift) -> u32 {
    ribbon_for_wrap(gift) + ribbon_for_bow(gift)
}

pub fn solve(input: &str) -> u32 {
    let (_, boxes) = parse_input(input).unwrap();

    boxes.iter().map(ribbon_needed).sum()
}
