use crate::intcode::{ExitCode, Program};
use std::fs;

enum Command {
    Movement(String),
    Take(String),
    Drop(String),
    Inventory,
}

impl Command {
    fn value(&self) -> String {
        match self {
            Command::Movement(s) => format!("{s}\n"),
            Command::Take(s) => format!("take {s}\n"),
            Command::Drop(s) => format!("drop {s}\n"),
            Command::Inventory => "inv\n".into(),
        }
    }
}

fn find_password(mut prog: Program) -> Option<String> {
    // explore to pick up all items that doesn't kill you and go to checkpoint
    // bfs dropping items to enter next root
    let input = vec![
        Command::Movement("north".into()).value(),
        Command::Take("photons".into()).value(),
        Command::Inventory.value(),
    ];
    for i in input {
        prog.input.extend(i.chars().map(|c| c as i64).rev());
        let exitcode = prog.run();

        if exitcode == ExitCode::Halted {
            return Some("died".into());
        }

        println!(
            "{exitcode:?}\n{}",
            prog.output
                .iter()
                .map(|&i| i as u8 as char)
                .collect::<String>()
        );
        prog.output.clear();
    }
    None
}

pub fn run() {
    println!("Day 25: Cryostasis");
    let file_path = "inputs/day25.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let prog = Program::new(&input_raw);
    println!("Part One: {}", find_password(prog.clone()).unwrap());
    println!("Part Two: {}", "TODO");
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
