use crate::intcode::Program;
use std::fs;

#[derive(Clone)]
struct Packet {
    addr: i64,
    x: i64,
    y: i64,
}

fn run_network(prog: Program) -> Option<Packet> {
    let size = 50;
    let target_addr = 255;
    let mut computers = Vec::with_capacity(size);
    let mut packets: Vec<Vec<Packet>> = vec![vec![]; size];
    let mut traffic = true;

    for i in 0..size {
        let mut comp = prog.clone();
        comp.input.push(i as i64);
        let _exitcode = comp.run();
        computers.push(comp);
    }

    while traffic {
        traffic = false;
        for (i, comp) in computers.iter_mut().enumerate() {
            for packet in packets[i].iter().rev() {
                comp.input.push(packet.y);
                comp.input.push(packet.x);
            }
            packets[i].clear();
            if comp.input.is_empty() {
                comp.input.push(-1);
            }

            let _exitcode = comp.run();
            let mut output = comp.output.chunks_exact(3);
            while let Some(&[addr, x, y]) = output.next() {
                traffic = true;
                let packet = Packet { addr, x, y };
                if packet.addr == target_addr {
                    return Some(packet);
                }
                packets[packet.addr as usize].push(packet);
            }
            comp.output.clear();
        }
    }

    None
}

fn run_network_with_nat(prog: Program) -> Option<Packet> {
    let size = 50;
    let nat_addr = 255;
    let mut nat = None;
    let mut computers = Vec::with_capacity(size);
    let mut packets: Vec<Vec<Packet>> = vec![vec![]; size];
    let mut seen_y = std::collections::HashSet::new();

    for i in 0..size {
        let mut comp = prog.clone();
        comp.input.push(i as i64);
        let _exitcode = comp.run();
        computers.push(comp);
    }

    loop {
        let mut idle = true;
        for (i, comp) in computers.iter_mut().enumerate() {
            for packet in packets[i].iter().rev() {
                comp.input.push(packet.y);
                comp.input.push(packet.x);
            }
            packets[i].clear();
            if comp.input.is_empty() {
                comp.input.push(-1);
            }

            let _exitcode = comp.run();
            let mut output = comp.output.chunks_exact(3);
            while let Some(&[addr, x, y]) = output.next() {
                idle = false;
                let packet = Packet { addr, x, y };
                if packet.addr == nat_addr {
                    nat = Some(packet);
                } else {
                    packets[packet.addr as usize].push(packet);
                }
            }
            comp.output.clear();
        }
        if idle {
            match nat.take() {
                Some(packet) => {
                    if !seen_y.insert(packet.y) {
                        return Some(packet);
                    }
                    packets[0].push(packet);
                }
                None => return None,
            }
        }
    }
}

pub fn run() {
    println!("Day 23: Category Six");
    let file_path = "inputs/day23.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let prog = Program::new(&input_raw);
    println!("Part One: {}", run_network(prog.clone()).unwrap().y);
    println!("Part Two: {}", run_network_with_nat(prog).unwrap().y);
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
