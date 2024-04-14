enum Op {
    Add(usize),
    Mul(usize),
}

fn evaluate_expression(exp: &mut std::str::Chars) -> usize {
    let mut stack = vec![Op::Add(0)];
    let mut op = Op::Add as fn(usize) -> Op;
    let mut int_literal = String::new();

    while let Some(c) = exp.next() {
        match c {
            ')' => break,
            '(' => stack.push(op(evaluate_expression(exp))),
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
    for op in stack {
        match op {
            Op::Add(value) => sum += value,
            Op::Mul(value) => sum *= value,
        }
    }
    sum
}

fn sum_expressions(input: &str) -> usize {
    input
        .lines()
        .map(|line| evaluate_expression(&mut line.chars()))
        .sum()
}
pub fn run() {
    let input_raw = crate::load_input(module_path!());
    println!("Day 18: Operation Order");
    println!("Part One: {}", sum_expressions(&input_raw));
    println!("Part Two: {}", "TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(sum_expressions("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(sum_expressions("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(sum_expressions("2 * 3 + (4 * 5)"), 26);
        assert_eq!(sum_expressions("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(
            sum_expressions("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            12240
        );
        assert_eq!(
            sum_expressions("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn test_part_two() {}
}
