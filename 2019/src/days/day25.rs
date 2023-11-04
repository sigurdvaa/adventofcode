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
    let output_str = output.iter().map(|&i| i as u8 as char).collect::<String>();
    output.clear();
    let mut lines = output_str.lines();
    let mut name = String::new();
    let mut desc = String::new();
    let mut doors = vec![];
    let mut items = vec![];
    let mut msg = String::new();

    while let Some(line) = lines.next() {
        if line.starts_with("==") {
            name = line.into();
            if let Some(d) = lines.next() {
                desc = d.into();
            }
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

        msg = line.into();
    }

    Location {
        name,
        desc,
        doors,
        items,
        msg,
    }
}

fn find_airlock_password(prog: &Program) -> Option<String> {
    let mut queue = VecDeque::from([(prog.clone(), vec![])]);
    let mut seen = HashSet::new();
    let skip_items = vec![
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

        if !seen.insert((curr_loc.clone(), curr_items.clone())) {
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
                let mut next_items = curr_items.clone();
                next_items.push(item.to_string());
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
                .iter()
                .map(|&i| i as u8 as char)
                .collect::<String>()
        );
        prog.output.clear();

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
    //control_droid(prog);
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
