use std::fs;

fn parse_numbers(input: &str) -> Vec<usize> {
    let mut numbers = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<_>>();
    numbers.sort();
    numbers
}

fn get_diffs(input: &str) -> Vec<usize> {
    let numbers = parse_numbers(input);
    let mut diffs = vec![3];
    let mut prev = 0;
    for n in numbers {
        let diff = n - prev;
        if diff > 3 {
            break;
        }
        diffs.push(diff);
        prev = n;
    }
    diffs
}

fn chain_jolt_diff(diffs: &[usize]) -> usize {
    let one = diffs.iter().filter(|&n| *n == 1).count();
    let three = diffs.iter().filter(|&n| *n == 3).count();
    one * three
}

fn permutation(diffs: &[usize]) -> usize {
    if diffs.is_empty() {
        return 1;
    }

    if diffs.len() > 2 && diffs.iter().take(3).sum::<usize>() == 3 {
        return 4 * permutation(&diffs[2..]);
    }

    if diffs.len() > 1 && diffs.iter().take(2).sum::<usize>() == 2 {
        return 2 * permutation(&diffs[2..]);
    }

    permutation(&diffs[1..])
}

fn chain_permutation(diffs: &[usize]) -> usize {
    let len = diffs.len();
    let mut perms = 1;
    let mut i = 0;

    perms = permutation(diffs);

    perms
}

pub fn run() {
    println!("Day 10: Adapter Array");
    let file_path = "inputs/day10.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    let diffs = get_diffs(&input_raw);
    println!("Part One: {}", chain_jolt_diff(&diffs));
    println!("Part Two: {}", chain_permutation(&diffs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST1: &str =
        concat!("16\n", "10\n", "15\n", "5\n", "1\n", "11\n", "7\n", "19\n", "6\n", "12\n", "4",);

    const INPUT_TEST2: &str = concat!(
        "28\n", "33\n", "18\n", "42\n", "31\n", "14\n", "46\n", "20\n", "48\n", "47\n", "24\n",
        "23\n", "49\n", "45\n", "19\n", "38\n", "39\n", "11\n", "1\n", "32\n", "25\n", "35\n",
        "8\n", "17\n", "7\n", "9\n", "4\n", "2\n", "34\n", "10\n", "3",
    );

    #[test]
    fn test_part_one() {
        let diffs = get_diffs(INPUT_TEST1);
        assert_eq!(chain_jolt_diff(&diffs), 7 * 5);

        let diffs = get_diffs(INPUT_TEST2);
        assert_eq!(chain_jolt_diff(&diffs), 22 * 10);
    }

    #[test]
    fn test_part_two() {
        let diffs = get_diffs(INPUT_TEST1);
        assert_eq!(chain_permutation(&diffs), 8);

        let diffs = get_diffs(INPUT_TEST2);
        assert_eq!(chain_permutation(&diffs), 19208);
    }
}
