use crate::intcode::Program;
use std::fs;

pub fn run() {
    println!("Day 5: Sunny with a Chance of Asteroids");
    let file_path = "inputs/day5.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let prog = Program::new(&input_raw);
    let input = vec![5, 1];

    let mut prog1 = prog.clone();
    prog1.input = input.clone();
    prog1.run();
    println!("Part One: {}", prog1.output.last().unwrap());

    let mut prog2 = prog.clone();
    prog2.input = prog1.input.clone();
    prog2.run();
    println!("Part Two: {}", prog2.output.last().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut prog = Program::new("1002,4,3,4,33");
        prog.run();
        assert_eq!(prog.intcode, vec![1002, 4, 3, 4, 99]);

        let mut prog = Program::new("3,2,0");
        prog.input.push(99);
        prog.run();
        assert_eq!(prog.intcode, vec![3, 2, 99]);

        let mut prog = Program::new("4,2,99");
        prog.run();
        assert_eq!(prog.output, vec![99]);
    }

    #[test]
    fn test_part_two() {
        let mut prog = Program::new("3,9,8,9,10,9,4,9,99,-1,8");
        let mut progc = prog.clone();
        progc.input.push(8);
        progc.run();
        assert_eq!(progc.output, vec![1]);
        prog.input.push(69);
        prog.run();
        assert_eq!(prog.output, vec![0]);

        let mut prog = Program::new("3,9,7,9,10,9,4,9,99,-1,8");
        let mut progc = prog.clone();
        progc.input.push(3);
        progc.run();
        assert_eq!(progc.output, vec![1]);
        prog.input.push(8);
        prog.run();
        assert_eq!(prog.output, vec![0]);

        let mut prog = Program::new("3,3,1108,-1,8,3,4,3,99");
        let mut progc = prog.clone();
        progc.input.push(8);
        progc.run();
        assert_eq!(progc.output, vec![1]);
        prog.input.push(69);
        prog.run();
        assert_eq!(prog.output, vec![0]);

        let mut prog = Program::new("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        let mut progc = prog.clone();
        progc.input.push(69);
        progc.run();
        assert_eq!(progc.output, vec![1]);
        prog.input.push(0);
        prog.run();
        assert_eq!(prog.output, vec![0]);

        let mut prog = Program::new("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        let mut progc = prog.clone();
        progc.input.push(69);
        progc.run();
        assert_eq!(progc.output, vec![1]);
        prog.input.push(0);
        prog.run();
        assert_eq!(prog.output, vec![0]);

        let mut prog = Program::new(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,\
            125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        );
        let mut progc = prog.clone();
        progc.input.push(0);
        progc.run();
        assert_eq!(progc.output, vec![999]);
        let mut progc = prog.clone();
        progc.input.push(8);
        progc.run();
        assert_eq!(progc.output, vec![1000]);
        prog.input.push(69);
        prog.run();
        assert_eq!(prog.output, vec![1001]);
    }
}
