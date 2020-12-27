use crate::d16p1::{parse_input, Rule};

fn find_valid_rules<'a>(rules: &'a [Rule], numbers: &[u32]) -> Vec<&'a str> {
    rules
        .iter()
        .filter(|rule| numbers.iter().all(|n| rule.validate(*n)))
        .map(|r| r.name.as_ref())
        .collect()
}

fn find_order_of_fields<'a> (rules: &'a [Rule], tickets: &[&[u32]]) -> Vec<&'a str> {
    let n_fields = tickets[0].len();
    let mut ret: Vec<(usize, &str)> = Vec::new();
    let mut matched_rules: Vec<(usize, Vec<&str>)> = Vec::new();

    // For each index, find all field that match
    for i in 0..n_fields {
        let numbers_in_i: Vec<u32> = tickets.iter().map(|t| t[i]).collect();

        matched_rules.push((i, find_valid_rules(&rules, &numbers_in_i)));
    }

    // Sort them by number of matched fields, we want to first assign the indexes that have less
    // possibilities.
    matched_rules.sort_by(|(_, ma), (_, mb)| ma.len().cmp(&mb.len()));

    // For each index, assign the first possibility, and remove that field from the rest
    for mi in 0..matched_rules.len() {
        let (index, matches) = &matched_rules[mi];
        assert!(!matches.is_empty());

        let field = &matches[0];

        ret.push((*index, field));

        matched_rules = matched_rules
            .iter()
            .map(|(i, matches)| {
                (
                    *i,
                    matches
                        .iter()
                        .filter(|m| *m != field)
                        .copied()
                        .collect::<Vec<&str>>(),
                )
            })
            .collect();
    }

    // Sort the assigned field by index order
    ret.sort_by(|(ia, _), (ib, _)| ia.cmp(&ib));

    ret.iter().map(|(_, field)| field).cloned().collect()
}

fn is_valid_ticket(rules: &[Rule], ticket: &[u32]) -> bool {
    ticket.iter().all(|n| rules.iter().any(|r| r.validate(*n)))
}

pub fn solve(input: &str) -> String {
    let (_, (rules, my_ticket, tickets)) = parse_input(input).unwrap();

    let mut valid_tickets: Vec<&[u32]> = tickets
        .iter()
        .filter(|t| is_valid_ticket(&rules, t))
        .map(|x| x.as_ref())
        .collect();


    valid_tickets.push(&my_ticket);

    let order = find_order_of_fields(&rules, &valid_tickets);


    let indexes_to_multiply: Vec<usize> = order
        .iter()
        .enumerate()
        .filter(|(_, name)| name.starts_with("departure"))
        .map(|(i, _)| i)
        .collect();


    my_ticket
        .iter()
        .enumerate()
        .filter(|(i, _)| indexes_to_multiply.contains(i))
        .map(|(_, val)| *val as u64)
        .product::<u64>()
        .to_string()
}
