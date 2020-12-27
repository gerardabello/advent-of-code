pub fn solve(input: &str) -> String {
    let mut adapters: Vec<u64> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    adapters.push(0); // plane adapter

    adapters.sort();


    let differences: Vec<u64> = adapters
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    let differences_of_1 = differences.iter().filter(|d| **d == 1).count();
    let differences_of_3 = differences.iter().filter(|d| **d == 3).count() + 1; // The device is always 3 bigger than the biggest adapter

    (differences_of_1 * differences_of_3).to_string()
}
