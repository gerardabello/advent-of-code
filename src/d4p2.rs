use regex::Regex;

#[derive(Debug)]
enum HeightUnit {
    Centimeters,
    Inches,
}

#[derive(Debug)]
struct Height {
    magnitude: u32,
    unit: HeightUnit,
}

impl Height {
    fn parse(raw: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)(\D+)").unwrap();
        }

        let caps = RE.captures(raw)?;

        let magnitude = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let unit_s = caps.get(2).unwrap().as_str();

        let unit = match unit_s {
            "in" => HeightUnit::Inches,
            "cm" => HeightUnit::Centimeters,
            unit => panic!("Unknown unit {}", unit),
        };

        Some(Height { magnitude, unit })
    }
}

#[derive(Debug, Default)]
struct Passport {
    byr: Option<u32>,    // (Birth Year)
    iyr: Option<u32>,    // (Issue Year)
    eyr: Option<u32>,    // (Expiration Year)
    hgt: Option<Height>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
    cid: Option<String>, // (Country ID)
}

impl Passport {
    fn new(raw: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w+):(\S+)").unwrap();
        }

        let mut passport = Passport::default();

        for cap in RE.captures_iter(raw) {
            let key = &cap[1];
            let val = &cap[2];

            match key {
                "byr" => passport.byr = Some(val.parse::<u32>().unwrap()),
                "iyr" => passport.iyr = Some(val.parse::<u32>().unwrap()),
                "eyr" => passport.eyr = Some(val.parse::<u32>().unwrap()),
                "hgt" => passport.hgt = Height::parse(val),
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
    let byr_valid = match passport.byr {
        Some(year) => year >= 1920 && year <= 2002,
        None => false,
    };

    let iyr_valid = match passport.iyr {
        Some(year) => year >= 2010 && year <= 2020,
        None => false,
    };

    let eyr_valid = match passport.eyr {
        Some(year) => year >= 2020 && year <= 2030,
        None => false,
    };

    let hgt_valid = match &passport.hgt {
        Some(height) => match height.unit {
            HeightUnit::Centimeters => height.magnitude >= 150 && height.magnitude <= 193,
            HeightUnit::Inches => height.magnitude >= 59 && height.magnitude <= 76,
        },
        None => false,
    };

    lazy_static! {
        static ref HCL_RE: Regex = Regex::new(r"^#[a-zA-Z-0-9]{6}$").unwrap();
        static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        static ref PID_RE: Regex = Regex::new(r"^(\d){9}$").unwrap();
    }

    let hcl_valid = match &passport.hcl {
        Some(color) => HCL_RE.is_match(&color),
        None => false,
    };

    let ecl_valid = match &passport.ecl {
        Some(color) => ECL_RE.is_match(&color),
        None => false,
    };

    let pid_valid = match &passport.pid {
        Some(pid) => PID_RE.is_match(&pid),
        None => false,
    };

    byr_valid && iyr_valid && eyr_valid && hgt_valid && hcl_valid && ecl_valid && pid_valid
}

pub fn solve(input: &str) -> String {
    input
        .split("\n\n")
        .map(|rp| Passport::new(rp))
        .filter(|p| validate_passport(p))
        .count()
        .to_string()
}
