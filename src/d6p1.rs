use std::collections::HashSet;

type CustomsDeclaration = Vec<char>;

fn parse_customs_declaration(input: &str) -> CustomsDeclaration {
    let mut map = HashSet::new();

    for line in input.split('\n') {
        for c in line.chars() {
            map.insert(c);
        }
    }

    map.into_iter().collect()
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
