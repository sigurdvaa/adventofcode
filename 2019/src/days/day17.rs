use crate::intcode::Program;
use std::fs;

fn sum_alignment_parameters(map: &str) -> usize {
    let mut sum = 0;
    let lines = map
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect::<Vec<Vec<char>>>();
    for y in 1..lines.len() - 1 {
        for x in 1..lines[y].len() - 1 {
            let units = vec![
                lines[y][x],
                lines[y][x + 1],
                lines[y][x - 1],
                lines[y - 1][x],
                lines[y + 1][x],
            ];
            if units.iter().all(|&c| c == '#') {
                sum += x * y;
            }
        }
    }
    sum
}

pub fn run() {
    println!("Day 17: Set and Forget");
    let file_path = "inputs/day17.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let prog = Program::new(&input_raw);

    let mut prog1 = prog.clone();
    prog1.run();

    let map_str = prog1
        .output
        .iter()
        .map(|&x| x as u8 as char)
        .collect::<String>();
    println!("Part One: {}", sum_alignment_parameters(&map_str));

    for l in map_str.lines() {
        println!("{}", l);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let map_str = "..#..........\n..#..........\n#######...###\n\
        #.#...#...#.#\n#############\n..#...#...#..\n..#####...^..";
        assert_eq!(sum_alignment_parameters(map_str), 76);
    }

    #[test]
    fn test_part_two() {}
}
