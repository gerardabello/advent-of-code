use std::collections::LinkedList;

use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    IResult,
};

// Implemented as a struct to avoid Copy, that way we can assure that we don't copy cards by
// mistake.
#[derive(Debug, Hash, Clone)]
pub struct Card {
    pub number: u32,
}

type Score = u128;

pub fn parse_deck(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(
        tag("\n"),
        map_res(digit1, |s: &str| {
            s.parse::<u32>().map(|number| Card { number })
        }),
    )(input)
}

pub fn parse_input(input: &str) -> IResult<&str, (Vec<Card>, Vec<Card>)> {
    let (input, _) = tag("Player 1:\n")(input)?;
    let (input, deck1) = parse_deck(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, _) = tag("Player 2:\n")(input)?;
    let (input, deck2) = parse_deck(input)?;

    Ok((input, (deck1, deck2)))
}

struct SpaceCardsGame {
    turn: u128,
    deck1: LinkedList<Card>,
    deck2: LinkedList<Card>,
}

impl SpaceCardsGame {
    fn new(deck1: Vec<Card>, deck2: Vec<Card>) -> Self {
        Self {
            turn: 0,
            deck1: deck1.into_iter().collect(),
            deck2: deck2.into_iter().collect(),
        }
    }

    fn calculate_score(deck: &LinkedList<Card>) -> Score {
        deck.iter()
            .rev()
            .enumerate()
            .map(|(i, card)| (i as u128 + 1) * card.number as u128)
            .sum()
    }

    fn play_turn(&mut self) -> Option<Score> {
        self.turn += 1;

        let deck1_top = self.deck1.pop_front().unwrap();
        let deck2_top = self.deck2.pop_front().unwrap();

        // there are no repeated cards
        assert!(deck1_top.number != deck2_top.number);

        if deck1_top.number > deck2_top.number {
            self.deck1.push_back(deck1_top);
            self.deck1.push_back(deck2_top);
        } else {
            self.deck2.push_back(deck2_top);
            self.deck2.push_back(deck1_top);
        }

        if self.deck1.is_empty() {
            return Some(Self::calculate_score(&self.deck2));
        } else if self.deck2.is_empty() {
            return Some(Self::calculate_score(&self.deck1));
        }

        None
    }
}

pub fn solve(input: &str) -> String {
    let (_, (deck1, deck2)) = parse_input(input).unwrap();

    let mut game = SpaceCardsGame::new(deck1, deck2);

    loop {
        if let Some(score) = game.play_turn() {
            return score.to_string();
        }
    }
}
