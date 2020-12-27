use regex::Regex;

struct SledPolicy {
    letter: char,
    min: u8,
    max: u8,
}

impl SledPolicy {
    fn new(policy_string: &str) -> Self {
        let re = Regex::new(r"(\d+)-(\d+) (\w)").unwrap();
        let cap = re
            .captures_iter(policy_string)
            .next()
            .expect("Error parsing policy");

        let letter_s = &cap[3];
        let min_s = &cap[1];
        let max_s = &cap[2];

        let letter = letter_s.chars().next().unwrap();
        let min = min_s.parse::<u8>().unwrap();
        let max = max_s.parse::<u8>().unwrap();

        Self { letter, min, max }
    }

    fn validate(&self, password: &str) -> bool {
        let count = password.chars().filter(|c| c == &self.letter).count();

        count >= self.min as usize && count <= self.max as usize
    }
}

fn is_valid_sled_password(entry: &str) -> bool {
    let vals: Vec<&str> = entry.split(':').map(|s| s.trim()).collect();

    let policy_string = vals[0];
    let password = vals[1];

    let policy = SledPolicy::new(policy_string);

    policy.validate(password)
}

pub fn solve(input: &str) -> String {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .filter(|s| is_valid_sled_password(s))
        .count()
        .to_string()
}
