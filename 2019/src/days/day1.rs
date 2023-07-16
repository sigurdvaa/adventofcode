// The Tyranny of the Rocket Equation
use std::fs;

fn fuel_required(mass: u32) -> u32 {
    (mass / 3) - 2
}

pub fn run() {
    let contents = fs::read_to_string("inputs/day1.txt").unwrap();
    println!("{contents}");
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
