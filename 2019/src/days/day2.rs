// The Tyranny of the Rocket Equation
use std::fs;

fn run_intcode(intcode: &mut Vec<u32>) -> () {
    let mut ip = 0;
    loop {
        match intcode[ip] {
            1 => {
                let a = intcode[ip + 1] as usize;
                let b = intcode[ip + 2] as usize;
                let c = intcode[ip + 3] as usize;
                intcode[c] = intcode[a] + intcode[b];
            }
            2 => {
                let a = intcode[ip + 1] as usize;
                let b = intcode[ip + 2] as usize;
                let c = intcode[ip + 3] as usize;
                intcode[c] = intcode[a] * intcode[b];
            }
            99 => break,
            _ => {
                let opcode = intcode[ip];
                panic!("invalid opcode '{opcode}' at ip '{ip}'");
            }
        }
        ip += 4;
    }
}

pub fn run() {
    let file_path = "inputs/day2.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let mut intcode: Vec<u32> = input_raw
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    intcode[1] = 12;
    intcode[2] = 2;
    run_intcode(&mut intcode);
    println!("Part One: {}", intcode[0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut intcode1 = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        run_intcode(&mut intcode1);
        assert_eq!(intcode1, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);

        let mut intcode2 = vec![1, 0, 0, 0, 99];
        run_intcode(&mut intcode2);
        assert_eq!(intcode2, vec![2, 0, 0, 0, 99]);

        let mut intcode3 = vec![2, 3, 0, 3, 99];
        run_intcode(&mut intcode3);
        assert_eq!(intcode3, vec![2, 3, 0, 6, 99]);

        let mut intcode4 = vec![2, 4, 4, 5, 99, 0];
        run_intcode(&mut intcode4);
        assert_eq!(intcode4, vec![2, 4, 4, 5, 99, 9801]);

        let mut intcode5 = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run_intcode(&mut intcode5);
        assert_eq!(intcode5, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_part_two() {}
}
