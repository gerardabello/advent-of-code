use crate::d9p1::find_first_invalid;

fn find_range_that_sums_invalid(numbers: &[u64], invalid: u64) -> Option<&[u64]> {
    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            let range = &numbers[i..j];
            if range.iter().sum::<u64>() == invalid {
                return Some(range);
            }
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

    let first_invalid = match find_first_invalid(&numbers, preamble) {
        Some(n) => n,
        None => panic!("First invalid number not found"),
    };

    let range = match find_range_that_sums_invalid(&numbers, first_invalid) {
        Some(n) => n,
        None => panic!("Range not found"),
    };

    let min_in_range = range.iter().min().unwrap();
    let max_in_range = range.iter().max().unwrap();

    (min_in_range + max_in_range).to_string()
}
