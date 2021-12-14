#![allow(dead_code)]

use std::collections::{HashSet,HashMap};

pub fn count_elements<T: std::hash::Hash + std::cmp::Eq>(
    iter: impl IntoIterator<Item = T>,
) -> HashMap<T, usize> {
    let mut hm = HashMap::new();
    for element in iter {
        match hm.get_mut(&element) {
            Some(v) => *v += 1,
            None => {
                hm.insert(element, 1);
            }
        }
    }

    hm
}

pub fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + std::hash::Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}
