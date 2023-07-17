use crate::intcode;
use std::fs;

pub fn run() {
    println!("Day 5: Sunny with a Chance of Asteroids");
    let file_path = "inputs/day5.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let mut prog = intcode::parse(&input_raw);
    let mut input = vec![1];
    let output = intcode::run(&mut prog, &mut input);
    println!("Part One: {}", output.last().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut intcode = vec![1002, 4, 3, 4, 33];
        intcode::run(&mut intcode, &mut vec![]);
        assert_eq!(intcode, vec![1002, 4, 3, 4, 99]);

        let mut intcode = vec![3, 2, 0];
        let mut input = vec![99];
        intcode::run(&mut intcode, &mut input);
        assert_eq!(intcode, vec![3, 2, 99]);

        let mut intcode = vec![4, 2, 99];
        let output = intcode::run(&mut intcode, &mut vec![]);
        assert_eq!(output, vec![99]);
    }

    #[test]
    fn test_part_two() {}
}
