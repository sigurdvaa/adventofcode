use crate::intcode;
use std::fs;

pub fn run() {
    println!("Day 5: Sunny with a Chance of Asteroids");
    let file_path = "inputs/day5.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let mut prog = intcode::parse(&input_raw);
    let mut input = vec![5, 1];

    let output = intcode::run(&mut prog.clone(), &mut input);
    println!("Part One: {}", output.last().unwrap());

    let output = intcode::run(&mut prog, &mut input);
    println!("Part Two: {}", output.last().unwrap());
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
        assert_eq!(intcode::run(&mut intcode, &mut vec![]), vec![99]);
    }

    #[test]
    fn test_part_two() {
        let mut intcode = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut input = vec![69, 8];
        assert_eq!(intcode::run(&mut intcode.clone(), &mut input), vec![1]);
        assert_eq!(intcode::run(&mut intcode, &mut input), vec![0]);

        let mut intcode = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut input = vec![8, 3];
        assert_eq!(intcode::run(&mut intcode.clone(), &mut input), vec![1]);
        assert_eq!(intcode::run(&mut intcode, &mut input), vec![0]);

        let mut intcode = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut input = vec![69, 8];
        assert_eq!(intcode::run(&mut intcode.clone(), &mut input), vec![1]);
        assert_eq!(intcode::run(&mut intcode, &mut input), vec![0]);

        let mut intcode = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut input = vec![0, 69];
        assert_eq!(intcode::run(&mut intcode.clone(), &mut input), vec![1]);
        assert_eq!(intcode::run(&mut intcode, &mut input), vec![0]);

        let mut intcode = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut input = vec![0, 69];
        assert_eq!(intcode::run(&mut intcode.clone(), &mut input), vec![1]);
        assert_eq!(intcode::run(&mut intcode, &mut input), vec![0]);

        let mut intcode = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut input = vec![69, 8, 0];
        assert_eq!(intcode::run(&mut intcode.clone(), &mut input), vec![999]);
        assert_eq!(intcode::run(&mut intcode.clone(), &mut input), vec![1000]);
        assert_eq!(intcode::run(&mut intcode, &mut input), vec![1001]);
    }
}
