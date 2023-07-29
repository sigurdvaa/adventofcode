use crate::intcode::Program;
use std::collections::{HashSet, VecDeque};
use std::fs;

fn walk_map(prog: Program) -> (HashSet<(i32, i32)>, (i32, i32), u32) {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([(prog.clone(), (0, 0), 0)]);
    let mut oxygen_pos = (0, 0);
    let mut oxygen_moves = 0;
    while queue.len() > 0 {
        let (curr_prog, pos, moves) = queue.pop_front().unwrap();
        seen.insert(pos);
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

            let mut new_prog = curr_prog.clone();
            new_prog.input.push(i);
            new_prog.run();
            match new_prog.output.pop().unwrap() {
                1 => {
                    queue.push_back((new_prog, new_pos, moves + 1));
                }
                2 => {
                    queue.push_back((new_prog, new_pos, moves + 1));
                    oxygen_pos = new_pos;
                    oxygen_moves = moves + 1;
                }
                _ => {}
            }
        }
    }
    (seen, oxygen_pos, oxygen_moves)
}

fn minutes_to_fill(map: HashSet<(i32, i32)>, start: (i32, i32)) -> u32 {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([(start, 0)]);
    let mut max_minutes = 0;
    while queue.len() > 0 {
        let (curr_pos, minutes) = queue.pop_front().unwrap();
        seen.insert(curr_pos);
        max_minutes = max_minutes.max(minutes);
        for i in 1..5 {
            let new_pos = match i {
                1 => (curr_pos.0, curr_pos.1 - 1),
                2 => (curr_pos.0, curr_pos.1 + 1),
                3 => (curr_pos.0 - 1, curr_pos.1),
                4 => (curr_pos.0 + 1, curr_pos.1),
                _ => unreachable!(),
            };
            if seen.contains(&new_pos) {
                continue;
            }
            if map.contains(&new_pos) {
                queue.push_back((new_pos, minutes + 1));
            }
        }
    }
    max_minutes
}

pub fn run() {
    println!("Day 15: Oxygen System");
    let file_path = "inputs/day15.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let prog = Program::new(&input_raw);
    let (map, oxygen_pos, oxygen_moves) = walk_map(prog);
    println!("Part One: {}", oxygen_moves);

    let minutes = minutes_to_fill(map, oxygen_pos);
    println!("Part Two: {}", minutes);
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
