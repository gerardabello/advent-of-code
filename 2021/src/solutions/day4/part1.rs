use nom::{
    bytes::complete::tag,
    multi::{many0, many1, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};

use crate::matrix::transposed_iter;
use crate::parsers::{full, lines, unsigned_int};

pub type Board = Vec<Vec<usize>>;
pub type BoardSlice<'a> = &'a[Vec<usize>];

fn parse_calls(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(","), unsigned_int::<usize>)(input)
}

fn parse_board(input: &str) -> IResult<&str, Board> {
    lines(preceded(
        many0(tag(" ")),
        separated_list1(many1(tag(" ")), unsigned_int::<usize>),
    ))(input)
}

fn parse_boards(input: &str) -> IResult<&str, Vec<Board>> {
    separated_list1(tag("\n\n"), parse_board)(input)
}

pub fn parse_bingo(input: &str) -> IResult<&str, (Vec<usize>, Vec<Board>)> {
    let (input, (calls, _, boards)) = tuple((parse_calls, tag("\n\n"), parse_boards))(input)?;
    Ok((input, (calls, boards)))
}

pub fn board_score(calls: &[usize], board: BoardSlice) -> usize {
    let mut sum_of_unmarked: usize = 0;
    for row in board {
        sum_of_unmarked += row.iter().filter(|v| !calls.contains(v)).sum::<usize>();
    }

    let last_call = calls[calls.len() - 1];


    sum_of_unmarked * last_call
}

pub fn check_board(calls: &[usize], board: BoardSlice) -> bool {
    for row in board {
        if row.iter().all(|v| calls.contains(v)) {
            return true;
        }
    }

    for mut row_iter in transposed_iter(board) {
        if row_iter.all(|v| calls.contains(v)) {
            return true;
        }
    }

    false
}

pub fn solve(input: &str) -> usize {
    let (_, (calls, boards)) = full(parse_bingo)(input).unwrap();

    for call_idx in 1..calls.len() {
        let tmp_calls = &calls[0..call_idx];
        for board in &boards {
            if check_board(tmp_calls, board) {
                return board_score(tmp_calls, board);
            }
        }
    }

    panic!("Not winner found");
}
