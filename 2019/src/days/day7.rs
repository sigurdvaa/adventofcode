use crate::intcode;
use std::fs;

pub fn run() {
    println!("Day 7: Amplification Circuit");
    let file_path = "inputs/day7.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let prog = intcode::parse(&input_raw);
    let input = vec![0, 1, 2, 3, 4];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let prog = intcode::parse("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        let mut input = vec![0, 1, 2, 3, 4];
        let mut output = vec![];
        for _ in 0..5 {
            output = intcode::run(&mut prog.clone(), &mut input);
            if input.len() > 0 {
                input.insert(1, *output.last().unwrap());
            }
        }
        println!("Output: {:?}", output);
        assert_eq!(*output.last().unwrap(), 43210);
    }

    #[test]
    fn test_part_two() {}
}
