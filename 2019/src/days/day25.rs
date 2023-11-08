use crate::intcode::{ExitCode, Program};
use std::collections::{HashSet, VecDeque};
use std::fs;

#[allow(dead_code)]
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
    desc: String,
    doors: Vec<String>,
    items: Vec<String>,
    msg: String,
}

fn parse_droid_output(output: &mut Vec<i64>) -> Location {
    let output_string = output
        .drain(..)
        .map(|i| i as u8 as char)
        .collect::<String>();
    let mut lines = output_string.lines();
    let mut name = "";
    let mut desc = "";
    let mut doors = vec![];
    let mut items = vec![];
    let mut msg = "";

    while let Some(line) = lines.next() {
        match line {
            "" => continue,
            line if line.starts_with("==") => {
                name = line;
                if let Some(d) = lines.next() {
                    desc = d;
                }
            }
            line if line.starts_with("Doors here lead:") => {
                while let Some(door) = lines.next() {
                    if !door.starts_with("- ") {
                        break;
                    }
                    doors.push(door[2..].into());
                }
            }
            line if line.starts_with("Items here:") => {
                while let Some(item) = lines.next() {
                    if !item.starts_with("- ") {
                        break;
                    }
                    items.push(item[2..].into());
                }
            }
            _ => msg = line,
        }
    }

    Location {
        name: name.into(),
        desc: desc.into(),
        doors,
        items,
        msg: msg.into(),
    }
}

fn find_airlock_password(prog: &Program) -> Option<String> {
    let mut queue = VecDeque::from([(prog.clone(), vec![])]);
    let mut seen = HashSet::new();
    let skip_items = [
        "photons".to_string(),
        "infinite loop".to_string(),
        "molten lava".to_string(),
        "escape pod".to_string(),
        "spool of cat6".to_string(),
    ];

    while let Some((mut curr_prog, curr_items)) = queue.pop_front() {
        let exitcode = curr_prog.run();
        let curr_loc = parse_droid_output(&mut curr_prog.output);

        if curr_loc.name == "== Pressure-Sensitive Floor ==" {
            if !curr_loc.msg.contains("Alert!") {
                return Some(curr_loc.msg.clone());
            }
        }

        if exitcode == ExitCode::Halted {
            continue;
        }

        if !seen.insert((curr_loc.name, curr_items.clone())) {
            continue;
        }

        for door in curr_loc.doors.iter() {
            let mut prog_next_loc = curr_prog.clone();
            prog_next_loc.input.extend(
                Command::Movement(door.into())
                    .value()
                    .chars()
                    .map(|c| c as i64)
                    .rev(),
            );
            queue.push_back((prog_next_loc.clone(), curr_items.clone()));

            for item in curr_loc.items.iter() {
                if skip_items.contains(item) {
                    continue;
                }

                let mut prog_next_item = prog_next_loc.clone();
                let mut next_items = curr_items.iter().chain([item]).cloned().collect::<Vec<_>>();
                next_items.sort();
                prog_next_item.input.extend(
                    Command::Take(item.into())
                        .value()
                        .chars()
                        .map(|c| c as i64)
                        .rev(),
                );
                queue.push_back((prog_next_item, next_items));
            }
        }
    }
    None
}

#[allow(dead_code)]
fn control_droid(mut prog: Program) {
    use std::io::{self, Write};
    while prog.run() != ExitCode::Halted {
        println!(
            "{}",
            prog.output
                .drain(..)
                .map(|i| i as u8 as char)
                .collect::<String>()
        );

        print!("cmd: ");
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Unable to read command");

        prog.input.extend(buffer.chars().map(|c| c as i64).rev());
    }
    println!(
        "\nHALTED:\n{}",
        prog.output
            .iter()
            .map(|&i| i as u8 as char)
            .collect::<String>()
    );
}

pub fn run() {
    println!("Day 25: Cryostasis");
    let file_path = "inputs/day25.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let prog = Program::new(&input_raw);
    println!("Part One: {}", find_airlock_password(&prog).unwrap());
    println!("Part Two: {}", "n/a");

    // Uncomment to play the game
    // control_droid(prog);
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
