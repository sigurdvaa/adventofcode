use std::fs;

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

    fn validate(&mut self) {
        todo!();
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
    passports
        .into_iter()
        .filter_map(|p| {
            p.validate();
            p.valid()
        })
        .count()
}

pub fn run() {
    println!("Day 4: Passport Processing");
    let file_path = "inputs/day04.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    println!("Part One: {}", valid_passports(&input_raw));
    println!("Part Two: {}", "TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_part_one() {
        assert_eq!(valid_passports(TEST_INPUT), 2);
    }

    #[test]
    fn test_part_two() {}
}
