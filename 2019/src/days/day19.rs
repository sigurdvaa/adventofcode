use crate::intcode::Program;
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

fn tractor_beam_points_affected(mut prog: Program, x_max: i64, y_max: i64) -> usize {
    let mut affected = 0;
    for y in 0..y_max {
        for x in 0..x_max {
            if point_affected(&mut prog, x, y) {
                affected += 1;
            }
        }
    }
    affected
}

fn find_santas_ship(mut prog: Program) -> (i64, i64) {
    let mut x = 0;
    let mut y = 99;

    loop {
        // track outside edge
        y += 1;
        while !point_affected(&mut prog, x + 1, y) {
            x += 1;
        }

        // check if beam is wide enough
        if point_affected(&mut prog, x + 100, y)
            && point_affected(&mut prog, x + 1, y - 99)
            && point_affected(&mut prog, x + 100, y - 99)
        {
            return (x + 1, y - 99);
        }
    }
}

pub fn run() {
    println!("Day 19: Tractor Beam");
    let file_path = "inputs/day19.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Error reading file '{file_path}'"));

    let prog = Program::new(&input_raw);

    println!(
        "Part One: {}",
        tractor_beam_points_affected(prog.clone(), 50, 50)
    );

    let pos = find_santas_ship(prog);
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
