pub fn run(intcode: &mut Vec<i32>, input: &mut Vec<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = vec![];
    let mut ip = 0;
    loop {
        let opcode = intcode[ip] % 100;
        let a_mode = (intcode[ip] / 100) % 10;
        let b_mode = (intcode[ip] / 1000) % 10;
        match opcode {
            1 => {
                let a = intcode[ip + 1];
                let b = intcode[ip + 2];
                let c = intcode[ip + 3];

                let a_value = match a_mode {
                    0 => intcode[a as usize],
                    _ => a,
                };
                let b_value = match b_mode {
                    0 => intcode[b as usize],
                    _ => b,
                };

                intcode[c as usize] = a_value + b_value;
                ip += 4;
            }
            2 => {
                let a = intcode[ip + 1];
                let b = intcode[ip + 2];
                let c = intcode[ip + 3];

                let a_value = match a_mode {
                    0 => intcode[a as usize],
                    _ => a,
                };
                let b_value = match b_mode {
                    0 => intcode[b as usize],
                    _ => b,
                };

                intcode[c as usize] = a_value * b_value;
                ip += 4;
            }
            3 => {
                let a = intcode[ip + 1] as usize;
                if let Some(n) = input.pop() {
                    intcode[a] = n;
                }
                ip += 2;
            }
            4 => {
                let a = intcode[ip + 1] as usize;
                output.push(intcode[a]);
                ip += 2;
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

pub fn parse(code: &str) -> Vec<i32> {
    let intcode: Vec<i32> = code.split(",").map(|x| x.trim().parse().unwrap()).collect();
    intcode
}
