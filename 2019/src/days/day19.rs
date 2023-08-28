use crate::intcode::Program;
use std::collections::{HashSet, VecDeque};
use std::fs;

fn tractor_beam_affected_points(prog: &Program, x_max: i64, y_max: i64) -> usize {
    let mut affected = 0;
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    seen.insert((0, 0));
    queue.push_back((0, 0));

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        let mut curr = prog.clone();
        curr.input.push(y);
        curr.input.push(x);
        curr.run();
        let res = curr.output.pop().unwrap();
        if res == 1 {
            affected += 1;
        }
        for i in 0..3 {
            let (next_x, next_y) = match i {
                0 => (x + 1, y),
                1 => (x, y + 1),
                2 => (x + 1, y + 1),
                _ => unreachable!(),
            };
            if seen.insert((next_x, next_y)) {
                if 0 <= next_x && next_x < x_max && 0 <= next_y && next_y < y_max {
                    queue.push_back((next_x, next_y));
                }
            }
        }
    }
    affected
}

pub fn run() {
    println!("Day 19: Tractor Beam");
    let file_path = "inputs/day19.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let prog = Program::new(&input_raw);
    println!("Part One: {}", tractor_beam_affected_points(&prog, 50, 50));
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
