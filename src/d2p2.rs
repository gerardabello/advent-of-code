use regex::Regex;

struct TobogganPolicy {
    letter: char,
    pos1: u8,
    pos2: u8,
}

impl TobogganPolicy {
    fn new(policy_string: &str) -> Self {
        let re = Regex::new(r"(\d+)-(\d+) (\w)").unwrap();
        let cap = re
            .captures_iter(policy_string)
            .next()
            .expect("Error parsing policy");

        let letter_s = &cap[3];
        let pos1_s = &cap[1];
        let pos2_s = &cap[2];

        let letter = letter_s.chars().next().unwrap();
        let pos1 = pos1_s.parse::<u8>().unwrap();
        let pos2 = pos2_s.parse::<u8>().unwrap();

        Self { letter, pos1, pos2 }
    }

    fn validate(&self, password: &str) -> bool {
        let mut count = 0;

        if password.chars().nth((self.pos1 - 1) as usize).unwrap() == self.letter {
            count += 1
        }

        if password.chars().nth((self.pos2 - 1) as usize).unwrap() == self.letter {
            count += 1
        }

        count == 1
    }
}

fn is_valid_toboggan_password(entry: &str) -> bool {
    let vals: Vec<&str> = entry.split(':').map(|s| s.trim()).collect();

    let policy_string = vals[0];
    let password = vals[1];

    let policy = TobogganPolicy::new(policy_string);

    policy.validate(password)
}

pub fn solve(input: &str) -> String {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .filter(|s| is_valid_toboggan_password(s))
        .count()
        .to_string()
}
