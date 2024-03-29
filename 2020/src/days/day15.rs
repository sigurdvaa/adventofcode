use std::fs;

fn parse_numbers(input: &str) -> Vec<usize> {
    input
        .lines()
        .flat_map(|line| {
            line.split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn run() {
    println!("Day 15: Rambunctious Recitation");
    let file_path = "inputs/day15.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    let numbers = parse_numbers(&input_raw);
    println!("Part One: {}", "TODO");
    println!("Part Two: {}", "TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
