struct InsV1 {
    addr: usize,
    mask_ones: u64,
    mask_zeros: u64,
    value: u64,
}

fn parse_prog_v1(input: &str) -> Vec<InsV1> {
    let mut prog = Vec::new();
    let mut mask_zeros = 0;
    let mut mask_ones = 0;
    for line in input.lines() {
        match line {
            line if line.starts_with("mask = ") => {
                mask_zeros = 0;
                mask_ones = 0;
                for c in line[7..].chars() {
                    mask_zeros <<= 1;
                    mask_ones <<= 1;
                    match c {
                        '0' => mask_zeros |= 1,
                        '1' => mask_ones |= 1,
                        _ => (),
                    }
                }
            }
            line if line.starts_with("mem[") => {
                if let Some(addr_end) = line.find(']') {
                    let addr = line[4..addr_end].parse::<usize>().unwrap();
                    if let Some(value_start) = line.find('=') {
                        let value = line[value_start + 2..].parse::<u64>().unwrap();
                        prog.push(InsV1 {
                            addr,
                            mask_ones,
                            mask_zeros,
                            value,
                        });
                    }
                }
            }
            _ => panic!("invalid initializer: {}", line),
        }
    }
    prog
}

fn sum_after_init_v1(prog: &[InsV1]) -> u64 {
    let mut mem = vec![];
    for ins in prog {
        if ins.addr >= mem.len() {
            mem.resize(ins.addr + 1, 0);
        }
        mem[ins.addr] = ins.value;
        mem[ins.addr] |= ins.mask_ones;
        mem[ins.addr] &= !(ins.mask_zeros);
    }
    mem.iter().sum()
}

struct InsV2 {
    addr: usize,
    mask: usize,
    floats: Vec<usize>,
    value: u64,
}

fn parse_prog_v2(input: &str) -> Vec<InsV2> {
    let mut prog = Vec::new();
    let mut mask = 0;
    let mut floats = Vec::new();
    for line in input.lines() {
        match line {
            line if line.starts_with("mask = ") => {
                floats.clear();
                mask = 0;
                let mask_len = line[7..].len() - 1;
                for (i, c) in line[7..].chars().enumerate() {
                    mask <<= 1;
                    match c {
                        '1' => mask |= 1,
                        'X' => floats.push(mask_len - i),
                        _ => (),
                    }
                }
            }
            line if line.starts_with("mem[") => {
                if let Some(addr_end) = line.find(']') {
                    let addr = line[4..addr_end].parse::<usize>().unwrap();
                    if let Some(value_start) = line.find('=') {
                        let value = line[value_start + 2..].parse::<u64>().unwrap();
                        prog.push(InsV2 {
                            addr,
                            mask,
                            floats: floats.clone(),
                            value,
                        });
                    }
                }
            }
            _ => panic!("invalid initializer: {}", line),
        }
    }
    prog
}

fn sum_after_init_v2(prog: &[InsV2]) -> u64 {
    let mut mem = vec![];
    for ins in prog {
        let mut addrs_curr = vec![ins.addr | ins.mask];
        let mut addrs_next = vec![];

        for i in ins.floats.iter() {
            addrs_next.clear();
            for addr in addrs_curr.iter() {
                addrs_next.push(*addr | (1 << i));
                addrs_next.push(*addr & !(1 << i));
            }
            std::mem::swap(&mut addrs_curr, &mut addrs_next);
        }

        for addr in addrs_curr.iter() {
            mem.push((*addr, ins.value));
        }
    }

    mem.sort_by(|a, b| a.0.cmp(&b.0));
    let mut sum = 0;
    for i in 1..mem.len() {
        if mem[i - 1].0 != mem[i].0 {
            sum += mem[i - 1].1;
        }
    }
    sum += mem[mem.len() - 1].1;
    sum
}

pub fn run() {
    println!("Day 14: Docking Data");
    let input_raw = crate::load_input(module_path!());
    let prog = parse_prog_v1(&input_raw);
    println!("Part One: {}", sum_after_init_v1(&prog));
    let prog = parse_prog_v2(&input_raw);
    println!("Part Two: {}", sum_after_init_v2(&prog));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        const INPUT_TEST: &str = concat!(
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n",
            "mem[8] = 11\n",
            "mem[7] = 101\n",
            "mem[8] = 0",
        );

        let prog = parse_prog_v1(INPUT_TEST);
        assert_eq!(prog[0].addr, 8);
        assert_eq!(prog[0].mask_ones, 64);
        assert_eq!(prog[0].mask_zeros, 2);
        assert_eq!(prog[0].value, 11);
        assert_eq!(sum_after_init_v1(&prog), 165);
    }

    #[test]
    fn test_part_two() {
        const INPUT_TEST: &str = concat!(
            "mask = 000000000000000000000000000000X1001X\n",
            "mem[42] = 100\n",
            "mask = 00000000000000000000000000000000X0XX\n",
            "mem[26] = 1",
        );
        let prog = parse_prog_v2(INPUT_TEST);
        assert_eq!(sum_after_init_v2(&prog), 208);
    }
}
