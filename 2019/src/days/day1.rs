// The Tyranny of the Rocket Equation
use std::fs;

fn fuel_required(mass: u32) -> u32 {
    (mass / 3) - 2
}

pub fn run() {
    let file_path = "inputs/day1.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());
    let sum_fuel: u32 = input_raw
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .map(|x| fuel_required(x))
        .sum();
    println!("Part One: {}", sum_fuel);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100756), 33583);
    }
}
