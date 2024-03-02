use std::collections::HashMap;
use std::fs;

fn parse_groups(input: &str) -> Vec<Vec<String>> {
    let mut groups = vec![];
    let mut group = vec![];
    for line in input.lines() {
        if line.is_empty() {
            groups.push(group);
            group = vec![];
        } else {
            group.push(line.to_string());
        }
    }
    groups.push(group);
    groups
}

fn sum_group_any_yes(input: &str) -> usize {
    let groups = parse_groups(input);
    groups
        .iter()
        .map(|g| {
            let mut f = g.join("").chars().collect::<Vec<char>>();
            f.sort();
            f.dedup();
            f.len()
        })
        .sum()
}

fn sum_group_all_yes(input: &str) -> usize {
    let groups = parse_groups(input);
    groups
        .iter()
        .map(|g| {
            let map = g.join("").chars().fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            });
            map.iter().filter(|&(_k, v)| *v == g.len()).count()
        })
        .sum()
}

pub fn run() {
    println!("Day 6: Custom Customs");
    let file_path = "inputs/day06.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    println!("Part One: {}", sum_group_any_yes(&input_raw));
    println!("Part Two: {}", sum_group_all_yes(&input_raw));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = concat!(
        "abc\n", "\n", "a\n", "b\n", "c\n", "\n", "ab\n", "ac\n", "\n", "a\n", "a\n", "a\n", "a\n",
        "\n", "b",
    );

    #[test]
    fn test_part_one() {
        assert_eq!(sum_group_any_yes(INPUT_TEST), 11);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(sum_group_all_yes(INPUT_TEST), 6);
    }
}
