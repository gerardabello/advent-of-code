use std::collections::HashMap;

fn number_of_combinations(
    cache: &mut HashMap<(u64, usize), Option<u64>>,
    adapters: &[u64],
    index: usize,
    base: u64,
    target: u64,
) -> Option<u64> {
    if let Some(n) = cache.get(&(base, index)) {
        return *n;
    }

    if base == target {
        cache.insert((base, index), Some(1));
        return Some(1);
    }

    let mut count = 0;

    for (i,adapter) in adapters[index..].iter().enumerate() {
        let d = *adapter - base;

        if d > 3 {
            break;
        }

        if let Some(c) = number_of_combinations(cache, adapters, index + i + 1, *adapter, target) {
            count += c;
        }
    }

    if count == 0 {
        cache.insert((base, index), None);
        return None;
    }

    cache.insert((base, index), Some(count));
    Some(count)
}

pub fn solve(input: &str) -> String {
    let mut adapters: Vec<u64> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    adapters.sort();

    let target_jolts = adapters[adapters.len() - 1];
    let port_jolts = 0;

    number_of_combinations(&mut HashMap::new(), &adapters, 0, port_jolts, target_jolts)
        .unwrap()
        .to_string()
}
