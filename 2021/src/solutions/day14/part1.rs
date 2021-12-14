use nom::{
    bytes::complete::tag,
    multi::many1,
    sequence::separated_pair,
    sequence::{pair, tuple},
    IResult,
};

use crate::iter::count_elements;
use crate::parsers::{full, lines, single_letter};

pub type Element = char;
pub type ExpansionRule = ((Element, Element), Element);

pub fn parse_template(input: &str) -> IResult<&str, Vec<Element>> {
    many1(single_letter)(input)
}

pub fn parse_rule(input: &str) -> IResult<&str, ExpansionRule> {
    separated_pair(
        pair(single_letter, single_letter),
        tag(" -> "),
        single_letter,
    )(input)
}

pub fn parse_rules(input: &str) -> IResult<&str, Vec<ExpansionRule>> {
    lines(parse_rule)(input)
}

pub fn parse_input(input: &str) -> (Vec<Element>, Vec<ExpansionRule>) {
    let (_, (template, _, rules)) =
        full(tuple((parse_template, tag("\n\n"), parse_rules)))(input).unwrap();
    (template, rules)
}

pub fn grow_polymer(template: &[Element], rules: &[ExpansionRule]) -> Vec<Element> {
    let mut i = 1;
    let mut result = vec![];
    loop {
        result.push(template[i - 1]);
        if i >= template.len() {
            break;
        }

        let current_pair = (template[i - 1], template[i]);
        for rule in rules {
            if rule.0 == current_pair {
                result.push(rule.1);
            }
        }

        i += 1;
    }

    result
}

pub fn solve(input: &str) -> usize {
    let (template, rules) = parse_input(input);

    let mut polymer = template;

    for _ in 0..10 {
        polymer = grow_polymer(&polymer, &rules);
    }

    let counts = count_elements(polymer);

    let mut count_values: Vec<usize> = counts.values().copied().collect();
    count_values.sort_unstable();

    count_values[count_values.len() - 1] - count_values[0]
}
