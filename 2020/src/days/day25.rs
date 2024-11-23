fn find_loop_size(public_keys: &[usize]) -> (usize, usize) {
    let subject = 7;
    let mut loop_size = 1;
    let mut transforms = vec![1; public_keys.len()];
    loop {
        for (i, transform) in transforms.iter_mut().enumerate() {
            *transform *= subject;
            *transform %= 20201227;
            if *transform == public_keys[i] {
                return (i, loop_size);
            }
        }
        loop_size += 1;
    }
}

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
    let (i, loop_size) = find_loop_size(&pub_keys);
    println!("Day 25: Combo Breaker");
    println!(
        "Part One: {}",
        transform_subject_number(pub_keys[(i + 1) % pub_keys.len()], loop_size)
    );
    println!("Part Two: n/a");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = concat!("5764801\n", "17807724",);

    #[test]
    fn test_part_one() {
        let pub_keys = parse_public_keys(INPUT_TEST);
        let enc_key = 14897079;

        // transform to public key
        assert_eq!(transform_subject_number(7, 8), pub_keys[0]);
        assert_eq!(transform_subject_number(7, 11), pub_keys[1]);

        // transform to encryption key
        assert_eq!(transform_subject_number(pub_keys[0], 11), enc_key);
        assert_eq!(transform_subject_number(pub_keys[1], 8), enc_key);

        // find loop_size and enc key
        let (i, loop_size) = find_loop_size(&pub_keys);
        assert_eq!(
            transform_subject_number(pub_keys[(i + 1) % pub_keys.len()], loop_size),
            enc_key
        );
    }

    #[test]
    fn test_part_two() {}
}
