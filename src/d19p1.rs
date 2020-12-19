use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until},
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
pub enum ParseRule {
    Base(char),
    Combined(Vec<Vec<usize>>),
}

fn parse_base_rule(input: &str) -> IResult<&str, ParseRule> {
    let (input, _) = tag("\"")(input)?;
    let (input, s) = take(1 as usize)(input)?;
    let (input, _) = tag("\"")(input)?;

    let c = s.chars().next().expect("expect 1 char in base rule");

    Ok((input, ParseRule::Base(c)))
}

fn parse_rule_index_list(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(" "), map_res(digit1, |s: &str| s.parse::<usize>()))(input)
}

fn parse_combined_rule(input: &str) -> IResult<&str, ParseRule> {
    let (input, list) = separated_list1(tag(" | "), parse_rule_index_list)(input)?;
    Ok((input, ParseRule::Combined(list)))
}

fn parse_rule(input: &str) -> IResult<&str, (usize, ParseRule)> {
    let (input, index) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, rule) = alt((parse_combined_rule, parse_base_rule))(input)?;

    Ok((input, (index, rule)))
}

fn parse_rules(input: &str) -> IResult<&str, HashMap<usize, ParseRule>> {
    let (input, list) = separated_list1(tag("\n"), parse_rule)(input)?;

    let mut hashmap: HashMap<usize, ParseRule> = HashMap::new();

    for (index, rule) in list {
        hashmap.insert(index, rule);
    }

    Ok((input, hashmap))
}

fn parse_message(input: &str) -> IResult<&str, String> {
    let (input, message_string) = take_until("\n")(input)?;
    Ok((input, message_string.to_owned()))
}

fn parse_messages(input: &str) -> IResult<&str, Vec<String>> {
    separated_list1(tag("\n"), parse_message)(input)
}

pub fn parse_input(input: &str) -> IResult<&str, (HashMap<usize, ParseRule>, Vec<String>)> {
    let (input, rules) = parse_rules(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, messages) = parse_messages(input)?;

    Ok((input, (rules, messages)))
}

 fn match_and_rule<'a>(
    rules: &HashMap<usize, ParseRule>,
    list: &[usize],
    message: &'a str,
) -> Vec<&'a str> {
    let mut ret = vec![];

    if list.is_empty() {
        return vec![message];
    }

    let matches = match_rule(rules, list[0], message);
    for m in matches {
        let new_matches = match_and_rule(rules, &list[1..], m);
        for nm in new_matches {
            ret.push(nm);
        }
    }

    ret
}

 fn match_combined_rule<'a>(
    rules: &HashMap<usize, ParseRule>,
    list: &[Vec<usize>],
    message: &'a str,
) -> Vec<&'a str> {
    let mut ret = vec![];

    for and_rule in list {
        for m in &match_and_rule(rules, and_rule, message) {
            ret.push(*m);
        }
    }

    ret
}

 fn match_rule<'a>(
    rules: &HashMap<usize, ParseRule>,
    rule_index: usize,
    message: &'a str,
) -> Vec<&'a str> {
    let rule = &rules[&rule_index];

    if message.is_empty() {
        return vec![];
    }

    match rule {
        ParseRule::Base(c) => {
            if message.chars().next().unwrap() == *c {
                vec![&message[1..]]
            } else {
                vec![]
            }
        }
        ParseRule::Combined(list) => match_combined_rule(rules, &list, message),
    }
}

pub fn message_matches_rule(
    rules: &HashMap<usize, ParseRule>,
    rule_index: usize,
    message: &str,
) -> bool {
    if message.is_empty() {
        return false;
    }

    match_rule(rules, rule_index, message).iter().any(|r| r.is_empty())
}

pub fn solve(input: &str) -> String {
    let (_, (rules, messages)) = parse_input(input).unwrap();

    messages
        .iter()
        .filter(|m| message_matches_rule(&rules, 0, m))
        .count()
        .to_string()
}
