#[derive(Clone, Debug)]
pub struct Program {
    pub intcode: Vec<i64>,
    orig_intcode: Vec<i64>,
    pub input: Vec<i64>,
    pub output: Vec<i64>,
    ip: usize,
    relative_base: i64,
}

#[derive(Debug, PartialEq)]
pub enum ExitCode {
    Halted,
    Input,
}

impl Program {
    pub fn new(code: &str) -> Program {
        let intcode = Program::parse(code);
        Program {
            intcode: intcode.clone(),
            orig_intcode: intcode,
            input: vec![],
            output: vec![],
            ip: 0,
            relative_base: 0,
        }
    }

    fn parse(code: &str) -> Vec<i64> {
        code.split(',')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect()
    }

    fn get_value(&self, param: i64, mode: i64) -> i64 {
        let addr = match mode {
            0 => param as usize,
            1 => return param,
            _ => (self.relative_base + param) as usize,
        };
        if addr >= self.intcode.len() {
            return 0;
        }
        self.intcode[addr]
    }

    fn set_value(&mut self, param: i64, mode: i64, value: i64) {
        let addr = match mode {
            2 => (self.relative_base + param) as usize,
            _ => param as usize,
        };
        if addr >= self.intcode.len() {
            self.intcode.resize(addr + 1, 0);
        }
        self.intcode[addr] = value;
    }

    pub fn reset(&mut self) {
        self.intcode = self.orig_intcode.clone();
        self.ip = 0;
        self.relative_base = 0;
        self.output.clear();
        self.input.clear();
    }

    pub fn run(&mut self) -> ExitCode {
        loop {
            let opcode = self.intcode[self.ip] % 100;
            let a_mode = (self.intcode[self.ip] / 100) % 10;
            let b_mode = (self.intcode[self.ip] / 1000) % 10;
            let c_mode = (self.intcode[self.ip] / 10000) % 10;

            match opcode {
                1..=2 => {
                    // add or multiply
                    let a = self.intcode[self.ip + 1];
                    let b = self.intcode[self.ip + 2];
                    let c = self.intcode[self.ip + 3];

                    let a_value = self.get_value(a, a_mode);
                    let b_value = self.get_value(b, b_mode);

                    match opcode {
                        1 => self.set_value(c, c_mode, a_value + b_value),
                        _ => self.set_value(c, c_mode, a_value * b_value),
                    }
                    self.ip += 4;
                }
                3 => {
                    // input
                    if self.input.is_empty() {
                        return ExitCode::Input;
                    }
                    let a = self.intcode[self.ip + 1];
                    if let Some(n) = self.input.pop() {
                        self.set_value(a, a_mode, n);
                    }
                    self.ip += 2;
                }
                4 => {
                    // output
                    let a = self.intcode[self.ip + 1];
                    let a_value = self.get_value(a, a_mode);
                    self.output.push(a_value);
                    self.ip += 2;
                }
                5..=6 => {
                    // jump if true or false
                    let a = self.intcode[self.ip + 1];
                    let b = self.intcode[self.ip + 2];

                    let a_value = self.get_value(a, a_mode);
                    let b_value = self.get_value(b, b_mode);

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

                    let a_value = self.get_value(a, a_mode);
                    let b_value = self.get_value(b, b_mode);

                    match opcode {
                        7 => self.set_value(c, c_mode, if a_value < b_value { 1 } else { 0 }),
                        _ => self.set_value(c, c_mode, if a_value == b_value { 1 } else { 0 }),
                    }
                    self.ip += 4;
                }
                9 => {
                    let a = self.intcode[self.ip + 1];
                    let a_value = self.get_value(a, a_mode);
                    self.relative_base += a_value;
                    self.ip += 2;
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
