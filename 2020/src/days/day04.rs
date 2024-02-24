use std::{any::Any, fs};

#[derive(Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn valid_data(&self) -> bool {
        (match &self.byr {
            Some(value) => matches!(value.parse(), Ok(1920..=2002)),
            None => false,
        }) && match &self.iyr {
            Some(value) => matches!(value.parse(), Ok(2010..=2020)),
            None => false,
        } && match &self.eyr {
            Some(value) => matches!(value.parse(), Ok(2020..=2030)),
            None => false,
        } && match &self.hgt {
            Some(value) => {
                if value.len() < 4 {
                    return false;
                }
                let value = &value[..value.len() - 2];
                let suffix = &value[(value.len() - 2)..];
                match suffix {
                    "cm" => {
                        matches!(value.parse(), Ok(150..=193))
                    }
                    "in" => {
                        matches!(value.parse(), Ok(59..=76))
                    }
                    _ => false,
                }
            }
            None => false,
        } && match &self.hcl {
            Some(value) => {
                if value.len() != 7 || value.get(0..1) != Some("#") {
                    return false;
                }
                value
                    .chars()
                    .skip(1)
                    .filter(|c| c.is_ascii_hexdigit())
                    .count()
                    == 6
            }
            None => false,
        } && match &self.ecl {
            Some(value) => matches!(
                *value,
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
            ),
            None => false,
        }
    }
}

fn parse_passports(file: &str) -> Vec<Passport> {
    let mut batches = vec![];
    let mut batch = String::new();
    for line in file.lines() {
        if line.is_empty() {
            batch.pop();
            batches.push(batch.clone());
            batch.clear();
        } else {
            batch.push_str(&format!("{} ", line));
        }
    }
    batch.pop();
    batches.push(batch.clone());

    let mut passports = vec![];
    for batch in batches {
        let mut passport = Passport::new();

        for seq in batch.split_whitespace() {
            let mut parts = seq.split(':');
            match parts.next() {
                Some("byr") => passport.byr = Some(parts.next().unwrap().to_string()),
                Some("iyr") => passport.iyr = Some(parts.next().unwrap().to_string()),
                Some("eyr") => passport.eyr = Some(parts.next().unwrap().to_string()),
                Some("hgt") => passport.hgt = Some(parts.next().unwrap().to_string()),
                Some("hcl") => passport.hcl = Some(parts.next().unwrap().to_string()),
                Some("ecl") => passport.ecl = Some(parts.next().unwrap().to_string()),
                Some("pid") => passport.pid = Some(parts.next().unwrap().to_string()),
                Some("cid") => passport.cid = Some(parts.next().unwrap().to_string()),
                Some(key) => panic!("unknown key: {}", key),
                None => panic!("missing key"),
            }
        }

        passports.push(passport);
    }

    passports
}

fn valid_passports(file: &str) -> usize {
    let passports = parse_passports(file);
    passports.iter().filter(|p| p.valid()).count()
}

fn valid_passports_with_data(file: &str) -> usize {
    let passports = parse_passports(file);
    passports.iter().filter(|p| p.valid_data()).count()
}

pub fn run() {
    println!("Day 4: Passport Processing");
    let file_path = "inputs/day04.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    println!("Part One: {}", valid_passports(&input_raw));
    println!("Part Two: {}", valid_passports_with_data(&input_raw));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        const TEST_INPUT: &str = concat!(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n",
            "byr:1937 iyr:2017 cid:147 hgt:183cm\n",
            "\n",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n",
            "hcl:#cfa07d byr:1929\n",
            "\n",
            "hcl:#ae17e1 iyr:2013\n",
            "eyr:2024\n",
            "ecl:brn pid:760753108 byr:1931\n",
            "hgt:179cm\n",
            "\n",
            "hcl:#cfa07d eyr:2025 pid:166559648\n",
            "iyr:2011 ecl:brn hgt:59in\n",
        );

        assert_eq!(valid_passports(TEST_INPUT), 2);
    }

    #[test]
    fn test_part_two() {
        const TEST_INPUT: &str = concat!(
            "eyr:1972 cid:100\n",
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n",
            "\n",
            "iyr:2019\n",
            "hcl:#602927 eyr:1967 hgt:170cm\n",
            "ecl:grn pid:012533040 byr:1946\n",
            "\n",
            "hcl:dab227 iyr:2012\n",
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n",
            "\n",
            "hgt:59cm ecl:zzz\n",
            "eyr:2038 hcl:74454a iyr:2023\n",
            "pid:3556412378 byr:2007\n",
            "\n",
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n",
            "hcl:#623a2f\n",
            "\n",
            "eyr:2029 ecl:blu cid:129 byr:1989\n",
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n",
            "\n",
            "hcl:#888785\n",
            "hgt:164cm byr:2001 iyr:2015 cid:88\n",
            "pid:545766238 ecl:hzl\n",
            "eyr:2022\n",
            "\n",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        );
        assert_eq!(valid_passports(TEST_INPUT), 4);
    }
}
