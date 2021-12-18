use super::part1::{magnitude, parse_input, reduce, sum};

pub fn solve(input: &str) -> usize {
    let numbers = parse_input(input);

    let mut max: usize = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            for (n1, n2) in [(&numbers[i], &numbers[j]), (&numbers[j], &numbers[i])] {
                let mut result = sum(n1, n2);
                reduce(&mut result);
                let m = magnitude(&result);
                if m > max {
                    max = m;
                }
            }
        }
    }

    max
}
