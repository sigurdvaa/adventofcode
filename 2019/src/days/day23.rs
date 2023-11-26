use crate::intcode::Program;
use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Clone)]
struct Packet {
    addr: i64,
    x: i64,
    y: i64,
}

fn run_network(prog: Program, with_nat: bool) -> Option<Packet> {
    const SIZE: usize = 50;
    const NAT_ADDR: i64 = 255;
    let mut nat = None;
    let mut computers = Vec::with_capacity(SIZE);
    let mut packets: Vec<VecDeque<Packet>> = vec![VecDeque::new(); SIZE];
    let mut seen_y = HashSet::new();

    for i in 0..SIZE {
        let mut comp = prog.clone();
        comp.input.push(i as i64);
        let _exitcode = comp.run();
        computers.push(comp);
    }

    loop {
        let mut idle = true;
        for (i, comp) in computers.iter_mut().enumerate() {
            while let Some(packet) = packets[i].pop_front() {
                comp.input.push(packet.y);
                comp.input.push(packet.x);
            }
            if comp.input.is_empty() {
                comp.input.push(-1);
            }

            let _exitcode = comp.run();
            let mut output = comp.output.chunks_exact(3);
            while let Some(&[addr, x, y]) = output.next() {
                idle = false;
                let packet = Packet { addr, x, y };
                if packet.addr == NAT_ADDR {
                    if !with_nat {
                        return Some(packet);
                    }
                    nat = Some(packet);
                } else {
                    packets[packet.addr as usize].push_back(packet);
                }
            }
            comp.output.clear();
        }
        if idle {
            if !with_nat {
                return None;
            }
            match nat.take() {
                Some(packet) => {
                    if !seen_y.insert(packet.y) {
                        return Some(packet);
                    }
                    packets[0].push_back(packet);
                }
                None => return None,
            }
        }
    }
}

pub fn run() {
    println!("Day 23: Category Six");
    let file_path = "inputs/day23.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Error reading file '{file_path}'"));

    let prog = Program::new(&input_raw);
    println!("Part One: {}", run_network(prog.clone(), false).unwrap().y);
    println!("Part Two: {}", run_network(prog, true).unwrap().y);
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
