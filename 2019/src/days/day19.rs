use crate::intcode::Program;
use std::collections::{HashSet, VecDeque};
use std::fs;

fn tractor_beam_affected_points(prog: &Program, x_max: i64, y_max: i64) -> usize {
    let mut affected = 0;
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
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
                if next_x < x_max && next_y < y_max {
                    queue.push_back((next_x, next_y));
                }
            }
        }
    }
    affected
}

fn find_santas_ship(prog: &Program) -> Option<(i64, i64)> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));

    'main: while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        let mut curr = prog.clone();
        curr.input.push(y);
        curr.input.push(x);
        curr.run();
        let res = curr.output.pop().unwrap();

        for i in 0..3 {
            let (next_x, next_y) = match i {
                0 => (x + 1, y),
                1 => (x, y + 1),
                2 => (x + 1, y + 1),
                _ => unreachable!(),
            };
            if seen.insert((next_x, next_y)) {
                queue.push_back((next_x, next_y));
            }
        }

        if res == 1 {
            for x2 in x + 1..x + 100 {
                let mut curr = prog.clone();
                curr.input.push(y);
                curr.input.push(x2);
                curr.run();
                let res = curr.output.pop().unwrap();
                if res == 0 {
                    continue 'main;
                }
            }
            for y2 in y + 1..y + 100 {
                let mut curr = prog.clone();
                curr.input.push(y2);
                curr.input.push(x);
                curr.run();
                let res = curr.output.pop().unwrap();
                if res == 0 {
                    continue 'main;
                }
            }
            return Some((x, y));
        }
    }
    None
}

pub fn run() {
    println!("Day 19: Tractor Beam");
    let file_path = "inputs/day19.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let prog = Program::new(&input_raw);
    println!("Part One: {}", tractor_beam_affected_points(&prog, 50, 50));

    let prog = Program::new(&input_raw);
    let pos = find_santas_ship(&prog).unwrap();
    println!("Part Two: {}", pos.0 * 10_000 + pos.1);
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
