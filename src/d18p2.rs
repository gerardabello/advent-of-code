use regex::{Captures, Regex};

pub fn pass_parenthesis(input: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\(([\s\d\+\*]+)\)").unwrap();
    }

    let result = RE.replace(input, |caps: &Captures| {
        let s = &caps[1];
        passes(s)
    });

    result.as_ref().to_owned()
}

pub fn pass_sum(input: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([\d]+)\s\+\s([\d]+)").unwrap();
    }
    let result = RE.replace(input, |caps: &Captures| {
        let n1 = &caps[1].parse::<u64>().unwrap();
        let n2 = &caps[2].parse::<u64>().unwrap();
        format!("{}", n1 + n2)
    });

    result.as_ref().to_owned()
}

pub fn pass_mul(input: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([\d]+)\s\*\s([\d]+)").unwrap();
    }
    let result = RE.replace(input, |caps: &Captures| {
        let n1 = &caps[1].parse::<u64>().unwrap();
        let n2 = &caps[2].parse::<u64>().unwrap();
        format!("{}", n1 * n2)
    });

    result.as_ref().to_owned()
}

pub fn passes(input: &str) -> String {
    let mut s = input.to_string();
    loop {

        let n_s = pass_parenthesis(&s);
        if n_s != s {
            s = n_s;
            continue;
        }
        let n_s = pass_sum(&s);
        if n_s != s {
            s = n_s;
            continue;
        }
        let n_s = pass_mul(&s);
        if n_s != s {
            s = n_s;
            continue;
        }

        return s;
    }
}

pub fn solve(input: &str) -> String {
    let numbers: Vec<u64> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(passes)
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    numbers.iter().sum::<u64>().to_string()
}
