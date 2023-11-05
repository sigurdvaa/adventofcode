use std::fs;

fn product_of_entries_eq_sum(numbers: &Vec<usize>, sum: usize) -> Option<usize> {
    for (i, n1) in numbers.iter().enumerate() {
        for n2 in numbers.iter().skip(i + 1) {
            if n1 + n2 == sum {
                return Some(n1 * n2);
            }
        }
    }
    None
}

pub fn run() {
    println!("Day 1: Report Repair");
    let file_path = "inputs/day01.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let numbers = input_raw
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    println!(
        "Part One: {}",
        product_of_entries_eq_sum(&numbers, 2020).unwrap()
    );
    println!("Part Two: {}", "TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input_test = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(
            product_of_entries_eq_sum(&input_test, 2020).unwrap(),
            514579
        );
    }

    #[test]
    fn test_part_two() {}
}
