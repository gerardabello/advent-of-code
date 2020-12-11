use std::env;
use std::time::Instant;

#[macro_use]
extern crate lazy_static;

extern crate nom;
extern crate regex;

mod utils;

mod d1p1;
mod d1p2;

mod d2p1;
mod d2p2;

mod d3p1;
mod d3p2;

mod d4p1;
mod d4p2;

mod d5p1;
mod d5p2;

mod d6p1;
mod d6p2;

mod d7p1;
mod d7p2;

mod d8p1;
mod d8p2;

mod d9p1;
mod d9p2;

mod d10p1;
mod d10p2;

mod d11p1;
mod d11p2;

mod d12p1;
mod d12p2;

mod d13p1;
mod d13p2;

mod d14p1;
mod d14p2;

mod d15p1;
mod d15p2;

mod d16p1;
mod d16p2;

mod d17p1;
mod d17p2;

mod d18p1;
mod d18p2;

mod d19p1;
mod d19p2;

mod d20p1;
mod d20p2;

mod d21p1;
mod d21p2;

mod d22p1;
mod d22p2;

mod d23p1;
mod d23p2;

mod d24p1;
mod d24p2;

mod d25p1;
mod d25p2;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = args[1].parse::<u8>().unwrap();
    let puzzle = args[2].parse::<u8>().unwrap();

    let input = utils::get_input(&format!("day{}", day));

    let now = Instant::now();

    let solution = match (day, puzzle) {
        (1, 1) => d1p1::solve(&input),
        (1, 2) => d1p2::solve(&input),

        (2, 1) => d2p1::solve(&input),
        (2, 2) => d2p2::solve(&input),

        (3, 1) => d3p1::solve(&input),
        (3, 2) => d3p2::solve(&input),

        (4, 1) => d4p1::solve(&input),
        (4, 2) => d4p2::solve(&input),

        (5, 1) => d5p1::solve(&input),
        (5, 2) => d5p2::solve(&input),

        (6, 1) => d6p1::solve(&input),
        (6, 2) => d6p2::solve(&input),

        (7, 1) => d7p1::solve(&input),
        (7, 2) => d7p2::solve(&input),

        (8, 1) => d8p1::solve(&input),
        (8, 2) => d8p2::solve(&input),

        (9, 1) => d9p1::solve(&input),
        (9, 2) => d9p2::solve(&input),

        (10, 1) => d10p1::solve(&input),
        (10, 2) => d10p2::solve(&input),

        (11, 1) => d11p1::solve(&input),
        (11, 2) => d11p2::solve(&input),

        (12, 1) => d12p1::solve(&input),
        (12, 2) => d12p2::solve(&input),

        (13, 1) => d13p1::solve(&input),
        (13, 2) => d13p2::solve(&input),

        (14, 1) => d14p1::solve(&input),
        (14, 2) => d14p2::solve(&input),

        (15, 1) => d15p1::solve(&input),
        (15, 2) => d15p2::solve(&input),

        (16, 1) => d16p1::solve(&input),
        (16, 2) => d16p2::solve(&input),

        (17, 1) => d17p1::solve(&input),
        (17, 2) => d17p2::solve(&input),

        (18, 1) => d18p1::solve(&input),
        (18, 2) => d18p2::solve(&input),

        (19, 1) => d19p1::solve(&input),
        (19, 2) => d19p2::solve(&input),

        (20, 1) => d20p1::solve(&input),
        (20, 2) => d20p2::solve(&input),

        (21, 1) => d21p1::solve(&input),
        (21, 2) => d21p2::solve(&input),

        (22, 1) => d22p1::solve(&input),
        (22, 2) => d22p2::solve(&input),

        (23, 1) => d23p1::solve(&input),
        (23, 2) => d23p2::solve(&input),

        (24, 1) => d24p1::solve(&input),
        (24, 2) => d24p2::solve(&input),

        (25, 1) => d25p1::solve(&input),
        (25, 2) => d25p2::solve(&input),

        _ => panic!("No solver found"),
    };

    let millis = now.elapsed().as_millis();
    let micros = now.elapsed().as_micros() - millis * 1000;
    let nanos = now.elapsed().as_nanos() - millis * 1_000_000 - micros * 1000;

    println!("time: {}ms {}μs {}ns", millis, micros, nanos);

    println!("Solution to day {} problem {}:", day, puzzle);
    println!("{}", solution);
}
