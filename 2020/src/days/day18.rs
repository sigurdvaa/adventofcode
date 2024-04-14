enum Op {
    Add,
    Mul,
    Int(usize),
}

impl Op {
    fn pri(&self) -> u32 {
        match self {
            Self::Add(_) => 10,
            Self::Mul(_) => 0,
        }
    }
}

fn evaluate_expression(exp: &mut std::str::Chars, advanced: bool) -> usize {
    let mut stack = vec![];
    let mut op = Op::Add as fn(usize) -> Op;
    let mut int_literal = String::new();

    while let Some(c) = exp.next() {
        match c {
            ')' => break,
            '(' => stack.push(op(evaluate_expression(exp, advanced))),
            '+' => op = Op::Add as fn(usize) -> Op,
            '*' => op = Op::Mul as fn(usize) -> Op,
            ' ' => {
                if let Ok(value) = int_literal.parse::<usize>() {
                    stack.push(op(value));
                    int_literal.clear();
                }
            }
            _ => int_literal.push(c),
        }
    }

    if let Ok(value) = int_literal.parse::<usize>() {
        stack.push(op(value));
    }

    let mut sum = 0;
    if advanced {
        let mut prev = stack.first().unwrap();
        for op in stack.iter().skip(1) {
            let pri = if op.pri() > prev.pri() {
                dbg!(op);
                op
            } else {
                dbg!(prev);
                let temp = prev;
                prev = &op;
                temp
            };
            match pri {
                Op::Add(value) => sum += value,
                Op::Mul(value) => sum *= value,
            }
        }
    } else {
        for op in stack {
            match op {
                Op::Add(value) => sum += value,
                Op::Mul(value) => sum *= value,
            }
        }
    }
    dbg!(sum);
    sum
}

fn sum_expressions(input: &str, advanced: bool) -> usize {
    input
        .lines()
        .map(|line| evaluate_expression(&mut line.chars(), advanced))
        .sum()
}
pub fn run() {
    let input_raw = crate::load_input(module_path!());
    println!("Day 18: Operation Order");
    println!("Part One: {}", sum_expressions(&input_raw, false));
    println!("Part Two: {}", sum_expressions(&input_raw, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST1: &str = "1 + 2 * 3 + 4 * 5 + 6";
    const INPUT_TEST2: &str = "1 + (2 * 3) + (4 * (5 + 6))";
    const INPUT_TEST3: &str = "2 * 3 + (4 * 5)";
    const INPUT_TEST4: &str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    const INPUT_TEST5: &str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    const INPUT_TEST6: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn test_part_one() {
        assert_eq!(sum_expressions(INPUT_TEST1, false), 71);
        assert_eq!(sum_expressions(INPUT_TEST2, false), 51);
        assert_eq!(sum_expressions(INPUT_TEST3, false), 26);
        assert_eq!(sum_expressions(INPUT_TEST4, false), 437);
        assert_eq!(sum_expressions(INPUT_TEST5, false), 12240);
        assert_eq!(sum_expressions(INPUT_TEST6, false), 13632);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(sum_expressions(INPUT_TEST1, true), 231);
        assert_eq!(sum_expressions(INPUT_TEST2, true), 51);
        assert_eq!(sum_expressions(INPUT_TEST3, true), 46);
        assert_eq!(sum_expressions(INPUT_TEST4, true), 1445);
        assert_eq!(sum_expressions(INPUT_TEST5, true), 669060);
        assert_eq!(sum_expressions(INPUT_TEST6, true), 23340);
    }
}
