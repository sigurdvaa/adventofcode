fn valid_password(password: &str, strict_double: bool) -> bool {
    let digits: Vec<char> = password.chars().collect();
    let mut double = false;
    let mut prev_count = 1;
    for (i, d) in digits.iter().skip(1).enumerate() {
        if *d < digits[i] {
            return false;
        }

        if strict_double {
            if *d == digits[i] {
                prev_count += 1;
            } else {
                if prev_count == 2 {
                    double = true;
                }
                prev_count = 1;
            }
        } else {
            if *d == digits[i] {
                double = true;
            }
        }
    }

    if strict_double && prev_count == 2 {
        double = true;
    }
    double
}

fn valid_passwords_in_range(range: (u32, u32), strict_double: bool) -> u32 {
    let mut count = 0;
    for i in range.0..=range.1 {
        if valid_password(&i.to_string(), strict_double) {
            count += 1;
        }
    }
    count
}

pub fn run() {
    println!("Day 4: Secure Container");
    let range = (284639, 748759);
    println!("Part One: {}", valid_passwords_in_range(range, false));
    println!("Part Two: {}", valid_passwords_in_range(range, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(valid_password("111111", false), true);
        assert_eq!(valid_password("223450", false), false);
        assert_eq!(valid_password("123789", false), false);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(valid_password("112233", true), true);
        assert_eq!(valid_password("123444", true), false);
        assert_eq!(valid_password("111122", true), true);
    }
}
