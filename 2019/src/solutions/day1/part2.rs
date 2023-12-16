use super::parse_input;

fn module_fuel(mass: u32) -> u32 {
    if mass < 9 {
        return 0;
    }

    let fuel_mass = mass / 3 - 2;
    fuel_mass + module_fuel(fuel_mass)
}

pub fn solve(input: &str) -> u32 {
    let input = parse_input(input);

    input.into_iter().map(|mass| module_fuel(mass)).sum()
}
