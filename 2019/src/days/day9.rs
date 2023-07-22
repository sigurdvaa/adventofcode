use crate::intcode::{ExitCode, Program};
use std::fs;

pub fn run() {
    println!("Day 9: Sensor Boost");
    let file_path = "inputs/day9.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let mut prog = Program::new(&input_raw);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
