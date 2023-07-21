use crate::intcode;
use std::fs;

fn permutations(items: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut perms = vec![vec![]];
    for _ in 0..items.len() {
        let mut next_perms = vec![];
        for perm in perms {
            for item in items {
                if !perm.contains(item) {
                    let mut next_perm = perm.clone();
                    next_perm.push(*item);
                    next_perms.push(next_perm);
                }
            }
        }
        perms = next_perms
    }
    perms
}

fn run_amp(prog: &Vec<i32>, mut input: &mut Vec<i32>) -> i32 {
    let mut output = vec![];
    for _ in 0..5 {
        output = intcode::run(&mut prog.clone(), &mut input);
        if input.len() > 0 {
            input.insert(input.len() - 1, *output.last().unwrap());
        }
    }
    *output.last().unwrap()
}

fn max_thrust_signal(prog: &Vec<i32>, input: &Vec<i32>) -> i32 {
    let mut max = 0;
    for mut perm in permutations(input) {
        perm.insert(perm.len() - 1, 0);
        let thrust_signal = run_amp(prog, &mut perm);
        if thrust_signal > max {
            max = thrust_signal;
        }
    }
    max
}

pub fn run() {
    println!("Day 7: Amplification Circuit");
    let file_path = "inputs/day7.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let prog = intcode::parse(&input_raw);
    let input = (0..=5).collect();
    println!("Part One: {}", max_thrust_signal(&prog, &input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let prog = intcode::parse("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        let mut input = vec![0, 1, 2, 3, 0, 4];
        assert_eq!(run_amp(&prog, &mut input), 43210);

        let prog = intcode::parse(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        );
        let mut input = vec![4, 3, 2, 1, 0, 0];
        assert_eq!(run_amp(&prog, &mut input), 54321);

        let prog = intcode::parse(
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,\
            1,33,31,31,1,32,31,31,4,31,99,0,0,0",
        );
        let mut input = vec![2, 3, 4, 0, 0, 1];
        assert_eq!(run_amp(&prog, &mut input), 65210);
    }

    #[test]
    fn test_part_two() {}
}
