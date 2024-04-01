use crate::intcode::Program;

fn walk_hull_damage(mut prog: Program) -> i64 {
    let springscript = concat!(
        // must jump
        "NOT A J\n",
        "NOT B T\n",
        "OR T J\n",
        "NOT C T\n",
        "OR T J\n",
        // and can land
        "AND D J\n",
        // start
        "WALK\n",
    );
    let mut springscript = springscript.chars().map(|c| c as i64).collect::<Vec<_>>();
    springscript.reverse();
    prog.input = springscript;
    let _exitcode = prog.run();
    *prog.output.last().unwrap()
}

fn run_hull_damage(mut prog: Program) -> i64 {
    let springscript = concat!(
        // must jump
        "NOT A J\n",
        "NOT B T\n",
        "OR T J\n",
        "NOT C T\n",
        "OR T J\n",
        // and can land
        "AND D J\n",
        // and can move after landing
        "OR J T\n",
        "AND E T\n",
        "AND J T\n",
        // or can jump after landing
        "AND H J\n",
        "OR T J\n",
        // start
        "RUN\n",
    );
    let mut springscript = springscript.chars().map(|c| c as i64).collect::<Vec<_>>();
    springscript.reverse();
    prog.input = springscript;
    let _exitcode = prog.run();
    *prog.output.last().unwrap()
}

pub fn run() {
    println!("Day 21: Springdroid Adventure");
    let input_raw = crate::load_input(module_path!());
    let prog = Program::new(&input_raw);
    println!("Part One: {}", walk_hull_damage(prog.clone()));
    println!("Part Two: {}", run_hull_damage(prog));
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
