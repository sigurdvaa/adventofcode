enum Token {
    Add,
    Mul,
    Int(usize),
}

impl Token {
    fn order(&self) -> u32 {
        match self {
            Self::Add => 0,
            Self::Mul => 1,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> usize {
        match self {
            Self::Int(value) => *value,
            _ => unreachable!(),
        }
    }
}

fn evaluate_expression(exp: &mut std::str::Chars, advanced: bool) -> usize {
    let mut stack = vec![];
    let mut int_literal = String::new();

    while let Some(c) = exp.next() {
        match c {
            ')' => break,
            '(' => stack.push(Token::Int(evaluate_expression(exp, advanced))),
            '+' => stack.push(Token::Add),
            '*' => stack.push(Token::Mul),
            ' ' => {
                if let Ok(value) = int_literal.parse::<usize>() {
                    stack.push(Token::Int(value));
                    int_literal.clear();
                }
            }
            _ => int_literal.push(c),
        }
    }
    if let Ok(value) = int_literal.parse::<usize>() {
        stack.push(Token::Int(value));
    }

    let mut order = 0;
    while stack.len() > 1 {
        let mut i = 0;
        while i + 2 < stack.len() {
            let lhs = &stack[i];
            let op = &stack[i + 1];
            let rhs = &stack[i + 2];

            if advanced {
                if op.order() == order {
                    stack[i] = match op {
                        Token::Add => Token::Int(lhs.value() + rhs.value()),
                        Token::Mul => Token::Int(lhs.value() * rhs.value()),
                        _ => unreachable!(),
                    };
                    stack.drain(i + 1..i + 3);
                } else {
                    i += 2;
                }
            } else {
                stack[i] = match op {
                    Token::Add => Token::Int(lhs.value() + rhs.value()),
                    Token::Mul => Token::Int(lhs.value() * rhs.value()),
                    _ => unreachable!(),
                };
                stack.drain(i + 1..i + 3);
            }
        }
        order += 1;
    }
    stack.first().unwrap().value()
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
