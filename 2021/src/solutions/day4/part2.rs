use crate::parsers::full;
use crate::solutions::day4::part1::{board_score, check_board, parse_bingo, Board};

pub fn solve(input: &str) -> usize {
    let (_, (calls, boards)) = full(parse_bingo)(input).unwrap();

    let mut losing_boards = boards;

    for call_idx in 1..calls.len() {
        let tmp_calls = &calls[0..call_idx];
        losing_boards = losing_boards
            .into_iter()
            .filter(|board| !check_board(tmp_calls, board))
            .collect::<Vec<Board>>();
        if losing_boards.len() == 1 {
            let last_board = &losing_boards[0];
            for last_call_idx in call_idx..calls.len() {
                let last_tmp_calls = &calls[0..last_call_idx];
                if check_board(last_tmp_calls, last_board) {
                    return board_score(last_tmp_calls, last_board);
                }
            }
        }
    }

    panic!("There is no ONE losing board");
}
