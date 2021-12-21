use crate::parsers::{full, unsigned_int};
use nom::{bytes::complete::tag, IResult};


pub struct Dice {
    pub times: usize
}

impl Dice {
    pub fn throw(&mut self) -> usize {
        self.times += 1;
        self.times
    }
}

pub fn parse_input_res(input: &str) -> IResult<&str, [usize; 2]> {
    let (input, _) = tag("Player 1 starting position: ")(input)?;
    let (input, p1) = unsigned_int::<usize>(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, _) = tag("Player 2 starting position: ")(input)?;
    let (input, p2) = unsigned_int::<usize>(input)?;
    Ok((input, [p1, p2]))
}

pub fn parse_input(input: &str) -> [usize; 2] {
    full(parse_input_res)(input).unwrap().1
}

pub fn step(position: &mut usize, score: &mut usize,dice: &mut Dice)
{
    let result = dice.throw() + dice.throw() + dice.throw();
    *position = ((*position + result - 1) % 10) + 1;
    *score += *position
}

pub fn solve(input: &str) -> usize {
    let starting_positions = parse_input(input);

    let mut p1_pos = starting_positions[0];
    let mut p2_pos = starting_positions[1];

    let mut p1_score = 0;
    let mut p2_score = 0;

    let mut p1_turn = true;

    let mut dice = Dice{times: 0};

    loop {
        if p1_turn {
            step(&mut p1_pos, &mut p1_score, &mut dice);
            if p1_score >= 1000 {
                return p2_score * dice.times;
            }
        } else {
            step(&mut p2_pos, &mut p2_score, &mut dice);
            if p2_score >= 1000 {
                return p1_score * dice.times;
            }
        }

        p1_turn = !p1_turn;
    }
}
