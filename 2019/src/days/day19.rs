use crate::intcode::Program;
use std::collections::{HashSet, VecDeque};
use std::fs;

fn point_affected(prog: &mut Program, x: i64, y: i64) -> bool {
    prog.reset();
    prog.input.push(y);
    prog.input.push(x);
    prog.run();
    match prog.output.pop() {
        Some(0) => false,
        Some(_) => true,
        None => unreachable!(),
    }
}

fn tractor_beam_affected_points(mut prog: Program, x_max: i64, y_max: i64) -> usize {
    let mut affected = 1;
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((1, 1));
    while let Some((x, y)) = queue.pop_front() {
        if point_affected(&mut prog, x, y) {
            affected += 1;
        }

        for i in 0..3 {
            let (next_x, next_y) = match i {
                0 => (x + 1, y),
                1 => (x, y + 1),
                2 => (x + 1, y + 1),
                _ => unreachable!(),
            };

            let slope = next_y as f32 / next_x as f32;
            if 0.99 < slope && slope < 1.69 {
                if seen.insert((next_x, next_y)) {
                    if next_x < x_max && next_y < y_max {
                        queue.push_back((next_x, next_y));
                    }
                }
            }
        }
    }
    affected
}

fn find_santas_ship(mut prog: Program) -> Option<(i64, i64)> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((1, 1));

    'main: while let Some((x, y)) = queue.pop_front() {
        for i in 0..3 {
            let (next_x, next_y) = match i {
                0 => (x + 1, y),
                1 => (x, y + 1),
                2 => (x + 1, y + 1),
                _ => unreachable!(),
            };
            let slope = next_y as f32 / next_x as f32;
            if 0.99 < slope && slope < 1.69 {
                if seen.insert((next_x, next_y)) {
                    queue.push_back((next_x, next_y));
                }
            }
        }
        if point_affected(&mut prog, x, y) {
            for i in 0..3 {
                let (cx, cy) = match i {
                    0 => (x + 99, y),
                    1 => (x, y + 99),
                    2 => (x + 99, y + 99),
                    _ => unreachable!(),
                };
                if !point_affected(&mut prog, cx, cy) {
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

    println!(
        "Part One: {}",
        tractor_beam_affected_points(prog.clone(), 50, 50)
    );

    let pos = find_santas_ship(prog).unwrap();
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
