use crate::intcode::{ExitCode, Program};
use std::collections::{HashSet, VecDeque};
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Location {
    name: String,
    doors: Vec<String>,
    items: Vec<String>,
}

fn parse_droid_output(output: &mut Vec<i64>) -> Location {
    let output_str = output.iter().map(|&i| i as u8 as char).collect::<String>();
    output.clear();
    let mut lines = output_str.lines();
    let mut name = String::new();
    let mut doors = vec![];
    let mut items = vec![];

    while let Some(line) = lines.next() {
        if line.starts_with("==") {
            name = line.into();
            continue;
        }

        if line.starts_with("Doors here lead:") {
            while let Some(door) = lines.next() {
                if !door.starts_with("- ") {
                    break;
                }
                doors.push(door[2..].into());
            }
        }

        if line.starts_with("Items here:") {
            while let Some(item) = lines.next() {
                if !item.starts_with("- ") {
                    break;
                }
                items.push(item[2..].into());
            }
        }
    }

    Location { name, doors, items }
}

fn find_airlock_password(prog: &Program) -> Option<String> {
    let mut queue = VecDeque::from([prog.clone()]);
    let mut seen = HashSet::new();

    while let Some(mut curr) = queue.pop_front() {
        let exitcode = curr.run();

        if exitcode == ExitCode::Halted {
            continue;
        }

        let curr_loc = parse_droid_output(&mut curr.output);
        if !seen.insert(curr_loc.clone()) {
            continue;
        }

        for door in curr_loc.doors.iter() {
            let mut next_prog = curr.clone();
            next_prog.input.extend(
                Command::Movement(door.into())
                    .value()
                    .chars()
                    .map(|c| c as i64)
                    .rev(),
            );
            queue.push_back(next_prog);
        }
    }
    None
}

pub fn run() {
    println!("Day 25: Cryostasis");
    let file_path = "inputs/day25.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let prog = Program::new(&input_raw);
    println!("Part One: {}", find_airlock_password(&prog).unwrap());
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
