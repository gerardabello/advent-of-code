use crate::d19p1::{parse_input, message_matches_rule ,ParseRule};

pub fn solve(input: &str) -> String {
    let (_, (mut rules, messages)) = parse_input(input).unwrap();

    rules.insert(8, ParseRule::Combined(vec![vec![42], vec![42, 8]]));
    rules.insert(
        11,
        ParseRule::Combined(vec![vec![42, 31], vec![42, 11, 31]]),
    );

    let matching_messages: Vec<String> = messages
        .iter()
        .filter(|m| message_matches_rule(&rules, 0, m))
        .cloned()
        .collect();

    matching_messages.len().to_string()
}
