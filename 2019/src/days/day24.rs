use std::fs;

pub fn run() {
    println!("Day 24: Planet of Discord");
    let file_path = "inputs/day24.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

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
