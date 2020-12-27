use std::collections::HashMap;

use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    IResult,
};

pub type Number = u32;
pub type Turn = u64;

struct GameState {
    number_last_said: HashMap<Number, Turn>,
    last_number_said: Number,
    turn: Turn,
}

impl GameState {
    fn new() -> Self {
        Self {
            last_number_said: 0, // not really true, but I'm too lazy to do Option
            turn: 1,

            // it does not include the last number, because we need the previous ocurrences.
            number_last_said: HashMap::new(),
        }
    }

    fn store_turn(&mut self, number: Number) {
        // we store the previous number
        if self.turn > 1 {
            self.number_last_said
                .insert(self.last_number_said, self.turn - 1);
        }

        self.last_number_said = number;
        self.turn += 1;
    }

    fn previous_turn_of_last_number(&self) -> Option<&Turn> {
        self.number_last_said.get(&self.last_number_said)
    }
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Number>> {
    let parse_number = map_res(digit1, |s: &str| s.parse::<Number>());
    separated_list1(tag(","), parse_number)(input)
}

pub fn play_game_until_turn(numbers: &[Number], turn: Turn) -> Number {
    let mut game = GameState::new();

    // First they say the numbers in order of input
    for n in numbers {
        game.store_turn(*n);
    }

    // Then they say 0 or age
    loop {
        let number_to_say = match game.previous_turn_of_last_number() {
            Some(n) => game.turn - 1 - n,
            None => 0,
        } as u32;

        if game.turn == turn {
            return number_to_say;
        }

        game.store_turn(number_to_say);
    }
}

pub fn solve(input: &str) -> String {
    let (_, numbers) = parse_input(input).unwrap();

    play_game_until_turn(&numbers, 2020).to_string()
}
