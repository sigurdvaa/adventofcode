use crate::intcode::Program;
use std::fs;

pub fn run() {
    println!("Day 17: Set and Forget");
    let file_path = "inputs/day17.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let mut prog = Program::new(&input_raw);
    prog.run();
    dbg!(prog
        .output
        .iter()
        .map(|&x| x as u8 as char)
        .collect::<String>()
        .lines()
        .collect::<Vec<_>>());
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
