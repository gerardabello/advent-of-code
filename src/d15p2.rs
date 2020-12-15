use crate::d15p1::{parse_input, play_game_until_turn};

pub fn solve(input: &str) -> String {
    let (_, numbers) = parse_input(input).unwrap();

    play_game_until_turn(&numbers, 30_000_000).to_string()
}
