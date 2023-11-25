use std::fs;

struct Password<'a> {
    min: usize,
    max: usize,
    letter: char,
    password: &'a str,
}

fn count_valid_passwords_char_count(passwords: &[Password]) -> usize {
    passwords
        .iter()
        .filter(|pw| {
            let count = pw.password.chars().filter(|&c| c == pw.letter).count();
            pw.min <= count && count <= pw.max
        })
        .count()
}

fn count_valid_passwords_char_pos(passwords: &[Password]) -> usize {
    passwords
        .iter()
        .filter(|pw| {
            let chars = pw.password.chars().collect::<Vec<_>>();
            let a = chars[pw.min - 1];
            let b = chars[pw.max - 1];
            (a == pw.letter || b == pw.letter) && a != b
        })
        .count()
}

fn parse_policy_and_password(input: &str) -> Vec<Password> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut split = line.split(' ');
            let mut range = split.next().unwrap().split('-');
            let min = range.next().unwrap().parse::<usize>().unwrap();
            let max = range.next().unwrap().parse::<usize>().unwrap();
            let letter = split.next().unwrap().chars().next().unwrap();
            let password = split.next().unwrap();
            Password {
                min,
                max,
                letter,
                password,
            }
        })
        .collect::<Vec<_>>()
}

pub fn run() {
    println!("Day 2: Password Philosophy");
    let file_path = "inputs/day02.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    let passwords = parse_policy_and_password(&input_raw);
    println!("Part One: {}", count_valid_passwords_char_count(&passwords));
    println!("Part Two: {}", count_valid_passwords_char_pos(&passwords));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_TEST: &str = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

    #[test]
    fn test_part_one() {
        let passwords = parse_policy_and_password(INPUT_TEST);
        assert_eq!(count_valid_passwords_char_count(&passwords), 2);
    }

    #[test]
    fn test_part_two() {
        let passwords = parse_policy_and_password(INPUT_TEST);
        assert_eq!(count_valid_passwords_char_pos(&passwords), 1);
    }
}
