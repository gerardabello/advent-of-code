use regex::Regex;

pub type BagRule = (String, Vec<(String, u32)>);

pub fn parse_rule_line(rule_line: &str) -> BagRule {
    if Regex::new(r"^([\D\s]*) bags contain no other bags")
        .unwrap()
        .is_match(rule_line)
    {
        let re = Regex::new(r"^([\D\s]*) bags contain no other bags").unwrap();
        let cap = re.captures_iter(rule_line).next().expect("Error parsing");

        let root_bag = &cap[1];
        return (root_bag.to_owned(), vec![]);
    }

    let re = Regex::new(r"^([\D\s]*) bags contain").unwrap();
    let cap = re
        .captures_iter(rule_line)
        .next()
        .expect("Error parsing policy");

    let root_bag = &cap[1];

    let vector: Vec<(String, u32)> = Regex::new(r"([\d]+) ([a-z ]+) (bag|bags)([,\.]{1})")
        .unwrap()
        .captures_iter(rule_line)
        .map(|cap| (cap[2].to_owned(), cap[1].parse::<u32>().unwrap()))
        .collect();

    (root_bag.to_owned(), vector)
}

fn find_parents<'a>(rules: &'a [BagRule], bag: &str) -> Vec<&'a str> {
    rules
        .iter()
        .filter(|(_, expected_contents)| expected_contents.iter().any(|(n, _)| n == bag))
        .map(|(name, _)| name)
        .map(|s| s.as_str())
        .collect()
}

fn find_all_parents<'a>(rules: &'a [BagRule], bag: &str) -> Vec<&'a str> {
    let parents = find_parents(rules, bag);

    let all_parents: Vec<&str> = parents
        .iter()
        .flat_map(|p| find_all_parents(rules, p).into_iter())
        .map(|s| s)
        .collect();

    [parents, all_parents].concat()
}

pub fn solve(input: &str) -> String {
    let bag_rules: Vec<BagRule> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|l| parse_rule_line(l))
        .collect();

     let mut all : Vec<&str> = find_all_parents(&bag_rules, "shiny gold");
     all.sort_unstable();

     all.dedup();

     all.len().to_string()
}
