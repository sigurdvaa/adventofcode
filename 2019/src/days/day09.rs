use crate::intcode::Program;
use std::fs;

pub fn run() {
    println!("Day 9: Sensor Boost");
    let file_path = "inputs/day09.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let mut prog = Program::new(&input_raw);
    prog.input.push(1);
    prog.run();
    println!("Part One: {}", prog.output.last().unwrap());

    let mut prog = Program::new(&input_raw);
    prog.input.push(2);
    prog.run();
    println!("Part Two: {}", prog.output.last().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // relative_base
        let mut prog = Program::new("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        prog.run();
        assert_eq!(
            prog.output,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );

        // large number
        let mut prog = Program::new("1102,34915192,34915192,7,4,7,99,0");
        prog.run();
        assert_eq!(prog.output, vec![1219070632396864]);

        let mut prog = Program::new("104,1125899906842624,99");
        prog.run();
        assert_eq!(prog.output, vec![1125899906842624]);

        let mut prog = Program::new("109,-2,203,8,99,0,0");
        prog.input.push(69);
        prog.run();
        assert_eq!(prog.intcode, vec![109, -2, 203, 8, 99, 0, 69]);
    }

    #[test]
    fn test_part_two() {}
}
