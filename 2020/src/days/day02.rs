use std::fs;

pub fn run() {
    println!("Day 2: Password Philosophy");
    let file_path = "inputs/day02.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let passwords = input_raw
        .lines()
        .filter(|&line| line != "")
        .map(|line| {
            let mut split = line.split(" ");
            let mut range = split.next().unwrap().split("-");
            let start = range.next().unwrap().parse::<usize>().unwrap();
            let end = range.next().unwrap().parse::<usize>().unwrap();
            let char = split.next().unwrap().chars().next().unwrap();
            let pw = split.next().unwrap();
            (start, end, char, pw)
        })
        .collect::<Vec<_>>();

    println!("Passwords: {:?}", passwords);
    println!("Part One: {}", "TODO");
    println!("Part Two: {}", "TODO");
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
