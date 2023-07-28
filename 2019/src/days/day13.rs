use crate::intcode::{ExitCode, Program};
use std::fs;

fn game_score(mut prog: Program) -> i64 {
    prog.intcode[0] = 2;
    let mut exitcode = prog.run();
    let mut paddle = (0, 0);
    let mut ball = (0, 0);
    for xyv in prog.output.chunks(3) {
        match xyv[2] {
            3 => paddle = (xyv[0], xyv[1]),
            4 => ball = (xyv[0], xyv[1]),
            _ => (),
        }
    }
    prog.output.clear();

    while exitcode == ExitCode::Input {
        for xyv in prog.output.chunks(3) {
            match xyv[2] {
                3 => paddle = (xyv[0], xyv[1]),
                4 => ball = (xyv[0], xyv[1]),
                _ => (),
            }
        }
        prog.output.clear();
        prog.input.push(match paddle.0 {
            n if n < ball.0 => 1,
            n if n > ball.0 => -1,
            _ => 0,
        });
        exitcode = prog.run();
    }

    *prog.output.last().unwrap_or(&0)
}

pub fn run() {
    println!("Day 13: Care Package");
    let file_path = "inputs/day13.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());
    let prog = Program::new(&input_raw);

    let mut prog1 = prog.clone();
    prog1.run();
    println!(
        "Part One: {}",
        prog1.output.chunks(3).filter(|xyv| xyv[2] == 2).count()
    );

    let score = game_score(prog);
    println!("Part Two: {}", score);
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
