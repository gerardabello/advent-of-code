use super::part1::parse_input;
use std::collections::HashMap;

type Score = usize;
type Position = usize;

const THROWS: [usize; 27] = [
    1 + 1 + 1,
    1 + 1 + 2,
    1 + 1 + 3,
    1 + 2 + 1,
    1 + 2 + 2,
    1 + 2 + 3,
    1 + 3 + 1,
    1 + 3 + 2,
    1 + 3 + 3,
    2 + 1 + 1,
    2 + 1 + 2,
    2 + 1 + 3,
    2 + 2 + 1,
    2 + 2 + 2,
    2 + 2 + 3,
    2 + 3 + 1,
    2 + 3 + 2,
    2 + 3 + 3,
    3 + 1 + 1,
    3 + 1 + 2,
    3 + 1 + 3,
    3 + 2 + 1,
    3 + 2 + 2,
    3 + 2 + 3,
    3 + 3 + 1,
    3 + 3 + 2,
    3 + 3 + 3,
];

fn new_position(position: Position, dice_result: usize) -> Position {
    ((position + dice_result - 1) % 10) + 1
}
fn new_score(score: Score, new_position: Position) -> Score {
    score + new_position
}

fn how_many_times_each_wins(
    cache: &mut HashMap<([Position; 2], [Score; 2], bool), [u128; 2]>,
    positions: &[Position; 2],
    scores: &[Score; 2],
    p1_turn: bool,
) -> [u128; 2] {
    match cache.get(&(*positions, *scores, p1_turn)) {
        Some(winners) => *winners,
        None => {
            let mut winners: [u128; 2] = [0, 0];
            let player_index = if p1_turn { 0 } else { 1 };
            for throw in THROWS {
                let new_pos = new_position(positions[player_index], throw);
                let new_scr = new_score(scores[player_index], new_pos);

                if new_scr >= 21 {
                    winners[player_index] += 1;
                } else {
                    let universe_winners = if p1_turn {
                        how_many_times_each_wins(
                            cache,
                            &[new_pos, positions[1]],
                            &[new_scr, scores[1]],
                            false,
                        )
                    } else {
                        how_many_times_each_wins(
                            cache,
                            &[positions[0], new_pos],
                            &[scores[0], new_scr],
                            true,
                        )
                    };

                    winners[0] += universe_winners[0];
                    winners[1] += universe_winners[1];
                }
            }

            cache.insert((*positions, *scores, p1_turn), winners);
            winners
        }
    }
}

pub fn solve(input: &str) -> u128 {
    let starting_positions = parse_input(input);

    let mut cache = HashMap::new();

    how_many_times_each_wins(&mut cache, &starting_positions, &[0, 0], true)
        .into_iter()
        .max()
        .unwrap()
}
