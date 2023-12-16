use std::env;
use std::fs;
use std::time::Instant;

extern crate nom;

mod intcode;
mod parser;
mod solutions;

pub fn get_input(day: usize, is_example: bool) -> String {
    let file = match is_example {
        true => format!("day{}example", day),
        false => format!("day{}", day),
    };

    match fs::read_to_string(format!("./inputs/{}", file)) {
        Ok(s) => s,
        Err(_) => panic!("Could not get input for day {}", day),
    }
}

fn report_time_since(since: Instant) {
    let millis = since.elapsed().as_millis();
    let micros = since.elapsed().as_micros() - millis * 1000;
    let nanos = since.elapsed().as_nanos() - millis * 1_000_000 - micros * 1000;

    println!("time: {}ms {}μs {}ns", millis, micros, nanos);
}

fn get_arguments() -> (usize, usize, bool) {
    let args: Vec<String> = env::args().collect();

    let day = args[1].parse::<usize>().unwrap();
    let part = args[2].parse::<usize>().unwrap();
    let is_example = args.len() > 3 && args[3] == "example";

    (day, part, is_example)
}

fn main() {
    let (day, part, is_example) = get_arguments();

    let input = get_input(day, is_example);

    let start = Instant::now();

    let solution = solve(day, part, &input);

    report_time_since(start);

    println!("Solution to day {} problem {}:", day, part);
    println!("{}", solution);
}

fn solve(day: usize, part: usize, input: &str) -> Box<dyn std::fmt::Display> {
    match (day, part) {
        (1, 1) => Box::new(solutions::day1::part1(input)),
        (1, 2) => Box::new(solutions::day1::part2(input)),

        (2, 1) => Box::new(solutions::day2::part1(input)),
        (2, 2) => Box::new(solutions::day2::part2(input)),

        (3, 1) => Box::new(solutions::day3::part1(input)),
        (3, 2) => Box::new(solutions::day3::part2(input)),

        (4, 1) => Box::new(solutions::day4::part1(input)),
        (4, 2) => Box::new(solutions::day4::part2(input)),

        (5, 1) => Box::new(solutions::day5::part1(input)),
        (5, 2) => Box::new(solutions::day5::part2(input)),

        (6, 1) => Box::new(solutions::day6::part1(input)),
        (6, 2) => Box::new(solutions::day6::part2(input)),

        (7, 1) => Box::new(solutions::day7::part1(input)),
        (7, 2) => Box::new(solutions::day7::part2(input)),

        (8, 1) => Box::new(solutions::day8::part1(input)),
        (8, 2) => Box::new(solutions::day8::part2(input)),

        (9, 1) => Box::new(solutions::day9::part1(input)),
        (9, 2) => Box::new(solutions::day9::part2(input)),

        (10, 1) => Box::new(solutions::day10::part1(input)),
        (10, 2) => Box::new(solutions::day10::part2(input)),

        (11, 1) => Box::new(solutions::day11::part1(input)),
        (11, 2) => Box::new(solutions::day11::part2(input)),

        (12, 1) => Box::new(solutions::day12::part1(input)),
        (12, 2) => Box::new(solutions::day12::part2(input)),

        (13, 1) => Box::new(solutions::day13::part1(input)),
        (13, 2) => Box::new(solutions::day13::part2(input)),

        (14, 1) => Box::new(solutions::day14::part1(input)),
        (14, 2) => Box::new(solutions::day14::part2(input)),

        (15, 1) => Box::new(solutions::day15::part1(input)),
        (15, 2) => Box::new(solutions::day15::part2(input)),

        (16, 1) => Box::new(solutions::day16::part1(input)),
        (16, 2) => Box::new(solutions::day16::part2(input)),

        (17, 1) => Box::new(solutions::day17::part1(input)),
        (17, 2) => Box::new(solutions::day17::part2(input)),

        (18, 1) => Box::new(solutions::day18::part1(input)),
        (18, 2) => Box::new(solutions::day18::part2(input)),

        (19, 1) => Box::new(solutions::day19::part1(input)),
        (19, 2) => Box::new(solutions::day19::part2(input)),

        (20, 1) => Box::new(solutions::day20::part1(input)),
        (20, 2) => Box::new(solutions::day20::part2(input)),

        (21, 1) => Box::new(solutions::day21::part1(input)),
        (21, 2) => Box::new(solutions::day21::part2(input)),

        (22, 1) => Box::new(solutions::day22::part1(input)),
        (22, 2) => Box::new(solutions::day22::part2(input)),

        (23, 1) => Box::new(solutions::day23::part1(input)),
        (23, 2) => Box::new(solutions::day23::part2(input)),

        (24, 1) => Box::new(solutions::day24::part1(input)),
        (24, 2) => Box::new(solutions::day24::part2(input)),

        (25, 1) => Box::new(solutions::day25::part1(input)),
        (25, 2) => Box::new(solutions::day25::part2(input)),

        _ => panic!("No solver found"),
    }
}
