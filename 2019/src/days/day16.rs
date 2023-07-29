use std::fs;

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
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
