use crate::intcode::{ExitCode, Program};
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

fn run_amp(prog: &Program, input: &mut Vec<i32>) -> i32 {
    let mut progs = vec![prog.clone(); input.len()];
    for i in 0..progs.len() {
        if i == 0 {
            progs[i].input.push(0);
        } else {
            let value = progs[i - 1].output.pop().unwrap();
            progs[i].input.push(value);
        }
        progs[i].input.push(input.pop().unwrap());
        progs[i].run();
    }

    *progs.last().unwrap().output.last().unwrap()
}

fn run_amp_feedback(prog: &Program, input: &mut Vec<i32>) -> i32 {
    let mut progs = vec![prog.clone(); input.len()];
    let mut exitcode = ExitCode::Input;
    loop {
        for i in 0..progs.len() {
            // feedback
            let prev = match i {
                0 => progs.len() - 1,
                _ => i - 1,
            };
            let value = progs[prev].output.pop().unwrap_or(0);
            progs[i].input.push(value);

            // settings
            if input.len() > 0 {
                progs[i].input.push(input.pop().unwrap());
            }

            exitcode = progs[i].run();
        }
        match exitcode {
            ExitCode::Halted => break,
            _ => (),
        }
    }

    *progs[progs.len() - 1].output.last().unwrap_or(&0)
}

fn max_thrust_signal(prog: &Program, input: &Vec<i32>, feedback: bool) -> i32 {
    let mut max = 0;
    for mut perm in permutations(input) {
        let thrust_signal = match feedback {
            true => run_amp_feedback(&prog, &mut perm),
            false => run_amp(&prog, &mut perm),
        };
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

    let prog = Program::new(&input_raw);
    let input = (0..5).collect();
    println!("Part One: {}", max_thrust_signal(&prog, &input, false));
    let input = (5..10).collect();
    println!("Part Two: {}", max_thrust_signal(&prog, &input, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let prog = Program::new("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        let mut input = vec![0, 1, 2, 3, 4];
        assert_eq!(run_amp(&prog, &mut input), 43210);

        let prog = Program::new(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        );
        let mut input = vec![4, 3, 2, 1, 0];
        assert_eq!(run_amp(&prog, &mut input), 54321);

        let prog = Program::new(
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,\
            1,33,31,31,1,32,31,31,4,31,99,0,0,0",
        );
        let mut input = vec![2, 3, 4, 0, 1];
        assert_eq!(run_amp(&prog, &mut input), 65210);
    }

    #[test]
    fn test_part_two() {
        let prog = Program::new(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,\
            28,1005,28,6,99,0,0,5",
        );
        let mut input = vec![5, 6, 7, 8, 9];
        assert_eq!(run_amp_feedback(&prog, &mut input), 139629729);

        let prog = Program::new(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,\
            -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,\
            53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
        );
        let mut input = vec![6, 5, 8, 7, 9];
        assert_eq!(run_amp_feedback(&prog, &mut input), 18216);
    }
}
