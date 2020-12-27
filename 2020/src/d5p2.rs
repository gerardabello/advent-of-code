use crate::d5p1::BoardingPass;

pub fn solve(input: &str) -> String {
    let mut ids = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| BoardingPass::parse(s))
        .map(|bp| bp.id())
        .collect::<Vec<u32>>();

    ids.sort();

    let first = ids[0];

    ids.iter()
        .enumerate()
        .position(|(i, id)| i + (first as usize) != (*id as usize))
        .map(|i| i + first as usize)
        .unwrap()
        .to_string()
}
