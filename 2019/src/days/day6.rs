use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_orbits(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut orbits = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(")").collect();
        if parts.len() != 2 {
            panic!("Invalid input: {}", line);
        }
        orbits.entry(parts[0]).or_insert(Vec::new()).push(parts[1]);
    }
    orbits
}

fn parse_transfers(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut orbits = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(")").collect();
        if parts.len() != 2 {
            panic!("Invalid input: {}", line);
        }
        orbits.entry(parts[0]).or_insert(Vec::new()).push(parts[1]);
        orbits.entry(parts[1]).or_insert(Vec::new()).push(parts[0]);
    }
    orbits
}

fn count_orbits(orbits: &HashMap<&str, Vec<&str>>, current: &str, depth: i32) -> i32 {
    let mut count = 0;
    if let Some(objects) = orbits.get(current) {
        for object in objects {
            count += depth + 1;
            count += count_orbits(orbits, object, depth + 1);
        }
    }
    count
}

fn count_transfers_between(
    orbits: &HashMap<&str, Vec<&str>>,
    seen: &mut HashSet<String>,
    current: &str,
    target: &str,
    depth: i32,
) -> i32 {
    if current == target {
        return depth - 2;
    }
    let mut transfers = 0;
    if let Some(objects) = orbits.get(current) {
        for object in objects {
            if seen.contains(&object.to_string()) {
                continue;
            }
            seen.insert(object.to_string());
            transfers += count_transfers_between(orbits, seen, object, target, depth + 1);
        }
    }
    transfers
}

pub fn run() {
    println!("Day 6: Universal Orbit Map");
    let file_path = "inputs/day6.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let orbits = parse_orbits(&input_raw);
    let count = count_orbits(&orbits, "COM", 0);
    println!("Part One: {}", count);
    let transfers = parse_transfers(&input_raw);
    let count = count_transfers_between(&transfers, &mut HashSet::new(), "YOU", "SAN", 0);
    println!("Part Two: {}", count);
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
        let transfers = parse_transfers(&input_raw);
        assert_eq!(
            count_transfers_between(&transfers, &mut HashSet::new(), "YOU", "SAN", 0),
            4
        );
    }
}
