use regex::Regex;

#[derive(Debug, Default)]
struct Passport {
    byr: Option<String>, // (Birth Year)
    iyr: Option<String>, // (Issue Year)
    eyr: Option<String>, // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
    cid: Option<String>, // (Country ID)
}

impl Passport {
    fn new(raw: &str) -> Self {
        let re = Regex::new(r"(\w+):(\S+)").unwrap();

        let mut passport = Passport::default();

        for cap in re.captures_iter(raw) {
            let key = &cap[1];
            let val = &cap[2];

            match key {
                "byr" => passport.byr = Some(val.to_owned()),
                "iyr" => passport.iyr = Some(val.to_owned()),
                "eyr" => passport.eyr = Some(val.to_owned()),
                "hgt" => passport.hgt = Some(val.to_owned()),
                "hcl" => passport.hcl = Some(val.to_owned()),
                "ecl" => passport.ecl = Some(val.to_owned()),
                "pid" => passport.pid = Some(val.to_owned()),
                "cid" => passport.cid = Some(val.to_owned()),
                unknown => panic!("Unknown field in password {}", unknown),
            }
        }

        passport
    }
}

fn validate_passport(passport: &Passport) -> bool {
    // cid is optional
    passport.byr.is_some()
        && passport.iyr.is_some()
        && passport.eyr.is_some()
        && passport.hgt.is_some()
        && passport.hcl.is_some()
        && passport.ecl.is_some()
        && passport.pid.is_some()
}

pub fn solve(input: &str) -> String {
    input
        .split("\n\n")
        .map(|rp| Passport::new(rp))
        .filter(|p| validate_passport(p))
        .count()
        .to_string()
}
