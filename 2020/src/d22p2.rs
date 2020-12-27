use std::collections::hash_map::DefaultHasher;
use std::collections::LinkedList;
use std::hash::{Hash, Hasher};

use crate::d22p1::{parse_input, Card};

type Score = u128;

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

struct SpaceCardsGame {
    turn: u128,
    deck1: LinkedList<Card>,
    deck2: LinkedList<Card>,
    previous_turn_hashes: Vec<u64>,
}

impl SpaceCardsGame {
    fn new(deck1: Vec<Card>, deck2: Vec<Card>) -> Self {
        Self {
            turn: 0,
            deck1: deck1.into_iter().collect(),
            deck2: deck2.into_iter().collect(),
            previous_turn_hashes: Vec::new(),
        }
    }

    fn calculate_score(deck: &LinkedList<Card>) -> Score {
        deck.iter()
            .rev()
            .enumerate()
            .map(|(i, card)| (i as u128 + 1) * card.number as u128)
            .sum()
    }

    fn calculate_turn_hash(&self) -> u64 {
        calculate_hash(&(&self.deck1, &self.deck2))
    }

    fn play(&mut self, nesting: u64) -> (bool, Score) {
        loop {
            if let Some(end) = self.play_turn(nesting) {
                return end;
            }
        }
    }

    fn play_turn(&mut self, nesting: u64) -> Option<(bool, Score)> {
        self.turn += 1;
        let turn_hash = self.calculate_turn_hash();

        if self.previous_turn_hashes.contains(&turn_hash) {
            return Some((true, SpaceCardsGame::calculate_score(&self.deck1)));
        }

        let deck1_top = self.deck1.pop_front().unwrap();
        let deck2_top = self.deck2.pop_front().unwrap();

        // there are no repeated cards
        assert!(deck1_top.number != deck2_top.number);

        let can_play_subgame = self.deck1.len() >= deck1_top.number as usize
            && self.deck2.len() >= deck2_top.number as usize;

        let player_1_winner = match can_play_subgame {
            true => {
                let mut subgame = Self::new(
                    self.deck1
                        .clone()
                        .into_iter()
                        .take(deck1_top.number as usize)
                        .collect(),
                    self.deck2
                        .clone()
                        .into_iter()
                        .take(deck2_top.number as usize)
                        .collect(),
                );

                subgame.play(nesting + 1).0
            }
            false => deck1_top.number > deck2_top.number,
        };

        if player_1_winner {
            self.deck1.push_back(deck1_top);
            self.deck1.push_back(deck2_top);
        } else {
            self.deck2.push_back(deck2_top);
            self.deck2.push_back(deck1_top);
        }

        if self.deck1.is_empty() {
            return Some((false, Self::calculate_score(&self.deck2)));
        } else if self.deck2.is_empty() {
            return Some((true, Self::calculate_score(&self.deck1)));
        }

        self.previous_turn_hashes.push(turn_hash);

        None
    }
}

pub fn solve(input: &str) -> String {
    let (_, (deck1, deck2)) = parse_input(input).unwrap();

    let mut game = SpaceCardsGame::new(deck1.into_iter().collect(), deck2.into_iter().collect());

    game.play(0).1.to_string()
}
