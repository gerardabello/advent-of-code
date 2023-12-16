use crate::intcode::run_intcode;

use super::parse_input;

pub fn solve(input: &str) -> usize {
    let input = parse_input(input);

    for verb in 1..1000 {
        for noun in 1..1000 {
            let result = run_intcode(&input, noun, verb);
            match result {
                Ok(ret) => {
                    if ret == 19690720 {
                        return 100 * noun + verb;
                    }
                }
                Err(_) => {}
            }
        }
    }

    panic!("Not found")
}
