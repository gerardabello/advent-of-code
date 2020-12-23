use std::collections::LinkedList;

use nom::{
    bytes::complete::take,
    combinator::map_res,
    multi::many1,
    IResult,
};

// Implemented as a struct to avoid Copy, that way we can assure that we don't copy cups by
// mistake.
#[derive(Debug, Clone)]
pub struct Cup {
    pub number: u8,
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Cup>> {
    many1(map_res(take(1 as usize), |s: &str| {
        s.parse::<u8>().map(|number| Cup { number })
    }))(input)
}

struct Game {
    // The current cup is always the first one
    cups: LinkedList<Cup>,
}

impl Game {
    fn find_destination_index(&self, current_cup: &Cup) -> usize {
        let mut possible = current_cup.number - 1;
        loop {
            if let Some(i) = self.cups.iter().position(|c| c.number == possible) {
                return i;
            }

            if possible == 0 {
                possible = 9;
            }else {
                possible -= 1;
            }
        }
    }

    fn cups_ordered_from_one(&self) -> LinkedList<Cup> {
        let mut cups = self.cups.clone();

        let mut from_one = cups.split_off(
            cups.iter().position(|c| c.number == 1).unwrap()
            );

        from_one.append(&mut cups);

        from_one
    }

    fn play_turn(&mut self) {
        let current_cup = self
            .cups
            .pop_front()
            .expect("There should be more than one cup");

        let mut picked: LinkedList<Cup> = {
            let mut tmp = LinkedList::new();

            for _ in 0..3 {
                tmp.push_back(
                    self.cups
                        .pop_front()
                        .expect("There should be 3 cups to pick"),
                );
            }

            tmp
        };

        let destination_index = self.find_destination_index(&current_cup);

        let mut following = self.cups.split_off(destination_index + 1);

        self.cups.append(&mut picked);
        self.cups.append(&mut following);

        // put current cup as last, so following cup is now current
        self.cups.push_back(current_cup);

    }
}

pub fn solve(input: &str) -> String {
    let (_, cups) = parse_input(input).unwrap();

    let mut game = Game {
        cups: cups.into_iter().collect(),
    };

    for _ in 0..100 {
        game.play_turn();
    }

    game.cups_ordered_from_one().into_iter().skip(1).map(|c| c.number.to_string()).collect::<Vec<String>>().join("")
}
