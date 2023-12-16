use super::parse_input;

fn module_fuel(mass: u32) -> u32 {
    mass / 3 - 2
}

pub fn solve(input: &str) -> u32 {
    let input = parse_input(input);

    input.into_iter().map(|mass| module_fuel(mass)).sum()
}
