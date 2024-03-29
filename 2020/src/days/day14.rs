use std::fs;

#[derive(Clone, Debug, PartialEq)]
enum Bit {
    Zero,
    One,
    Nop,
}

struct Init {
    mask: Vec<Bit>,
    addr: usize,
    value: u64,
}

fn parse_prog(input: &str) -> Vec<Init> {
    let mut inits = Vec::new();
    let mut mask: Option<Vec<Bit>> = None;
    for line in input.lines() {
        match line {
            line if line.starts_with("mask = ") => {
                mask = Some(
                    line[7..]
                        .chars()
                        .map(|c| match c {
                            '1' => Bit::One,
                            '0' => Bit::Zero,
                            _ => Bit::Nop,
                        })
                        .collect(),
                );
            }
            line if line.starts_with("mem[") && mask.is_some() => {
                let chars = line.chars().collect::<Vec<_>>();
                if let Some(addr_end) = chars.iter().position(|c| *c == ']') {
                    let addr = chars[4..addr_end]
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
                    if let Some(value_start) = chars.iter().position(|c| *c == '=') {
                        let value = chars[value_start + 2..]
                            .iter()
                            .collect::<String>()
                            .parse::<u64>()
                            .unwrap();
                        inits.push(Init {
                            mask: mask.clone().unwrap(),
                            addr,
                            value,
                        });
                    }
                }
            }
            _ => panic!("Invalid initializer: {}", line),
        }
    }
    inits
}

fn sum_mem_after_init(inits: &[Init]) -> u64 {
    let mut mem = vec![];

    for init in inits {
        let mut value = init.value;
        let mask_len = init.mask.len() - 1;
        for (i, bit) in init.mask.iter().enumerate() {
            match bit {
                Bit::One => value |= 1 << (mask_len - i),
                Bit::Zero => value &= !(1 << (mask_len - i)),
                Bit::Nop => (),
            }
        }

        if init.addr >= mem.len() {
            mem.resize(init.addr + 1, 0);
        }
        mem[init.addr] = value;
    }

    mem.iter().sum()
}

pub fn run() {
    println!("Day 14: Docking Data");
    let file_path = "inputs/day14.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    let init_prog = parse_prog(&input_raw);
    println!("Part One: {}", sum_mem_after_init(&init_prog));
    println!("Part Two: {}", "TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = concat!(
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n",
        "mem[8] = 11\n",
        "mem[7] = 101\n",
        "mem[8] = 0",
    );

    #[test]
    fn test_part_one() {
        let init_prog = parse_prog(INPUT_TEST);
        assert_eq!(
            init_prog[0].mask.iter().rev().take(2).collect::<Vec<_>>(),
            &[&Bit::Nop, &Bit::Zero]
        );
        assert_eq!(init_prog[0].addr, 8);
        assert_eq!(init_prog[0].value, 11);
        assert_eq!(sum_mem_after_init(&init_prog), 165);
    }

    #[test]
    fn test_part_two() {}
}
