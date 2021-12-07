use nom::{bytes::complete::tag, multi::separated_list1};

use crate::parsers::{full, unsigned_int};

fn advance_day(frequencies: &mut [usize;9]) {
    let mut new_frequencies = [0,0,0,0,0,0,0,0,0];

    new_frequencies[7] = frequencies[8];
    new_frequencies[6] = frequencies[7];
    new_frequencies[5] = frequencies[6];
    new_frequencies[4] = frequencies[5];
    new_frequencies[3] = frequencies[4];
    new_frequencies[2] = frequencies[3];
    new_frequencies[1] = frequencies[2];
    new_frequencies[0] = frequencies[1];

    new_frequencies[6] += frequencies[0];
    new_frequencies[8] += frequencies[0];

    *frequencies = new_frequencies;
}

fn initial_state_to_frequencies(state: &[usize]) -> [usize;9] {
    let mut frequencies = [0,0,0,0,0,0,0,0,0];

    for s in state {
        frequencies[*s] += 1;
    }

    frequencies
}


pub fn solve(input: &str) -> usize {
    let (_, initial_state) = full(separated_list1(tag(","), unsigned_int::<usize>))(input).unwrap();

    let mut frequencies = initial_state_to_frequencies(&initial_state);

    for i in 0..256 {
        dbg!(i);
        advance_day(&mut frequencies);
    }

    frequencies.iter().sum()
}
