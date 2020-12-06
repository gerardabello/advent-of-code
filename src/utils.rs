use std::fs;

pub fn get_input(file: &str) -> String {
    fs::read_to_string(format!("./inputs/{}", file))
        .expect("Something went wrong reading the input file")
}
