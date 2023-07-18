pub fn parse(code: &str) -> Vec<i32> {
    code.split(",")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect()
}

pub fn run(intcode: &mut Vec<i32>, input: &mut Vec<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = vec![];
    let mut ip = 0;
    loop {
        let opcode = intcode[ip] % 100;
        let a_mode = (intcode[ip] / 100) % 10;
        let b_mode = (intcode[ip] / 1000) % 10;

        match opcode {
            1..=2 => {
                // add or multiply
                let a = intcode[ip + 1];
                let b = intcode[ip + 2];
                let c = intcode[ip + 3];

                let a_value = if a_mode == 0 { intcode[a as usize] } else { a };
                let b_value = if b_mode == 0 { intcode[b as usize] } else { b };

                match opcode {
                    1 => intcode[c as usize] = a_value + b_value,
                    2 => intcode[c as usize] = a_value * b_value,
                    _ => unreachable!(),
                }
                ip += 4;
            }
            3 => {
                // input
                let a = intcode[ip + 1] as usize;
                if let Some(n) = input.pop() {
                    intcode[a] = n;
                }
                ip += 2;
            }
            4 => {
                // output
                let a = intcode[ip + 1];
                let a_value = if a_mode == 0 { intcode[a as usize] } else { a };
                output.push(a_value);
                ip += 2;
            }
            5..=6 => {
                // jump if true or false
                let a = intcode[ip + 1];
                let b = intcode[ip + 2];

                let a_value = if a_mode == 0 { intcode[a as usize] } else { a };
                let b_value = if b_mode == 0 { intcode[b as usize] } else { b };

                ip = match opcode {
                    5 if a_value != 0 => b_value as usize,
                    6 if a_value == 0 => b_value as usize,
                    _ => ip + 3,
                }
            }
            7..=8 => {
                // less than or equal
                let a = intcode[ip + 1];
                let b = intcode[ip + 2];
                let c = intcode[ip + 3];

                let a_value = if a_mode == 0 { intcode[a as usize] } else { a };
                let b_value = if b_mode == 0 { intcode[b as usize] } else { b };

                match opcode {
                    7 => intcode[c as usize] = if a_value < b_value { 1 } else { 0 },
                    8 => intcode[c as usize] = if a_value == b_value { 1 } else { 0 },
                    _ => unreachable!(),
                }
                ip += 4;
            }
            99 => break,
            _ => {
                let opcode = intcode[ip];
                panic!("invalid opcode '{opcode}' at ip '{ip}'");
            }
        }
    }
    output
}
