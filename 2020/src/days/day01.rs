use std::fs;

fn product_of_entries_eq_sum(numbers: &[usize], sum: usize) -> Option<usize> {
    for (i, n1) in numbers.iter().enumerate() {
        for n2 in numbers.iter().skip(i + 1) {
            if n1 + n2 == sum {
                return Some(n1 * n2);
            }
        }
    }
    None
}

fn product_of_entries_eq_sum2(numbers: &[usize], size: usize, sum: usize) -> Option<usize> {
    // combinations of size
    // for each comb, do sum and return product if sum eq sum
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
    println!(
        "Part Two: {}",
        product_of_entries_eq_sum2(&numbers, 3, 2020).unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_TEST: [usize; 6] = [1721, 979, 366, 299, 675, 1456];

    #[test]
    fn test_part_one() {
        assert_eq!(
            product_of_entries_eq_sum(&INPUT_TEST, 2020).unwrap(),
            514579
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            product_of_entries_eq_sum2(&INPUT_TEST, 3, 2020).unwrap(),
            241861950
        );
    }
}
