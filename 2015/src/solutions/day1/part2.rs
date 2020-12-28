pub fn solve(input: &str) -> usize {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        floor = match c {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => floor,
        };

        if floor == -1 {
            return i + 1;
        }
    }

    unreachable!();
}
