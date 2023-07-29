use crate::intcode::Program;
use std::collections::{HashSet, VecDeque};
use std::fs;

fn fewest_movments(prog: &Program) -> u32 {
    let mut seen = HashSet::from([(0, 0)]);
    let mut queue = VecDeque::from([(prog.clone(), (0, 0), 0)]);
    while queue.len() > 0 {
        let (curr_prog, pos, moves) = queue.pop_front().unwrap();
        for i in 1..5 {
            let new_pos = match i {
                1 => (pos.0, pos.1 - 1),
                2 => (pos.0, pos.1 + 1),
                3 => (pos.0 - 1, pos.1),
                4 => (pos.0 + 1, pos.1),
                _ => unreachable!(),
            };
            if seen.contains(&new_pos) {
                continue;
            }
            seen.insert(new_pos);
            let mut new_prog = curr_prog.clone();
            new_prog.input.push(i);
            new_prog.run();
            match new_prog.output.pop().unwrap() {
                1 => queue.push_back((new_prog, new_pos, moves + 1)),
                2 => return moves + 1,
                _ => {}
            }
        }
    }
    0
}

pub fn run() {
    println!("Day 15: Oxygen System");
    let file_path = "inputs/day15.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());
    let prog = Program::new(&input_raw);
    println!("Part One: {}", fewest_movments(&prog));
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
