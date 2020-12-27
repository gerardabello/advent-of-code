use crate::d7p1::{parse_rule_line, BagRule};

fn number_of_bags(rules: &[BagRule], bag: &str) -> u32 {
    let (_, expected_contents) = rules.iter().find(|(name, _)| name == bag).unwrap();

    expected_contents
        .iter()
        .map(|(name, quantity)| quantity + quantity * number_of_bags(rules, name))
        .sum()
}

pub fn solve(input: &str) -> String {
    let bag_rules: Vec<BagRule> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|l| parse_rule_line(l))
        .collect();

    number_of_bags(&bag_rules, "shiny gold").to_string()
}
