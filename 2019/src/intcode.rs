#[derive(Clone, Debug)]
pub struct Program {
    pub intcode: Vec<i32>,
    pub input: Vec<i32>,
    pub output: Vec<i32>,
    pub ip: usize,
}

pub enum ExitCode {
    Halted,
    Input,
}

impl Program {
    pub fn new(code: &str) -> Program {
        Program {
            intcode: Program::parse(code),
            input: vec![],
            output: vec![],
            ip: 0,
        }
    }

    fn parse(code: &str) -> Vec<i32> {
        code.split(",")
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect()
    }

    pub fn run(&mut self) -> ExitCode {
        loop {
            let opcode = self.intcode[self.ip] % 100;
            let a_mode = (self.intcode[self.ip] / 100) % 10;
            let b_mode = (self.intcode[self.ip] / 1000) % 10;

            match opcode {
                1..=2 => {
                    // add or multiply
                    let a = self.intcode[self.ip + 1];
                    let b = self.intcode[self.ip + 2];
                    let c = self.intcode[self.ip + 3];

                    let a_value = if a_mode == 0 {
                        self.intcode[a as usize]
                    } else {
                        a
                    };
                    let b_value = if b_mode == 0 {
                        self.intcode[b as usize]
                    } else {
                        b
                    };

                    match opcode {
                        1 => self.intcode[c as usize] = a_value + b_value,
                        2 => self.intcode[c as usize] = a_value * b_value,
                        _ => unreachable!(),
                    }
                    self.ip += 4;
                }
                3 => {
                    // input
                    if self.input.len() == 0 {
                        return ExitCode::Input;
                    }
                    let a = self.intcode[self.ip + 1] as usize;
                    if let Some(n) = self.input.pop() {
                        self.intcode[a] = n;
                    }
                    self.ip += 2;
                }
                4 => {
                    // output
                    let a = self.intcode[self.ip + 1];
                    let a_value = if a_mode == 0 {
                        self.intcode[a as usize]
                    } else {
                        a
                    };
                    self.output.push(a_value);
                    self.ip += 2;
                }
                5..=6 => {
                    // jump if true or false
                    let a = self.intcode[self.ip + 1];
                    let b = self.intcode[self.ip + 2];

                    let a_value = if a_mode == 0 {
                        self.intcode[a as usize]
                    } else {
                        a
                    };
                    let b_value = if b_mode == 0 {
                        self.intcode[b as usize]
                    } else {
                        b
                    };

                    self.ip = match opcode {
                        5 if a_value != 0 => b_value as usize,
                        6 if a_value == 0 => b_value as usize,
                        _ => self.ip + 3,
                    }
                }
                7..=8 => {
                    // less than or equal
                    let a = self.intcode[self.ip + 1];
                    let b = self.intcode[self.ip + 2];
                    let c = self.intcode[self.ip + 3];

                    let a_value = if a_mode == 0 {
                        self.intcode[a as usize]
                    } else {
                        a
                    };
                    let b_value = if b_mode == 0 {
                        self.intcode[b as usize]
                    } else {
                        b
                    };

                    match opcode {
                        7 => self.intcode[c as usize] = if a_value < b_value { 1 } else { 0 },
                        8 => self.intcode[c as usize] = if a_value == b_value { 1 } else { 0 },
                        _ => unreachable!(),
                    }
                    self.ip += 4;
                }
                99 => return ExitCode::Halted,
                _ => {
                    let opcode = self.intcode[self.ip];
                    let ip = self.ip;
                    panic!("invalid opcode '{opcode}' at instruction pointer '{ip}'");
                }
            }
        }
    }
}
