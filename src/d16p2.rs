use crate::d16p1::{parse_input, Rule, Ticket};

fn find_valid_rules<'a>(rules: &'a [Rule], numbers: &[u32]) -> Vec<String> {
    rules
        .iter()
        .filter(|rule| numbers.iter().all(|n| rule.validate(*n)))
        .map(|r| r.name.clone())
        .collect()
}

fn find_order_of_fields(rules: &[Rule], tickets: &[&Ticket]) -> Vec<String> {
    let n_fields = tickets[0].len();
    let mut ret: Vec<(usize, String)> = Vec::new();
    let mut matched_rules: Vec<(usize, Vec<String>)> = Vec::new();

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

        ret.push((*index, field.clone()));

        matched_rules = matched_rules
            .iter()
            .map(|(i, matches)| {
                (
                    *i,
                    matches
                        .iter()
                        .filter(|m| *m != field)
                        .cloned()
                        .collect::<Vec<String>>(),
                )
            })
            .collect();
    }

    // Sort the assigned field by index order
    ret.sort_by(|(ia, _), (ib, _)| ia.cmp(&ib));

    ret.iter().map(|(_, field)| field).cloned().collect()
}

fn is_valid_ticket(rules: &[Rule], ticket: &Ticket) -> bool {
    ticket.iter().all(|n| rules.iter().any(|r| r.validate(*n)))
}

pub fn solve(input: &str) -> String {
    let (_, (rules, my_ticket, tickets)) = parse_input(input).unwrap();

    let mut valid_tickets: Vec<&Ticket> = tickets
        .iter()
        .filter(|t| is_valid_ticket(&rules, t))
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
