use std::collections::HashMap;

type CustomsDeclaration = Vec<char>;

fn parse_customs_declaration(input: &str) -> CustomsDeclaration {
    let mut map = HashMap::new();

    let people = input.split('\n').filter(|s| !s.is_empty()).count();

    for line in input.split('\n').filter(|s| !s.is_empty()) {
        for c in line.chars() {
            let count = match map.get(&c) {
                Some(c) => c + 1,
                None => 1,
            };
            map.insert(c, count);
        }
    }

    map.iter()
        .filter(|(_, count)| **count as usize == people)
        .map(|(c, _)| c)
        .cloned()
        .collect()
}

pub fn solve(input: &str) -> String {
    input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|s| parse_customs_declaration(s))
        .map(|cd| cd.len())
        .sum::<usize>()
        .to_string()
}
