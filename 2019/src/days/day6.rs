use std::collections::HashMap;
use std::fs;

fn parse_orbits(input: &str) -> HashMap<String, Vec<String>> {
    let mut orbits = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(")").collect();
        if parts.len() != 2 {
            panic!("Invalid input: {}", line);
        }
        orbits
            .entry(parts[0].to_string())
            .or_insert(Vec::new())
            .push(parts[1].to_string());
    }
    orbits
}

fn count_orbits(orbits: &HashMap<String, Vec<String>>, current: &str, depth: i32) -> i32 {
    let mut count = 0;
    if let Some(objects) = orbits.get(current) {
        for object in objects {
            count += depth + 1;
            count += count_orbits(orbits, object, depth + 1);
        }
    }
    count
}

pub fn run() {
    println!("Day 6: Universal Orbit Map");
    let file_path = "inputs/day6.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let orbits = parse_orbits(&input_raw);
    let count = count_orbits(&orbits, "COM", 0);
    println!("Part One: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input_raw = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        let orbits = parse_orbits(&input_raw);
        assert_eq!(count_orbits(&orbits, "COM", 0), 42);
    }

    #[test]
    fn test_part_two() {
        let input_raw = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
    }
}
