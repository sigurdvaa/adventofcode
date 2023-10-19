use crate::intcode::Program;
use std::fs;

fn walk_hull_damage(mut prog: Program) -> i64 {
    let springscript = concat!(
        "NOT A J\n",
        "NOT B T\n",
        "OR T J\n",
        "NOT C T\n",
        "OR T J\n",
        "AND D J\n",
        "WALK\n",
    );
    let mut springscript = springscript.chars().map(|c| c as i64).collect::<Vec<_>>();
    springscript.reverse();
    prog.input.extend(springscript);
    let _exitcode = prog.run();
    *prog.output.last().unwrap()
}

fn run_hull_damage(mut prog: Program) -> i64 {
    let springscript = concat!(
        "NOT A J\n",
        "NOT B T\n",
        "OR T J\n",
        "NOT C T\n",
        "OR T J\n",
        "AND D J\n",
        "AND H J\n",
        "RUN\n",
    );
    let mut springscript = springscript.chars().map(|c| c as i64).collect::<Vec<_>>();
    springscript.reverse();
    prog.input.extend(springscript);
    let _exitcode = prog.run();
    println!(
        "{}",
        prog.output
            .iter()
            .map(|n| *n as u8 as char)
            .collect::<String>()
    );
    *prog.output.last().unwrap()
}

pub fn run() {
    println!("Day 21: Springdroid Adventure");
    let file_path = "inputs/day21.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let prog = Program::new(&input_raw);
    println!("Part One: {}", walk_hull_damage(prog.clone()));
    println!("Part Two: {}", run_hull_damage(prog.clone()));
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
