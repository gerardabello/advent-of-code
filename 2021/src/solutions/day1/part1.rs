use crate::parsers::{full, unsigned_int, lines};

pub fn solve(input: &str) -> u32 {
    let (_, heights) = full(lines(unsigned_int::<u32>))(input).unwrap();

    let mut increments: u32 = 0;
    let mut last_height: Option<u32> = None;
    for height in heights {
        match last_height {
            None => {}
            Some(lh) => {
                if lh < height {
                    increments += 1;
                }
            }
        }

        last_height = Some(height)
    }

    increments
}
