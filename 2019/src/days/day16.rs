use std::fs;

fn parse_numbers(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn fft_phases(mut input: Vec<u32>, phases: u32) -> Vec<u32> {
    let mut base = vec![0, 1, 0, -1];
    let mut output = vec![0; input.len()];
    for _ in 0..phases {
        output.clear();

        input = output;
    }
    output[..8].to_vec()
}

pub fn run() {
    println!("Day 16: Flawed Frequency Transmission");
    let file_path = "inputs/day16.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input_raw = "12345678";
        let numbers = parse_numbers(&input_raw);
        assert_eq!(fft_phases(numbers, 4), vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }

    #[test]
    fn test_part_two() {}
}
