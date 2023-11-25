use std::fs;

fn combinations(items: &[usize], size: usize, comb: Vec<usize>, combs: &mut Vec<Vec<usize>>) {
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
    combinations(items, size, vec![], &mut combs);
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

fn entries_eq_sum(numbers: &[usize], size: usize, sum: usize) -> Option<Vec<usize>> {
    if size == 1 {
        return match numbers.contains(&sum) {
            true => Some(vec![sum]),
            false => None,
        };
    }

    for (i, n) in numbers.iter().enumerate() {
        if *n < sum {
            if let Some(combo) = entries_eq_sum(&numbers[i + 1..], size - 1, sum - n) {
                return Some(combo.iter().chain([n]).cloned().collect());
            }
        }
    }

    None
}

pub fn run() {
    println!("Day 1: Report Repair");
    let file_path = "inputs/day01.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    let numbers = input_raw
        .lines()
        .map(|line| line.parse::<usize>().expect("only positive integers"))
        .collect::<Vec<_>>();

    // two solutions to the same problem

    // solve by finding all combos
    println!(
        "Part One: {:?}",
        product_of_entries_eq_sum(&numbers, 2, 2020).unwrap()
    );

    // solve by recursivly checking if sum - n exists
    println!(
        "Part Two: {:?}",
        entries_eq_sum(&numbers, 3, 2020)
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
            entries_eq_sum(&INPUT_TEST, 3, 2020)
                .unwrap()
                .iter()
                .product::<usize>(),
            241861950
        );
    }
}
