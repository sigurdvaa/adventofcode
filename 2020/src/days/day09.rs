use std::fs;

fn parse_numbers(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn first_invalid_number(numbers: &[usize], preamble: usize) -> Option<usize> {
    let mut i = preamble;
    let len = numbers.len();
    while i < len {
        let mut valid = false;
        for n1 in &numbers[i - preamble..i - 1] {
            for n2 in &numbers[i - preamble + 1..i] {
                if n1 + n2 == numbers[i] {
                    valid = true;
                    break;
                }
            }
            if valid {
                break;
            }
        }
        if !valid {
            return Some(numbers[i]);
        }
        i += 1;
    }
    None
}

pub fn run() {
    println!("Day 9: Encoding Error");
    let file_path = "inputs/day09.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    let numbers = parse_numbers(&input_raw);
    println!("Part One: {}", first_invalid_number(&numbers, 25).unwrap());
    println!("Part Two: {}", "TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        const INPUT_TEST: &str = concat!(
            "35\n", "20\n", "15\n", "25\n", "47\n", "40\n", "62\n", "55\n", "65\n", "95\n",
            "102\n", "117\n", "150\n", "182\n", "127\n", "219\n", "299\n", "277\n", "309\n", "576",
        );
        let numbers = parse_numbers(INPUT_TEST);
        assert_eq!(first_invalid_number(&numbers, 5), Some(127));
    }

    #[test]
    fn test_part_two() {}
}
