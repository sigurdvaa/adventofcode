use crate::intcode::{ExitCode, Program};
use std::collections::HashMap;

fn painted_panels(mut prog: Program, start_color: i64) -> HashMap<(i32, i32), i64> {
    let mut panels: HashMap<(i32, i32), i64> = HashMap::new();
    let mut pos: (i32, i32) = (0, 0);
    let mut dir: i32 = 0;

    prog.input.push(start_color);
    let mut exitcode = prog.run();

    while exitcode == ExitCode::Input {
        let turn = prog.output.pop().unwrap();
        let color = prog.output.pop().unwrap();
        panels.insert(pos, color);
        dir = (dir + if turn == 0 { -1 } else { 1 }).rem_euclid(4);
        match dir {
            0 => pos.1 -= 1,
            1 => pos.0 += 1,
            2 => pos.1 += 1,
            3 => pos.0 -= 1,
            _ => unreachable!(),
        }
        prog.input.push(*panels.get(&pos).unwrap_or(&0));
        exitcode = prog.run();
    }
    panels
}

fn print_panels(panels: &HashMap<(i32, i32), i64>) {
    let max_y = panels.keys().max_by_key(|(_, y)| *y).unwrap().1;
    let max_x = panels.keys().max_by_key(|(x, _)| *x).unwrap().0;
    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            print!(
                "{}",
                if *panels.get(&(x, y)).unwrap_or(&0) == 1 {
                    '#'
                } else {
                    ' '
                }
            );
        }
        println!();
    }
}

pub fn run() {
    println!("Day 11: Space Police");
    let input_raw = crate::load_input(module_path!());
    let prog = Program::new(&input_raw);

    let panels = painted_panels(prog.clone(), 0);
    println!("Part One: {}", panels.len());
    let panels = painted_panels(prog, 1);
    println!("Part Two:");
    print_panels(&panels);
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
