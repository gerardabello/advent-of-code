pub fn solve(input: &str) -> String {
    let numbers: Vec<i32> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    for n1 in &numbers {
        for n2 in &numbers {
            for n3 in &numbers {
                if n1 + n2 + n3 == 2020 {
                    let solution = n1 * n2 * n3;
                    return solution.to_string();
                }
            }
        }
    }

    panic!("No solution found")
}
