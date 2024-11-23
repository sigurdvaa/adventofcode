fn parse_public_keys(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().expect("valid int")).collect()
}

fn transform_subject_number(subject: usize, loop_size: usize) -> usize {
    let mut transform = 1;
    for _ in 0..loop_size {
        transform *= subject;
        transform %= 20201227;
    }
    transform
}

pub fn run() {
    let input_raw = crate::load_input(module_path!());
    let pub_keys = parse_public_keys(&input_raw);
    println!("Day 25: Combo Breaker");
    println!("Part One: {}", "TODO");
    println!("Part Two: {}", "TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = concat!("5764801\n", "17807724",);

    #[test]
    fn test_part_one() {
        let pub_keys = parse_public_keys(INPUT_TEST);

        // transform to public key
        assert_eq!(transform_subject_number(7, 8), pub_keys[0]);
        assert_eq!(transform_subject_number(7, 11), pub_keys[1]);

        // transform to encryption key
        assert_eq!(transform_subject_number(pub_keys[0], 11), 14897079);
        assert_eq!(transform_subject_number(pub_keys[1], 8), 14897079);
    }

    #[test]
    fn test_part_two() {}
}
