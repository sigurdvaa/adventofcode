use crate::intcode::Program;
use std::fs;

fn find_noun_verb(prog: &Program) -> i64 {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut curr_prog = prog.clone();
            curr_prog.intcode[1] = noun;
            curr_prog.intcode[2] = verb;
            curr_prog.run();
            if curr_prog.intcode[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

pub fn run() {
    println!("Day 2: 1202 Program Alarm");
    let file_path = "inputs/day02.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    let mut prog = Program::new(&input_raw);
    prog.intcode[1] = 12;
    prog.intcode[2] = 2;
    prog.run();
    println!("Part One: {}", prog.intcode[0]);

    let prog = Program::new(&input_raw);
    let noun_verb = find_noun_verb(&prog);
    println!("Part Two: {noun_verb}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut prog = Program::new("1,9,10,3,2,3,11,0,99,30,40,50");
        prog.run();
        assert_eq!(
            prog.intcode,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );

        let mut prog = Program::new("1,0,0,0,99");
        prog.run();
        assert_eq!(prog.intcode, vec![2, 0, 0, 0, 99]);

        let mut prog = Program::new("2,3,0,3,99");
        prog.run();
        assert_eq!(prog.intcode, vec![2, 3, 0, 6, 99]);

        let mut prog = Program::new("2,4,4,5,99,0");
        prog.run();
        assert_eq!(prog.intcode, vec![2, 4, 4, 5, 99, 9801]);

        let mut prog = Program::new("1,1,1,4,99,5,6,0,99");
        prog.run();
        assert_eq!(prog.intcode, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_part_two() {}
}
