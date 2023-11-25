use std::fs;

fn fuel_required(mass: u32) -> u32 {
    (mass / 3) - 2
}

fn fuel_required_recur(mass: i32) -> i32 {
    let fuel = (mass / 3) - 2;
    if fuel < 0 {
        return 0;
    }
    fuel + fuel_required_recur(fuel)
}

pub fn run() {
    println!("Day 1: The Tyranny of the Rocket Equation");
    let file_path = "inputs/day01.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    let sum_fuel: u32 = input_raw
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .map(fuel_required)
        .sum();
    println!("Part One: {}", sum_fuel);

    let sum_fuel_recur: i32 = input_raw
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .map(fuel_required_recur)
        .sum();
    println!("Part Two: {}", sum_fuel_recur);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100756), 33583);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(fuel_required_recur(14), 2);
        assert_eq!(fuel_required_recur(1969), 966);
        assert_eq!(fuel_required_recur(100756), 50346);
    }
}
