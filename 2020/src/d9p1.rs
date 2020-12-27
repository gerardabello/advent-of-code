fn is_sum_of_two_previous(num: u64, previous: &[u64]) -> bool {
    for p1 in previous {
        for p2 in previous {
            if p1 + p2 == num && p1 != p2 {
                return true;
            }
        }
    }

    false
}

pub fn find_first_invalid(numbers: &[u64], preamble: usize) -> Option<u64> {
    for i in preamble..numbers.len() {
        let previous = &numbers[(i - preamble)..i];
        let num = numbers[i];

        if !is_sum_of_two_previous(num, previous) {
            return Some(num);
        }
    }

    None
}

pub fn solve(input: &str) -> String {
    let preamble = 25;

    let numbers: Vec<u64> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    match find_first_invalid(&numbers, preamble) {
        Some(n) => n.to_string(),
        None => String::from("Not found"),
    }
}
