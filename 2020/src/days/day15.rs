use std::{collections::HashMap, fs};

fn parse_start_numbers(input: &str) -> Vec<usize> {
    input
        .lines()
        .flat_map(|line| {
            line.split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn number_spoken(nums: &[usize], spoken: usize) -> usize {
    let mut last_spoken = HashMap::new();
    let mut prev = 0;

    for (i, n) in nums.iter().enumerate() {
        last_spoken.insert(*n, (i + 1, i + 1));
        prev = *n;
    }

    for i in (nums.len() + 1)..spoken + 1 {
        let last = last_spoken.entry(prev).or_insert((i, i));
        prev = last.1 - last.0;
        let next = last_spoken.entry(prev).or_insert((i, i));
        *next = (next.1, i);
    }

    prev
}

pub fn run() {
    println!("Day 15: Rambunctious Recitation");
    let file_path = "inputs/day15.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    let start = parse_start_numbers(&input_raw);
    println!("Part One: {}", number_spoken(&start, 2020));
    println!("Part Two: {}", "TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(number_spoken(&[0, 3, 6], 2020), 436);
        assert_eq!(number_spoken(&[1, 3, 2], 2020), 1);
        assert_eq!(number_spoken(&[2, 1, 3], 2020), 10);
        assert_eq!(number_spoken(&[1, 2, 3], 2020), 27);
        assert_eq!(number_spoken(&[2, 3, 1], 2020), 78);
        assert_eq!(number_spoken(&[3, 2, 1], 2020), 438);
        assert_eq!(number_spoken(&[3, 1, 2], 2020), 1836);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(number_spoken(&[0, 3, 6], 30000000), 175594);
        assert_eq!(number_spoken(&[1, 3, 2], 30000000), 2578);
        assert_eq!(number_spoken(&[2, 1, 3], 30000000), 3544142);
        assert_eq!(number_spoken(&[1, 2, 3], 30000000), 261214);
        assert_eq!(number_spoken(&[2, 3, 1], 30000000), 6895259);
        assert_eq!(number_spoken(&[3, 2, 1], 30000000), 18);
        assert_eq!(number_spoken(&[3, 1, 2], 30000000), 362);
    }
}
