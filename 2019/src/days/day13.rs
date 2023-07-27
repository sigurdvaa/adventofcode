use crate::intcode::Program;
use std::fs;

pub fn run() {
    println!("Day 13: Care Package");
    let file_path = "inputs/day13.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let prog = Program::new(&input_raw);
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
