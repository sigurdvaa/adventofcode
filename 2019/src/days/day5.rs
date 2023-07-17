use std::fs;

pub fn run() {
    println!("Day 5: Sunny with a Chance of Asteroids");
    let file_path = "inputs/day5.txt";
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
