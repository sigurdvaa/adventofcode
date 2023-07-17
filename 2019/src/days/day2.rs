use crate::intcode;
use std::fs;

fn find_noun_verb(intcode: &Vec<i32>) -> i32 {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut curr_intcode = intcode.clone();
            curr_intcode[1] = noun;
            curr_intcode[2] = verb;
            intcode::run(&mut curr_intcode, &mut vec![]);
            if curr_intcode[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

pub fn run() {
    println!("Day 2: 1202 Program Alarm");
    let file_path = "inputs/day2.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let mut prog = intcode::parse(&input_raw);
    prog[1] = 12;
    prog[2] = 2;
    intcode::run(&mut prog, &mut vec![]);
    println!("Part One: {}", prog[0]);

    let prog = intcode::parse(&input_raw);
    let noun_verb = find_noun_verb(&prog);
    println!("Part Two: {noun_verb}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut prog = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        intcode::run(&mut prog, &mut vec![]);
        assert_eq!(prog, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);

        let mut prog = vec![1, 0, 0, 0, 99];
        intcode::run(&mut prog, &mut vec![]);
        assert_eq!(prog, vec![2, 0, 0, 0, 99]);

        let mut prog = vec![2, 3, 0, 3, 99];
        intcode::run(&mut prog, &mut vec![]);
        assert_eq!(prog, vec![2, 3, 0, 6, 99]);

        let mut prog = vec![2, 4, 4, 5, 99, 0];
        intcode::run(&mut prog, &mut vec![]);
        assert_eq!(prog, vec![2, 4, 4, 5, 99, 9801]);

        let mut prog = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        intcode::run(&mut prog, &mut vec![]);
        assert_eq!(prog, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_part_two() {}
}
