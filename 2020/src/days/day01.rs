use std::fs;

fn combinations(items: &[usize], size: usize, comb: Vec<usize>, combs: &mut Vec<Vec<usize>>) {
    if items.is_empty() {
        return;
    }
    for (i, n) in items.iter().enumerate() {
        let next_comb = comb.iter().chain([n]).cloned().collect::<Vec<_>>();
        if next_comb.len() == size {
            combs.push(next_comb);
        } else {
            combinations(&items[i + 1..], size, next_comb, combs);
        }
    }
}

fn get_combinations(items: &[usize], size: usize) -> Vec<Vec<usize>> {
    let mut combs = vec![];
    combinations(&items, size, vec![], &mut combs);
    combs
}

fn product_of_entries_eq_sum(numbers: &[usize], size: usize, sum: usize) -> Option<usize> {
    for comb in get_combinations(numbers, size) {
        if comb.iter().sum::<usize>() == sum {
            return Some(comb.iter().product());
        }
    }
    None
}

fn product_of_entries_eq_sum2(numbers: &[usize], size: usize, sum: usize) -> Option<Vec<usize>> {
    if size == 1 {
        if numbers.contains(&sum) {
            return Some(vec![sum]);
        }
        return None;
    }

    for (i, n) in numbers.iter().enumerate() {
        if *n > sum {
            continue;
        }
        if let Some(combo) = product_of_entries_eq_sum2(&numbers[i + 1..], size - 1, sum - n) {
            return Some(combo.iter().chain([n]).cloned().collect());
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
        .map(|line| line.parse::<usize>().expect("only integers"))
        .collect::<Vec<_>>();
    println!(
        "Part One: {:?}",
        product_of_entries_eq_sum(&numbers, 2, 2020).unwrap()
    );
    println!(
        "Part Two: {:?}",
        product_of_entries_eq_sum2(&numbers, 3, 2020)
            .unwrap()
            .iter()
            .product::<usize>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_TEST: [usize; 6] = [1721, 979, 366, 299, 675, 1456];

    #[test]
    fn test_part_one() {
        assert_eq!(
            product_of_entries_eq_sum(&INPUT_TEST, 2, 2020).unwrap(),
            514579
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            product_of_entries_eq_sum2(&INPUT_TEST, 3, 2020)
                .unwrap()
                .iter()
                .product::<usize>(),
            241861950
        );
    }
}
