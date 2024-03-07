use std::fs;

#[derive(Clone)]
enum Ins {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Debug, PartialEq)]
enum Acc {
    Loop(i32),
    Done(i32),
}

impl Acc {
    fn get_value(&self) -> i32 {
        match *self {
            Self::Loop(val) => val,
            Self::Done(val) => val,
        }
    }
}

fn run_prog(prog: &[Ins]) -> Acc {
    let mut visited = vec![false; prog.len()];
    let mut ip: i32 = 0;
    let mut acc = 0;
    while ip >= 0 && (ip as usize) < prog.len() {
        if visited[ip as usize] {
            return Acc::Loop(acc);
        }
        visited[ip as usize] = true;
        match prog[ip as usize] {
            Ins::Acc(val) => {
                acc += val;
                ip += 1;
            }
            Ins::Jmp(val) => ip += val,
            Ins::Nop(_) => ip += 1,
        }
    }
    Acc::Done(acc)
}

fn acc_after_repair(prog: &[Ins]) -> Option<i32> {
    for i in 0..prog.len() {
        match run_prog(prog) {
            Acc::Loop(_) => {
                let mut prog2 = prog.to_vec();
                match prog[i] {
                    Ins::Jmp(val) => prog2[i] = Ins::Nop(val),
                    Ins::Nop(val) => prog2[i] = Ins::Jmp(val),
                    _ => (),
                }
                if let Acc::Done(val) = run_prog(&prog2) {
                    return Some(val);
                }
            }
            Acc::Done(val) => return Some(val),
        }
    }
    None
}

fn parse_ins(input: &str) -> Vec<Ins> {
    let mut ins = Vec::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let op = split.next();
        let val = split
            .next()
            .expect("Missing number")
            .parse::<i32>()
            .expect("Invalid number");

        if let Some(valid_ins) = match op {
            Some("acc") => Some(Ins::Acc(val)),
            Some("jmp") => Some(Ins::Jmp(val)),
            Some("nop") => Some(Ins::Nop(val)),
            _ => None,
        } {
            ins.push(valid_ins);
        };
    }
    ins
}

pub fn run() {
    println!("Day 8: Handheld Halting");
    let file_path = "inputs/day08.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    let prog = parse_ins(&input_raw);
    println!("Part One: {}", run_prog(&prog).get_value());
    println!("Part Two: {}", acc_after_repair(&prog).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_TEST: &str = concat!(
        "nop +0\n",
        "acc +1\n",
        "jmp +4\n",
        "acc +3\n",
        "jmp -3\n",
        "acc -99\n",
        "acc +1\n",
        "jmp -4\n",
        "acc +6",
    );

    #[test]
    fn test_part_one() {
        let prog = parse_ins(INPUT_TEST);
        assert_eq!(run_prog(&prog), Acc::Loop(5));
    }

    #[test]
    fn test_part_two() {
        let prog = parse_ins(INPUT_TEST);
        assert_eq!(acc_after_repair(&prog), Some(8));
    }
}
