use crate::parsers::{full, unsigned_int, lines};

pub fn solve(input: &str) -> u32 {
    let (_, heights) = full(lines(unsigned_int::<u32>))(input).unwrap();

    let window_sums: Vec<u32> = heights
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();

    let mut increments: u32 = 0;
    let mut last_sum: Option<u32> = None;

    for sum in window_sums {
        match last_sum {
            None => {}
            Some(ls) => {
                if ls < sum {
                    increments += 1;
                }
            }
        }

        last_sum = Some(sum)
    }

    increments
}
