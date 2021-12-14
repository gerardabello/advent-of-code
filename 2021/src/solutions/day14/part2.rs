use crate::solutions::day14::part1::{parse_input, Element};
use std::collections::HashMap;

use crate::iter::count_elements;

pub fn add_or_start_count<T: std::hash::Hash + std::cmp::Eq + Clone>(
    counts: &mut HashMap<T, usize>,
    key: &T,
    ammount: usize,
) {
    match counts.get_mut(key) {
        Some(v) => *v += ammount,
        None => {
            counts.insert(key.clone(), ammount);
        }
    }
}

fn pair_counts_to_element_counts(
    pair_counts: &HashMap<(Element, Element), usize>,
    template: &[Element],
) -> HashMap<Element, usize> {
    let mut element_counts = HashMap::new();

    for (pair, count) in pair_counts {
        add_or_start_count(&mut element_counts, &pair.0, *count);
        add_or_start_count(&mut element_counts, &pair.1, *count);
    }

    // Elements are duplicated in the pairs, so we need to divide by 2
    for (_, count) in element_counts.iter_mut() {
        *count /= 2;
    }

    // The first and last elements are not duplicated, so we need to add 1
    add_or_start_count(&mut element_counts, template.first().unwrap(), 1);
    add_or_start_count(&mut element_counts, template.last().unwrap(), 1);

    element_counts
}

pub fn solve(input: &str) -> usize {
    let (template, rules) = parse_input(input);

    let pairs: Vec<(Element, Element)> = template.windows(2).map(|w| (w[0], w[1])).collect();
    let mut pair_counts = count_elements(pairs);

    for _ in 0..40 {
        let mut new_pair_counts = pair_counts.clone();
        for rule in &rules {
            for (pair, count) in &pair_counts {
                if rule.0 == *pair {
                    match new_pair_counts.get_mut(pair) {
                        Some(v) => *v -= *count,
                        None => {
                            unreachable!();
                        }
                    }
                    add_or_start_count(&mut new_pair_counts, &(pair.0, rule.1), *count);
                    add_or_start_count(&mut new_pair_counts, &(rule.1, pair.1), *count);
                }
            }
        }
        pair_counts = new_pair_counts;
    }

    let element_counts = pair_counts_to_element_counts(&pair_counts, &template);
    let mut count_values: Vec<usize> = element_counts.values().copied().collect();
    count_values.sort_unstable();

    count_values[count_values.len() - 1] - count_values[0]
}
